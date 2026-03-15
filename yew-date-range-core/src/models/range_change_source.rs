/// The source of a range change.
///
/// Identifies how a date range modification was triggered,
/// enabling consumers to differentiate between user interaction types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RangeChangeSource {
    /// Range changed via a day cell click.
    Click,

    /// Range changed via drag selection.
    Drag,

    /// Range changed via text input.
    Input,

    /// Range changed via a predefined range selection.
    DefinedRange,

    /// Range changed via keyboard navigation.
    Keyboard,
}
