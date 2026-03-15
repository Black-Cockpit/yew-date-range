use std::fmt;
use std::rc::Rc;

use crate::models::range_selection::RangeSelection;

/// Represents a predefined/static range for the sidebar.
///
/// Provides a label and a factory closure that produces a
/// `RangeSelection` for quick range selection from the sidebar.
#[derive(Clone)]
pub struct StaticRange {
    /// The display label for this predefined range.
    pub label: String,

    /// Factory closure that creates the range selection.
    range_fn: Rc<dyn Fn() -> RangeSelection>,
}

impl fmt::Debug for StaticRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Only include the label in the debug output.
        f.debug_struct("StaticRange")
            .field("label", &self.label)
            .finish()
    }
}

impl StaticRange {
    /// Creates a new static range with a label and factory closure.
    ///
    /// # Parameters
    ///
    /// - `label`: The display label.
    /// - `range_fn`: A closure that produces the date range.
    ///
    /// # Returns
    ///
    /// - A new `StaticRange` instance.
    pub fn new<F>(label: &str, range_fn: F) -> Self
    where
        F: Fn() -> RangeSelection + 'static,
    {
        Self {
            label: label.into(),
            range_fn: Rc::new(range_fn),
        }
    }

    /// Invokes the factory closure to get the range selection.
    ///
    /// # Returns
    ///
    /// - A `RangeSelection` produced by the factory closure.
    pub fn get_range(&self) -> RangeSelection {
        // Call the stored factory closure.
        (self.range_fn)()
    }
}
