use crate::models::range_change_source::RangeChangeSource;
use crate::models::range_selection::RangeSelection;

/// Change event payload when a range is modified.
///
/// Emitted whenever a date range is updated, carrying the new range
/// and the source of the change for consumer logic.
#[derive(Clone, Debug)]
pub struct RangeChange {
    /// The updated range selection.
    pub range: RangeSelection,

    /// The source that triggered this change.
    pub source: RangeChangeSource,
}
