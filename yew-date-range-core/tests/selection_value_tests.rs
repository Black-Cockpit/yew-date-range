//! Tests for selection value functionality.

use time::macros::date;

use yew_date_range_core::models::selection_value::SelectionValue;

/// Validates that a single-date selection contains only the exact selected date.
#[test]
fn test_single_contains() {
    let v = SelectionValue::Single(Some(date!(2024 - 03 - 15)));
    assert!(v.contains(date!(2024 - 03 - 15)));
    assert!(!v.contains(date!(2024 - 03 - 16)));
}

/// Validates that a range selection contains dates on and between the boundaries.
#[test]
fn test_range_contains() {
    let v = SelectionValue::Range {
        start: Some(date!(2024 - 01 - 10)),
        end: Some(date!(2024 - 01 - 20)),
    };

    // On boundaries and inside.
    assert!(v.contains(date!(2024 - 01 - 10)));
    assert!(v.contains(date!(2024 - 01 - 15)));
    assert!(v.contains(date!(2024 - 01 - 20)));

    // Outside boundaries.
    assert!(!v.contains(date!(2024 - 01 - 09)));
    assert!(!v.contains(date!(2024 - 01 - 21)));
}

/// Validates that a reversed range (start > end) still correctly contains dates between the boundaries.
#[test]
fn test_range_reversed_contains() {
    let v = SelectionValue::Range {
        start: Some(date!(2024 - 01 - 20)),
        end: Some(date!(2024 - 01 - 10)),
    };
    assert!(v.contains(date!(2024 - 01 - 15)));
}

/// Validates that a multiple-date selection contains only the specific listed dates.
#[test]
fn test_multiple_contains() {
    let v = SelectionValue::Multiple(vec![date!(2024 - 01 - 05), date!(2024 - 01 - 15)]);
    assert!(v.contains(date!(2024 - 01 - 05)));
    assert!(v.contains(date!(2024 - 01 - 15)));
    assert!(!v.contains(date!(2024 - 01 - 10)));
}

/// Validates that is_empty correctly identifies empty selections across all variants.
#[test]
fn test_is_empty() {
    // Empty variants.
    assert!(SelectionValue::Single(None).is_empty());
    assert!(SelectionValue::Range { start: None, end: None }.is_empty());
    assert!(SelectionValue::Multiple(vec![]).is_empty());

    // Non-empty variants.
    assert!(!SelectionValue::Single(Some(date!(2024 - 01 - 01))).is_empty());
    assert!(
        !SelectionValue::Range {
            start: Some(date!(2024 - 01 - 01)),
            end: None,
        }
        .is_empty()
    );
    assert!(!SelectionValue::Multiple(vec![date!(2024 - 01 - 01)]).is_empty());
}

/// Validates that to_dates extracts the correct date vector from each selection variant.
#[test]
fn test_to_dates() {
    // Single date produces a one-element vector.
    let v = SelectionValue::Single(Some(date!(2024 - 03 - 15)));
    assert_eq!(v.to_dates(), vec![date!(2024 - 03 - 15)]);

    // Range produces both boundary dates.
    let v = SelectionValue::Range {
        start: Some(date!(2024 - 01 - 01)),
        end: Some(date!(2024 - 01 - 31)),
    };
    assert_eq!(v.to_dates(), vec![date!(2024 - 01 - 01), date!(2024 - 01 - 31)]);

    // Empty single produces an empty vector.
    let v = SelectionValue::Single(None);
    assert!(v.to_dates().is_empty());
}
