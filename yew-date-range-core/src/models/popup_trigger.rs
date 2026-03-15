/// Trigger mode for opening the popup.
///
/// Determines which user interaction opens the date picker popup
/// when the display mode is set to popup.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopupTrigger {
    /// Open on input click.
    #[default]
    Click,

    /// Open on input focus.
    Focus,

    /// Open on both click and focus.
    Both,
}
