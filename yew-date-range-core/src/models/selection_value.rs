use time::Date;

/// Represents the current selection value across all modes.
///
/// Unifies single date, date range, and multiple date selections
/// into a single enum for consistent handling in the date picker.
#[derive(Clone, Debug, PartialEq)]
pub enum SelectionValue {
    /// A single selected date.
    Single(Option<Date>),

    /// A date range (start, end).
    Range {
        /// The start date of the range.
        start: Option<Date>,

        /// The end date of the range.
        end: Option<Date>,
    },

    /// Multiple selected dates.
    Multiple(Vec<Date>),
}

impl Default for SelectionValue {
    fn default() -> Self {
        SelectionValue::Single(None)
    }
}

impl SelectionValue {
    /// Creates a single-date selection value.
    ///
    /// # Parameters
    ///
    /// - `date`: The optional selected date.
    ///
    /// # Returns
    ///
    /// - A `SelectionValue::Single` variant.
    pub fn single(date: Option<Date>) -> Self {
        SelectionValue::Single(date)
    }

    /// Creates a range selection value.
    ///
    /// # Parameters
    ///
    /// - `start`: The optional start date.
    /// - `end`: The optional end date.
    ///
    /// # Returns
    ///
    /// - A `SelectionValue::Range` variant.
    pub fn range(start: Option<Date>, end: Option<Date>) -> Self {
        SelectionValue::Range { start, end }
    }

    /// Creates a multiple-date selection value.
    ///
    /// # Parameters
    ///
    /// - `dates`: The vector of selected dates.
    ///
    /// # Returns
    ///
    /// - A `SelectionValue::Multiple` variant.
    pub fn multiple(dates: Vec<Date>) -> Self {
        SelectionValue::Multiple(dates)
    }

    /// Checks if a date is selected in this value.
    ///
    /// # Parameters
    ///
    /// - `date`: The date to check.
    ///
    /// # Returns
    ///
    /// - `true` if the date is part of the current selection.
    pub fn contains(&self, date: Date) -> bool {
        match self {
            SelectionValue::Single(Some(d)) => *d == date,
            SelectionValue::Range { start, end } => {
                // Normalize the range and check containment.
                match (start, end) {
                    (Some(s), Some(e)) => {
                        let (lo, hi) = if s <= e { (s, e) } else { (e, s) };
                        date >= *lo && date <= *hi
                    }
                    (Some(s), None) => date == *s,
                    (None, Some(e)) => date == *e,
                    _ => false,
                }
            }
            SelectionValue::Multiple(dates) => dates.contains(&date),
            _ => false,
        }
    }

    /// Checks if the selection is empty.
    ///
    /// # Returns
    ///
    /// - `true` if no dates are selected.
    pub fn is_empty(&self) -> bool {
        match self {
            SelectionValue::Single(None) => true,
            SelectionValue::Range { start: None, end: None } => true,
            SelectionValue::Multiple(v) => v.is_empty(),
            _ => false,
        }
    }

    /// Collects all selected dates into a vector.
    ///
    /// # Returns
    ///
    /// - A vector of all selected dates.
    pub fn to_dates(&self) -> Vec<Date> {
        match self {
            SelectionValue::Single(Some(d)) => vec![*d],
            SelectionValue::Range { start, end } => {
                // Collect whichever boundary dates are present.
                let mut v = Vec::new();
                if let Some(s) = start { v.push(*s); }
                if let Some(e) = end { v.push(*e); }
                v
            }
            SelectionValue::Multiple(dates) => dates.clone(),
            _ => Vec::new(),
        }
    }
}
