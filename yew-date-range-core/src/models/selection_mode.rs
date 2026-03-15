/// Selection mode for the date picker.
///
/// Determines the type of date selection the picker supports,
/// affecting both the UI behavior and the callback payload.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectionMode {
    /// Select a single date.
    Single,

    /// Select a date range (start + end).
    #[default]
    Range,

    /// Select multiple individual dates.
    Multiple,
}
