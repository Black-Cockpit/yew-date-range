//! Tests for range helper utility functionality.

use time::macros::date;
use time::Weekday;

use yew_date_range_core::models::range_selection::RangeSelection;
use yew_date_range_core::utils::range_helper::RangeHelper;

/// Validates that is_date_disabled rejects dates before the min_date and accepts dates on the boundary.
#[test]
fn test_is_date_disabled_min() {
    let d = date!(2024 - 01 - 05);
    assert!(RangeHelper::is_date_disabled(d, Some(date!(2024 - 01 - 10)), None, &[], None));
    assert!(!RangeHelper::is_date_disabled(d, Some(date!(2024 - 01 - 05)), None, &[], None));
}

/// Validates that is_date_disabled rejects dates after the max_date and accepts dates on the boundary.
#[test]
fn test_is_date_disabled_max() {
    let d = date!(2024 - 01 - 15);
    assert!(RangeHelper::is_date_disabled(d, None, Some(date!(2024 - 01 - 10)), &[], None));
    assert!(!RangeHelper::is_date_disabled(d, None, Some(date!(2024 - 01 - 15)), &[], None));
}

/// Validates that is_date_disabled rejects dates present in the disabled dates list.
#[test]
fn test_is_date_disabled_specific() {
    let d = date!(2024 - 01 - 10);
    let disabled = vec![date!(2024 - 01 - 10), date!(2024 - 01 - 20)];
    assert!(RangeHelper::is_date_disabled(d, None, None, &disabled, None));
    assert!(!RangeHelper::is_date_disabled(date!(2024 - 01 - 11), None, None, &disabled, None));
}

/// Validates that is_date_disabled_with_weekdays rejects dates falling on disabled weekdays.
#[test]
fn test_is_date_disabled_with_weekdays() {
    let sunday = date!(2024 - 01 - 07);
    let monday = date!(2024 - 01 - 08);
    let disabled_days = vec![Weekday::Sunday, Weekday::Saturday];

    assert!(RangeHelper::is_date_disabled_with_weekdays(sunday, None, None, &[], &disabled_days, None));
    assert!(!RangeHelper::is_date_disabled_with_weekdays(monday, None, None, &[], &disabled_days, None));
}

/// Validates that is_range_selected correctly compares two ranges by their normalized boundaries.
#[test]
fn test_is_range_selected() {
    let a = RangeSelection::new("s").with_dates(
        Some(date!(2024 - 01 - 01)),
        Some(date!(2024 - 01 - 31)),
    );
    let b = RangeSelection::new("s").with_dates(
        Some(date!(2024 - 01 - 01)),
        Some(date!(2024 - 01 - 31)),
    );
    let c = RangeSelection::new("s").with_dates(
        Some(date!(2024 - 02 - 01)),
        Some(date!(2024 - 02 - 28)),
    );
    assert!(RangeHelper::is_range_selected(&a, &b));
    assert!(!RangeHelper::is_range_selected(&a, &c));
}

/// Validates that compute_preview returns None when no hover date is provided.
#[test]
fn test_compute_preview_no_hover() {
    let ranges = vec![RangeSelection::new("s").with_dates(Some(date!(2024 - 01 - 10)), None)];
    assert_eq!(RangeHelper::compute_preview(None, &ranges, 0, false, true), None);
}

/// Validates that compute_preview returns the correct range when focusing on the end date.
#[test]
fn test_compute_preview_end_focus() {
    let ranges = vec![RangeSelection::new("s").with_dates(Some(date!(2024 - 01 - 10)), None)];
    let result = RangeHelper::compute_preview(Some(date!(2024 - 01 - 20)), &ranges, 0, false, true);
    assert_eq!(result, Some((date!(2024 - 01 - 10), date!(2024 - 01 - 20))));
}

/// Validates that compute_preview returns None when the user is not actively selecting.
#[test]
fn test_compute_preview_not_selecting() {
    let ranges = vec![RangeSelection::new("s").with_dates(
        Some(date!(2024 - 01 - 10)),
        Some(date!(2024 - 01 - 20)),
    )];
    let result = RangeHelper::compute_preview(Some(date!(2024 - 01 - 15)), &ranges, 0, true, false);
    assert_eq!(result, None);
}

/// Validates that is_date_disabled_full rejects dates inside disabled date ranges including boundaries.
#[test]
fn test_is_date_disabled_full_with_ranges() {
    let ranges = vec![
        (date!(2024 - 03 - 10), date!(2024 - 03 - 15)),
        (date!(2024 - 04 - 01), date!(2024 - 04 - 05)),
    ];
    assert!(RangeHelper::is_date_disabled_full(date!(2024 - 03 - 12), None, None, &[], &[], &ranges, None));
    assert!(RangeHelper::is_date_disabled_full(date!(2024 - 03 - 10), None, None, &[], &[], &ranges, None));
    assert!(RangeHelper::is_date_disabled_full(date!(2024 - 03 - 15), None, None, &[], &[], &ranges, None));
    assert!(!RangeHelper::is_date_disabled_full(date!(2024 - 03 - 09), None, None, &[], &[], &ranges, None));
    assert!(!RangeHelper::is_date_disabled_full(date!(2024 - 03 - 16), None, None, &[], &[], &ranges, None));
    assert!(RangeHelper::is_date_disabled_full(date!(2024 - 04 - 03), None, None, &[], &[], &ranges, None));
}

/// Validates that is_date_disabled_full handles reversed disabled ranges correctly.
#[test]
fn test_is_date_disabled_full_reversed_range() {
    let ranges = vec![(date!(2024 - 03 - 15), date!(2024 - 03 - 10))];
    assert!(RangeHelper::is_date_disabled_full(date!(2024 - 03 - 12), None, None, &[], &[], &ranges, None));
}

/// Validates that is_date_disabled_full respects a custom disabled function.
#[test]
fn test_is_date_disabled_full_with_custom_fn() {
    let custom_fn = |d: time::Date| d.month() == time::Month::February;
    assert!(RangeHelper::is_date_disabled_full(
        date!(2024 - 02 - 15), None, None, &[], &[], &[],
        Some(&custom_fn)
    ));
    assert!(!RangeHelper::is_date_disabled_full(
        date!(2024 - 03 - 15), None, None, &[], &[], &[],
        Some(&custom_fn)
    ));
}

/// Validates that is_date_disabled_full correctly evaluates all constraint types together.
#[test]
fn test_is_date_disabled_full_combined_constraints() {
    let disabled_dates = vec![date!(2024 - 01 - 25)];
    let disabled_weekdays = vec![Weekday::Sunday];
    let disabled_ranges = vec![(date!(2024 - 01 - 10), date!(2024 - 01 - 15))];

    assert!(RangeHelper::is_date_disabled_full(
        date!(2024 - 01 - 12), None, None, &disabled_dates, &disabled_weekdays, &disabled_ranges, None
    ));
    assert!(RangeHelper::is_date_disabled_full(
        date!(2024 - 01 - 25), None, None, &disabled_dates, &disabled_weekdays, &disabled_ranges, None
    ));
    assert!(RangeHelper::is_date_disabled_full(
        date!(2024 - 01 - 07), None, None, &disabled_dates, &disabled_weekdays, &disabled_ranges, None
    ));
    assert!(!RangeHelper::is_date_disabled_full(
        date!(2024 - 01 - 08), None, None, &disabled_dates, &disabled_weekdays, &disabled_ranges, None
    ));
}

/// Validates that is_span_valid returns true when no min or max span constraints are set.
#[test]
fn test_is_span_valid_no_constraints() {
    assert!(RangeHelper::is_span_valid(date!(2024 - 01 - 01), date!(2024 - 12 - 31), None, None));
}

/// Validates that is_span_valid rejects spans exceeding the max_span constraint.
#[test]
fn test_is_span_valid_max_span() {
    assert!(RangeHelper::is_span_valid(date!(2024 - 01 - 01), date!(2024 - 01 - 10), None, Some(10)));
    assert!(!RangeHelper::is_span_valid(date!(2024 - 01 - 01), date!(2024 - 01 - 11), None, Some(10)));
}

/// Validates that is_span_valid rejects spans shorter than the min_span constraint.
#[test]
fn test_is_span_valid_min_span() {
    assert!(RangeHelper::is_span_valid(date!(2024 - 01 - 01), date!(2024 - 01 - 03), Some(3), None));
    assert!(!RangeHelper::is_span_valid(date!(2024 - 01 - 01), date!(2024 - 01 - 02), Some(3), None));
}

/// Validates that is_span_valid enforces both min and max span constraints simultaneously.
#[test]
fn test_is_span_valid_both_constraints() {
    assert!(RangeHelper::is_span_valid(date!(2024 - 01 - 01), date!(2024 - 01 - 05), Some(3), Some(7)));
    assert!(!RangeHelper::is_span_valid(date!(2024 - 01 - 01), date!(2024 - 01 - 02), Some(3), Some(7)));
    assert!(!RangeHelper::is_span_valid(date!(2024 - 01 - 01), date!(2024 - 01 - 10), Some(3), Some(7)));
}

/// Validates that is_span_valid handles a single-day span correctly with both min and max.
#[test]
fn test_is_span_valid_single_day() {
    assert!(RangeHelper::is_span_valid(date!(2024 - 01 - 01), date!(2024 - 01 - 01), None, Some(1)));
    assert!(RangeHelper::is_span_valid(date!(2024 - 01 - 01), date!(2024 - 01 - 01), Some(1), None));
}

/// Validates that is_span_valid handles reversed dates by using the absolute difference.
#[test]
fn test_is_span_valid_reversed_dates() {
    assert!(RangeHelper::is_span_valid(date!(2024 - 01 - 10), date!(2024 - 01 - 01), None, Some(15)));
    assert!(!RangeHelper::is_span_valid(date!(2024 - 01 - 10), date!(2024 - 01 - 01), None, Some(5)));
}

/// Validates that find_focused_range returns the index of the first auto-focused non-disabled range.
#[test]
fn test_find_focused_range_found() {
    let ranges = vec![
        RangeSelection { auto_focus: false, disabled: false, ..RangeSelection::new("a") },
        RangeSelection { auto_focus: true, disabled: false, ..RangeSelection::new("b") },
    ];
    assert_eq!(RangeHelper::find_focused_range(&ranges), Some(1));
}

/// Validates that find_focused_range skips disabled ranges even if they have auto_focus enabled.
#[test]
fn test_find_focused_range_disabled_skipped() {
    let ranges = vec![
        RangeSelection { auto_focus: true, disabled: true, ..RangeSelection::new("a") },
        RangeSelection { auto_focus: true, disabled: false, ..RangeSelection::new("b") },
    ];
    assert_eq!(RangeHelper::find_focused_range(&ranges), Some(1));
}

/// Validates that find_focused_range returns None when no range has auto_focus enabled.
#[test]
fn test_find_focused_range_none() {
    let ranges = vec![
        RangeSelection { auto_focus: false, disabled: false, ..RangeSelection::new("a") },
    ];
    assert_eq!(RangeHelper::find_focused_range(&ranges), None);
}

/// Validates that find_focused_range returns None for an empty range list.
#[test]
fn test_find_focused_range_empty() {
    let ranges: Vec<RangeSelection> = vec![];
    assert_eq!(RangeHelper::find_focused_range(&ranges), None);
}

/// Validates that compute_preview with start focus shows a preview from hover to the existing end date.
#[test]
fn test_compute_preview_start_focus_with_end() {
    let ranges = vec![RangeSelection::new("s").with_dates(None, Some(date!(2024 - 01 - 20)))];
    let result = RangeHelper::compute_preview(Some(date!(2024 - 01 - 10)), &ranges, 0, true, true);
    assert_eq!(result, Some((date!(2024 - 01 - 10), date!(2024 - 01 - 20))));
}

/// Validates that compute_preview with start focus normalizes when hover is after the end date.
#[test]
fn test_compute_preview_start_focus_hover_after_end() {
    let ranges = vec![RangeSelection::new("s").with_dates(None, Some(date!(2024 - 01 - 10)))];
    let result = RangeHelper::compute_preview(Some(date!(2024 - 01 - 20)), &ranges, 0, true, true);
    assert_eq!(result, Some((date!(2024 - 01 - 10), date!(2024 - 01 - 20))));
}

/// Validates that compute_preview with start focus and no end date produces a single-day preview.
#[test]
fn test_compute_preview_start_focus_no_end() {
    let ranges = vec![RangeSelection::new("s").with_dates(None, None)];
    let result = RangeHelper::compute_preview(Some(date!(2024 - 01 - 15)), &ranges, 0, true, true);
    assert_eq!(result, Some((date!(2024 - 01 - 15), date!(2024 - 01 - 15))));
}

/// Validates that compute_preview with end focus normalizes when hover is before the start date.
#[test]
fn test_compute_preview_end_focus_hover_before_start() {
    let ranges = vec![RangeSelection::new("s").with_dates(Some(date!(2024 - 01 - 20)), None)];
    let result = RangeHelper::compute_preview(Some(date!(2024 - 01 - 10)), &ranges, 0, false, true);
    assert_eq!(result, Some((date!(2024 - 01 - 10), date!(2024 - 01 - 20))));
}

/// Validates that compute_preview with end focus and no start date produces a single-day preview.
#[test]
fn test_compute_preview_end_focus_no_start() {
    let ranges = vec![RangeSelection::new("s").with_dates(None, None)];
    let result = RangeHelper::compute_preview(Some(date!(2024 - 01 - 15)), &ranges, 0, false, true);
    assert_eq!(result, Some((date!(2024 - 01 - 15), date!(2024 - 01 - 15))));
}

/// Validates that compute_preview returns None when the focused range index is out of bounds.
#[test]
fn test_compute_preview_invalid_index() {
    let ranges = vec![RangeSelection::new("s")];
    let result = RangeHelper::compute_preview(Some(date!(2024 - 01 - 15)), &ranges, 5, false, true);
    assert_eq!(result, None);
}

/// Validates that is_date_disabled returns false when no constraints are provided.
#[test]
fn test_is_date_disabled_no_constraints() {
    assert!(!RangeHelper::is_date_disabled(date!(2024 - 06 - 15), None, None, &[], None));
}

/// Validates that is_date_disabled_full returns false when no constraints are provided.
#[test]
fn test_is_date_disabled_full_no_constraints() {
    assert!(!RangeHelper::is_date_disabled_full(date!(2024 - 06 - 15), None, None, &[], &[], &[], None));
}
