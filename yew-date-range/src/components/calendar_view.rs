/// Internal view mode for the calendar drill-down navigation.
///
/// Controls which level of the calendar hierarchy is currently
/// displayed, enabling drill-down from days to months to years.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum CalendarView {
    /// Normal day grid view.
    Days,

    /// Month picker grid (3x4).
    Months,

    /// Year picker grid (decade, 2x5).
    Years,
}
