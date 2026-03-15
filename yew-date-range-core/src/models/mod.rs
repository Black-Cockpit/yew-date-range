/// Locale configuration for the calendar.
///
/// Provides localized month names, day names, and week start
/// configuration for rendering the calendar in different locales.
pub mod calendar_locale;

/// Direction for calendar layout.
///
/// Controls whether multiple months are displayed horizontally
/// or vertically in the calendar view.
pub mod calendar_direction;

/// Configurable date format for display and parsing.
///
/// Supports pattern-based formatting and parsing of dates using
/// configurable token patterns and separators.
pub mod date_format;

/// Visual and interactive state of a day cell.
///
/// Encapsulates all the flags needed by the calendar renderer
/// to determine how a particular day should be displayed.
pub mod day_state;

/// Rendering mode for the date picker.
///
/// Controls whether the calendar is always visible or opens
/// as a popup overlay triggered by user interaction.
pub mod display_mode;

/// Hour display format.
///
/// Controls whether the time picker displays hours in 24-hour
/// or 12-hour format with AM/PM indicator.
pub mod hour_format;

/// Editable date input range.
///
/// Provides a label and factory function for creating an editable
/// range selection from text input fields.
pub mod input_range;

/// Displayed month data for the calendar grid.
///
/// Contains the year, month, and all week rows needed to render
/// the calendar grid for a single month.
pub mod month_data;

/// Calendar navigation action.
///
/// Represents the possible navigation directions a user can trigger
/// when browsing through the calendar months and years.
pub mod navigation_action;

/// Trigger mode for opening the popup.
///
/// Determines which user interaction opens the date picker popup
/// when the display mode is set to popup.
pub mod popup_trigger;

/// Change event payload when a range is modified.
///
/// Emitted whenever a date range is updated, carrying the new range
/// and the source of the change.
pub mod range_change;

/// The source of a range change.
///
/// Identifies how a date range modification was triggered,
/// enabling consumers to differentiate between interaction types.
pub mod range_change_source;

/// Focus index for range editing.
///
/// Indicates which end of a date range the user is currently
/// editing or about to select.
pub mod range_focus;

/// Date range selection model.
///
/// Holds the start and end dates of a range along with display
/// configuration such as color, focus, and disabled state.
pub mod range_selection;

/// Selection mode for the date picker.
///
/// Determines the type of date selection the picker supports,
/// affecting both the UI behavior and the callback payload.
pub mod selection_mode;

/// Current selection value across all modes.
///
/// Unifies single date, date range, and multiple date selections
/// into a single enum for consistent handling in the date picker.
pub mod selection_value;

/// Predefined static range for the sidebar.
///
/// Provides a label and a factory function that produces a
/// range selection for quick range selection from the sidebar.
pub mod static_range;

/// Controls which time fields are visible.
///
/// Allows fine-grained control over which time components
/// the time picker displays.
pub mod time_granularity;

/// AM/PM period indicator.
///
/// Represents the time-of-day period used in 12-hour clock format.
pub mod time_period;

/// Time-of-day selection model.
///
/// Stores hour, minute, and second values with methods for
/// formatting and converting between clock formats.
pub mod time_selection;

/// Week row data for the calendar grid.
///
/// Contains the days for a single week and an optional ISO week number.
pub mod week_data;
