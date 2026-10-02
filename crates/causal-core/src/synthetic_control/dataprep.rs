//! Numeric-key panel preparation following Synth 1.1-9 dataprep/spec.pred.func.
//! Matrix axes record actual sorted units/periods, not upstream's potentially
//! misleading labels when identifier/window vectors arrive out of order.
use super::{
    statistics,
    synth::{oracle_product, FixedPredictorFit, SynthError, SynthMatrices},
};
use nalgebra::{DMatrix, DVector};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Summary {
    Mean,
    Median,
    Minimum,
    Maximum,
    Sum,
    Variance,
    StandardDeviation,
}
#[derive(Debug, Clone)]
pub struct PanelRow {
    pub unit: f64,
    pub period: f64,
    pub values: Vec<Option<f64>>,
}
pub struct PanelData {
    pub columns: Vec<String>,
    pub rows: Vec<PanelRow>,
}
#[derive(Debug, Clone)]
pub struct SpecialPredictor {
    pub column: usize,
    pub periods: Vec<f64>,
    pub summary: Summary,
}
#[derive(Debug, Clone)]
pub struct Preparation {
    pub treated: f64,
    pub donors: Vec<f64>,
    pub predictors: Vec<usize>,
    pub summary: Summary,
    pub special: Vec<SpecialPredictor>,
    pub outcome: usize,
    pub predictor_periods: Vec<f64>,
    pub fit_periods: Vec<f64>,
    pub plot_periods: Vec<f64>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Window {
    Predictors,
    Fit,
    Plot,
    Special(usize),
}
#[derive(Debug, PartialEq)]
pub enum PreparationError {
    InvalidKeys,
    InvalidRows,
    InvalidColumns,
    EmptyPredictors,
    InsufficientDonors,
    DuplicateDonor,
    TreatedAmongDonors,
    MissingUnit {
        unit: f64,
    },
    Unbalanced {
        unit: f64,
        period: f64,
        rows: usize,
    },
    InvalidWindow {
        window: Window,
    },
    MissingPeriod {
        window: Window,
        period: f64,
    },
    InvalidColumn {
        column: usize,
    },
    DuplicatePredictor,
    AllMissingPredictor {
        unit: f64,
        predictor: usize,
    },
    UndefinedSummary {
        unit: f64,
        predictor: usize,
    },
    NonFiniteValue {
        unit: f64,
        period: f64,
        column: usize,
    },
    MissingOutcome {
        unit: f64,
        period: f64,
        window: Window,
    },
}
#[derive(Debug, PartialEq)]
pub enum PreparationEvidence {
    SortedDonors {
        requested: Vec<f64>,
        actual: Vec<f64>,
    },
    SortedPeriods {
        window: Window,
        requested: Vec<f64>,
        actual: Vec<f64>,
    },
    ControlPredictorsUseMean {
        requested: Summary,
    },
    OmittedPredictorValues {
        unit: f64,
        predictor: usize,
        periods: Vec<f64>,
    },
}
pub struct PreparedPanel {
    pub treated: f64,
    pub donors: Vec<f64>,
    pub predictor_names: Vec<String>,
    pub fit_periods: Vec<f64>,
    pub plot_periods: Vec<f64>,
    pub x0: DMatrix<f64>,
    pub x1: DVector<f64>,
    pub z0: DMatrix<f64>,
    pub z1: DVector<f64>,
    /// Synth rejects missing fitting outcomes but retains missing donor plot values.
    pub y0_plot: DMatrix<Option<f64>>,
    pub y1_plot: DVector<f64>,
    pub evidence: Vec<PreparationEvidence>,
}

/// Predictor balance and plotted values on the prepared axes. Missing donor
/// plot cells remain missing, as in Synth's matrix multiplication.
pub struct FitSummary {
    pub treated_predictors: Vec<f64>,
    pub synthetic_predictors: Vec<f64>,
    pub donor_mean_predictors: Vec<f64>,
    pub synthetic_path: Vec<Option<f64>>,
    pub gaps: Vec<Option<f64>>,
}
impl PreparedPanel {
    pub fn matrices(&self) -> Result<SynthMatrices, SynthError> {
        SynthMatrices::prepare(&self.x0, &self.x1, &self.z0, &self.z1)
    }

    pub fn summarize_fit(&self, fit: &FixedPredictorFit) -> Result<FitSummary, SynthError> {
        if fit.donor_weights.len() != self.donors.len() {
            return Err(SynthError::DimensionMismatch);
        }
        let weights = DMatrix::from_column_slice(self.donors.len(), 1, &fit.donor_weights);
        let synthetic_predictors = oracle_product(&self.x0, &weights)?.as_slice().to_vec();
        let donor_mean_predictors = (0..self.x0.nrows())
            .map(|r| {
                statistics::mean(
                    &(0..self.x0.ncols())
                        .map(|c| self.x0[(r, c)])
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
        let observed = DMatrix::from_fn(self.y0_plot.nrows(), self.y0_plot.ncols(), |r, c| {
            self.y0_plot[(r, c)].unwrap_or(0.)
        });
        let values = oracle_product(&observed, &weights)?;
        let synthetic_path: Vec<_> = (0..values.nrows())
            .map(|r| {
                if (0..self.y0_plot.ncols()).any(|c| self.y0_plot[(r, c)].is_none()) {
                    None
                } else {
                    Some(values[(r, 0)])
                }
            })
            .collect();
        let gaps = synthetic_path
            .iter()
            .enumerate()
            .map(|(r, v)| v.map(|v| self.y1_plot[r] - v))
            .collect();
        Ok(FitSummary {
            treated_predictors: self.x1.as_slice().to_vec(),
            synthetic_predictors,
            donor_mean_predictors,
            synthetic_path,
            gaps,
        })
    }
}
fn sorted(mut values: Vec<f64>) -> Vec<f64> {
    values.sort_by(f64::total_cmp);
    values
}
fn periods(
    input: &[f64],
    window: Window,
    available: &[f64],
    evidence: &mut Vec<PreparationEvidence>,
) -> Result<Vec<f64>, PreparationError> {
    if input.is_empty() || input.iter().any(|v| !v.is_finite()) {
        return Err(PreparationError::InvalidWindow { window });
    }
    let actual = sorted(input.to_vec());
    if actual.windows(2).any(|v| v[0] == v[1]) {
        return Err(PreparationError::InvalidWindow { window });
    }
    for &period in &actual {
        if !available.contains(&period) {
            return Err(PreparationError::MissingPeriod { window, period });
        }
    }
    if actual != input {
        evidence.push(PreparationEvidence::SortedPeriods {
            window,
            requested: input.to_vec(),
            actual: actual.clone(),
        });
    }
    Ok(actual)
}
fn summarize(values: &[f64], operation: Summary) -> f64 {
    match operation {
        Summary::Mean => statistics::mean(values),
        Summary::Sum => statistics::sum(values),
        Summary::Minimum => values.iter().copied().fold(f64::INFINITY, f64::min),
        Summary::Maximum => values.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        Summary::Median => {
            let values = sorted(values.to_vec());
            let n = values.len();
            if n % 2 == 1 {
                values[n / 2]
            } else {
                statistics::mean(&values[n / 2 - 1..n / 2 + 1])
            }
        }
        Summary::Variance => {
            if values.len() > 1 {
                statistics::variance(values)
            } else {
                f64::NAN
            }
        }
        Summary::StandardDeviation => {
            if values.len() > 1 {
                statistics::variance(values).sqrt()
            } else {
                f64::NAN
            }
        }
    }
}
pub fn prepare(data: &PanelData, request: &Preparation) -> Result<PreparedPanel, PreparationError> {
    if data.columns.is_empty()
        || data
            .columns
            .iter()
            .enumerate()
            .any(|(i, c)| data.columns[..i].contains(c))
    {
        return Err(PreparationError::InvalidColumns);
    }
    if data
        .rows
        .iter()
        .any(|r| r.values.len() != data.columns.len())
    {
        return Err(PreparationError::InvalidRows);
    }
    if !request.treated.is_finite()
        || request.donors.iter().any(|v| !v.is_finite())
        || data
            .rows
            .iter()
            .any(|r| !r.unit.is_finite() || !r.period.is_finite())
    {
        return Err(PreparationError::InvalidKeys);
    }
    if request.donors.len() < 2 {
        return Err(PreparationError::InsufficientDonors);
    }
    let donors = sorted(request.donors.clone());
    if donors.windows(2).any(|v| v[0] == v[1]) {
        return Err(PreparationError::DuplicateDonor);
    }
    if donors.contains(&request.treated) {
        return Err(PreparationError::TreatedAmongDonors);
    }
    let units: Vec<f64> = std::iter::once(request.treated)
        .chain(donors.iter().copied())
        .collect();
    for &unit in &units {
        if !data.rows.iter().any(|r| r.unit == unit) {
            return Err(PreparationError::MissingUnit { unit });
        }
    }
    let mut rows: Vec<&PanelRow> = data
        .rows
        .iter()
        .filter(|r| units.contains(&r.unit))
        .collect();
    rows.sort_by(|a, b| {
        a.unit
            .total_cmp(&b.unit)
            .then(a.period.total_cmp(&b.period))
    });
    let mut available = sorted(rows.iter().map(|r| r.period).collect());
    available.dedup();
    let find = |unit: f64, period: f64| {
        rows.binary_search_by(|r| {
            r.unit
                .partial_cmp(&unit)
                .unwrap()
                .then(r.period.partial_cmp(&period).unwrap())
        })
    };
    for &unit in &units {
        for &period in &available {
            let count = match find(unit, period) {
                Err(_) => 0,
                Ok(index) => {
                    let mut first = index;
                    let mut last = index + 1;
                    while first > 0
                        && rows[first - 1].unit == unit
                        && rows[first - 1].period == period
                    {
                        first -= 1;
                    }
                    while last < rows.len()
                        && rows[last].unit == unit
                        && rows[last].period == period
                    {
                        last += 1;
                    }
                    last - first
                }
            };
            if count != 1 {
                return Err(PreparationError::Unbalanced {
                    unit,
                    period,
                    rows: count,
                });
            }
        }
    }
    let mut evidence = Vec::new();
    if donors != request.donors {
        evidence.push(PreparationEvidence::SortedDonors {
            requested: request.donors.clone(),
            actual: donors.clone(),
        });
    }
    let prior = periods(
        &request.predictor_periods,
        Window::Predictors,
        &available,
        &mut evidence,
    )?;
    let fit = periods(&request.fit_periods, Window::Fit, &available, &mut evidence)?;
    let plot = periods(
        &request.plot_periods,
        Window::Plot,
        &available,
        &mut evidence,
    )?;
    for (i, &c) in request.predictors.iter().enumerate() {
        if request.predictors[..i].contains(&c) {
            return Err(PreparationError::DuplicatePredictor);
        }
    }
    for c in std::iter::once(request.outcome)
        .chain(request.predictors.iter().copied())
        .chain(request.special.iter().map(|s| s.column))
    {
        if c >= data.columns.len() {
            return Err(PreparationError::InvalidColumn { column: c });
        }
    }
    if request.predictors.is_empty() && request.special.is_empty() {
        return Err(PreparationError::EmptyPredictors);
    }
    if !request.predictors.is_empty() && request.summary != Summary::Mean {
        evidence.push(PreparationEvidence::ControlPredictorsUseMean {
            requested: request.summary,
        });
    }
    let cell = |unit: f64, period: f64, column: usize| -> Result<Option<f64>, PreparationError> {
        // Balance and window validation above establish exactly one selected row.
        let row = rows[find(unit, period).expect("validated panel cell")];
        match row.values[column] {
            // R is.na includes NaN. Treat it like a null measurement, not zero.
            Some(value) if value.is_nan() => Ok(None),
            Some(value) if !value.is_finite() => Err(PreparationError::NonFiniteValue {
                unit,
                period,
                column,
            }),
            value => Ok(value),
        }
    };
    let mut names: Vec<String> = request
        .predictors
        .iter()
        .map(|&c| data.columns[c].clone())
        .collect();
    let mut specifications: Vec<(usize, Vec<f64>, Summary, bool)> = request
        .predictors
        .iter()
        .map(|&c| (c, prior.clone(), request.summary, false))
        .collect();
    for (i, s) in request.special.iter().enumerate() {
        let selected = periods(&s.periods, Window::Special(i), &available, &mut evidence)?;
        let suffix = if s.periods.len() == 1 {
            format!("{}", s.periods[0])
        } else {
            format!("{}.{}", s.periods[0], s.periods[s.periods.len() - 1])
        };
        names.push(format!("special.{}.{}", data.columns[s.column], suffix));
        specifications.push((s.column, selected, s.summary, true));
    }
    let mut x0 = DMatrix::zeros(specifications.len(), donors.len());
    let mut x1 = DVector::zeros(specifications.len());
    for (predictor, (column, window, operation, special)) in specifications.iter().enumerate() {
        for (u, &unit) in units.iter().enumerate() {
            let mut values = Vec::new();
            let mut omitted = Vec::new();
            for &period in window {
                match cell(unit, period, *column)? {
                    Some(v) => values.push(v),
                    None => omitted.push(period),
                }
            }
            if values.is_empty() {
                return Err(PreparationError::AllMissingPredictor { unit, predictor });
            }
            if !omitted.is_empty() {
                evidence.push(PreparationEvidence::OmittedPredictorValues {
                    unit,
                    predictor,
                    periods: omitted,
                });
            }
            let operation = if !special && u > 0 {
                Summary::Mean
            } else {
                *operation
            };
            let value = if *special && window.len() == 1 {
                values[0]
            } else {
                summarize(&values, operation)
            };
            if !value.is_finite() {
                return Err(PreparationError::UndefinedSummary { unit, predictor });
            }
            if u == 0 {
                x1[predictor] = value
            } else {
                x0[(predictor, u - 1)] = value;
            }
        }
    }
    let mut z0 = DMatrix::zeros(fit.len(), donors.len());
    let mut z1 = DVector::zeros(fit.len());
    for (i, &period) in fit.iter().enumerate() {
        for (u, &unit) in units.iter().enumerate() {
            let value =
                cell(unit, period, request.outcome)?.ok_or(PreparationError::MissingOutcome {
                    unit,
                    period,
                    window: Window::Fit,
                })?;
            if u == 0 {
                z1[i] = value
            } else {
                z0[(i, u - 1)] = value;
            }
        }
    }
    let mut y0 = DMatrix::from_element(plot.len(), donors.len(), None);
    let mut y1 = DVector::zeros(plot.len());
    for (i, &period) in plot.iter().enumerate() {
        for (u, &unit) in units.iter().enumerate() {
            let value = cell(unit, period, request.outcome)?;
            if u == 0 {
                y1[i] = value.ok_or(PreparationError::MissingOutcome {
                    unit,
                    period,
                    window: Window::Plot,
                })?;
            } else {
                y0[(i, u - 1)] = value;
            }
        }
    }
    Ok(PreparedPanel {
        treated: request.treated,
        donors,
        predictor_names: names,
        fit_periods: fit,
        plot_periods: plot,
        x0,
        x1,
        z0,
        z1,
        y0_plot: y0,
        y1_plot: y1,
        evidence,
    })
}
