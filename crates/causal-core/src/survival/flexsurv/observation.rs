//! Valid time-to-event observation forms accepted by flexsurvreg.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NonNegativeTime(f64);

impl NonNegativeTime {
    pub fn new(value: f64) -> Result<Self, ObservationError> {
        if !value.is_finite() {
            return Err(ObservationError::NonFiniteTime);
        }
        if value < 0.0 {
            return Err(ObservationError::NegativeTime);
        }
        Ok(Self(value))
    }

    pub fn get(self) -> f64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ObservationKind {
    Exact {
        time: NonNegativeTime,
    },
    RightCensored {
        time: NonNegativeTime,
    },
    LeftCensored {
        upper: NonNegativeTime,
    },
    IntervalCensored {
        lower: NonNegativeTime,
        upper: NonNegativeTime,
    },
    DelayedEvent {
        entry: NonNegativeTime,
        time: NonNegativeTime,
    },
    DelayedRightCensored {
        entry: NonNegativeTime,
        exit: NonNegativeTime,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurvivalObservation(ObservationKind);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ObservationError {
    NonFiniteTime,
    NegativeTime,
    IntervalNotOrdered,
    DelayedEntryNotOrdered,
}

impl SurvivalObservation {
    pub fn exact(time: f64) -> Result<Self, ObservationError> {
        Ok(Self(ObservationKind::Exact {
            time: NonNegativeTime::new(time)?,
        }))
    }

    pub fn right_censored(time: f64) -> Result<Self, ObservationError> {
        Ok(Self(ObservationKind::RightCensored {
            time: NonNegativeTime::new(time)?,
        }))
    }

    pub fn left_censored(upper: f64) -> Result<Self, ObservationError> {
        Ok(Self(ObservationKind::LeftCensored {
            upper: NonNegativeTime::new(upper)?,
        }))
    }

    pub fn interval_censored(lower: f64, upper: f64) -> Result<Self, ObservationError> {
        let lower = NonNegativeTime::new(lower)?;
        let upper = NonNegativeTime::new(upper)?;
        if lower.get() >= upper.get() {
            return Err(ObservationError::IntervalNotOrdered);
        }
        Ok(Self(ObservationKind::IntervalCensored { lower, upper }))
    }

    pub fn delayed_event(entry: f64, event_time: f64) -> Result<Self, ObservationError> {
        let entry = NonNegativeTime::new(entry)?;
        let time = NonNegativeTime::new(event_time)?;
        if entry.get() >= time.get() {
            return Err(ObservationError::DelayedEntryNotOrdered);
        }
        Ok(Self(ObservationKind::DelayedEvent { entry, time }))
    }

    pub fn delayed_right_censored(entry: f64, exit: f64) -> Result<Self, ObservationError> {
        let entry = NonNegativeTime::new(entry)?;
        let exit = NonNegativeTime::new(exit)?;
        if entry.get() >= exit.get() {
            return Err(ObservationError::DelayedEntryNotOrdered);
        }
        Ok(Self(ObservationKind::DelayedRightCensored { entry, exit }))
    }

    pub(crate) fn bounds(self) -> ObservationBounds {
        match self.0 {
            ObservationKind::Exact { time } => ObservationBounds {
                entry: 0.0,
                lower: time.get(),
                upper: time.get(),
                exact: true,
            },
            ObservationKind::RightCensored { time } => ObservationBounds {
                entry: 0.0,
                lower: time.get(),
                upper: f64::INFINITY,
                exact: false,
            },
            ObservationKind::LeftCensored { upper } => ObservationBounds {
                entry: 0.0,
                lower: 0.0,
                upper: upper.get(),
                exact: false,
            },
            ObservationKind::IntervalCensored { lower, upper } => ObservationBounds {
                entry: 0.0,
                lower: lower.get(),
                upper: upper.get(),
                exact: false,
            },
            ObservationKind::DelayedEvent { entry, time } => ObservationBounds {
                entry: entry.get(),
                lower: time.get(),
                upper: time.get(),
                exact: true,
            },
            ObservationKind::DelayedRightCensored { entry, exit } => ObservationBounds {
                entry: entry.get(),
                lower: exit.get(),
                upper: f64::INFINITY,
                exact: false,
            },
        }
    }

    pub(crate) fn is_counting(self) -> bool {
        matches!(
            self.0,
            ObservationKind::DelayedEvent { .. } | ObservationKind::DelayedRightCensored { .. }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ObservationBounds {
    pub entry: f64,
    pub lower: f64,
    pub upper: f64,
    pub exact: bool,
}
