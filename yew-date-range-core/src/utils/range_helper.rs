use time::{Date, Weekday};

use crate::models::calendar_locale::CalendarLocale;
use crate::models::range_selection::RangeSelection;
use crate::models::static_range::StaticRange;
use crate::utils::date_helper::DateHelper;

/// Range validation and computation utilities.
///
/// Provides static helper methods for range-related operations
/// including disabled date checks, span validation, preview
/// computation, and default static range generation.
pub struct RangeHelper;

impl RangeHelper {
    /// Creates default static ranges using locale labels and week start.
    ///
    /// # Parameters
    ///
    /// - `locale`: The calendar locale for labels and week start day.
    ///
    /// # Returns
    ///
    /// - A vector of predefined `StaticRange` instances.
    pub fn default_static_ranges(locale: &CalendarLocale) -> Vec<StaticRange> {
        // Capture the locale week start for This Week / Last Week closures.
        let week_start = locale.start_of_week;

        vec![
            StaticRange::new(&locale.today_range_label, || {
                let t = DateHelper::today();
                RangeSelection::new("selection").with_dates(Some(t), Some(t))
            }),
            StaticRange::new(&locale.yesterday_range_label, || {
                let y = DateHelper::add_days(DateHelper::today(), -1);
                RangeSelection::new("selection").with_dates(Some(y), Some(y))
            }),
            StaticRange::new(&locale.this_week_label, move || {
                let t = DateHelper::today();
                let start = DateHelper::start_of_week(t, week_start);
                let end = DateHelper::end_of_week(t, week_start);
                RangeSelection::new("selection").with_dates(Some(start), Some(end))
            }),
            StaticRange::new(&locale.last_week_label, move || {
                let t = DateHelper::add_days(DateHelper::today(), -7);
                let start = DateHelper::start_of_week(t, week_start);
                let end = DateHelper::end_of_week(t, week_start);
                RangeSelection::new("selection").with_dates(Some(start), Some(end))
            }),
            StaticRange::new(&locale.this_month_label, || {
                let t = DateHelper::today();
                let start = DateHelper::start_of_month(t);
                let end = DateHelper::end_of_month(t);
                RangeSelection::new("selection").with_dates(Some(start), Some(end))
            }),
            StaticRange::new(&locale.last_month_label, || {
                let t = DateHelper::today();
                let prev = DateHelper::sub_months(t, 1);
                let start = DateHelper::start_of_month(prev);
                let end = DateHelper::end_of_month(prev);
                RangeSelection::new("selection").with_dates(Some(start), Some(end))
            }),
        ]
    }

    /// Checks if a date should be disabled given min/max constraints and disabled dates.
    ///
    /// # Parameters
    ///
    /// - `date`: The date to check.
    /// - `min_date`: The optional minimum allowed date.
    /// - `max_date`: The optional maximum allowed date.
    /// - `disabled_dates`: A slice of specifically disabled dates.
    /// - `disabled_fn`: An optional custom disabled check function.
    ///
    /// # Returns
    ///
    /// - `true` if the date should be disabled.
    pub fn is_date_disabled(
        date: Date,
        min_date: Option<Date>,
        max_date: Option<Date>,
        disabled_dates: &[Date],
        disabled_fn: Option<&dyn Fn(Date) -> bool>,
    ) -> bool {
        // Delegate to the full version with empty weekday and range lists.
        Self::is_date_disabled_with_weekdays(date, min_date, max_date, disabled_dates, &[], disabled_fn)
    }

    /// Extended disabled check that also supports disabled weekdays.
    ///
    /// # Parameters
    ///
    /// - `date`: The date to check.
    /// - `min_date`: The optional minimum allowed date.
    /// - `max_date`: The optional maximum allowed date.
    /// - `disabled_dates`: A slice of specifically disabled dates.
    /// - `disabled_weekdays`: A slice of disabled weekdays.
    /// - `disabled_fn`: An optional custom disabled check function.
    ///
    /// # Returns
    ///
    /// - `true` if the date should be disabled.
    pub fn is_date_disabled_with_weekdays(
        date: Date,
        min_date: Option<Date>,
        max_date: Option<Date>,
        disabled_dates: &[Date],
        disabled_weekdays: &[Weekday],
        disabled_fn: Option<&dyn Fn(Date) -> bool>,
    ) -> bool {
        // Delegate to the full version with an empty range list.
        Self::is_date_disabled_full(date, min_date, max_date, disabled_dates, disabled_weekdays, &[], disabled_fn)
    }

    /// Full disabled check with all constraint types.
    ///
    /// # Parameters
    ///
    /// - `date`: The date to check.
    /// - `min_date`: The optional minimum allowed date.
    /// - `max_date`: The optional maximum allowed date.
    /// - `disabled_dates`: A slice of specifically disabled dates.
    /// - `disabled_weekdays`: A slice of disabled weekdays.
    /// - `disabled_ranges`: A slice of disabled date range tuples.
    /// - `disabled_fn`: An optional custom disabled check function.
    ///
    /// # Returns
    ///
    /// - `true` if the date should be disabled.
    pub fn is_date_disabled_full(
        date: Date,
        min_date: Option<Date>,
        max_date: Option<Date>,
        disabled_dates: &[Date],
        disabled_weekdays: &[Weekday],
        disabled_ranges: &[(Date, Date)],
        disabled_fn: Option<&dyn Fn(Date) -> bool>,
    ) -> bool {
        // Check the minimum date constraint.
        if let Some(min) = min_date {
            if date < min {
                return true;
            }
        }

        // Check the maximum date constraint.
        if let Some(max) = max_date {
            if date > max {
                return true;
            }
        }

        // Check the specific disabled dates list.
        if disabled_dates.contains(&date) {
            return true;
        }

        // Check the disabled weekdays list.
        if disabled_weekdays.contains(&date.weekday()) {
            return true;
        }

        // Check the disabled date ranges.
        for (range_start, range_end) in disabled_ranges {
            let (s, e) = if range_start <= range_end {
                (*range_start, *range_end)
            } else {
                (*range_end, *range_start)
            };
            if date >= s && date <= e {
                return true;
            }
        }

        // Check the custom disabled function.
        if let Some(f) = disabled_fn {
            if f(date) {
                return true;
            }
        }

        false
    }

    /// Checks if a range span is valid given min/max span constraints.
    ///
    /// # Parameters
    ///
    /// - `start`: The range start date.
    /// - `end`: The range end date.
    /// - `min_span`: The optional minimum number of days required.
    /// - `max_span`: The optional maximum number of days allowed.
    ///
    /// # Returns
    ///
    /// - `true` if the span is within the constraints.
    pub fn is_span_valid(
        start: Date,
        end: Date,
        min_span: Option<i64>,
        max_span: Option<i64>,
    ) -> bool {
        // Calculate the absolute number of days in the span.
        let days = DateHelper::diff_days(end, start).abs() + 1;

        // Validate the minimum span constraint.
        if let Some(min) = min_span {
            if days < min {
                return false;
            }
        }

        // Validate the maximum span constraint.
        if let Some(max) = max_span {
            if days > max {
                return false;
            }
        }

        true
    }

    /// Checks if a range matches the current selection.
    ///
    /// # Parameters
    ///
    /// - `range`: The range to compare.
    /// - `current`: The current selection.
    ///
    /// # Returns
    ///
    /// - `true` if both ranges have the same normalized boundaries.
    pub fn is_range_selected(range: &RangeSelection, current: &RangeSelection) -> bool {
        // Compare the normalized boundaries.
        let (rs, re) = range.normalized();
        let (cs, ce) = current.normalized();
        rs == cs && re == ce
    }

    /// Finds the focused range index from a list of ranges.
    ///
    /// # Parameters
    ///
    /// - `ranges`: The list of range selections.
    ///
    /// # Returns
    ///
    /// - `Some(index)` of the first auto-focused, non-disabled range.
    /// - `None` if no range qualifies.
    pub fn find_focused_range(ranges: &[RangeSelection]) -> Option<usize> {
        ranges.iter().position(|r| r.auto_focus && !r.disabled)
    }

    /// Computes the preview range for hover during mid-selection.
    ///
    /// Only shows a preview when the user is actively selecting
    /// (clicked start, picking end). When the range is already
    /// complete, no preview is shown.
    ///
    /// # Parameters
    ///
    /// - `hover_date`: The currently hovered date.
    /// - `ranges`: The list of range selections.
    /// - `focused_range_idx`: The index of the focused range.
    /// - `range_focus_start`: Whether focusing on the start date.
    /// - `is_selecting`: Whether the user is actively mid-selection.
    ///
    /// # Returns
    ///
    /// - `Some((start, end))` preview range if applicable.
    /// - `None` if no preview should be shown.
    pub fn compute_preview(
        hover_date: Option<Date>,
        ranges: &[RangeSelection],
        focused_range_idx: usize,
        range_focus_start: bool,
        is_selecting: bool,
    ) -> Option<(Date, Date)> {
        // Require a hovered date.
        let hover = hover_date?;

        // Get the focused range.
        let range = ranges.get(focused_range_idx)?;

        // Only show preview during active mid-selection.
        if !is_selecting {
            return None;
        }

        // Compute preview based on which end is being edited.
        if range_focus_start {
            // Focusing on start date: preview from hover to current end.
            if let Some(end) = range.end_date {
                let (s, e) = if hover <= end {
                    (hover, end)
                } else {
                    (end, hover)
                };
                return Some((s, e));
            }
            return Some((hover, hover));
        }

        // Focusing on end date: preview from current start to hover.
        if let Some(start) = range.start_date {
            let (s, e) = if hover >= start {
                (start, hover)
            } else {
                (hover, start)
            };
            return Some((s, e));
        }

        Some((hover, hover))
    }
}
