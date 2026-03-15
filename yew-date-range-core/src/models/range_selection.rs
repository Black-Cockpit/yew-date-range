use time::Date;

/// Represents a date range selection.
///
/// Holds the start and end dates of a range along with display
/// configuration such as color, focus, and disabled state.
#[derive(Clone, Debug, PartialEq)]
pub struct RangeSelection {
    /// The start date of the range, if selected.
    pub start_date: Option<Date>,

    /// The end date of the range, if selected.
    pub end_date: Option<Date>,

    /// A unique key identifying this range selection.
    pub key: String,

    /// Optional color used to highlight this range.
    pub color: Option<String>,

    /// Whether this range should receive auto-focus.
    pub auto_focus: bool,

    /// Whether this range is disabled.
    pub disabled: bool,

    /// Whether to show the date display header for this range.
    pub show_date_display: bool,
}

impl Default for RangeSelection {
    fn default() -> Self {
        Self {
            start_date: None,
            end_date: None,
            key: "selection".into(),
            color: Some("#3d91ff".into()),
            auto_focus: true,
            disabled: false,
            show_date_display: true,
        }
    }
}

impl RangeSelection {
    /// Creates a new range selection with the given key.
    ///
    /// # Parameters
    ///
    /// - `key`: A unique identifier for this range.
    ///
    /// # Returns
    ///
    /// - A new `RangeSelection` with default values and the provided key.
    pub fn new(key: &str) -> Self {
        Self {
            key: key.into(),
            ..Default::default()
        }
    }

    /// Sets the start and end dates on this range (builder pattern).
    ///
    /// # Parameters
    ///
    /// - `start`: The start date of the range.
    /// - `end`: The end date of the range.
    ///
    /// # Returns
    ///
    /// - The modified `RangeSelection` instance.
    pub fn with_dates(mut self, start: Option<Date>, end: Option<Date>) -> Self {
        self.start_date = start;
        self.end_date = end;
        self
    }

    /// Sets the highlight color on this range (builder pattern).
    ///
    /// # Parameters
    ///
    /// - `color`: A CSS color string.
    ///
    /// # Returns
    ///
    /// - The modified `RangeSelection` instance.
    pub fn with_color(mut self, color: &str) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Returns the normalized range where start is always less than or equal to end.
    ///
    /// # Returns
    ///
    /// - A tuple of `(Option<Date>, Option<Date>)` in ascending order.
    pub fn normalized(&self) -> (Option<Date>, Option<Date>) {
        // Swap start and end if start comes after end.
        match (self.start_date, self.end_date) {
            (Some(s), Some(e)) if s > e => (Some(e), Some(s)),
            other => other,
        }
    }

    /// Checks if a date falls within this range.
    ///
    /// # Parameters
    ///
    /// - `date`: The date to test.
    ///
    /// # Returns
    ///
    /// - `true` if the date is within the normalized range bounds.
    pub fn contains(&self, date: Date) -> bool {
        // Normalize the range boundaries.
        let (start, end) = self.normalized();

        // Check containment based on which boundaries are set.
        match (start, end) {
            (Some(s), Some(e)) => date >= s && date <= e,
            (Some(s), None) => date == s,
            (None, Some(e)) => date == e,
            (None, None) => false,
        }
    }

    /// Checks if this range is complete (has both dates).
    ///
    /// # Returns
    ///
    /// - `true` if both start and end dates are set.
    pub fn is_complete(&self) -> bool {
        self.start_date.is_some() && self.end_date.is_some()
    }
}
