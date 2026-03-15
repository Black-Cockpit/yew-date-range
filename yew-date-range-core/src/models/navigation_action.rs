/// Calendar navigation action.
///
/// Represents the possible navigation directions a user can trigger
/// when browsing through the calendar months and years.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavigationAction {
    /// Navigate to the previous month.
    PrevMonth,

    /// Navigate to the next month.
    NextMonth,

    /// Navigate to the previous year.
    PrevYear,

    /// Navigate to the next year.
    NextYear,
}
