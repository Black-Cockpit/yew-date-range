/// Hour display format.
///
/// Controls whether the time picker displays hours in 24-hour
/// or 12-hour format with AM/PM indicator.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HourFormat {
    /// 24-hour format (0-23).
    #[default]
    H24,

    /// 12-hour format with AM/PM.
    H12,
}
