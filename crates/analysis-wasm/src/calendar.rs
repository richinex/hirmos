use hirmos_causal_core::calendar::{inspect, Calendar, Coverage, GridError};
use jiff::civil::Weekday;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Schedule { Daily, Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday, MonthStart, MonthEnd, QuarterStart, QuarterEnd, YearStart, YearEnd }
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input { schedule: Schedule, units: Vec<Unit> }
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Unit { name: String, times: Vec<i64> }
#[derive(Serialize)]
struct Gap { unit: String, first: String, last: String, count: u64 }
#[derive(Serialize)]
struct Issue { unit: String, detail: String }
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Report { units: usize, missing: u64, ranges: usize, gaps: Vec<Gap>, issues: Vec<Issue> }

#[wasm_bindgen(js_name = inspectCalendar)]
pub fn inspect_calendar(json: &str) -> Result<String, JsError> {
    let input: Input = serde_json::from_str(json).map_err(|e| JsError::new(&e.to_string()))?;
    let calendar = match input.schedule {
        Schedule::Daily => Calendar::Daily,
        Schedule::Monday => Calendar::Weekly(Weekday::Monday),
        Schedule::Tuesday => Calendar::Weekly(Weekday::Tuesday),
        Schedule::Wednesday => Calendar::Weekly(Weekday::Wednesday),
        Schedule::Thursday => Calendar::Weekly(Weekday::Thursday),
        Schedule::Friday => Calendar::Weekly(Weekday::Friday),
        Schedule::Saturday => Calendar::Weekly(Weekday::Saturday),
        Schedule::Sunday => Calendar::Weekly(Weekday::Sunday),
        Schedule::MonthStart => Calendar::MonthStart,
        Schedule::MonthEnd => Calendar::MonthEnd,
        Schedule::QuarterStart => Calendar::QuarterStart,
        Schedule::QuarterEnd => Calendar::QuarterEnd,
        Schedule::YearStart => Calendar::YearStart,
        Schedule::YearEnd => Calendar::YearEnd,
    };
    let mut report = Report { units: input.units.len(), missing: 0, ranges: 0, gaps: vec![], issues: vec![] };
    for unit in input.units {
        match inspect(&unit.times, calendar, Coverage::Observed) {
            Ok(grid) => {
                report.missing += grid.missing();
                report.ranges += grid.gaps().len();
                for gap in grid.gaps() {
                    if report.gaps.len() < 200 { report.gaps.push(Gap { unit: unit.name.clone(), first: gap.first().to_string(), last: gap.last().to_string(), count: gap.count() }); }
                }
            }
            Err(error) => {
                let detail = match error {
                    GridError::Empty => "No observations.".to_owned(),
                    GridError::InvalidTimestamp { row } => format!("Invalid date at sorted row {}.", row + 1),
                    GridError::DuplicatePeriod { row } => format!("More than one observation in a calendar period at sorted row {}.", row + 1),
                    GridError::OffCalendar { row } => format!("Date at sorted row {} does not match the selected calendar alignment.", row + 1),
                    GridError::OutOfOrder { row } => format!("Dates are not ordered at row {}.", row + 1),
                    GridError::ReversedWindow | GridError::OffCalendarWindow | GridError::OutsideWindow { .. } => "Invalid expected observation range.".to_owned(),
                };
                report.issues.push(Issue { unit: unit.name, detail });
            }
        }
    }
    serde_json::to_string(&report).map_err(|e| JsError::new(&e.to_string()))
}
