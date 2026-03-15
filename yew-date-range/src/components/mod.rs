/// Base calendar component with day, month, and year views.
///
/// Displays one or more months with day cells and supports
/// drill-down navigation between days, months, and years.
pub mod calendar;

/// Internal view mode for calendar drill-down navigation.
///
/// Controls which level of the calendar hierarchy is currently
/// displayed (days, months, or years).
pub(crate) mod calendar_view;

/// Internal rendering utilities for the calendar component.
///
/// Provides static methods for rendering month grids, day cells,
/// month picker grids, and year picker grids.
pub(crate) mod calendar_renderer;

/// Full-featured date picker with inline and popup modes.
///
/// Supports single date, date range, and multiple date selection
/// with optional time picking and text input.
pub mod date_picker;

/// Calendar component with range selection.
///
/// Provides a calendar view focused on selecting date ranges
/// with hover preview and focus tracking.
pub mod date_range;

/// Combined date range picker with sidebar.
///
/// Combines a defined range sidebar with a date range calendar
/// for a complete range selection experience.
pub mod date_range_picker;

/// Sidebar with predefined date ranges.
///
/// Displays a list of commonly used date ranges (Today, This Week, etc.)
/// for quick selection.
pub mod defined_range;

/// Positioned popup overlay component.
///
/// Renders content in a positioned popup with support for
/// outside click and escape key dismissal.
pub mod overlay;

/// Time picker component for hours, minutes, and seconds.
///
/// Supports both 12-hour and 24-hour formats with configurable
/// granularity for which time fields are shown.
pub mod time_picker;
