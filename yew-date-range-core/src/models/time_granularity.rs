/// Controls which time fields are visible.
///
/// Allows fine-grained control over which time components
/// (hours, minutes, seconds) the time picker displays.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeGranularity {
    /// Whether the hour field is shown.
    pub show_hours: bool,

    /// Whether the minute field is shown.
    pub show_minutes: bool,

    /// Whether the second field is shown.
    pub show_seconds: bool,
}

impl Default for TimeGranularity {
    fn default() -> Self {
        Self {
            show_hours: true,
            show_minutes: true,
            show_seconds: false,
        }
    }
}
