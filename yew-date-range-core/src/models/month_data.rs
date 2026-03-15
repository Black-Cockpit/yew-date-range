use time::Month;

use crate::models::week_data::WeekData;

/// Represents a displayed month in the calendar.
///
/// Contains the year, month, and all week rows needed to render
/// the calendar grid for a single month.
#[derive(Clone, Debug, PartialEq)]
pub struct MonthData {
    /// The year of this displayed month.
    pub year: i32,

    /// The month being displayed.
    pub month: Month,

    /// The week rows that make up this month's grid.
    pub weeks: Vec<WeekData>,
}
