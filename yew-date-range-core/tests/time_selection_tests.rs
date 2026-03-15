//! Tests for time selection functionality.

use yew_date_range_core::models::time_period::TimePeriod;
use yew_date_range_core::models::time_selection::TimeSelection;

/// Validates that new clamps out-of-range hour, minute, and second values to their maximums.
#[test]
fn test_new_clamps() {
    let t = TimeSelection::new(25, 70, 80);
    assert_eq!(t.hour, 23);
    assert_eq!(t.minute, 59);
    assert_eq!(t.second, 59);
}

/// Validates that hour_12 converts 24-hour values to 12-hour format correctly.
#[test]
fn test_hour_12() {
    // Midnight maps to 12, noon maps to 12, and 13-23 map to 1-11.
    assert_eq!(TimeSelection::new(0, 0, 0).hour_12(), 12);
    assert_eq!(TimeSelection::new(1, 0, 0).hour_12(), 1);
    assert_eq!(TimeSelection::new(12, 0, 0).hour_12(), 12);
    assert_eq!(TimeSelection::new(13, 0, 0).hour_12(), 1);
    assert_eq!(TimeSelection::new(23, 0, 0).hour_12(), 11);
}

/// Validates that period returns AM for hours 0-11 and PM for hours 12-23.
#[test]
fn test_period() {
    assert_eq!(TimeSelection::new(0, 0, 0).period(), TimePeriod::AM);
    assert_eq!(TimeSelection::new(11, 59, 59).period(), TimePeriod::AM);
    assert_eq!(TimeSelection::new(12, 0, 0).period(), TimePeriod::PM);
    assert_eq!(TimeSelection::new(23, 59, 59).period(), TimePeriod::PM);
}

/// Validates that increment_hour wraps from 23 to 0 and decrement_hour wraps from 0 to 23.
#[test]
fn test_increment_decrement_hour() {
    // Incrementing 23 should wrap to 0.
    let t = TimeSelection::new(23, 0, 0);
    assert_eq!(t.increment_hour().hour, 0);

    // Decrementing 0 should wrap to 23.
    let t = TimeSelection::new(0, 0, 0);
    assert_eq!(t.decrement_hour().hour, 23);
}

/// Validates that increment_minute wraps from 59 to 0 and decrement_minute wraps from 0 to 59.
#[test]
fn test_increment_decrement_minute() {
    // Incrementing 59 should wrap to 0.
    let t = TimeSelection::new(0, 59, 0);
    assert_eq!(t.increment_minute().minute, 0);

    // Decrementing 0 should wrap to 59.
    let t = TimeSelection::new(0, 0, 0);
    assert_eq!(t.decrement_minute().minute, 59);
}

/// Validates that increment_second wraps from 59 to 0 and decrement_second wraps from 0 to 59.
#[test]
fn test_increment_decrement_second() {
    // Incrementing 59 should wrap to 0.
    let t = TimeSelection::new(0, 0, 59);
    assert_eq!(t.increment_second().second, 0);

    // Decrementing 0 should wrap to 59.
    let t = TimeSelection::new(0, 0, 0);
    assert_eq!(t.decrement_second().second, 59);
}

/// Validates that toggle_period switches between AM and PM by adding or subtracting 12 hours.
#[test]
fn test_toggle_period() {
    // Toggle AM to PM.
    let am = TimeSelection::new(9, 30, 0);
    let pm = am.toggle_period();
    assert_eq!(pm.hour, 21);
    assert_eq!(pm.period(), TimePeriod::PM);

    // Toggle PM back to AM.
    let back = pm.toggle_period();
    assert_eq!(back.hour, 9);
    assert_eq!(back.period(), TimePeriod::AM);
}

/// Validates that format_24h and format_24h_short produce zero-padded 24-hour time strings.
#[test]
fn test_format_24h() {
    let t = TimeSelection::new(9, 5, 3);
    assert_eq!(t.format_24h(), "09:05:03");
    assert_eq!(t.format_24h_short(), "09:05");
}

/// Validates that format_12h_short produces the correct 12-hour AM/PM time string.
#[test]
fn test_format_12h() {
    // Afternoon time.
    let t = TimeSelection::new(14, 30, 0);
    assert_eq!(t.format_12h_short(), "02:30 PM");

    // Midnight.
    let t = TimeSelection::new(0, 0, 0);
    assert_eq!(t.format_12h_short(), "12:00 AM");
}

/// Validates that with_12h correctly converts 12-hour format with AM/PM to 24-hour internal representation.
#[test]
fn test_with_12h() {
    // 12 AM is midnight (hour 0).
    let t = TimeSelection::with_12h(12, TimePeriod::AM);
    assert_eq!(t.hour, 0);

    // 12 PM is noon (hour 12).
    let t = TimeSelection::with_12h(12, TimePeriod::PM);
    assert_eq!(t.hour, 12);

    // 3 PM is hour 15.
    let t = TimeSelection::with_12h(3, TimePeriod::PM);
    assert_eq!(t.hour, 15);

    // 3 AM is hour 3.
    let t = TimeSelection::with_12h(3, TimePeriod::AM);
    assert_eq!(t.hour, 3);
}
