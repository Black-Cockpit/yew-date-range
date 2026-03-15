use time::macros::date;
use time::{Date, Duration, Month, Weekday};

use crate::models::month_data::MonthData;
use crate::models::week_data::WeekData;

/// Compile-time verified fallback date, used when date construction fails.
const FALLBACK_DATE: Date = date!(2000 - 01 - 01);

/// Date manipulation and query utilities.
///
/// Provides static helper methods for common date operations
/// including arithmetic, formatting, parsing, and calendar grid building.
pub struct DateHelper;

impl DateHelper {
    /// Gets today's date using js_sys for WASM compatibility.
    ///
    /// # Returns
    ///
    /// - The current date from the browser environment.
    pub fn today() -> Date {
        // Retrieve the current date from the JavaScript runtime.
        let js_date = js_sys::Date::new_0();
        let year = js_date.get_full_year() as i32;
        let month = js_date.get_month() as usize;
        let day = js_date.get_date() as u8;

        // Convert the zero-indexed JS month to a Month enum.
        let month = Self::index_to_month(month);

        // Construct the date, falling back on failure.
        Date::from_calendar_date(year, month, day).unwrap_or(FALLBACK_DATE)
    }

    /// Gets the first day of the month for a given date.
    ///
    /// # Parameters
    ///
    /// - `date`: The reference date.
    ///
    /// # Returns
    ///
    /// - The first day of the same month.
    pub fn start_of_month(date: Date) -> Date {
        // Construct a date with day set to 1.
        Date::from_calendar_date(date.year(), date.month(), 1).unwrap_or(date)
    }

    /// Gets the last day of the month for a given date.
    ///
    /// # Parameters
    ///
    /// - `date`: The reference date.
    ///
    /// # Returns
    ///
    /// - The last day of the same month.
    pub fn end_of_month(date: Date) -> Date {
        // Calculate the number of days in the month.
        let days = Self::days_in_month(date.year(), date.month());

        // Construct a date with the last valid day.
        Date::from_calendar_date(date.year(), date.month(), days).unwrap_or(date)
    }

    /// Gets the number of days in a given month and year.
    ///
    /// # Parameters
    ///
    /// - `year`: The year.
    /// - `month`: The month.
    ///
    /// # Returns
    ///
    /// - The number of days in that month.
    pub fn days_in_month(year: i32, month: Month) -> u8 {
        // Calculate by finding the difference between the first of this month and next.
        let next = Self::add_months_to_ym(year, month, 1);
        match (
            Date::from_calendar_date(next.0, next.1, 1),
            Date::from_calendar_date(year, month, 1),
        ) {
            (Ok(next_first), Ok(current_first)) => (next_first - current_first).whole_days() as u8,
            _ => 30,
        }
    }

    /// Gets the start of the week for a given date.
    ///
    /// # Parameters
    ///
    /// - `date`: The reference date.
    /// - `week_start`: The configured first day of the week.
    ///
    /// # Returns
    ///
    /// - The first day of the week containing the given date.
    pub fn start_of_week(date: Date, week_start: Weekday) -> Date {
        // Calculate the day offset from the week start.
        let date_dow = Self::weekday_to_index(date.weekday());
        let start_dow = Self::weekday_to_index(week_start);
        let diff = ((date_dow as i64 - start_dow as i64) + 7) % 7;

        // Subtract the offset to reach the week start.
        date - Duration::days(diff)
    }

    /// Gets the end of the week for a given date.
    ///
    /// # Parameters
    ///
    /// - `date`: The reference date.
    /// - `week_start`: The configured first day of the week.
    ///
    /// # Returns
    ///
    /// - The last day of the week containing the given date.
    pub fn end_of_week(date: Date, week_start: Weekday) -> Date {
        // Add 6 days to the start of the week.
        Self::start_of_week(date, week_start) + Duration::days(6)
    }

    /// Checks if a date is a weekend (Saturday or Sunday).
    ///
    /// # Parameters
    ///
    /// - `date`: The date to check.
    ///
    /// # Returns
    ///
    /// - `true` if the date is a Saturday or Sunday.
    pub fn is_weekend(date: Date) -> bool {
        matches!(date.weekday(), Weekday::Saturday | Weekday::Sunday)
    }

    /// Adds months to a year-month pair, returning the new year and month.
    ///
    /// # Parameters
    ///
    /// - `year`: The starting year.
    /// - `month`: The starting month.
    /// - `count`: The number of months to add (can be negative).
    ///
    /// # Returns
    ///
    /// - A tuple of `(year, month)` after the addition.
    pub fn add_months_to_ym(year: i32, month: Month, count: i32) -> (i32, Month) {
        // Convert to a total month index for arithmetic.
        let month_idx = month as i32 - 1;
        let total = year * 12 + month_idx + count;

        // Decompose back into year and month.
        let new_year = total.div_euclid(12);
        let new_month_idx = total.rem_euclid(12);
        let new_month = Self::index_to_month(new_month_idx as usize);

        (new_year, new_month)
    }

    /// Adds months to a date, clamping the day to the valid range.
    ///
    /// # Parameters
    ///
    /// - `date`: The starting date.
    /// - `count`: The number of months to add (can be negative).
    ///
    /// # Returns
    ///
    /// - The resulting date after adding months.
    pub fn add_months(date: Date, count: i32) -> Date {
        // Calculate the target year and month.
        let (new_year, new_month) = Self::add_months_to_ym(date.year(), date.month(), count);

        // Clamp the day to the maximum valid day in the target month.
        let max_day = Self::days_in_month(new_year, new_month);
        let day = date.day().min(max_day);

        // Construct the new date.
        Date::from_calendar_date(new_year, new_month, day).unwrap_or(date)
    }

    /// Subtracts months from a date.
    ///
    /// # Parameters
    ///
    /// - `date`: The starting date.
    /// - `count`: The number of months to subtract.
    ///
    /// # Returns
    ///
    /// - The resulting date after subtracting months.
    pub fn sub_months(date: Date, count: i32) -> Date {
        Self::add_months(date, -count)
    }

    /// Adds days to a date.
    ///
    /// # Parameters
    ///
    /// - `date`: The starting date.
    /// - `count`: The number of days to add (can be negative).
    ///
    /// # Returns
    ///
    /// - The resulting date.
    pub fn add_days(date: Date, count: i64) -> Date {
        date + Duration::days(count)
    }

    /// Gets the difference in calendar days between two dates.
    ///
    /// # Parameters
    ///
    /// - `a`: The first date.
    /// - `b`: The second date.
    ///
    /// # Returns
    ///
    /// - The number of days from `b` to `a`.
    pub fn diff_days(a: Date, b: Date) -> i64 {
        (a - b).whole_days()
    }

    /// Gets the difference in calendar months between two dates.
    ///
    /// # Parameters
    ///
    /// - `a`: The first date.
    /// - `b`: The second date.
    ///
    /// # Returns
    ///
    /// - The number of months from `b` to `a`.
    pub fn diff_months(a: Date, b: Date) -> i32 {
        (a.year() - b.year()) * 12 + (a.month() as i32 - b.month() as i32)
    }

    /// Checks if two optional dates represent the same day.
    ///
    /// # Parameters
    ///
    /// - `a`: The first optional date.
    /// - `b`: The second optional date.
    ///
    /// # Returns
    ///
    /// - `true` if both are `Some` and equal.
    pub fn is_same_day(a: Option<Date>, b: Option<Date>) -> bool {
        match (a, b) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        }
    }

    /// Checks if a date is before another.
    ///
    /// # Parameters
    ///
    /// - `a`: The first date.
    /// - `b`: The second date.
    ///
    /// # Returns
    ///
    /// - `true` if `a` is before `b`.
    pub fn is_before(a: Date, b: Date) -> bool {
        a < b
    }

    /// Checks if a date is after another.
    ///
    /// # Parameters
    ///
    /// - `a`: The first date.
    /// - `b`: The second date.
    ///
    /// # Returns
    ///
    /// - `true` if `a` is after `b`.
    pub fn is_after(a: Date, b: Date) -> bool {
        a > b
    }

    /// Checks if a date is between two dates (inclusive).
    ///
    /// # Parameters
    ///
    /// - `date`: The date to test.
    /// - `start`: The range start.
    /// - `end`: The range end.
    ///
    /// # Returns
    ///
    /// - `true` if the date is within the inclusive range.
    pub fn is_between(date: Date, start: Date, end: Date) -> bool {
        // Normalize the range so start <= end.
        let (s, e) = if start <= end { (start, end) } else { (end, start) };
        date >= s && date <= e
    }

    /// Gets the ISO week number for a date.
    ///
    /// # Parameters
    ///
    /// - `date`: The date.
    ///
    /// # Returns
    ///
    /// - The ISO week number.
    pub fn week_number(date: Date) -> u8 {
        date.iso_week()
    }

    /// Builds month data for rendering a calendar month grid.
    ///
    /// # Parameters
    ///
    /// - `year`: The year.
    /// - `month`: The month.
    /// - `week_start`: The configured first day of the week.
    ///
    /// # Returns
    ///
    /// - A `MonthData` containing all week rows for the month.
    pub fn build_month_data(year: i32, month: Month, week_start: Weekday) -> MonthData {
        // Determine the first and last day of the month.
        let first = Date::from_calendar_date(year, month, 1).unwrap_or(FALLBACK_DATE);
        let last = Self::end_of_month(first);

        // Find the start of the week containing the first day.
        let week_start_date = Self::start_of_week(first, week_start);

        // Build week rows until the entire month is covered.
        let mut weeks = Vec::new();
        let mut current = week_start_date;

        loop {
            // Collect 7 days for this week row.
            let mut days = Vec::with_capacity(7);
            for _ in 0..7 {
                if current.month() == month {
                    days.push(Some(current));
                } else if current < first || current > last {
                    // Show adjacent month days as passive.
                    days.push(Some(current));
                } else {
                    days.push(None);
                }
                current += Duration::days(1);
            }

            // Extract the week number from the first valid day.
            let wn = days.iter().find_map(|d| d.map(Self::week_number));

            weeks.push(WeekData { week_number: wn, days });

            // Stop after covering the last day and completing the week.
            if current > last && current.weekday() == week_start {
                break;
            }
        }

        MonthData { year, month, weeks }
    }

    /// Converts a zero-indexed month number to a Month enum.
    ///
    /// # Parameters
    ///
    /// - `idx`: The zero-based month index (0 = January).
    ///
    /// # Returns
    ///
    /// - The corresponding `Month` enum value.
    pub fn index_to_month(idx: usize) -> Month {
        // Use modulo to handle overflow and map to the correct month.
        match idx % 12 {
            0 => Month::January,
            1 => Month::February,
            2 => Month::March,
            3 => Month::April,
            4 => Month::May,
            5 => Month::June,
            6 => Month::July,
            7 => Month::August,
            8 => Month::September,
            9 => Month::October,
            10 => Month::November,
            _ => Month::December,
        }
    }

    /// Generates a list of months to display starting from a given date.
    ///
    /// # Parameters
    ///
    /// - `start_year`: The starting year.
    /// - `start_month`: The starting month.
    /// - `count`: The number of months to generate.
    /// - `week_start`: The configured first day of the week.
    ///
    /// # Returns
    ///
    /// - A vector of `MonthData` for each month to display.
    pub fn get_months_to_display(
        start_year: i32,
        start_month: Month,
        count: usize,
        week_start: Weekday,
    ) -> Vec<MonthData> {
        // Build month data for each month in the range.
        let mut months = Vec::with_capacity(count);
        for i in 0..count {
            let (y, m) = Self::add_months_to_ym(start_year, start_month, i as i32);
            months.push(Self::build_month_data(y, m, week_start));
        }
        months
    }

    /// Formats a date as YYYY-MM-DD.
    ///
    /// # Parameters
    ///
    /// - `date`: The date to format.
    ///
    /// # Returns
    ///
    /// - A formatted date string.
    pub fn format_date(date: Date) -> String {
        format!("{:04}-{:02}-{:02}", date.year(), date.month() as u8, date.day())
    }

    /// Parses a date from a YYYY-MM-DD string.
    ///
    /// # Parameters
    ///
    /// - `s`: The date string to parse.
    ///
    /// # Returns
    ///
    /// - `Some(Date)` if parsing succeeds.
    /// - `None` if the string is invalid.
    pub fn parse_date(s: &str) -> Option<Date> {
        // Split the string by dashes.
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return None;
        }

        // Parse each component.
        let year: i32 = parts[0].parse().ok()?;
        let month: u8 = parts[1].parse().ok()?;
        let day: u8 = parts[2].parse().ok()?;

        // Convert the month number to a Month enum.
        let month = Self::index_to_month((month as usize).wrapping_sub(1));

        // Construct the date.
        Date::from_calendar_date(year, month, day).ok()
    }

    /// Formats a date for display as "Jan 1, 2024".
    ///
    /// # Parameters
    ///
    /// - `date`: The date to format.
    /// - `month_names`: The list of full month names.
    ///
    /// # Returns
    ///
    /// - A formatted display string.
    pub fn format_date_display(date: Date, month_names: &[String]) -> String {
        // Determine the month index.
        let month_idx = date.month() as usize - 1;

        // Look up the month name, falling back to "???".
        let month_name = if month_idx < month_names.len() {
            &month_names[month_idx]
        } else {
            "???"
        };

        // Format as abbreviated month name, day, year.
        let abbr = month_name.get(..3).unwrap_or(month_name);
        format!("{} {}, {}", abbr, date.day(), date.year())
    }

    /// Converts a Weekday to a zero-indexed number (Sunday = 0).
    ///
    /// # Parameters
    ///
    /// - `weekday`: The weekday to convert.
    ///
    /// # Returns
    ///
    /// - A zero-based index for the weekday.
    pub fn weekday_to_index(weekday: Weekday) -> usize {
        match weekday {
            Weekday::Sunday => 0,
            Weekday::Monday => 1,
            Weekday::Tuesday => 2,
            Weekday::Wednesday => 3,
            Weekday::Thursday => 4,
            Weekday::Friday => 5,
            Weekday::Saturday => 6,
        }
    }

    /// Converts a Month to a zero-indexed number.
    ///
    /// # Parameters
    ///
    /// - `month`: The month to convert.
    ///
    /// # Returns
    ///
    /// - A zero-based index for the month.
    pub fn month_to_index(month: Month) -> usize {
        match month {
            Month::January => 0,
            Month::February => 1,
            Month::March => 2,
            Month::April => 3,
            Month::May => 4,
            Month::June => 5,
            Month::July => 6,
            Month::August => 7,
            Month::September => 8,
            Month::October => 9,
            Month::November => 10,
            Month::December => 11,
        }
    }

    /// Validates a date against min/max constraints.
    ///
    /// # Parameters
    ///
    /// - `date`: The date to validate.
    /// - `min`: The optional minimum allowed date.
    /// - `max`: The optional maximum allowed date.
    ///
    /// # Returns
    ///
    /// - `true` if the date is within the constraints.
    pub fn validate_against_constraints(date: Date, min: Option<Date>, max: Option<Date>) -> bool {
        // Check the minimum constraint.
        if let Some(min) = min {
            if date < min {
                return false;
            }
        }

        // Check the maximum constraint.
        if let Some(max) = max {
            if date > max {
                return false;
            }
        }

        true
    }
}
