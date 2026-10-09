//! Validated parametric-regression model used by `flexsurvreg`.

use std::num::NonZeroUsize;

use super::distribution::{DistributionError, FlexSurvDistribution};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FlexSurvFamily {
    Exponential,
    Weibull,
    WeibullPh,
    LogNormal,
    Gamma,
    Gompertz,
    LogLogistic,
    GeneralizedGamma,
    GeneralizedGammaOriginal,
    GeneralizedF,
    GeneralizedFOriginal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DistributionParameter {
    Rate,
    Shape,
    GompertzShape,
    Scale,
    MeanLog,
    SdLog,
    Mu,
    Sigma,
    Q,
    P,
    K,
    S1,
    S2,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ModelError {
    EmptyRows,
    EmptyColumns,
    MatrixLength {
        expected: usize,
        actual: usize,
    },
    NonFiniteCovariate {
        index: usize,
    },
    RowCount {
        expected: usize,
        actual: usize,
    },
    ParameterDoesNotBelong {
        family: FlexSurvFamily,
        parameter: DistributionParameter,
    },
    DuplicateParameterDesign(DistributionParameter),
    BaselineLength {
        expected: usize,
        actual: usize,
    },
    CoefficientLength {
        expected: usize,
        actual: usize,
    },
    NonFiniteInitialValue {
        index: usize,
    },
    PredictionCovariateLength {
        expected: usize,
        actual: usize,
    },
    NonFinitePredictionCovariate {
        index: usize,
    },
    Distribution(DistributionError),
}

/// One prediction profile in flexsurv's coefficient order.
///
/// This is opaque because the required length belongs to a particular model.
/// Construct it through [`RegressionModel::prediction_covariates`].
#[derive(Clone, Debug, PartialEq)]
pub struct PredictionCovariates(Vec<f64>);

impl From<DistributionError> for ModelError {
    fn from(value: DistributionError) -> Self {
        Self::Distribution(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CovariateMatrix {
    rows: NonZeroUsize,
    columns: NonZeroUsize,
    values: Vec<f64>,
}

impl CovariateMatrix {
    pub fn new(rows: usize, columns: usize, values: Vec<f64>) -> Result<Self, ModelError> {
        let rows = NonZeroUsize::new(rows).ok_or(ModelError::EmptyRows)?;
        let columns = NonZeroUsize::new(columns).ok_or(ModelError::EmptyColumns)?;
        let expected = rows.get() * columns.get();
        if values.len() != expected {
            return Err(ModelError::MatrixLength {
                expected,
                actual: values.len(),
            });
        }
        if let Some(index) = values.iter().position(|value| !value.is_finite()) {
            return Err(ModelError::NonFiniteCovariate { index });
        }
        Ok(Self {
            rows,
            columns,
            values,
        })
    }

    pub fn rows(&self) -> usize {
        self.rows.get()
    }

    pub fn columns(&self) -> usize {
        self.columns.get()
    }

    pub(crate) fn linear_predictor(&self, row: usize, coefficients: &[f64]) -> f64 {
        let mut value = 0.0;
        for column in 0..self.columns() {
            value += self.values[row * self.columns() + column] * coefficients[column];
        }
        value
    }

    pub(crate) fn value(&self, row: usize, column: usize) -> f64 {
        self.values[row * self.columns() + column]
    }

    pub(crate) fn select_rows(&self, rows: &[usize]) -> Result<Self, ModelError> {
        let mut values = Vec::with_capacity(rows.len() * self.columns());
        for row in rows {
            if *row >= self.rows() {
                return Err(ModelError::RowCount {
                    expected: self.rows(),
                    actual: *row + 1,
                });
            }
            let start = *row * self.columns();
            values.extend_from_slice(&self.values[start..start + self.columns()]);
        }
        Self::new(rows.len(), self.columns(), values)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RegressionModel {
    family: FlexSurvFamily,
    rows: NonZeroUsize,
    designs: Vec<Option<CovariateMatrix>>,
}

impl FlexSurvFamily {
    pub fn parameters(self) -> &'static [DistributionParameter] {
        use DistributionParameter::*;
        match self {
            Self::Exponential => &[Rate],
            Self::Weibull | Self::WeibullPh => &[Shape, Scale],
            Self::LogNormal => &[MeanLog, SdLog],
            Self::Gamma => &[Shape, Rate],
            Self::Gompertz => &[GompertzShape, Rate],
            Self::LogLogistic => &[Shape, Scale],
            Self::GeneralizedGamma => &[Mu, Sigma, Q],
            Self::GeneralizedGammaOriginal => &[Shape, Scale, K],
            Self::GeneralizedF => &[Mu, Sigma, Q, P],
            Self::GeneralizedFOriginal => &[Mu, Sigma, S1, S2],
        }
    }

    pub fn location_parameter(self) -> DistributionParameter {
        use DistributionParameter::*;
        match self {
            Self::Exponential => Rate,
            Self::Weibull
            | Self::WeibullPh
            | Self::LogLogistic
            | Self::GeneralizedGammaOriginal => Scale,
            Self::LogNormal => MeanLog,
            Self::Gamma | Self::Gompertz => Rate,
            Self::GeneralizedGamma | Self::GeneralizedF | Self::GeneralizedFOriginal => Mu,
        }
    }

    fn parameter_index(self, parameter: DistributionParameter) -> Option<usize> {
        self.parameters().iter().position(|item| *item == parameter)
    }

    fn location_index(self) -> usize {
        match self {
            Self::Exponential
            | Self::LogNormal
            | Self::GeneralizedGamma
            | Self::GeneralizedF
            | Self::GeneralizedFOriginal => 0,
            Self::Weibull
            | Self::WeibullPh
            | Self::Gamma
            | Self::Gompertz
            | Self::LogLogistic
            | Self::GeneralizedGammaOriginal => 1,
        }
    }

    fn inverse_transform(self, parameter: DistributionParameter, value: f64) -> f64 {
        use DistributionParameter::*;
        match parameter {
            Rate | Shape | Scale | SdLog | Sigma | P | K | S1 | S2 => value.exp(),
            GompertzShape | MeanLog | Mu | Q => value,
        }
    }

    fn transform_initial(
        self,
        parameter: DistributionParameter,
        value: f64,
    ) -> Result<f64, DistributionError> {
        use DistributionParameter::*;
        let transformed = match parameter {
            Rate | Shape | Scale | SdLog | Sigma | P | K | S1 | S2 => {
                if !value.is_finite() {
                    return Err(DistributionError::NonFiniteParameter);
                }
                if value <= 0.0 {
                    return Err(DistributionError::NonPositiveParameter(match parameter {
                        Rate => "rate",
                        Shape => "shape",
                        Scale => "scale",
                        SdLog => "sd_log",
                        Sigma => "sigma",
                        P => "p",
                        K => "k",
                        S1 => "s1",
                        S2 => "s2",
                        GompertzShape => "gompertz_shape",
                        MeanLog => "mean_log",
                        Mu => "mu",
                        Q => "q",
                    }));
                }
                value.ln()
            }
            GompertzShape | MeanLog | Mu | Q => {
                if !value.is_finite() {
                    return Err(DistributionError::NonFiniteParameter);
                }
                value
            }
        };
        Ok(transformed)
    }

    fn distribution(self, parameters: &[f64]) -> Result<FlexSurvDistribution, DistributionError> {
        match self {
            Self::Exponential => FlexSurvDistribution::exponential(parameters[0]),
            Self::Weibull => FlexSurvDistribution::weibull(parameters[0], parameters[1]),
            Self::WeibullPh => FlexSurvDistribution::weibull_ph(parameters[0], parameters[1]),
            Self::LogNormal => FlexSurvDistribution::log_normal(parameters[0], parameters[1]),
            Self::Gamma => FlexSurvDistribution::gamma(parameters[0], parameters[1]),
            Self::Gompertz => FlexSurvDistribution::gompertz(parameters[0], parameters[1]),
            Self::LogLogistic => FlexSurvDistribution::log_logistic(parameters[0], parameters[1]),
            Self::GeneralizedGamma => {
                FlexSurvDistribution::generalized_gamma(parameters[0], parameters[1], parameters[2])
            }
            Self::GeneralizedGammaOriginal => FlexSurvDistribution::generalized_gamma_original(
                parameters[0],
                parameters[1],
                parameters[2],
            ),
            Self::GeneralizedF => FlexSurvDistribution::generalized_f(
                parameters[0],
                parameters[1],
                parameters[2],
                parameters[3],
            ),
            Self::GeneralizedFOriginal => FlexSurvDistribution::generalized_f_original(
                parameters[0],
                parameters[1],
                parameters[2],
                parameters[3],
            ),
        }
    }

    pub(crate) fn has_analytic_gradient(self) -> bool {
        matches!(
            self,
            Self::Exponential
                | Self::Weibull
                | Self::WeibullPh
                | Self::Gompertz
                | Self::LogLogistic
        )
    }

    pub(crate) fn has_analytic_hessian(self) -> bool {
        matches!(
            self,
            Self::Exponential | Self::Weibull | Self::WeibullPh | Self::Gompertz
        )
    }

    pub(crate) fn score(self, event: bool, time: f64, parameters: &[f64]) -> Vec<f64> {
        match self {
            Self::Exponential => {
                let rate = parameters[0];
                vec![if event {
                    1.0 - time * rate
                } else {
                    -time * rate
                }]
            }
            Self::Weibull => {
                let shape = parameters[0];
                let scale = parameters[1];
                let power = (time / scale).powf(shape);
                if event {
                    let log_scaled = (time / scale).ln();
                    vec![
                        1.0 + shape * log_scaled * (1.0 - power),
                        -shape + shape * power,
                    ]
                } else if time == 0.0 {
                    vec![0.0, 0.0]
                } else {
                    vec![-shape * (time / scale).ln() * power, power * shape]
                }
            }
            Self::WeibullPh => {
                let shape = parameters[0];
                let scale = parameters[1];
                let power = time.powf(shape);
                if event {
                    vec![
                        1.0 + shape * time.ln() * (1.0 - scale * power),
                        1.0 - scale * power,
                    ]
                } else if time == 0.0 {
                    vec![0.0, 0.0]
                } else {
                    vec![-scale * shape * time.ln() * power, -scale * power]
                }
            }
            Self::Gompertz => {
                let shape = parameters[0];
                let rate = parameters[1];
                if shape == 0.0 {
                    vec![
                        0.0,
                        if event {
                            1.0 - rate * time
                        } else {
                            -rate * time
                        },
                    ]
                } else {
                    let exponential = (shape * time).exp();
                    let shape_score =
                        -rate / shape * ((1.0 - exponential) / shape + time * exponential);
                    let rate_score = rate / shape * (1.0 - exponential);
                    vec![
                        shape_score + if event { time } else { 0.0 },
                        rate_score + if event { 1.0 } else { 0.0 },
                    ]
                }
            }
            Self::LogLogistic => {
                let shape = parameters[0];
                let scale = parameters[1];
                let power = (time / scale).powf(shape);
                let fraction = power / (1.0 + power);
                if event {
                    vec![
                        1.0 + (1.0 - 2.0 * fraction) * shape * (time / scale).ln(),
                        -shape + 2.0 * fraction * shape,
                    ]
                } else if time == 0.0 {
                    vec![0.0, 0.0]
                } else {
                    vec![-fraction * (time / scale).ln() * shape, shape * fraction]
                }
            }
            Self::LogNormal
            | Self::Gamma
            | Self::GeneralizedGamma
            | Self::GeneralizedGammaOriginal
            | Self::GeneralizedF
            | Self::GeneralizedFOriginal => {
                unreachable!("this family has no flexsurv analytic score")
            }
        }
    }

    pub(crate) fn second_score(self, event: bool, time: f64, parameters: &[f64]) -> Vec<f64> {
        match self {
            Self::Exponential => vec![-time * parameters[0]],
            Self::Weibull => {
                let shape = parameters[0];
                let scale = parameters[1];
                if time == 0.0 {
                    return vec![0.0; 4];
                }
                let power = (time / scale).powf(shape);
                let log_scaled = (time / scale).ln();
                let shape_shape = if event {
                    shape * log_scaled * (1.0 - power - shape * log_scaled * power)
                } else {
                    -shape * log_scaled * power * (1.0 + shape * log_scaled)
                };
                let scale_scale = -shape * shape * power;
                let cross = if event {
                    shape * (power - 1.0 + shape * log_scaled * power)
                } else {
                    shape * power * (1.0 + shape * log_scaled)
                };
                vec![shape_shape, cross, cross, scale_scale]
            }
            Self::WeibullPh => {
                let shape = parameters[0];
                let scale = parameters[1];
                if time == 0.0 {
                    return vec![0.0; 4];
                }
                let log_time = time.ln();
                let power = time.powf(shape);
                let shape_shape = if event {
                    shape * log_time * (1.0 - scale * power * (1.0 + shape * log_time))
                } else {
                    -shape * scale * log_time * (power + shape * log_time * power)
                };
                let scale_scale = -scale * power;
                let cross = -scale * shape * log_time * power;
                vec![shape_shape, cross, cross, scale_scale]
            }
            Self::Gompertz => {
                let shape = parameters[0];
                let rate = parameters[1];
                if shape == 0.0 {
                    return vec![0.0, 0.0, 0.0, -rate * time];
                }
                let exponential = (shape * time).exp();
                let shape_shape = 2.0 * rate / shape.powi(3) * (1.0 - exponential)
                    + 2.0 * rate / shape.powi(2) * time * exponential
                    - rate / shape * time * time * exponential;
                let rate_rate = rate / shape * (1.0 - exponential);
                let cross = -rate / shape * ((1.0 - exponential) / shape + time * exponential);
                vec![shape_shape, cross, cross, rate_rate]
            }
            _ => unreachable!("family has no source analytic Hessian"),
        }
    }
}

impl RegressionModel {
    pub fn new(family: FlexSurvFamily, rows: usize) -> Result<Self, ModelError> {
        let rows = NonZeroUsize::new(rows).ok_or(ModelError::EmptyRows)?;
        Ok(Self {
            family,
            rows,
            designs: vec![None; family.parameters().len()],
        })
    }

    pub fn with_location_covariates(self, matrix: CovariateMatrix) -> Result<Self, ModelError> {
        let location = self.family.location_parameter();
        self.with_parameter_covariates(location, matrix)
    }

    pub fn with_parameter_covariates(
        mut self,
        parameter: DistributionParameter,
        matrix: CovariateMatrix,
    ) -> Result<Self, ModelError> {
        if matrix.rows() != self.rows.get() {
            return Err(ModelError::RowCount {
                expected: self.rows.get(),
                actual: matrix.rows(),
            });
        }
        let index =
            self.family
                .parameter_index(parameter)
                .ok_or(ModelError::ParameterDoesNotBelong {
                    family: self.family,
                    parameter,
                })?;
        if self.designs[index].is_some() {
            return Err(ModelError::DuplicateParameterDesign(parameter));
        }
        self.designs[index] = Some(matrix);
        Ok(self)
    }

    pub fn family(&self) -> FlexSurvFamily {
        self.family
    }

    pub fn rows(&self) -> usize {
        self.rows.get()
    }

    pub fn parameter_count(&self) -> usize {
        self.family.parameters().len() + self.coefficient_count()
    }

    pub fn coefficient_count(&self) -> usize {
        self.designs
            .iter()
            .flatten()
            .map(CovariateMatrix::columns)
            .sum()
    }

    pub(crate) fn select_rows(&self, rows: &[usize]) -> Result<Self, ModelError> {
        let mut selected = Self::new(self.family, rows.len())?;
        for (index, design) in self.designs.iter().enumerate() {
            if let Some(design) = design {
                selected.designs[index] = Some(design.select_rows(rows)?);
            }
        }
        Ok(selected)
    }

    pub(crate) fn location_design(&self) -> Vec<f64> {
        let location = self.family.location_index();
        let columns = 1 + self.designs[location]
            .as_ref()
            .map(CovariateMatrix::columns)
            .unwrap_or(0);
        let mut values = Vec::with_capacity(self.rows() * columns);
        for row in 0..self.rows() {
            values.push(1.0);
            if let Some(matrix) = &self.designs[location] {
                for column in 0..matrix.columns() {
                    values.push(matrix.value(row, column));
                }
            }
        }
        values
    }

    pub(crate) fn location_coefficient_count(&self) -> usize {
        self.designs[self.family.location_index()]
            .as_ref()
            .map(CovariateMatrix::columns)
            .unwrap_or(0)
    }

    pub fn prediction_covariates(
        &self,
        values: Vec<f64>,
    ) -> Result<PredictionCovariates, ModelError> {
        let expected = self.coefficient_count();
        if values.len() != expected {
            return Err(ModelError::PredictionCovariateLength {
                expected,
                actual: values.len(),
            });
        }
        if let Some(index) = values.iter().position(|value| !value.is_finite()) {
            return Err(ModelError::NonFinitePredictionCovariate { index });
        }
        Ok(PredictionCovariates(values))
    }

    fn coefficient_parameter_order(&self) -> impl Iterator<Item = usize> {
        let location = self.family.location_index();
        std::iter::once(location).chain((0..self.designs.len()).filter(move |index| *index != location))
    }

    pub fn transformed_initial_values(
        &self,
        natural_baseline: &[f64],
        coefficients: &[f64],
    ) -> Result<Vec<f64>, ModelError> {
        let expected_baseline = self.family.parameters().len();
        if natural_baseline.len() != expected_baseline {
            return Err(ModelError::BaselineLength {
                expected: expected_baseline,
                actual: natural_baseline.len(),
            });
        }
        if coefficients.len() != self.coefficient_count() {
            return Err(ModelError::CoefficientLength {
                expected: self.coefficient_count(),
                actual: coefficients.len(),
            });
        }

        let mut transformed = Vec::with_capacity(self.parameter_count());
        for (parameter, value) in self
            .family
            .parameters()
            .iter()
            .copied()
            .zip(natural_baseline.iter().copied())
        {
            transformed.push(self.family.transform_initial(parameter, value)?);
        }
        for (index, value) in coefficients.iter().copied().enumerate() {
            if !value.is_finite() {
                return Err(ModelError::NonFiniteInitialValue {
                    index: expected_baseline + index,
                });
            }
            transformed.push(value);
        }
        Ok(transformed)
    }

    pub(crate) fn distribution_at(
        &self,
        row: usize,
        transformed: &[f64],
    ) -> Result<FlexSurvDistribution, DistributionError> {
        let baseline_count = self.family.parameters().len();
        let mut storage = [0.0; 4];
        let linear = &mut storage[..baseline_count];
        linear.copy_from_slice(&transformed[..baseline_count]);
        let mut coefficient_offset = baseline_count;
        for parameter_index in self.coefficient_parameter_order() {
            if let Some(matrix) = &self.designs[parameter_index] {
                let end = coefficient_offset + matrix.columns();
                linear[parameter_index] +=
                    matrix.linear_predictor(row, &transformed[coefficient_offset..end]);
                coefficient_offset = end;
            }
        }
        for (parameter, value) in self.family.parameters().iter().copied().zip(linear.iter_mut()) {
            *value = self.family.inverse_transform(parameter, *value);
        }
        self.family.distribution(linear)
    }

    pub(crate) fn distribution_for_prediction(
        &self,
        covariates: &PredictionCovariates,
        transformed: &[f64],
    ) -> Result<FlexSurvDistribution, DistributionError> {
        let baseline_count = self.family.parameters().len();
        let mut linear = transformed[..baseline_count].to_vec();
        let mut coefficient_offset = baseline_count;
        let mut covariate_offset = 0;

        for parameter_index in self.coefficient_parameter_order() {
            if let Some(matrix) = &self.designs[parameter_index] {
                let columns = matrix.columns();
                let coefficient_end = coefficient_offset + columns;
                let covariate_end = covariate_offset + columns;
                linear[parameter_index] += covariates.0[covariate_offset..covariate_end]
                    .iter()
                    .zip(&transformed[coefficient_offset..coefficient_end])
                    .map(|(value, coefficient)| value * coefficient)
                    .sum::<f64>();
                coefficient_offset = coefficient_end;
                covariate_offset = covariate_end;
            }
        }

        let natural = self
            .family
            .parameters()
            .iter()
            .copied()
            .zip(linear)
            .map(|(parameter, value)| self.family.inverse_transform(parameter, value))
            .collect::<Vec<_>>();
        self.family.distribution(&natural)
    }

    pub(crate) fn natural_parameters_at(&self, row: usize, transformed: &[f64]) -> Vec<f64> {
        let baseline_count = self.family.parameters().len();
        let mut linear = transformed[..baseline_count].to_vec();
        let mut coefficient_offset = baseline_count;
        for parameter_index in self.coefficient_parameter_order() {
            if let Some(matrix) = &self.designs[parameter_index] {
                let end = coefficient_offset + matrix.columns();
                linear[parameter_index] +=
                    matrix.linear_predictor(row, &transformed[coefficient_offset..end]);
                coefficient_offset = end;
            }
        }
        self.family
            .parameters()
            .iter()
            .copied()
            .zip(linear)
            .map(|(parameter, value)| self.family.inverse_transform(parameter, value))
            .collect()
    }

    pub(crate) fn expand_score(&self, row: usize, baseline: &[f64]) -> Vec<f64> {
        let mut score = baseline.to_vec();
        for parameter_index in self.coefficient_parameter_order() {
            if let Some(matrix) = &self.designs[parameter_index] {
                for column in 0..matrix.columns() {
                    score.push(matrix.value(row, column) * baseline[parameter_index]);
                }
            }
        }
        score
    }

    pub(crate) fn expand_second_score(&self, row: usize, baseline: &[f64]) -> Vec<f64> {
        let baseline_count = self.family.parameters().len();
        let parameter_count = self.parameter_count();
        let mut jacobian = vec![0.0; baseline_count * parameter_count];
        for index in 0..baseline_count {
            jacobian[index * parameter_count + index] = 1.0;
        }
        let mut coefficient = baseline_count;
        for parameter_index in self.coefficient_parameter_order() {
            if let Some(matrix) = &self.designs[parameter_index] {
                for column in 0..matrix.columns() {
                    jacobian[parameter_index * parameter_count + coefficient] =
                        matrix.value(row, column);
                    coefficient += 1;
                }
            }
        }

        let mut expanded = vec![0.0; parameter_count * parameter_count];
        for left in 0..parameter_count {
            for right in 0..parameter_count {
                let mut value = 0.0;
                for base_left in 0..baseline_count {
                    for base_right in 0..baseline_count {
                        value += jacobian[base_left * parameter_count + left]
                            * baseline[base_left * baseline_count + base_right]
                            * jacobian[base_right * parameter_count + right];
                    }
                }
                expanded[left * parameter_count + right] = value;
            }
        }
        expanded
    }

    pub(crate) fn natural_parameter_value(&self, index: usize, transformed: f64) -> f64 {
        if index < self.family.parameters().len() {
            self.family
                .inverse_transform(self.family.parameters()[index], transformed)
        } else {
            transformed
        }
    }

    pub(crate) fn natural_standard_error(
        &self,
        index: usize,
        transformed: f64,
        standard_error: f64,
    ) -> f64 {
        if index >= self.family.parameters().len() {
            return standard_error;
        }
        use DistributionParameter::*;
        match self.family.parameters()[index] {
            Rate | Shape | Scale | SdLog | Sigma | P | K | S1 | S2 => {
                transformed.exp() * standard_error
            }
            GompertzShape | MeanLog | Mu | Q => standard_error,
        }
    }

    pub fn natural_baseline(&self, transformed: &[f64]) -> Result<Vec<f64>, ModelError> {
        let expected = self.parameter_count();
        if transformed.len() != expected {
            return Err(ModelError::BaselineLength {
                expected,
                actual: transformed.len(),
            });
        }
        Ok(self
            .family
            .parameters()
            .iter()
            .enumerate()
            .map(|(index, parameter)| {
                self.family
                    .inverse_transform(*parameter, transformed[index])
            })
            .collect())
    }
}
