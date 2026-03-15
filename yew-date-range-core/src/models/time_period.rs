/// AM/PM period indicator.
///
/// Represents the time-of-day period used in 12-hour clock format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimePeriod {
    /// Ante meridiem (before noon).
    AM,

    /// Post meridiem (after noon).
    PM,
}

impl core::fmt::Display for TimePeriod {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Write the period abbreviation.
        match self {
            TimePeriod::AM => write!(f, "AM"),
            TimePeriod::PM => write!(f, "PM"),
        }
    }
}
