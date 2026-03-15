use time::Date;

use crate::utils::date_helper::DateHelper;

/// Configurable date format for display and parsing.
///
/// Supports pattern-based formatting and parsing of dates using
/// tokens like `yyyy`, `MM`, `dd` with configurable separators.
#[derive(Clone, Debug, PartialEq)]
pub struct DateFormat {
    /// The format pattern string (e.g., "yyyy-MM-dd").
    pub pattern: String,
}

impl Default for DateFormat {
    fn default() -> Self {
        Self {
            pattern: "yyyy-MM-dd".into(),
        }
    }
}

impl DateFormat {
    /// Creates a new date format with the given pattern.
    ///
    /// # Parameters
    ///
    /// - `pattern`: The format pattern string.
    ///
    /// # Returns
    ///
    /// - A new `DateFormat` instance.
    pub fn new(pattern: &str) -> Self {
        Self {
            pattern: pattern.into(),
        }
    }

    /// Formats a date according to the configured pattern.
    ///
    /// # Parameters
    ///
    /// - `date`: The date to format.
    ///
    /// # Returns
    ///
    /// - A formatted date string.
    pub fn format(&self, date: Date) -> String {
        // Extract date components.
        let y = date.year();
        let m = date.month() as u8;
        let d = date.day();

        // Start with the pattern and replace tokens.
        let mut result = self.pattern.clone();

        // Replace year tokens.
        result = result.replace("yyyy", &format!("{:04}", y));
        result = result.replace("yy", &format!("{:02}", y % 100));

        // Replace month tokens.
        result = result.replace("MM", &format!("{:02}", m));
        result = result.replace('M', &m.to_string());

        // Replace day tokens.
        result = result.replace("dd", &format!("{:02}", d));
        result = result.replace('d', &d.to_string());

        result
    }

    /// Parses a date string according to the configured pattern.
    ///
    /// # Parameters
    ///
    /// - `input`: The date string to parse.
    ///
    /// # Returns
    ///
    /// - `Some(Date)` if parsing succeeds.
    /// - `None` if the input does not match the pattern.
    pub fn parse(&self, input: &str) -> Option<Date> {
        let pattern = &self.pattern;

        // Determine the separator character from the pattern.
        let sep = pattern.chars().find(|c| !c.is_alphanumeric()).unwrap_or('-');

        // Split both the pattern and input by the separator.
        let pattern_parts: Vec<&str> = pattern.split(sep).collect();
        let input_parts: Vec<&str> = input.split(sep).collect();

        // Validate that both have exactly three parts.
        if pattern_parts.len() != input_parts.len() || pattern_parts.len() != 3 {
            return None;
        }

        // Extract year, month, and day values from the parts.
        let year_val = Self::extract_i32(&pattern_parts, &input_parts, &["yyyy", "yy"]);
        let month_val = Self::extract_u8(&pattern_parts, &input_parts, &["MM", "M"]);
        let day_val = Self::extract_u8(&pattern_parts, &input_parts, &["dd", "d"]);

        // Unwrap the extracted values.
        let y = year_val?;
        let m = month_val?;
        let d = day_val?;

        // Validate the month range.
        if !(1..=12).contains(&m) {
            return None;
        }

        // Convert the month number to a Month enum.
        let month = DateHelper::index_to_month((m as usize).wrapping_sub(1));

        // Construct the date, returning None if invalid.
        Date::from_calendar_date(y, month, d).ok()
    }

    /// Formats multiple dates with a separator.
    ///
    /// # Parameters
    ///
    /// - `dates`: The dates to format.
    /// - `separator`: The string to place between formatted dates.
    ///
    /// # Returns
    ///
    /// - A single string with all dates formatted and joined.
    pub fn format_multiple(&self, dates: &[Date], separator: &str) -> String {
        dates
            .iter()
            .map(|d| self.format(*d))
            .collect::<Vec<_>>()
            .join(separator)
    }

    /// Formats a range as "start - end".
    ///
    /// # Parameters
    ///
    /// - `start`: The optional start date.
    /// - `end`: The optional end date.
    ///
    /// # Returns
    ///
    /// - A formatted range string.
    pub fn format_range(&self, start: Option<Date>, end: Option<Date>) -> String {
        // Format each boundary, defaulting to an empty string.
        let s = start.map(|d| self.format(d)).unwrap_or_default();
        let e = end.map(|d| self.format(d)).unwrap_or_default();
        format!("{} - {}", s, e)
    }

    /// Extracts an i32 value from the matching pattern part.
    ///
    /// # Parameters
    ///
    /// - `pattern_parts`: The split pattern segments.
    /// - `input_parts`: The split input segments.
    /// - `tokens`: The token strings to match against.
    ///
    /// # Returns
    ///
    /// - `Some(i32)` if a matching token is found and parsed.
    fn extract_i32(pattern_parts: &[&str], input_parts: &[&str], tokens: &[&str]) -> Option<i32> {
        // Iterate over pattern parts to find a matching token.
        for (i, pp) in pattern_parts.iter().enumerate() {
            for token in tokens {
                if pp.contains(token) {
                    // Parse the corresponding input part.
                    let val: i32 = input_parts.get(i)?.parse().ok()?;

                    // Handle two-digit year conversion.
                    if *token == "yy" && val < 100 {
                        return Some(2000 + val);
                    }
                    return Some(val);
                }
            }
        }
        None
    }

    /// Extracts a u8 value from the matching pattern part.
    ///
    /// # Parameters
    ///
    /// - `pattern_parts`: The split pattern segments.
    /// - `input_parts`: The split input segments.
    /// - `tokens`: The token strings to match against.
    ///
    /// # Returns
    ///
    /// - `Some(u8)` if a matching token is found and parsed.
    fn extract_u8(pattern_parts: &[&str], input_parts: &[&str], tokens: &[&str]) -> Option<u8> {
        // Iterate over pattern parts to find a matching token.
        for (i, pp) in pattern_parts.iter().enumerate() {
            for token in tokens {
                if pp.contains(token) {
                    // Parse the corresponding input part.
                    return input_parts.get(i)?.parse().ok();
                }
            }
        }
        None
    }
}
