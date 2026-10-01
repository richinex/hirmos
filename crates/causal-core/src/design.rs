#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DesignError {
    EmptyFrame,
    RowMismatch { column: String },
    NonFiniteValue { column: String, row: usize },
    NoLevels { column: String },
    SingleLevel { column: String },
    InvalidTerm { term: usize },
    InvalidReference { column: usize },
    DuplicateName { column: String },
    NonFiniteInteraction { term: usize, row: usize },
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
            distinct
                .into_iter()
                .map(|value| level_label(&value))
                .collect()
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
    push_encoded_reference(column, None, names, series)
}

fn push_encoded_reference(
    column: &Column,
    reference: Option<&str>,
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
    let omitted = if column.encoding == Encoding::TreatmentContrast {
        reference.or_else(|| found.first().map(String::as_str))
    } else {
        None
    };
    for level in found.iter().filter(|level| Some(level.as_str()) != omitted) {
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

/// A model term refers to source columns, before categorical expansion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Term {
    Main(usize),
    Pair(usize, usize),
    Triple(usize, usize, usize),
}

/// Hierarchical interactions with an intercept. Reuses the ordinary numeric and
/// categorical encoders; this is not a formula parser. Terms include all their
/// lower-order terms. Main effects precede pairs, which precede triples. Within
/// a degree, first appearance determines order. The first component varies
/// fastest within a categorical interaction, as in R's model.matrix.
///
/// References are explicit labels from the existing level encoder. Missing
/// entries retain its default first-level reference. Indicator encoding remains
/// full indicator encoding; rank decisions belong to the fitter, not this step.
pub fn hierarchical_design(
    columns: &[Column],
    terms: &[Term],
    references: &std::collections::BTreeMap<usize, String>,
) -> Result<Design, DesignError> {
    let rows = validate(columns)?;
    let mut seen_names = std::collections::BTreeSet::new();
    for c in columns {
        if !seen_names.insert(&c.name) {
            return Err(DesignError::DuplicateName {
                column: c.name.clone(),
            });
        }
    }
    for (&i, level) in references {
        let column = columns
            .get(i)
            .ok_or(DesignError::InvalidReference { column: i })?;
        if column.encoding != Encoding::TreatmentContrast || !levels(&column.values).contains(level)
        {
            return Err(DesignError::InvalidReference { column: i });
        }
    }
    let mut expanded: Vec<Vec<usize>> = Vec::new();
    for (index, term) in terms.iter().enumerate() {
        let mut factors = match *term {
            Term::Main(a) => vec![a],
            Term::Pair(a, b) => vec![a, b],
            Term::Triple(a, b, c) => vec![a, b, c],
        };
        factors.sort_unstable();
        if factors.iter().any(|i| *i >= columns.len()) || factors.windows(2).any(|p| p[0] == p[1]) {
            return Err(DesignError::InvalidTerm { term: index });
        }
        for mask in 1..(1usize << factors.len()) {
            let subset: Vec<_> = factors
                .iter()
                .enumerate()
                .filter_map(|(j, i)| (mask & (1 << j) != 0).then_some(*i))
                .collect();
            if !expanded.contains(&subset) {
                expanded.push(subset);
            }
        }
    }
    expanded.sort_by_key(Vec::len);
    let mut bases = Vec::with_capacity(columns.len());
    for (i, column) in columns.iter().enumerate() {
        let mut names = Vec::new();
        let mut series = Vec::new();
        match column.encoding {
            Encoding::Numeric => push_numeric(column, &mut names, &mut series)?,
            Encoding::Indicators | Encoding::TreatmentContrast => push_encoded_reference(
                column,
                references.get(&i).map(String::as_str),
                &mut names,
                &mut series,
            )?,
        }
        bases.push((names, series));
    }
    let mut names = vec!["Intercept".to_string()];
    let mut series = vec![vec![1.; rows]];
    for (term, factors) in expanded.iter().enumerate() {
        let mut products = vec![(String::new(), vec![1.; rows])];
        for &factor in factors {
            let (labels, values) = &bases[factor];
            let mut next = Vec::new();
            for (label, column) in labels.iter().zip(values) {
                for (prefix, product) in &products {
                    let name = if prefix.is_empty() {
                        label.clone()
                    } else {
                        format!("{prefix}:{label}")
                    };
                    let values: Vec<_> = product.iter().zip(column).map(|(a, b)| a * b).collect();
                    if let Some(row) = values.iter().position(|v| !v.is_finite()) {
                        return Err(DesignError::NonFiniteInteraction { term, row });
                    }
                    next.push((name, values));
                }
            }
            products = next;
        }
        for (name, values) in products {
            names.push(name);
            series.push(values);
        }
    }
    Ok(assemble(rows, names, series))
}
