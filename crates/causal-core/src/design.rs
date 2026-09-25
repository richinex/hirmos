#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DesignError {
    EmptyFrame,
    RowMismatch { column: String },
    NonFiniteValue { column: String, row: usize },
    NoLevels { column: String },
    SingleLevel { column: String },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Values {
    Numeric(Vec<f64>),
    Text(Vec<String>),
}

impl Values {
    pub fn len(&self) -> usize {
        match self {
            Self::Numeric(values) => values.len(),
            Self::Text(values) => values.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// How one column enters a design. The same variable can be a number in one model and indicators
/// in another, so this is declared per run rather than stored with the data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Encoding {
    Numeric,
    /// `pd.get_dummies`: one column per level, none dropped.
    Indicators,
    /// patsy `C(x)`: the first level is the baseline and is dropped.
    TreatmentContrast,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Column {
    pub name: String,
    pub values: Values,
    pub encoding: Encoding,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Design {
    pub names: Vec<String>,
    pub rows: Vec<Vec<f64>>,
}

fn level_label(value: &str) -> String {
    value.to_string()
}

fn numeric_label(value: f64) -> String {
    if value == value.trunc() && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

/// Distinct levels in the order an encoder lays them out: ascending for numbers, byte order for
/// text, which is what `pd.get_dummies` and patsy both use.
fn levels(values: &Values) -> Vec<String> {
    match values {
        Values::Numeric(numbers) => {
            let mut distinct: Vec<f64> = numbers.to_vec();
            distinct.sort_by(f64::total_cmp);
            distinct.dedup();
            distinct.into_iter().map(numeric_label).collect()
        }
        Values::Text(text) => {
            let mut distinct: Vec<String> = text.to_vec();
            distinct.sort();
            distinct.dedup();
            distinct.into_iter().map(|value| level_label(&value)).collect()
        }
    }
}

fn indicator(values: &Values, level: &str) -> Vec<f64> {
    match values {
        Values::Numeric(numbers) => numbers
            .iter()
            .map(|value| f64::from(numeric_label(*value) == level))
            .collect(),
        Values::Text(text) => text
            .iter()
            .map(|value| f64::from(value.as_str() == level))
            .collect(),
    }
}

fn validate(columns: &[Column]) -> Result<usize, DesignError> {
    let first = columns.first().ok_or(DesignError::EmptyFrame)?;
    let rows = first.values.len();
    if rows == 0 {
        return Err(DesignError::EmptyFrame);
    }
    for column in columns {
        if column.values.len() != rows {
            return Err(DesignError::RowMismatch {
                column: column.name.clone(),
            });
        }
        if let Values::Numeric(numbers) = &column.values {
            if let Some(row) = numbers.iter().position(|value| !value.is_finite()) {
                return Err(DesignError::NonFiniteValue {
                    column: column.name.clone(),
                    row,
                });
            }
        }
    }
    Ok(rows)
}

fn push_numeric(
    column: &Column,
    names: &mut Vec<String>,
    series: &mut Vec<Vec<f64>>,
) -> Result<(), DesignError> {
    match &column.values {
        Values::Numeric(numbers) => {
            names.push(column.name.clone());
            series.push(numbers.clone());
            Ok(())
        }
        Values::Text(_) => Err(DesignError::NoLevels {
            column: column.name.clone(),
        }),
    }
}

fn push_encoded(
    column: &Column,
    names: &mut Vec<String>,
    series: &mut Vec<Vec<f64>>,
) -> Result<(), DesignError> {
    let found = levels(&column.values);
    if found.is_empty() {
        return Err(DesignError::NoLevels {
            column: column.name.clone(),
        });
    }
    if found.len() < 2 {
        return Err(DesignError::SingleLevel {
            column: column.name.clone(),
        });
    }
    let dropped = usize::from(column.encoding == Encoding::TreatmentContrast);
    for level in found.iter().skip(dropped) {
        names.push(match column.encoding {
            Encoding::TreatmentContrast => format!("C({})[T.{level}]", column.name),
            _ => format!("{}_{level}", column.name),
        });
        series.push(indicator(&column.values, level));
    }
    Ok(())
}

fn assemble(rows: usize, names: Vec<String>, series: Vec<Vec<f64>>) -> Design {
    Design {
        names,
        rows: (0..rows)
            .map(|row| series.iter().map(|column| column[row]).collect())
            .collect(),
    }
}

/// `pd.get_dummies(frame)`: every column that stays numeric first, in source order, then one block
/// of indicators per encoded column, also in source order.
pub fn get_dummies(columns: &[Column]) -> Result<Design, DesignError> {
    let rows = validate(columns)?;
    let mut names = Vec::new();
    let mut series: Vec<Vec<f64>> = Vec::new();
    for column in columns.iter().filter(|c| c.encoding == Encoding::Numeric) {
        push_numeric(column, &mut names, &mut series)?;
    }
    for column in columns.iter().filter(|c| c.encoding != Encoding::Numeric) {
        push_encoded(column, &mut names, &mut series)?;
    }
    Ok(assemble(rows, names, series))
}

/// patsy's layout for a formula of main effects, which does not follow the formula's own order:
/// the intercept, then every encoded term in source order, then every numeric term in source order.
pub fn patsy_dmatrix(columns: &[Column]) -> Result<Design, DesignError> {
    let rows = validate(columns)?;
    let mut names = vec!["Intercept".to_string()];
    let mut series: Vec<Vec<f64>> = vec![vec![1.0; rows]];
    for column in columns.iter().filter(|c| c.encoding != Encoding::Numeric) {
        push_encoded(column, &mut names, &mut series)?;
    }
    for column in columns.iter().filter(|c| c.encoding == Encoding::Numeric) {
        push_numeric(column, &mut names, &mut series)?;
    }
    Ok(assemble(rows, names, series))
}
