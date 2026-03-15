/// Direction for calendar layout.
///
/// Controls whether multiple months are displayed side by side
/// or stacked vertically in the calendar view.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CalendarDirection {
    /// Display months side by side.
    #[default]
    Horizontal,

    /// Display months stacked vertically.
    Vertical,
}

impl From<&str> for CalendarDirection {
    fn from(s: &str) -> Self {
        // Match the lowercase variant name.
        match s.to_lowercase().as_str() {
            "vertical" => CalendarDirection::Vertical,
            _ => CalendarDirection::Horizontal,
        }
    }
}
