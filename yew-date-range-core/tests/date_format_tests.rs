//! Tests for date format parsing and formatting functionality.

use time::macros::date;

use yew_date_range_core::models::date_format::DateFormat;

/// Validates that format produces a correct yyyy-MM-dd output.
#[test]
fn test_format_yyyy_mm_dd() {
    let fmt = DateFormat::new("yyyy-MM-dd");
    assert_eq!(fmt.format(date!(2024 - 03 - 15)), "2024-03-15");
}

/// Validates that format produces a correct dd/MM/yyyy output.
#[test]
fn test_format_dd_mm_yyyy() {
    let fmt = DateFormat::new("dd/MM/yyyy");
    assert_eq!(fmt.format(date!(2024 - 03 - 15)), "15/03/2024");
}

/// Validates that format produces a correct MM-dd-yyyy output.
#[test]
fn test_format_mm_dd_yyyy() {
    let fmt = DateFormat::new("MM-dd-yyyy");
    assert_eq!(fmt.format(date!(2024 - 03 - 15)), "03-15-2024");
}

/// Validates that parse correctly round-trips a yyyy-MM-dd string.
#[test]
fn test_parse_yyyy_mm_dd() {
    let fmt = DateFormat::new("yyyy-MM-dd");
    assert_eq!(fmt.parse("2024-03-15"), Some(date!(2024 - 03 - 15)));
}

/// Validates that parse correctly round-trips a dd/MM/yyyy string.
#[test]
fn test_parse_dd_mm_yyyy() {
    let fmt = DateFormat::new("dd/MM/yyyy");
    assert_eq!(fmt.parse("15/03/2024"), Some(date!(2024 - 03 - 15)));
}

/// Validates that parse returns None for invalid inputs including out-of-range months and days.
#[test]
fn test_parse_invalid() {
    let fmt = DateFormat::new("yyyy-MM-dd");
    assert_eq!(fmt.parse("invalid"), None);
    assert_eq!(fmt.parse("2024-13-01"), None);
    assert_eq!(fmt.parse("2024-02-30"), None);
}

/// Validates that format_range produces a "start - end" string from two dates.
#[test]
fn test_format_range() {
    let fmt = DateFormat::new("yyyy-MM-dd");
    let result = fmt.format_range(Some(date!(2024 - 01 - 01)), Some(date!(2024 - 01 - 31)));
    assert_eq!(result, "2024-01-01 - 2024-01-31");
}

/// Validates that format_multiple joins formatted dates with the given separator.
#[test]
fn test_format_multiple() {
    let fmt = DateFormat::new("yyyy-MM-dd");
    let dates = vec![date!(2024 - 01 - 01), date!(2024 - 01 - 15)];
    assert_eq!(fmt.format_multiple(&dates, ", "), "2024-01-01, 2024-01-15");
}

/// Validates that the default DateFormat uses the yyyy-MM-dd pattern.
#[test]
fn test_default_pattern() {
    let fmt = DateFormat::default();
    assert_eq!(fmt.pattern, "yyyy-MM-dd");
    assert_eq!(fmt.format(date!(2024 - 03 - 15)), "2024-03-15");
}

/// Validates that format handles two-digit year tokens correctly.
#[test]
fn test_format_two_digit_year() {
    let fmt = DateFormat::new("yy-MM-dd");
    let result = fmt.format(date!(2024 - 03 - 15));
    assert_eq!(result, "24-03-15");
}

/// Validates that parse expands two-digit years by adding 2000.
#[test]
fn test_parse_two_digit_year() {
    let fmt = DateFormat::new("yy-MM-dd");
    let result = fmt.parse("24-03-15");
    assert_eq!(result, Some(date!(2024 - 03 - 15)));
}

/// Validates that parse returns None when the input has the wrong number of segments.
#[test]
fn test_parse_wrong_part_count() {
    let fmt = DateFormat::new("yyyy-MM-dd");
    assert_eq!(fmt.parse("2024-03"), None);
    assert_eq!(fmt.parse("2024-03-15-01"), None);
}

/// Validates that format_range handles partial boundaries with missing start or end.
#[test]
fn test_format_range_partial() {
    let fmt = DateFormat::new("yyyy-MM-dd");
    assert_eq!(fmt.format_range(Some(date!(2024 - 01 - 01)), None), "2024-01-01 - ");
    assert_eq!(fmt.format_range(None, Some(date!(2024 - 01 - 31))), " - 2024-01-31");
    assert_eq!(fmt.format_range(None, None), " - ");
}

/// Validates that format_multiple returns an empty string for an empty date list.
#[test]
fn test_format_multiple_empty() {
    let fmt = DateFormat::new("yyyy-MM-dd");
    let dates: Vec<time::Date> = vec![];
    assert_eq!(fmt.format_multiple(&dates, ", "), "");
}

/// Validates that single-character M and d tokens produce non-padded output.
#[test]
fn test_format_single_digit_month_day() {
    let fmt = DateFormat::new("M/d/yyyy");
    let result = fmt.format(date!(2024 - 03 - 05));
    assert_eq!(result, "3/5/2024");
}

/// Validates that parse recognizes dot separators auto-detected from the pattern.
#[test]
fn test_parse_with_dot_separator() {
    let fmt = DateFormat::new("dd.MM.yyyy");
    assert_eq!(fmt.parse("15.03.2024"), Some(date!(2024 - 03 - 15)));
}
