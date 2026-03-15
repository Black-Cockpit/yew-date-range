/// Rendering mode for the date picker.
///
/// Controls whether the calendar is always visible or opens
/// as a popup overlay triggered by user interaction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DisplayMode {
    /// Calendar is always visible (embedded).
    #[default]
    Inline,

    /// Calendar opens as a popup overlay triggered by an input.
    Popup,
}
