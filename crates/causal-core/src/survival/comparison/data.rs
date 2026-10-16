//! Validated two-group right-censored data.

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Event {
    Censored,
    Observed,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Group {
    Zero,
    One,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurvivalSample {
    time: f64,
    event: Event,
    group: Group,
}

impl SurvivalSample {
    pub fn new(time: f64, event: Event, group: Group) -> Result<Self, ComparisonDataError> {
        if !time.is_finite() || time < 0.0 {
            return Err(ComparisonDataError::InvalidTime);
        }
        Ok(Self { time, event, group })
    }

    pub(crate) fn time(self) -> f64 {
        self.time
    }

    pub(crate) fn event(self) -> Event {
        self.event
    }

    pub(crate) fn group(self) -> Group {
        self.group
    }

    pub(crate) fn censored_at(time: f64, group: Group) -> Self {
        debug_assert!(time.is_finite() && time >= 0.0);
        Self {
            time,
            event: Event::Censored,
            group,
        }
    }

    pub(crate) fn with_group(self, group: Group) -> Self {
        Self { group, ..self }
    }

    pub(crate) fn with_event(self, event: Event) -> Self {
        Self { event, ..self }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ComparisonData(Vec<SurvivalSample>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComparisonDataError {
    Empty,
    InvalidTime,
    MissingGroup(Group),
    GroupWithoutEvent(Group),
}

impl ComparisonData {
    pub fn new(samples: Vec<SurvivalSample>) -> Result<Self, ComparisonDataError> {
        if samples.is_empty() {
            return Err(ComparisonDataError::Empty);
        }
        for group in [Group::Zero, Group::One] {
            if !samples.iter().any(|sample| sample.group == group) {
                return Err(ComparisonDataError::MissingGroup(group));
            }
            if !samples
                .iter()
                .any(|sample| sample.group == group && sample.event == Event::Observed)
            {
                return Err(ComparisonDataError::GroupWithoutEvent(group));
            }
        }
        Ok(Self(samples))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub(crate) fn samples(&self) -> &[SurvivalSample] {
        &self.0
    }

    pub(crate) fn group_samples(&self, group: Group) -> Vec<SurvivalSample> {
        self.0
            .iter()
            .copied()
            .filter(|sample| sample.group == group)
            .collect()
    }
}
