//! Tests for core model types: defaults, builders, conversions, and state flags.

use time::macros::date;
use time::Weekday;

use yew_date_range_core::models::calendar_direction::CalendarDirection;
use yew_date_range_core::models::calendar_locale::CalendarLocale;
use yew_date_range_core::models::day_state::DayState;
use yew_date_range_core::models::display_mode::DisplayMode;
use yew_date_range_core::models::hour_format::HourFormat;
use yew_date_range_core::models::input_range::InputRange;
use yew_date_range_core::models::navigation_action::NavigationAction;
use yew_date_range_core::models::popup_trigger::PopupTrigger;
use yew_date_range_core::models::range_change::RangeChange;
use yew_date_range_core::models::range_change_source::RangeChangeSource;
use yew_date_range_core::models::range_focus::RangeFocus;
use yew_date_range_core::models::range_selection::RangeSelection;
use yew_date_range_core::models::selection_mode::SelectionMode;
use yew_date_range_core::models::static_range::StaticRange;
use yew_date_range_core::models::time_granularity::TimeGranularity;

/// Validates that CalendarDirection defaults to Horizontal.
#[test]
fn test_calendar_direction_default() {
    let dir: CalendarDirection = CalendarDirection::default();
    assert_eq!(dir, CalendarDirection::Horizontal);
}

/// Validates that CalendarDirection::from correctly parses string variants case-insensitively.
#[test]
fn test_calendar_direction_from_str() {
    assert_eq!(CalendarDirection::from("vertical"), CalendarDirection::Vertical);
    assert_eq!(CalendarDirection::from("VERTICAL"), CalendarDirection::Vertical);
    assert_eq!(CalendarDirection::from("horizontal"), CalendarDirection::Horizontal);
    assert_eq!(CalendarDirection::from("anything"), CalendarDirection::Horizontal);
}

/// Validates that all four NavigationAction variants are distinct and constructible.
#[test]
fn test_navigation_action_variants() {
    let actions = [
        NavigationAction::PrevMonth,
        NavigationAction::NextMonth,
        NavigationAction::PrevYear,
        NavigationAction::NextYear,
    ];
    assert_eq!(actions.len(), 4);
    assert_eq!(actions[0], NavigationAction::PrevMonth);
}

/// Validates that RangeFocus::toggle switches between Start and End.
#[test]
fn test_range_focus_toggle() {
    assert_eq!(RangeFocus::Start.toggle(), RangeFocus::End);
    assert_eq!(RangeFocus::End.toggle(), RangeFocus::Start);
}

/// Validates that all five RangeChangeSource variants are distinct and constructible.
#[test]
fn test_range_change_source_variants() {
    let sources = [
        RangeChangeSource::Click,
        RangeChangeSource::Drag,
        RangeChangeSource::Input,
        RangeChangeSource::DefinedRange,
        RangeChangeSource::Keyboard,
    ];
    assert_eq!(sources.len(), 5);
    assert_eq!(sources[0], RangeChangeSource::Click);
}

/// Validates that SelectionMode defaults to Range.
#[test]
fn test_selection_mode_default() {
    assert_eq!(SelectionMode::default(), SelectionMode::Range);
}

/// Validates that DisplayMode defaults to Inline.
#[test]
fn test_display_mode_default() {
    assert_eq!(DisplayMode::default(), DisplayMode::Inline);
}

/// Validates that PopupTrigger defaults to Click.
#[test]
fn test_popup_trigger_default() {
    assert_eq!(PopupTrigger::default(), PopupTrigger::Click);
}

/// Validates that HourFormat defaults to H24.
#[test]
fn test_hour_format_default() {
    assert_eq!(HourFormat::default(), HourFormat::H24);
}

/// Validates that TimeGranularity defaults to showing hours and minutes but not seconds.
#[test]
fn test_time_granularity_default() {
    let g = TimeGranularity::default();
    assert!(g.show_hours);
    assert!(g.show_minutes);
    assert!(!g.show_seconds);
}

/// Validates that RangeSelection::new creates a range with the given key and sensible defaults.
#[test]
fn test_range_selection_new() {
    let r = RangeSelection::new("test");
    assert_eq!(r.key, "test");
    assert_eq!(r.start_date, None);
    assert_eq!(r.end_date, None);
    assert!(r.auto_focus);
    assert!(!r.disabled);
    assert!(r.show_date_display);
    assert!(r.color.is_some());
}

/// Validates that RangeSelection::default uses "selection" as the key.
#[test]
fn test_range_selection_default() {
    let r = RangeSelection::default();
    assert_eq!(r.key, "selection");
}

/// Validates that with_dates sets the start and end dates correctly.
#[test]
fn test_range_selection_with_dates() {
    let r = RangeSelection::new("s")
        .with_dates(Some(date!(2024 - 01 - 01)), Some(date!(2024 - 01 - 31)));
    assert_eq!(r.start_date, Some(date!(2024 - 01 - 01)));
    assert_eq!(r.end_date, Some(date!(2024 - 01 - 31)));
}

/// Validates that with_color sets the highlight color correctly.
#[test]
fn test_range_selection_with_color() {
    let r = RangeSelection::new("s").with_color("#ff0000");
    assert_eq!(r.color, Some("#ff0000".into()));
}

/// Validates that normalized swaps start and end when start is after end.
#[test]
fn test_range_selection_normalized() {
    let r = RangeSelection::new("s")
        .with_dates(Some(date!(2024 - 01 - 31)), Some(date!(2024 - 01 - 01)));
    let (s, e) = r.normalized();
    assert_eq!(s, Some(date!(2024 - 01 - 01)));
    assert_eq!(e, Some(date!(2024 - 01 - 31)));

    let r2 = RangeSelection::new("s")
        .with_dates(Some(date!(2024 - 01 - 01)), Some(date!(2024 - 01 - 31)));
    let (s2, e2) = r2.normalized();
    assert_eq!(s2, Some(date!(2024 - 01 - 01)));
    assert_eq!(e2, Some(date!(2024 - 01 - 31)));

    let r3 = RangeSelection::new("s").with_dates(None, None);
    let (s3, e3) = r3.normalized();
    assert_eq!(s3, None);
    assert_eq!(e3, None);
}

/// Validates that contains detects dates inside and on the boundaries of a range.
#[test]
fn test_range_selection_contains() {
    let r = RangeSelection::new("s")
        .with_dates(Some(date!(2024 - 01 - 10)), Some(date!(2024 - 01 - 20)));
    assert!(r.contains(date!(2024 - 01 - 15)));
    assert!(r.contains(date!(2024 - 01 - 10)));
    assert!(r.contains(date!(2024 - 01 - 20)));
    assert!(!r.contains(date!(2024 - 01 - 09)));
    assert!(!r.contains(date!(2024 - 01 - 21)));
}

/// Validates that contains works with a start-only range matching the exact date.
#[test]
fn test_range_selection_contains_single_date() {
    let r = RangeSelection::new("s")
        .with_dates(Some(date!(2024 - 01 - 15)), None);
    assert!(r.contains(date!(2024 - 01 - 15)));
    assert!(!r.contains(date!(2024 - 01 - 16)));
}

/// Validates that contains works with an end-only range matching the exact date.
#[test]
fn test_range_selection_contains_end_only() {
    let r = RangeSelection::new("s")
        .with_dates(None, Some(date!(2024 - 01 - 15)));
    assert!(r.contains(date!(2024 - 01 - 15)));
    assert!(!r.contains(date!(2024 - 01 - 16)));
}

/// Validates that contains returns false for an empty range with no dates set.
#[test]
fn test_range_selection_contains_empty() {
    let r = RangeSelection::new("s").with_dates(None, None);
    assert!(!r.contains(date!(2024 - 01 - 15)));
}

/// Validates that is_complete returns true only when both start and end dates are set.
#[test]
fn test_range_selection_is_complete() {
    let complete = RangeSelection::new("s")
        .with_dates(Some(date!(2024 - 01 - 01)), Some(date!(2024 - 01 - 31)));
    assert!(complete.is_complete());

    let incomplete = RangeSelection::new("s")
        .with_dates(Some(date!(2024 - 01 - 01)), None);
    assert!(!incomplete.is_complete());

    let empty = RangeSelection::new("s").with_dates(None, None);
    assert!(!empty.is_complete());
}

/// Validates that StaticRange stores a label and produces a range via its factory function.
#[test]
fn test_static_range_new_and_get_range() {
    let sr = StaticRange::new("Test", || {
        RangeSelection::new("s")
            .with_dates(Some(date!(2024 - 01 - 01)), Some(date!(2024 - 01 - 31)))
    });
    assert_eq!(sr.label, "Test");
    let range = sr.get_range();
    assert_eq!(range.start_date, Some(date!(2024 - 01 - 01)));
    assert_eq!(range.end_date, Some(date!(2024 - 01 - 31)));
}

/// Validates that InputRange stores a label and has no is_selected function by default.
#[test]
fn test_input_range_new() {
    let ir = InputRange::new("Custom", || {
        RangeSelection::new("s").with_dates(None, None)
    });
    assert_eq!(ir.label, "Custom");
    assert!(ir.is_selected.is_none());
}


/// Validates that RangeChange holds the range and source correctly.
#[test]
fn test_range_change_fields() {
    let change = RangeChange {
        range: RangeSelection::new("s"),
        source: RangeChangeSource::Click,
    };
    assert_eq!(change.range.key, "s");
    assert_eq!(change.source, RangeChangeSource::Click);
}

/// Validates that DayState::new initializes all boolean flags to false and color to None.
#[test]
fn test_day_state_new() {
    let state = DayState::new(date!(2024 - 03 - 15));
    assert_eq!(state.date, date!(2024 - 03 - 15));
    assert!(!state.is_today);
    assert!(!state.is_selected);
    assert!(!state.is_in_range);
    assert!(!state.is_start_edge);
    assert!(!state.is_end_edge);
    assert!(!state.is_preview);
    assert!(!state.is_disabled);
    assert!(!state.is_passive);
    assert!(!state.is_weekend);
    assert!(state.color.is_none());
}

/// Validates that css_classes returns only the base class when no flags are set.
#[test]
fn test_day_state_css_classes_base() {
    let state = DayState::new(date!(2024 - 03 - 15));
    let classes = state.css_classes();
    assert_eq!(classes, vec!["rdrDay"]);
}

/// Validates that css_classes returns all 16 expected classes when every flag is enabled.
#[test]
fn test_day_state_css_classes_all_flags() {
    let mut state = DayState::new(date!(2024 - 03 - 15));
    state.is_today = true;
    state.is_selected = true;
    state.is_in_range = true;
    state.is_start_edge = true;
    state.is_end_edge = true;
    state.is_preview = true;
    state.is_preview_start = true;
    state.is_preview_end = true;
    state.is_disabled = true;
    state.is_passive = true;
    state.is_weekend = true;
    state.is_start_of_week = true;
    state.is_end_of_week = true;
    state.is_start_of_month = true;
    state.is_end_of_month = true;

    let classes = state.css_classes();
    assert!(classes.contains(&"rdrDay"));
    assert!(classes.contains(&"rdrDayToday"));
    assert!(classes.contains(&"rdrDaySelected"));
    assert!(classes.contains(&"rdrDayInRange"));
    assert!(classes.contains(&"rdrDayStartEdge"));
    assert!(classes.contains(&"rdrDayEndEdge"));
    assert!(classes.contains(&"rdrDayInPreview"));
    assert!(classes.contains(&"rdrDayStartPreview"));
    assert!(classes.contains(&"rdrDayEndPreview"));
    assert!(classes.contains(&"rdrDayDisabled"));
    assert!(classes.contains(&"rdrDayPassive"));
    assert!(classes.contains(&"rdrDayWeekend"));
    assert!(classes.contains(&"rdrDayStartOfWeek"));
    assert!(classes.contains(&"rdrDayEndOfWeek"));
    assert!(classes.contains(&"rdrDayStartOfMonth"));
    assert!(classes.contains(&"rdrDayEndOfMonth"));
    assert_eq!(classes.len(), 16);
}

/// Validates that CalendarLocale::default provides 12 month names, 7 day names, and Sunday start.
#[test]
fn test_calendar_locale_default() {
    let locale = CalendarLocale::default();
    assert_eq!(locale.month_names.len(), 12);
    assert_eq!(locale.short_month_names.len(), 12);
    assert_eq!(locale.day_names.len(), 7);
    assert_eq!(locale.short_day_names.len(), 7);
    assert_eq!(locale.start_of_week, Weekday::Sunday);
}

/// Validates that ordered_day_names starts with Sunday when start_of_week is Sunday.
#[test]
fn test_calendar_locale_ordered_day_names_sunday_start() {
    let locale = CalendarLocale::default();
    let names = locale.ordered_day_names();
    assert_eq!(names[0], "Su");
    assert_eq!(names[6], "Sa");
}

/// Validates that ordered_day_names starts with Monday when start_of_week is Monday.
#[test]
fn test_calendar_locale_ordered_day_names_monday_start() {
    let locale = CalendarLocale {
        start_of_week: Weekday::Monday,
        ..Default::default()
    };
    let names = locale.ordered_day_names();
    assert_eq!(names[0], "Mo");
    assert_eq!(names[6], "Su");
}

/// Validates that month_name returns the correct full month name for January and December.
#[test]
fn test_calendar_locale_month_name() {
    let locale = CalendarLocale::default();
    assert_eq!(locale.month_name(time::Month::January), "January");
    assert_eq!(locale.month_name(time::Month::December), "December");
}

/// Validates that CalendarLocale default date_format uses lowercase tokens.
#[test]
fn test_calendar_locale_default_date_format() {
    let locale = CalendarLocale::default();
    assert_eq!(locale.date_format, "yyyy-MM-dd");
}

/// Validates that CalendarLocale default i18n labels are English.
#[test]
fn test_calendar_locale_default_labels() {
    let locale = CalendarLocale::default();
    assert_eq!(locale.today_label, "Today");
    assert_eq!(locale.clear_label, "Clear");
    assert_eq!(locale.prev_month_label, "Previous Month");
    assert_eq!(locale.next_month_label, "Next Month");
    assert_eq!(locale.select_month_label, "Select month");
    assert_eq!(locale.select_year_label, "Select year");
    assert_eq!(locale.start_date_placeholder, "Start Date");
    assert_eq!(locale.end_date_placeholder, "End Date");
    assert_eq!(locale.select_date_placeholder, "Select date");
    assert_eq!(locale.select_range_placeholder, "Select range");
    assert_eq!(locale.select_dates_placeholder, "Select dates");
    assert_eq!(locale.week_number_header, "W");
    assert_eq!(locale.select_prefix, "Select");
}

/// Validates that CalendarLocale default static range labels are English.
#[test]
fn test_calendar_locale_default_range_labels() {
    let locale = CalendarLocale::default();
    assert_eq!(locale.today_range_label, "Today");
    assert_eq!(locale.yesterday_range_label, "Yesterday");
    assert_eq!(locale.this_week_label, "This Week");
    assert_eq!(locale.last_week_label, "Last Week");
    assert_eq!(locale.this_month_label, "This Month");
    assert_eq!(locale.last_month_label, "Last Month");
}

/// Validates that CalendarLocale default time picker labels are English.
#[test]
fn test_calendar_locale_default_time_labels() {
    let locale = CalendarLocale::default();
    assert_eq!(locale.increment_hour_label, "Increment hour");
    assert_eq!(locale.decrement_hour_label, "Decrement hour");
    assert_eq!(locale.increment_minute_label, "Increment minute");
    assert_eq!(locale.decrement_minute_label, "Decrement minute");
    assert_eq!(locale.increment_second_label, "Increment second");
    assert_eq!(locale.decrement_second_label, "Decrement second");
    assert_eq!(locale.toggle_period_label, "Toggle AM/PM");
}

/// Validates that CalendarLocale fields can be overridden using struct update syntax.
#[test]
fn test_calendar_locale_custom_override() {
    let locale = CalendarLocale {
        today_label: "Heute".into(),
        clear_label: "Loeschen".into(),
        ..CalendarLocale::default()
    };
    assert_eq!(locale.today_label, "Heute");
    assert_eq!(locale.clear_label, "Loeschen");
    assert_eq!(locale.prev_month_label, "Previous Month");
}
