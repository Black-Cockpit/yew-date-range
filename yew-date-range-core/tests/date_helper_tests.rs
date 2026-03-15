//! Tests for date helper utility functionality.

use time::macros::date;
use time::{Month, Weekday};

use yew_date_range_core::utils::date_helper::DateHelper;

/// Validates that days_in_month returns the correct day count for various months and years.
#[test]
fn test_days_in_month() {
    assert_eq!(DateHelper::days_in_month(2024, Month::January), 31);
    assert_eq!(DateHelper::days_in_month(2024, Month::February), 29);
    assert_eq!(DateHelper::days_in_month(2023, Month::February), 28);
    assert_eq!(DateHelper::days_in_month(2024, Month::April), 30);
    assert_eq!(DateHelper::days_in_month(2024, Month::June), 30);
    assert_eq!(DateHelper::days_in_month(2024, Month::December), 31);
}

/// Validates that start_of_month always returns the first day of the same month.
#[test]
fn test_start_of_month() {
    assert_eq!(DateHelper::start_of_month(date!(2024 - 03 - 15)), date!(2024 - 03 - 01));
    assert_eq!(DateHelper::start_of_month(date!(2024 - 01 - 01)), date!(2024 - 01 - 01));
    assert_eq!(DateHelper::start_of_month(date!(2024 - 12 - 31)), date!(2024 - 12 - 01));
}

/// Validates that end_of_month returns the last day of the same month, including leap years.
#[test]
fn test_end_of_month() {
    assert_eq!(DateHelper::end_of_month(date!(2024 - 03 - 15)), date!(2024 - 03 - 31));
    assert_eq!(DateHelper::end_of_month(date!(2024 - 02 - 01)), date!(2024 - 02 - 29));
    assert_eq!(DateHelper::end_of_month(date!(2023 - 02 - 01)), date!(2023 - 02 - 28));
}

/// Validates that add_months advances the month correctly and clamps the day when needed.
#[test]
fn test_add_months() {
    // Adding one month to Jan 31 should clamp to Feb 29 in a leap year.
    let d = date!(2024 - 01 - 31);
    assert_eq!(DateHelper::add_months(d, 1), date!(2024 - 02 - 29));
    assert_eq!(DateHelper::add_months(d, 2), date!(2024 - 03 - 31));
    assert_eq!(DateHelper::add_months(d, 12), date!(2025 - 01 - 31));
}

/// Validates that sub_months subtracts months correctly and clamps the day when needed.
#[test]
fn test_sub_months() {
    // Subtracting one month from Mar 31 should clamp to Feb 29 in a leap year.
    let d = date!(2024 - 03 - 31);
    assert_eq!(DateHelper::sub_months(d, 1), date!(2024 - 02 - 29));
    assert_eq!(DateHelper::sub_months(d, 3), date!(2023 - 12 - 31));
}

/// Validates that add_days correctly adds positive and negative day offsets.
#[test]
fn test_add_days() {
    assert_eq!(DateHelper::add_days(date!(2024 - 01 - 01), 10), date!(2024 - 01 - 11));
    assert_eq!(DateHelper::add_days(date!(2024 - 01 - 31), 1), date!(2024 - 02 - 01));
    assert_eq!(DateHelper::add_days(date!(2024 - 03 - 01), -1), date!(2024 - 02 - 29));
}

/// Validates that diff_days returns the correct signed difference in calendar days.
#[test]
fn test_diff_days() {
    assert_eq!(DateHelper::diff_days(date!(2024 - 01 - 10), date!(2024 - 01 - 01)), 9);
    assert_eq!(DateHelper::diff_days(date!(2024 - 01 - 01), date!(2024 - 01 - 10)), -9);
    assert_eq!(DateHelper::diff_days(date!(2024 - 01 - 01), date!(2024 - 01 - 01)), 0);
}

/// Validates that diff_months returns the correct signed difference in calendar months.
#[test]
fn test_diff_months() {
    assert_eq!(DateHelper::diff_months(date!(2024 - 06 - 15), date!(2024 - 01 - 15)), 5);
    assert_eq!(
        DateHelper::diff_months(date!(2025 - 01 - 01), date!(2024 - 01 - 01)),
        12
    );
    assert_eq!(
        DateHelper::diff_months(date!(2024 - 01 - 01), date!(2024 - 06 - 01)),
        -5
    );
}

/// Validates that is_same_day returns true only when both options contain the same date.
#[test]
fn test_is_same_day() {
    assert!(DateHelper::is_same_day(
        Some(date!(2024 - 01 - 01)),
        Some(date!(2024 - 01 - 01))
    ));
    assert!(!DateHelper::is_same_day(
        Some(date!(2024 - 01 - 01)),
        Some(date!(2024 - 01 - 02))
    ));
    assert!(!DateHelper::is_same_day(Some(date!(2024 - 01 - 01)), None));
    assert!(!DateHelper::is_same_day(None, Some(date!(2024 - 01 - 01))));
    assert!(!DateHelper::is_same_day(None, None));
}

/// Validates that is_before returns true only when the first date precedes the second.
#[test]
fn test_is_before() {
    assert!(DateHelper::is_before(date!(2024 - 01 - 01), date!(2024 - 01 - 02)));
    assert!(!DateHelper::is_before(date!(2024 - 01 - 02), date!(2024 - 01 - 01)));
    assert!(!DateHelper::is_before(date!(2024 - 01 - 01), date!(2024 - 01 - 01)));
}

/// Validates that is_after returns true only when the first date follows the second.
#[test]
fn test_is_after() {
    assert!(DateHelper::is_after(date!(2024 - 01 - 02), date!(2024 - 01 - 01)));
    assert!(!DateHelper::is_after(date!(2024 - 01 - 01), date!(2024 - 01 - 02)));
    assert!(!DateHelper::is_after(date!(2024 - 01 - 01), date!(2024 - 01 - 01)));
}

/// Validates that is_between correctly detects dates inside an inclusive range, including reversed boundaries.
#[test]
fn test_is_between() {
    let start = date!(2024 - 01 - 01);
    let end = date!(2024 - 01 - 31);

    // Mid-range, on boundaries, outside, and reversed boundaries.
    assert!(DateHelper::is_between(date!(2024 - 01 - 15), start, end));
    assert!(DateHelper::is_between(start, start, end));
    assert!(DateHelper::is_between(end, start, end));
    assert!(!DateHelper::is_between(date!(2024 - 02 - 01), start, end));
    assert!(DateHelper::is_between(date!(2024 - 01 - 15), end, start));
}

/// Validates that is_weekend returns true for Saturday and Sunday only.
#[test]
fn test_is_weekend() {
    // 2024-01-06 is Saturday, 2024-01-07 is Sunday.
    assert!(DateHelper::is_weekend(date!(2024 - 01 - 06)));
    assert!(DateHelper::is_weekend(date!(2024 - 01 - 07)));
    assert!(!DateHelper::is_weekend(date!(2024 - 01 - 08)));
    assert!(!DateHelper::is_weekend(date!(2024 - 01 - 10)));
}

/// Validates that start_of_week returns the correct first day of the week for different week starts.
#[test]
fn test_start_of_week() {
    // 2024-01-10 is Wednesday.
    let sunday_start = DateHelper::start_of_week(date!(2024 - 01 - 10), Weekday::Sunday);
    assert_eq!(sunday_start, date!(2024 - 01 - 07));

    let monday_start = DateHelper::start_of_week(date!(2024 - 01 - 10), Weekday::Monday);
    assert_eq!(monday_start, date!(2024 - 01 - 08));
}

/// Validates that end_of_week returns the correct last day of the week for different week starts.
#[test]
fn test_end_of_week() {
    // 2024-01-10 is Wednesday.
    let sunday_end = DateHelper::end_of_week(date!(2024 - 01 - 10), Weekday::Sunday);
    assert_eq!(sunday_end, date!(2024 - 01 - 13));

    let monday_end = DateHelper::end_of_week(date!(2024 - 01 - 10), Weekday::Monday);
    assert_eq!(monday_end, date!(2024 - 01 - 14));
}

/// Validates that week_number returns a valid ISO week number within the 1-53 range.
#[test]
fn test_week_number() {
    let wn = DateHelper::week_number(date!(2024 - 01 - 01));
    assert!((1..=53).contains(&wn));
}

/// Validates that add_months_to_ym correctly advances and wraps year-month pairs including negative offsets.
#[test]
fn test_add_months_to_ym() {
    // Forward within same year.
    let (y, m) = DateHelper::add_months_to_ym(2024, Month::January, 1);
    assert_eq!(y, 2024);
    assert_eq!(m, Month::February);

    // Forward across year boundary.
    let (y, m) = DateHelper::add_months_to_ym(2024, Month::December, 1);
    assert_eq!(y, 2025);
    assert_eq!(m, Month::January);

    // Backward across year boundary.
    let (y, m) = DateHelper::add_months_to_ym(2024, Month::January, -1);
    assert_eq!(y, 2023);
    assert_eq!(m, Month::December);
}

/// Validates that index_to_month maps zero-based indices to the correct Month enum, wrapping on overflow.
#[test]
fn test_index_to_month() {
    assert_eq!(DateHelper::index_to_month(0), Month::January);
    assert_eq!(DateHelper::index_to_month(5), Month::June);
    assert_eq!(DateHelper::index_to_month(11), Month::December);
    assert_eq!(DateHelper::index_to_month(12), Month::January);
}

/// Validates that weekday_to_index maps Weekday to the correct zero-based index with Sunday as 0.
#[test]
fn test_weekday_to_index() {
    assert_eq!(DateHelper::weekday_to_index(Weekday::Sunday), 0);
    assert_eq!(DateHelper::weekday_to_index(Weekday::Monday), 1);
    assert_eq!(DateHelper::weekday_to_index(Weekday::Saturday), 6);
}

/// Validates that month_to_index maps Month to the correct zero-based index.
#[test]
fn test_month_to_index() {
    assert_eq!(DateHelper::month_to_index(Month::January), 0);
    assert_eq!(DateHelper::month_to_index(Month::June), 5);
    assert_eq!(DateHelper::month_to_index(Month::December), 11);
}

/// Validates that format_date produces a zero-padded YYYY-MM-DD string.
#[test]
fn test_format_date() {
    assert_eq!(DateHelper::format_date(date!(2024 - 01 - 05)), "2024-01-05");
    assert_eq!(DateHelper::format_date(date!(2024 - 12 - 31)), "2024-12-31");
}

/// Validates that parse_date correctly parses valid YYYY-MM-DD strings and returns None for invalid input.
#[test]
fn test_parse_date() {
    assert_eq!(DateHelper::parse_date("2024-06-15"), Some(date!(2024 - 06 - 15)));
    assert_eq!(DateHelper::parse_date("invalid"), None);
    assert_eq!(DateHelper::parse_date("abc-de-fg"), None);
    assert_eq!(DateHelper::parse_date("2024-02-30"), None);
    assert_eq!(DateHelper::parse_date(""), None);
}

/// Validates that format_date_display produces an abbreviated "Mon D, YYYY" display string.
#[test]
fn test_format_date_display() {
    // Build a full set of English month names.
    let month_names: Vec<String> = vec![
        "January".into(),
        "February".into(),
        "March".into(),
        "April".into(),
        "May".into(),
        "June".into(),
        "July".into(),
        "August".into(),
        "September".into(),
        "October".into(),
        "November".into(),
        "December".into(),
    ];
    assert_eq!(
        DateHelper::format_date_display(date!(2024 - 01 - 15), &month_names),
        "Jan 15, 2024"
    );
    assert_eq!(
        DateHelper::format_date_display(date!(2024 - 12 - 01), &month_names),
        "Dec 1, 2024"
    );
}

/// Validates that format_date_display falls back to "???" when month names are empty.
#[test]
fn test_format_date_display_empty_names() {
    let empty: Vec<String> = vec![];
    assert_eq!(
        DateHelper::format_date_display(date!(2024 - 01 - 15), &empty),
        "??? 15, 2024"
    );
}

/// Validates that validate_against_constraints correctly accepts and rejects dates based on min/max bounds.
#[test]
fn test_validate_against_constraints() {
    let d = date!(2024 - 06 - 15);

    // No constraints should pass.
    assert!(DateHelper::validate_against_constraints(d, None, None));

    // Within bounds should pass.
    assert!(DateHelper::validate_against_constraints(
        d,
        Some(date!(2024 - 01 - 01)),
        None
    ));
    assert!(DateHelper::validate_against_constraints(
        d,
        None,
        Some(date!(2024 - 12 - 31))
    ));

    // Outside bounds should fail.
    assert!(!DateHelper::validate_against_constraints(
        d,
        Some(date!(2024 - 07 - 01)),
        None
    ));
    assert!(!DateHelper::validate_against_constraints(
        d,
        None,
        Some(date!(2024 - 05 - 31))
    ));

    // Exact boundary match should pass.
    assert!(DateHelper::validate_against_constraints(d, Some(d), Some(d)));
}

/// Validates that build_month_data produces a valid grid with 4-6 weeks of 7-day rows.
#[test]
fn test_build_month_data() {
    let data = DateHelper::build_month_data(2024, Month::January, Weekday::Sunday);
    assert_eq!(data.year, 2024);
    assert_eq!(data.month, Month::January);
    assert!(!data.weeks.is_empty());
    assert!(data.weeks.len() >= 4 && data.weeks.len() <= 6);

    // Each week row must have exactly 7 day slots.
    for week in &data.weeks {
        assert_eq!(week.days.len(), 7);
    }
}

/// Validates that get_months_to_display generates the correct sequence of consecutive months.
#[test]
fn test_get_months_to_display() {
    let months = DateHelper::get_months_to_display(2024, Month::January, 3, Weekday::Sunday);
    assert_eq!(months.len(), 3);
    assert_eq!(months[0].month, Month::January);
    assert_eq!(months[1].month, Month::February);
    assert_eq!(months[2].month, Month::March);
}

/// Validates that get_months_to_display works correctly for a single month request.
#[test]
fn test_get_months_to_display_single() {
    let months = DateHelper::get_months_to_display(2024, Month::December, 1, Weekday::Monday);
    assert_eq!(months.len(), 1);
    assert_eq!(months[0].month, Month::December);
}
