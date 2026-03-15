/// Focus index for range editing.
///
/// Indicates which end of a date range the user is currently
/// editing or about to select.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RangeFocus {
    /// Focusing on the start date of the range.
    Start,

    /// Focusing on the end date of the range.
    End,
}

impl RangeFocus {
    /// Toggles the focus between start and end.
    ///
    /// # Returns
    ///
    /// - `RangeFocus::End` if currently `Start`.
    /// - `RangeFocus::Start` if currently `End`.
    pub fn toggle(self) -> Self {
        // Swap to the opposite focus.
        match self {
            RangeFocus::Start => RangeFocus::End,
            RangeFocus::End => RangeFocus::Start,
        }
    }
}
