use crate::models::range_selection::RangeSelection;

/// Represents an input range (editable date inputs).
///
/// Provides a label and factory function for creating an editable
/// range selection from text input fields.
#[derive(Clone, Debug)]
pub struct InputRange {
    /// The display label for this input range.
    pub label: String,

    /// Factory function that creates the range selection.
    pub range_fn: fn() -> RangeSelection,

    /// Optional function to check if a range matches this input range.
    pub is_selected: Option<fn(&RangeSelection) -> bool>,
}

impl InputRange {
    /// Creates a new input range with a label and factory function.
    ///
    /// # Parameters
    ///
    /// - `label`: The display label.
    /// - `range_fn`: A function that produces the date range.
    ///
    /// # Returns
    ///
    /// - A new `InputRange` instance.
    pub fn new(label: &str, range_fn: fn() -> RangeSelection) -> Self {
        Self {
            label: label.into(),
            range_fn,
            is_selected: None,
        }
    }
}
