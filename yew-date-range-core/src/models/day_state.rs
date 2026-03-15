use time::Date;

/// Represents the visual and interactive state of a day cell.
///
/// Encapsulates all the flags needed by the calendar renderer to
/// determine how a particular day should be displayed and whether
/// it is interactive.
#[derive(Clone, Debug, PartialEq)]
pub struct DayState {
    /// The calendar date this state represents.
    pub date: Date,

    /// Whether this day is today.
    pub is_today: bool,

    /// Whether this day is part of the current selection.
    pub is_selected: bool,

    /// Whether this day falls within a selected range.
    pub is_in_range: bool,

    /// Whether this day is the start edge of a range.
    pub is_start_edge: bool,

    /// Whether this day is the end edge of a range.
    pub is_end_edge: bool,

    /// Whether this day is within the hover preview range.
    pub is_preview: bool,

    /// Whether this day is the start of the preview range.
    pub is_preview_start: bool,

    /// Whether this day is the end of the preview range.
    pub is_preview_end: bool,

    /// Whether this day is disabled and non-interactive.
    pub is_disabled: bool,

    /// Whether this day belongs to an adjacent month (passive display).
    pub is_passive: bool,

    /// Whether this day falls on a weekend.
    pub is_weekend: bool,

    /// Whether this day is the first day of its week.
    pub is_start_of_week: bool,

    /// Whether this day is the last day of its week.
    pub is_end_of_week: bool,

    /// Whether this day is the first day of its month.
    pub is_start_of_month: bool,

    /// Whether this day is the last day of its month.
    pub is_end_of_month: bool,

    /// Optional color for range highlighting.
    pub color: Option<String>,
}

impl DayState {
    /// Creates a new day state with all flags set to their defaults.
    ///
    /// # Parameters
    ///
    /// - `date`: The calendar date for this day state.
    ///
    /// # Returns
    ///
    /// - A `DayState` with all boolean flags set to `false`.
    pub fn new(date: Date) -> Self {
        Self {
            date,
            is_today: false,
            is_selected: false,
            is_in_range: false,
            is_start_edge: false,
            is_end_edge: false,
            is_preview: false,
            is_preview_start: false,
            is_preview_end: false,
            is_disabled: false,
            is_passive: false,
            is_weekend: false,
            is_start_of_week: false,
            is_end_of_week: false,
            is_start_of_month: false,
            is_end_of_month: false,
            color: None,
        }
    }

    /// Builds CSS class names based on the current state flags.
    ///
    /// # Returns
    ///
    /// - A vector of static CSS class name strings.
    pub fn css_classes(&self) -> Vec<&'static str> {
        // Start with the base day class.
        let mut classes = vec!["rdrDay"];

        // Append conditional classes based on state flags.
        if self.is_today {
            classes.push("rdrDayToday");
        }
        if self.is_selected {
            classes.push("rdrDaySelected");
        }
        if self.is_in_range {
            classes.push("rdrDayInRange");
        }
        if self.is_start_edge {
            classes.push("rdrDayStartEdge");
        }
        if self.is_end_edge {
            classes.push("rdrDayEndEdge");
        }
        if self.is_preview {
            classes.push("rdrDayInPreview");
        }
        if self.is_preview_start {
            classes.push("rdrDayStartPreview");
        }
        if self.is_preview_end {
            classes.push("rdrDayEndPreview");
        }
        if self.is_disabled {
            classes.push("rdrDayDisabled");
        }
        if self.is_passive {
            classes.push("rdrDayPassive");
        }
        if self.is_weekend {
            classes.push("rdrDayWeekend");
        }
        if self.is_start_of_week {
            classes.push("rdrDayStartOfWeek");
        }
        if self.is_end_of_week {
            classes.push("rdrDayEndOfWeek");
        }
        if self.is_start_of_month {
            classes.push("rdrDayStartOfMonth");
        }
        if self.is_end_of_month {
            classes.push("rdrDayEndOfMonth");
        }

        classes
    }
}
