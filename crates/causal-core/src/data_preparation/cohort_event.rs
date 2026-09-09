//! Preparation of grouped first-event cohorts for survival and calendar analyses.
//!
//! A cohort enters observation at one calendar time. Its members either have a
//! first event during follow-up or are right-censored at the cohort cutoff. The
//! same validated history produces weighted duration rows for survival methods
//! and calendar flow rows for temporal analyses.

use super::survival::EventStatus;
use super::NonEmptyVec;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CohortId(String);

impl CohortId {
    pub fn new(value: impl Into<String>) -> Result<Self, CohortEventPreparationError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(CohortEventPreparationError::EmptyCohortId);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StratumId(String);

impl StratumId {
    pub fn new(value: impl Into<String>) -> Result<Self, CohortEventPreparationError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(CohortEventPreparationError::EmptyStratumId);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PositiveCount(usize);

impl PositiveCount {
    pub fn new(value: usize) -> Result<Self, CohortEventPreparationError> {
        if value == 0 {
            return Err(CohortEventPreparationError::ZeroEntrants);
        }
        Ok(Self(value))
    }

    pub const fn get(self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CohortEntry {
    id: CohortId,
    stratum: StratumId,
    entry_time: f64,
    observation_end: f64,
    entrants: PositiveCount,
}

impl CohortEntry {
    pub fn new(
        id: CohortId,
        stratum: StratumId,
        entry_time: f64,
        observation_end: f64,
        entrants: PositiveCount,
    ) -> Result<Self, CohortEventPreparationError> {
        if !entry_time.is_finite() {
            return Err(CohortEventPreparationError::NonFiniteTime { value: entry_time });
        }
        if !observation_end.is_finite() {
            return Err(CohortEventPreparationError::NonFiniteTime {
                value: observation_end,
            });
        }
        if observation_end <= entry_time {
            return Err(CohortEventPreparationError::NonPositiveFollowUp {
                cohort: id,
                entry_time,
                observation_end,
            });
        }
        Ok(Self {
            id,
            stratum,
            entry_time,
            observation_end,
            entrants,
        })
    }

    pub fn id(&self) -> &CohortId {
        &self.id
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CohortEvent {
    cohort: CohortId,
    time: f64,
    events: PositiveCount,
}

impl CohortEvent {
    pub fn new(
        cohort: CohortId,
        time: f64,
        events: PositiveCount,
    ) -> Result<Self, CohortEventPreparationError> {
        if !time.is_finite() {
            return Err(CohortEventPreparationError::NonFiniteTime { value: time });
        }
        Ok(Self {
            cohort,
            time,
            events,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CohortEventHistory {
    cohorts: NonEmptyVec<CohortEntry>,
    events: Vec<CohortEvent>,
}

impl CohortEventHistory {
    pub fn new(
        cohorts: NonEmptyVec<CohortEntry>,
        events: Vec<CohortEvent>,
    ) -> Result<Self, CohortEventPreparationError> {
        let mut known = HashMap::new();
        for cohort in cohorts.as_slice() {
            if known.insert(cohort.id.clone(), cohort).is_some() {
                return Err(CohortEventPreparationError::DuplicateCohort {
                    cohort: cohort.id.clone(),
                });
            }
        }

        let mut totals = HashMap::<CohortId, usize>::new();
        for event in &events {
            let cohort = known.get(&event.cohort).ok_or_else(|| {
                CohortEventPreparationError::UnknownCohort {
                    cohort: event.cohort.clone(),
                }
            })?;
            if event.time <= cohort.entry_time || event.time > cohort.observation_end {
                return Err(CohortEventPreparationError::EventOutsideFollowUp {
                    cohort: event.cohort.clone(),
                    event_time: event.time,
                    entry_time: cohort.entry_time,
                    observation_end: cohort.observation_end,
                });
            }
            let total = totals.entry(event.cohort.clone()).or_default();
            *total = total.checked_add(event.events.get()).ok_or_else(|| {
                CohortEventPreparationError::EventCountOverflow {
                    cohort: event.cohort.clone(),
                }
            })?;
            if *total > cohort.entrants.get() {
                return Err(CohortEventPreparationError::MoreEventsThanEntrants {
                    cohort: event.cohort.clone(),
                    entrants: cohort.entrants.get(),
                    events: *total,
                });
            }
        }

        Ok(Self { cohorts, events })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GroupedSurvivalRow {
    pub cohort: CohortId,
    pub stratum: StratumId,
    pub duration: f64,
    pub calendar_time: f64,
    pub status: EventStatus,
    pub frequency: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CalendarFlowRow {
    pub stratum: StratumId,
    pub time: f64,
    pub entrants: usize,
    pub events: usize,
    pub censored: usize,
    pub at_risk_after: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PreparedCohortEventHistory {
    survival_rows: NonEmptyVec<GroupedSurvivalRow>,
    calendar_rows: NonEmptyVec<CalendarFlowRow>,
}

impl PreparedCohortEventHistory {
    pub fn survival_rows(&self) -> &[GroupedSurvivalRow] {
        self.survival_rows.as_slice()
    }

    pub fn calendar_rows(&self) -> &[CalendarFlowRow] {
        self.calendar_rows.as_slice()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CohortEventPreparationError {
    EmptyCohortId,
    EmptyStratumId,
    ZeroEntrants,
    NonFiniteTime {
        value: f64,
    },
    NonPositiveFollowUp {
        cohort: CohortId,
        entry_time: f64,
        observation_end: f64,
    },
    DuplicateCohort {
        cohort: CohortId,
    },
    UnknownCohort {
        cohort: CohortId,
    },
    EventOutsideFollowUp {
        cohort: CohortId,
        event_time: f64,
        entry_time: f64,
        observation_end: f64,
    },
    EventCountOverflow {
        cohort: CohortId,
    },
    MoreEventsThanEntrants {
        cohort: CohortId,
        entrants: usize,
        events: usize,
    },
    CalendarRiskUnderflow {
        stratum: StratumId,
        time: f64,
    },
}

impl Display for CohortEventPreparationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyCohortId => write!(formatter, "cohort id must not be empty"),
            Self::EmptyStratumId => write!(formatter, "stratum id must not be empty"),
            Self::ZeroEntrants => write!(formatter, "a cohort must contain at least one entrant"),
            Self::NonFiniteTime { value } => write!(formatter, "time must be finite, received {value}"),
            Self::NonPositiveFollowUp { cohort, entry_time, observation_end } => write!(formatter, "cohort {} enters at {entry_time}, so its observation end must be later than {entry_time}; received {observation_end}", cohort.as_str()),
            Self::DuplicateCohort { cohort } => write!(formatter, "cohort {} appears more than once", cohort.as_str()),
            Self::UnknownCohort { cohort } => write!(formatter, "an event refers to unknown cohort {}", cohort.as_str()),
            Self::EventOutsideFollowUp { cohort, event_time, entry_time, observation_end } => write!(formatter, "cohort {} has an event at {event_time}, outside its follow-up interval ({entry_time}, {observation_end}]", cohort.as_str()),
            Self::EventCountOverflow { cohort } => write!(formatter, "event counts overflow for cohort {}", cohort.as_str()),
            Self::MoreEventsThanEntrants { cohort, entrants, events } => write!(formatter, "cohort {} has {events} first events but only {entrants} entrants", cohort.as_str()),
            Self::CalendarRiskUnderflow { stratum, time } => write!(formatter, "events and censoring exceed the risk set for stratum {} at time {time}", stratum.as_str()),
        }
    }
}

impl Error for CohortEventPreparationError {}

#[derive(Clone, Copy, Debug, Default)]
struct FlowCounts {
    entrants: usize,
    events: usize,
    censored: usize,
}

pub fn prepare_cohort_events(
    history: CohortEventHistory,
) -> Result<PreparedCohortEventHistory, CohortEventPreparationError> {
    let mut events_by_cohort = BTreeMap::<CohortId, BTreeMap<OrderedTime, usize>>::new();
    for event in history.events {
        let by_time = events_by_cohort.entry(event.cohort).or_default();
        let count = by_time.entry(OrderedTime(event.time)).or_default();
        *count += event.events.get();
    }

    let mut survival_rows = Vec::new();
    let mut flows = BTreeMap::<(StratumId, OrderedTime), FlowCounts>::new();
    for cohort in history.cohorts {
        flows
            .entry((cohort.stratum.clone(), OrderedTime(cohort.entry_time)))
            .or_default()
            .entrants += cohort.entrants.get();

        let mut observed = 0_usize;
        if let Some(events) = events_by_cohort.remove(&cohort.id) {
            for (time, count) in events {
                observed += count;
                survival_rows.push(GroupedSurvivalRow {
                    cohort: cohort.id.clone(),
                    stratum: cohort.stratum.clone(),
                    duration: time.0 - cohort.entry_time,
                    calendar_time: time.0,
                    status: EventStatus::Observed,
                    frequency: count,
                });
                flows
                    .entry((cohort.stratum.clone(), time))
                    .or_default()
                    .events += count;
            }
        }

        let censored = cohort.entrants.get() - observed;
        if censored > 0 {
            survival_rows.push(GroupedSurvivalRow {
                cohort: cohort.id.clone(),
                stratum: cohort.stratum.clone(),
                duration: cohort.observation_end - cohort.entry_time,
                calendar_time: cohort.observation_end,
                status: EventStatus::Censored,
                frequency: censored,
            });
            flows
                .entry((cohort.stratum.clone(), OrderedTime(cohort.observation_end)))
                .or_default()
                .censored += censored;
        }
    }

    let mut risk = HashMap::<StratumId, usize>::new();
    let mut calendar_rows = Vec::with_capacity(flows.len());
    for ((stratum, time), counts) in flows {
        let current = risk.entry(stratum.clone()).or_default();
        *current += counts.entrants;
        let removed = counts.events + counts.censored;
        if removed > *current {
            return Err(CohortEventPreparationError::CalendarRiskUnderflow {
                stratum,
                time: time.0,
            });
        }
        *current -= removed;
        calendar_rows.push(CalendarFlowRow {
            stratum,
            time: time.0,
            entrants: counts.entrants,
            events: counts.events,
            censored: counts.censored,
            at_risk_after: *current,
        });
    }

    Ok(PreparedCohortEventHistory {
        survival_rows: NonEmptyVec::try_from_vec(survival_rows)
            .expect("a non-empty set of cohorts always produces at least one survival row"),
        calendar_rows: NonEmptyVec::try_from_vec(calendar_rows)
            .expect("a non-empty set of cohorts always produces at least one calendar row"),
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct OrderedTime(f64);

impl Eq for OrderedTime {}

impl PartialOrd for OrderedTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}
