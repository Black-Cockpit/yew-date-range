use time::Date;

/// Represents a week row in the calendar.
///
/// Contains the days for a single week and an optional ISO week number
/// used when week number display is enabled.
#[derive(Clone, Debug, PartialEq)]
pub struct WeekData {
    /// The ISO week number, if available.
    pub week_number: Option<u8>,

    /// The days in this week row, with `None` for empty cells.
    pub days: Vec<Option<Date>>,
}
