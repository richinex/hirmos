//! Source-compatible preparation of multi-state survival rows.
//!
//! `prepare_wide_events` ports `mstate::msprep` 0.3.3. It accepts one record
//! per subject and an acyclic, numbered transition matrix.
//! `prepare_longitudinal_states` ports `msm::msm2Surv` 1.8.2. It accepts
//! ordered exact state observations and an allowed-transition matrix.

use super::NonEmptyVec;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubjectId(String);

impl SubjectId {
    pub fn new(value: impl Into<String>) -> Result<Self, SurvivalPreparationError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(SurvivalPreparationError::EmptySubjectId);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateId(usize);

impl StateId {
    pub fn new(value: usize) -> Result<Self, SurvivalPreparationError> {
        if value == 0 {
            return Err(SurvivalPreparationError::InvalidState { state: value });
        }
        Ok(Self(value))
    }

    pub const fn get(self) -> usize {
        self.0
    }

    fn index(self) -> usize {
        self.0 - 1
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransitionId(usize);

impl TransitionId {
    pub fn new(value: usize) -> Result<Self, SurvivalPreparationError> {
        if value == 0 {
            return Err(SurvivalPreparationError::InvalidTransitionNumber { value });
        }
        Ok(Self(value))
    }

    pub const fn get(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventStatus {
    Censored,
    Observed,
}

impl EventStatus {
    pub const fn indicator(self) -> u8 {
        match self {
            Self::Censored => 0,
            Self::Observed => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StateObservation {
    NotApplicable,
    Recorded { time: f64, status: EventStatus },
}

impl StateObservation {
    pub fn recorded(time: f64, status: EventStatus) -> Result<Self, SurvivalPreparationError> {
        if !time.is_finite() {
            return Err(SurvivalPreparationError::NonFiniteTime { value: time });
        }
        Ok(Self::Recorded { time, status })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransitionMatrix {
    transitions: Vec<Vec<Option<TransitionId>>>,
}

impl TransitionMatrix {
    pub fn from_numbered(
        values: Vec<Vec<Option<usize>>>,
    ) -> Result<Self, SurvivalPreparationError> {
        validate_square(&values)?;

        let state_count = values.len();
        let mut numbers = Vec::new();
        let mut transitions = Vec::with_capacity(state_count);
        for (from, row) in values.into_iter().enumerate() {
            let mut parsed = Vec::with_capacity(state_count);
            for (to, value) in row.into_iter().enumerate() {
                if from == to && value.is_some() {
                    return Err(SurvivalPreparationError::DiagonalTransition { state: from + 1 });
                }
                let transition = value.map(TransitionId::new).transpose()?;
                if let Some(id) = transition {
                    numbers.push(id.get());
                }
                parsed.push(transition);
            }
            transitions.push(parsed);
        }

        numbers.sort_unstable();
        let expected: Vec<usize> = (1..=numbers.len()).collect();
        if numbers != expected {
            return Err(SurvivalPreparationError::NonSequentialTransitions { numbers });
        }

        Ok(Self { transitions })
    }

    pub fn from_allowed(values: Vec<Vec<bool>>) -> Result<Self, SurvivalPreparationError> {
        validate_square(&values)?;

        let mut next = 1;
        let mut transitions = Vec::with_capacity(values.len());
        for (from, row) in values.into_iter().enumerate() {
            let mut numbered = Vec::with_capacity(row.len());
            for (to, allowed) in row.into_iter().enumerate() {
                let transition = if from != to && allowed {
                    let id = TransitionId(next);
                    next += 1;
                    Some(id)
                } else {
                    None
                };
                numbered.push(transition);
            }
            transitions.push(numbered);
        }
        Ok(Self { transitions })
    }

    pub fn state_count(&self) -> usize {
        self.transitions.len()
    }

    pub fn as_numbered(&self) -> Vec<Vec<Option<usize>>> {
        self.transitions
            .iter()
            .map(|row| {
                row.iter()
                    .map(|value| value.map(TransitionId::get))
                    .collect()
            })
            .collect()
    }

    fn transition(&self, from: StateId, to: StateId) -> Option<TransitionId> {
        self.transitions[from.index()][to.index()]
    }

    fn outgoing(&self, from: StateId, active: &BTreeSet<StateId>) -> Vec<StateId> {
        active
            .iter()
            .copied()
            .filter(|to| self.transition(from, *to).is_some())
            .collect()
    }

    fn is_acyclic(&self) -> bool {
        let mut indegree = vec![0_usize; self.state_count()];
        for row in &self.transitions {
            for (to, transition) in row.iter().enumerate() {
                if transition.is_some() {
                    indegree[to] += 1;
                }
            }
        }

        let mut ready: Vec<usize> = indegree
            .iter()
            .enumerate()
            .filter_map(|(state, degree)| (*degree == 0).then_some(state))
            .collect();
        let mut visited = 0;
        while let Some(from) = ready.pop() {
            visited += 1;
            for (to, transition) in self.transitions[from].iter().enumerate() {
                if transition.is_none() {
                    continue;
                }
                indegree[to] -= 1;
                if indegree[to] == 0 {
                    ready.push(to);
                }
            }
        }
        visited == self.state_count()
    }
}

fn validate_square<T>(values: &[Vec<T>]) -> Result<(), SurvivalPreparationError> {
    if values.is_empty() {
        return Err(SurvivalPreparationError::EmptyTransitionMatrix);
    }
    if values.iter().any(|row| row.len() != values.len()) {
        return Err(SurvivalPreparationError::NonSquareTransitionMatrix);
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
pub struct WideSubject {
    id: SubjectId,
    states: NonEmptyVec<StateObservation>,
    entry_state: StateId,
    entry_time: f64,
    covariates: Vec<f64>,
}

impl WideSubject {
    pub fn new(
        id: SubjectId,
        states: NonEmptyVec<StateObservation>,
        entry_state: StateId,
        entry_time: f64,
        covariates: Vec<f64>,
    ) -> Result<Self, SurvivalPreparationError> {
        validate_finite(entry_time, &covariates)?;
        Ok(Self {
            id,
            states,
            entry_state,
            entry_time,
            covariates,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WideEventHistory {
    transitions: TransitionMatrix,
    subjects: NonEmptyVec<WideSubject>,
}

impl WideEventHistory {
    pub fn new(
        transitions: TransitionMatrix,
        subjects: NonEmptyVec<WideSubject>,
    ) -> Result<Self, SurvivalPreparationError> {
        if !transitions.is_acyclic() {
            return Err(SurvivalPreparationError::CircularTransitionMatrix);
        }

        let state_count = transitions.state_count();
        let covariate_count = subjects.first().covariates.len();
        let mut ids = HashSet::new();
        for subject in subjects.as_slice() {
            validate_subject_id_unique(&mut ids, &subject.id)?;
            validate_state(subject.entry_state, state_count)?;
            if subject.states.len() != state_count {
                return Err(SurvivalPreparationError::StateObservationCount {
                    subject: subject.id.clone(),
                    expected: state_count,
                    actual: subject.states.len(),
                });
            }
            if subject.covariates.len() != covariate_count {
                return Err(SurvivalPreparationError::CovariateCount {
                    subject: subject.id.clone(),
                    expected: covariate_count,
                    actual: subject.covariates.len(),
                });
            }
        }

        Ok(Self {
            transitions,
            subjects,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LongitudinalObservation {
    subject: SubjectId,
    time: f64,
    state: StateId,
    covariates: Vec<f64>,
}

impl LongitudinalObservation {
    pub fn new(
        subject: SubjectId,
        time: f64,
        state: StateId,
        covariates: Vec<f64>,
    ) -> Result<Self, SurvivalPreparationError> {
        validate_finite(time, &covariates)?;
        Ok(Self {
            subject,
            time,
            state,
            covariates,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LongitudinalStateHistory {
    transitions: TransitionMatrix,
    observations: NonEmptyVec<LongitudinalObservation>,
}

impl LongitudinalStateHistory {
    pub fn new(
        transitions: TransitionMatrix,
        observations: NonEmptyVec<LongitudinalObservation>,
    ) -> Result<Self, SurvivalPreparationError> {
        let state_count = transitions.state_count();
        let covariate_count = observations.first().covariates.len();
        let mut completed = HashSet::new();
        let mut current_subject = observations.first().subject.clone();
        let mut previous_time = observations.first().time;

        for (index, observation) in observations.as_slice().iter().enumerate() {
            validate_state(observation.state, state_count)?;
            if observation.covariates.len() != covariate_count {
                return Err(SurvivalPreparationError::CovariateCount {
                    subject: observation.subject.clone(),
                    expected: covariate_count,
                    actual: observation.covariates.len(),
                });
            }

            if observation.subject == current_subject {
                if index > 0 && observation.time < previous_time {
                    return Err(SurvivalPreparationError::DecreasingObservationTime {
                        subject: observation.subject.clone(),
                        previous: previous_time,
                        current: observation.time,
                    });
                }
            } else {
                completed.insert(current_subject);
                if completed.contains(&observation.subject) {
                    return Err(SurvivalPreparationError::NonContiguousSubject {
                        subject: observation.subject.clone(),
                    });
                }
                current_subject = observation.subject.clone();
            }
            previous_time = observation.time;
        }

        Ok(Self {
            transitions,
            observations,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PreparedTransitionRow {
    pub subject: SubjectId,
    pub from: StateId,
    pub to: StateId,
    pub transition: TransitionId,
    pub start: f64,
    pub stop: f64,
    pub duration: f64,
    pub status: EventStatus,
    pub covariates: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PreparedMultiStateData {
    rows: NonEmptyVec<PreparedTransitionRow>,
    transitions: TransitionMatrix,
    notices: Vec<SurvivalPreparationNotice>,
}

impl PreparedMultiStateData {
    pub fn rows(&self) -> &[PreparedTransitionRow] {
        self.rows.as_slice()
    }

    pub fn transitions(&self) -> &TransitionMatrix {
        &self.transitions
    }

    pub fn notices(&self) -> &[SurvivalPreparationNotice] {
        &self.notices
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SurvivalPreparationNotice {
    EarlierCensoringBeforeObservedTransition {
        subject: SubjectId,
        from: StateId,
    },
    SimultaneousObservedTransitions {
        subject: SubjectId,
        from: StateId,
        time: f64,
        chosen_to: StateId,
    },
    DuplicateObservationTimesOmitted {
        count: usize,
    },
}

impl Display for SurvivalPreparationNotice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EarlierCensoringBeforeObservedTransition { subject, from } => write!(
                f,
                "From starting state {}, subject {} has smallest transition time with status=0, larger transition time with status=1",
                from.get(),
                subject.as_str()
            ),
            Self::SimultaneousObservedTransitions {
                subject,
                from,
                time,
                chosen_to: _,
            } => write!(
                f,
                "Starting from state {}, simultaneous transitions possible for subjects {} at times {}; smallest receiving state chosen",
                from.get(),
                subject.as_str(),
                format_number(*time)
            ),
            Self::DuplicateObservationTimesOmitted { count } => write!(
                f,
                "Omitting {count} rows with two observations at the same time"
            ),
        }
    }
}

fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SurvivalPreparationError {
    EmptySubjectId,
    EmptyTransitionMatrix,
    NonSquareTransitionMatrix,
    DiagonalTransition {
        state: usize,
    },
    InvalidTransitionNumber {
        value: usize,
    },
    NonSequentialTransitions {
        numbers: Vec<usize>,
    },
    CircularTransitionMatrix,
    InvalidState {
        state: usize,
    },
    StateOutsideMatrix {
        state: usize,
        state_count: usize,
    },
    NonFiniteTime {
        value: f64,
    },
    NonFiniteCovariate {
        index: usize,
        value: f64,
    },
    DuplicateSubject {
        subject: SubjectId,
    },
    StateObservationCount {
        subject: SubjectId,
        expected: usize,
        actual: usize,
    },
    CovariateCount {
        subject: SubjectId,
        expected: usize,
        actual: usize,
    },
    MissingStateObservation {
        subject: SubjectId,
        state: StateId,
    },
    NoFiniteFollowUp {
        subject: SubjectId,
        from: StateId,
    },
    NonContiguousSubject {
        subject: SubjectId,
    },
    DecreasingObservationTime {
        subject: SubjectId,
        previous: f64,
        current: f64,
    },
    DisallowedObservedTransition {
        subject: SubjectId,
        from: StateId,
        to: StateId,
    },
    NoPreparedRows,
}

impl Display for SurvivalPreparationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptySubjectId => write!(f, "subject id must not be empty"),
            Self::EmptyTransitionMatrix => write!(f, "transition matrix must not be empty"),
            Self::NonSquareTransitionMatrix => write!(f, "transition matrix must be square"),
            Self::DiagonalTransition { state } => write!(f, "state {state} cannot transition to itself"),
            Self::InvalidTransitionNumber { value } => write!(f, "transition number must be positive, received {value}"),
            Self::NonSequentialTransitions { numbers } => write!(f, "transition numbers must be exactly 1 through K, received {numbers:?}"),
            Self::CircularTransitionMatrix => write!(f, "wide event preparation requires an acyclic transition matrix"),
            Self::InvalidState { state } => write!(f, "state ids are one-based, received {state}"),
            Self::StateOutsideMatrix { state, state_count } => write!(f, "state {state} is outside the transition matrix with {state_count} states"),
            Self::NonFiniteTime { value } => write!(f, "time must be finite, received {value}"),
            Self::NonFiniteCovariate { index, value } => write!(f, "covariate {index} must be finite, received {value}"),
            Self::DuplicateSubject { subject } => write!(f, "subject {} appears more than once in wide input", subject.as_str()),
            Self::StateObservationCount { subject, expected, actual } => write!(f, "subject {} has {actual} state observations; expected {expected}", subject.as_str()),
            Self::CovariateCount { subject, expected, actual } => write!(f, "subject {} has {actual} covariates; expected {expected}", subject.as_str()),
            Self::MissingStateObservation { subject, state } => write!(f, "subject {} has no time and status for reachable state {}", subject.as_str(), state.get()),
            Self::NoFiniteFollowUp { subject, from } => write!(f, "subject {} has no finite follow-up time from state {}", subject.as_str(), from.get()),
            Self::NonContiguousSubject { subject } => write!(f, "observations for subject {} are not contiguous", subject.as_str()),
            Self::DecreasingObservationTime { subject, previous, current } => write!(f, "times for subject {} decrease from {previous} to {current}", subject.as_str()),
            Self::DisallowedObservedTransition { subject, from, to } => write!(f, "subject {} has an observed transition from state {} to state {}, but that transition is not allowed", subject.as_str(), from.get(), to.get()),
            Self::NoPreparedRows => write!(f, "the input does not produce any transition-risk rows"),
        }
    }
}

impl Error for SurvivalPreparationError {}

fn validate_finite(time: f64, covariates: &[f64]) -> Result<(), SurvivalPreparationError> {
    if !time.is_finite() {
        return Err(SurvivalPreparationError::NonFiniteTime { value: time });
    }
    for (index, value) in covariates.iter().copied().enumerate() {
        if !value.is_finite() {
            return Err(SurvivalPreparationError::NonFiniteCovariate { index, value });
        }
    }
    Ok(())
}

fn validate_subject_id_unique(
    ids: &mut HashSet<SubjectId>,
    subject: &SubjectId,
) -> Result<(), SurvivalPreparationError> {
    if !ids.insert(subject.clone()) {
        return Err(SurvivalPreparationError::DuplicateSubject {
            subject: subject.clone(),
        });
    }
    Ok(())
}

fn validate_state(state: StateId, state_count: usize) -> Result<(), SurvivalPreparationError> {
    if state.get() > state_count {
        return Err(SurvivalPreparationError::StateOutsideMatrix {
            state: state.get(),
            state_count,
        });
    }
    Ok(())
}

#[derive(Clone)]
struct WideProgress {
    subject: WideSubject,
    position: WidePosition,
}

#[derive(Clone, Copy)]
enum WidePosition {
    At { state: StateId, time: f64 },
    Complete,
}

pub fn prepare_wide_events(
    history: WideEventHistory,
) -> Result<PreparedMultiStateData, SurvivalPreparationError> {
    let transitions = history.transitions;
    let mut active: BTreeSet<StateId> = (1..=transitions.state_count()).map(StateId).collect();
    let mut progress: Vec<WideProgress> = history
        .subjects
        .into_iter()
        .map(|subject| WideProgress {
            position: WidePosition::At {
                state: subject.entry_state,
                time: subject.entry_time,
            },
            subject,
        })
        .collect();
    let mut rows = Vec::new();
    let mut notices = Vec::new();

    while !active.is_empty() && !progress.is_empty() {
        let starting_states = starting_states(&transitions, &active);
        for from in &starting_states {
            let outgoing = transitions.outgoing(*from, &active);
            let absorbing: BTreeSet<StateId> = active
                .iter()
                .copied()
                .filter(|state| transitions.outgoing(*state, &active).is_empty())
                .collect();

            for current in progress.iter_mut().filter(
                |item| matches!(item.position, WidePosition::At { state, .. } if state == *from),
            ) {
                if outgoing.is_empty() {
                    continue;
                }
                let WidePosition::At { time: start, .. } = current.position else {
                    unreachable!("completed subjects were filtered out")
                };
                let next = select_wide_transition(
                    &current.subject,
                    start,
                    *from,
                    &outgoing,
                    &mut notices,
                )?;
                let stop = next.time();
                for to in &outgoing {
                    let transition = transitions
                        .transition(*from, *to)
                        .expect("outgoing transition");
                    rows.push(PreparedTransitionRow {
                        subject: current.subject.id.clone(),
                        from: *from,
                        to: *to,
                        transition,
                        start,
                        stop,
                        duration: stop - start,
                        status: if next.observed_destination() == Some(*to) {
                            EventStatus::Observed
                        } else {
                            EventStatus::Censored
                        },
                        covariates: current.subject.covariates.clone(),
                    });
                }

                current.position = match next {
                    SelectedTransition::Observed { to, time } if !absorbing.contains(&to) => {
                        WidePosition::At { state: to, time }
                    }
                    SelectedTransition::Observed { .. } | SelectedTransition::Censored { .. } => {
                        WidePosition::Complete
                    }
                };
            }
        }

        progress.retain(|item| matches!(item.position, WidePosition::At { .. }));
        for state in starting_states {
            active.remove(&state);
        }

        // msprepEngine indexes the remaining time/status columns without
        // `drop = FALSE`. With one subject, R therefore turns the matrices
        // into vectors and the next recursive call returns immediately.
        // Preserve that observable 0.3.3 behavior for source parity.
        if progress.len() == 1 && !active.is_empty() {
            break;
        }
    }

    rows.sort_by(|left, right| {
        left.subject
            .cmp(&right.subject)
            .then_with(|| left.start.total_cmp(&right.start))
            .then(left.from.cmp(&right.from))
            .then(left.to.cmp(&right.to))
    });
    let rows =
        NonEmptyVec::try_from_vec(rows).map_err(|_| SurvivalPreparationError::NoPreparedRows)?;
    Ok(PreparedMultiStateData {
        rows,
        transitions,
        notices,
    })
}

fn starting_states(transitions: &TransitionMatrix, active: &BTreeSet<StateId>) -> Vec<StateId> {
    active
        .iter()
        .copied()
        .filter(|candidate| {
            active
                .iter()
                .all(|from| transitions.transition(*from, *candidate).is_none())
        })
        .collect()
}

enum SelectedTransition {
    Censored { time: f64 },
    Observed { time: f64, to: StateId },
}

impl SelectedTransition {
    fn time(&self) -> f64 {
        match self {
            Self::Censored { time } | Self::Observed { time, .. } => *time,
        }
    }

    fn observed_destination(&self) -> Option<StateId> {
        match self {
            Self::Censored { .. } => None,
            Self::Observed { to, .. } => Some(*to),
        }
    }
}

fn select_wide_transition(
    subject: &WideSubject,
    start: f64,
    from: StateId,
    outgoing: &[StateId],
    notices: &mut Vec<SurvivalPreparationNotice>,
) -> Result<SelectedTransition, SurvivalPreparationError> {
    let mut candidates = Vec::with_capacity(outgoing.len());
    for to in outgoing {
        let observation = subject.states.as_slice()[to.index()];
        let StateObservation::Recorded { time, status } = observation else {
            return Err(SurvivalPreparationError::MissingStateObservation {
                subject: subject.id.clone(),
                state: *to,
            });
        };
        let time = if time < start { f64::INFINITY } else { time };
        candidates.push((*to, time, status));
    }

    let smallest = candidates
        .iter()
        .map(|(_, time, _)| *time)
        .min_by(f64::total_cmp)
        .ok_or(SurvivalPreparationError::NoFiniteFollowUp {
            subject: subject.id.clone(),
            from,
        })?;
    let mut observed: Vec<(StateId, f64)> = candidates
        .iter()
        .filter_map(|(to, time, status)| (*status == EventStatus::Observed).then_some((*to, *time)))
        .collect();
    observed.sort_by(|left, right| left.1.total_cmp(&right.1).then(left.0.cmp(&right.0)));

    if observed.is_empty() {
        if !smallest.is_finite() {
            return Err(SurvivalPreparationError::NoFiniteFollowUp {
                subject: subject.id.clone(),
                from,
            });
        }
        return Ok(SelectedTransition::Censored { time: smallest });
    }

    let next_time = observed[0].1;
    if smallest < next_time {
        notices.push(
            SurvivalPreparationNotice::EarlierCensoringBeforeObservedTransition {
                subject: subject.id.clone(),
                from,
            },
        );
    }
    if observed.len() > 1 && observed[1].1 == next_time {
        notices.push(SurvivalPreparationNotice::SimultaneousObservedTransitions {
            subject: subject.id.clone(),
            from,
            time: next_time,
            chosen_to: observed[0].0,
        });
    }
    Ok(SelectedTransition::Observed {
        time: next_time,
        to: observed[0].0,
    })
}

pub fn prepare_longitudinal_states(
    history: LongitudinalStateHistory,
) -> Result<PreparedMultiStateData, SurvivalPreparationError> {
    let transitions = history.transitions;
    let observations = history.observations.into_vec();
    let mut rows = Vec::new();
    let mut duplicate_times = 0;

    let mut by_subject: BTreeMap<SubjectId, Vec<LongitudinalObservation>> = BTreeMap::new();
    for observation in observations {
        by_subject
            .entry(observation.subject.clone())
            .or_default()
            .push(observation);
    }

    for (subject, observations) in by_subject {
        for pair in observations.windows(2) {
            let current = &pair[0];
            let next = &pair[1];
            if current.time == next.time {
                duplicate_times += 1;
                continue;
            }

            let observed_transition = current.state != next.state;
            if observed_transition && transitions.transition(current.state, next.state).is_none() {
                return Err(SurvivalPreparationError::DisallowedObservedTransition {
                    subject: subject.clone(),
                    from: current.state,
                    to: next.state,
                });
            }

            for to in transitions.outgoing(
                current.state,
                &(1..=transitions.state_count()).map(StateId).collect(),
            ) {
                let transition = transitions
                    .transition(current.state, to)
                    .expect("outgoing transition");
                rows.push(PreparedTransitionRow {
                    subject: subject.clone(),
                    from: current.state,
                    to,
                    transition,
                    start: current.time,
                    stop: next.time,
                    duration: next.time - current.time,
                    status: if observed_transition && to == next.state {
                        EventStatus::Observed
                    } else {
                        EventStatus::Censored
                    },
                    covariates: current.covariates.clone(),
                });
            }
        }
    }

    rows.sort_by(|left, right| {
        left.subject
            .cmp(&right.subject)
            .then_with(|| left.start.total_cmp(&right.start))
            .then(left.to.cmp(&right.to))
    });
    let rows =
        NonEmptyVec::try_from_vec(rows).map_err(|_| SurvivalPreparationError::NoPreparedRows)?;
    let notices = if duplicate_times == 0 {
        Vec::new()
    } else {
        vec![
            SurvivalPreparationNotice::DuplicateObservationTimesOmitted {
                count: duplicate_times,
            },
        ]
    };
    Ok(PreparedMultiStateData {
        rows,
        transitions,
        notices,
    })
}
