use time::Weekday;

use crate::utils::date_helper::DateHelper;

/// Locale configuration for the calendar (yew_date_range.* namespace).
///
/// Provides localized month names, day names, UI labels, and the starting day
/// of the week for rendering the calendar in different locales. All fields
/// default to English values for full backward compatibility.
#[derive(Clone, Debug, PartialEq)]
pub struct CalendarLocale {
    /// Full month names (yew_date_range.month_names).
    pub month_names: Vec<String>,

    /// Abbreviated month names (yew_date_range.short_month_names).
    pub short_month_names: Vec<String>,

    /// Full day names (yew_date_range.day_names).
    pub day_names: Vec<String>,

    /// Abbreviated day names (yew_date_range.short_day_names).
    pub short_day_names: Vec<String>,

    /// The first day of the week for this locale.
    pub start_of_week: Weekday,

    /// The date format pattern string (yew_date_range.date_format).
    pub date_format: String,

    /// Navigation label for the previous month button (yew_date_range.nav.prev_month).
    pub prev_month_label: String,

    /// Navigation label for the next month button (yew_date_range.nav.next_month).
    pub next_month_label: String,

    /// Navigation label for the previous year button (yew_date_range.nav.prev_year).
    pub prev_year_label: String,

    /// Navigation label for the next year button (yew_date_range.nav.next_year).
    pub next_year_label: String,

    /// Navigation label for the previous decade button (yew_date_range.nav.prev_decade).
    pub prev_decade_label: String,

    /// Navigation label for the next decade button (yew_date_range.nav.next_decade).
    pub next_decade_label: String,

    /// Aria label for the month picker button (yew_date_range.nav.select_month).
    pub select_month_label: String,

    /// Aria label for the year picker button (yew_date_range.nav.select_year).
    pub select_year_label: String,

    /// Placeholder for the start date display (yew_date_range.display.start_date).
    pub start_date_placeholder: String,

    /// Placeholder for the end date display (yew_date_range.display.end_date).
    pub end_date_placeholder: String,

    /// Label for the Today action button (yew_date_range.action.today).
    pub today_label: String,

    /// Label for the Clear action button (yew_date_range.action.clear).
    pub clear_label: String,

    /// Placeholder for single date input (yew_date_range.input.select_date).
    pub select_date_placeholder: String,

    /// Placeholder for range date input (yew_date_range.input.select_range).
    pub select_range_placeholder: String,

    /// Placeholder for multiple dates input (yew_date_range.input.select_dates).
    pub select_dates_placeholder: String,

    /// Aria label for increment hour button (yew_date_range.time.increment_hour).
    pub increment_hour_label: String,

    /// Aria label for decrement hour button (yew_date_range.time.decrement_hour).
    pub decrement_hour_label: String,

    /// Aria label for increment minute button (yew_date_range.time.increment_minute).
    pub increment_minute_label: String,

    /// Aria label for decrement minute button (yew_date_range.time.decrement_minute).
    pub decrement_minute_label: String,

    /// Aria label for increment second button (yew_date_range.time.increment_second).
    pub increment_second_label: String,

    /// Aria label for decrement second button (yew_date_range.time.decrement_second).
    pub decrement_second_label: String,

    /// Aria label for toggle AM/PM button (yew_date_range.time.toggle_period).
    pub toggle_period_label: String,

    /// Header label for the week number column (yew_date_range.week.header).
    pub week_number_header: String,

    /// Static range label for Today (yew_date_range.range.today).
    pub today_range_label: String,

    /// Static range label for Yesterday (yew_date_range.range.yesterday).
    pub yesterday_range_label: String,

    /// Static range label for This Week (yew_date_range.range.this_week).
    pub this_week_label: String,

    /// Static range label for Last Week (yew_date_range.range.last_week).
    pub last_week_label: String,

    /// Static range label for This Month (yew_date_range.range.this_month).
    pub this_month_label: String,

    /// Static range label for Last Month (yew_date_range.range.last_month).
    pub last_month_label: String,

    /// Prefix for select aria labels (yew_date_range.select.prefix).
    pub select_prefix: String,
}

impl Default for CalendarLocale {
    fn default() -> Self {
        Self {
            month_names: vec![
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
            ],
            short_month_names: vec![
                "Jan".into(),
                "Feb".into(),
                "Mar".into(),
                "Apr".into(),
                "May".into(),
                "Jun".into(),
                "Jul".into(),
                "Aug".into(),
                "Sep".into(),
                "Oct".into(),
                "Nov".into(),
                "Dec".into(),
            ],
            day_names: vec![
                "Sunday".into(),
                "Monday".into(),
                "Tuesday".into(),
                "Wednesday".into(),
                "Thursday".into(),
                "Friday".into(),
                "Saturday".into(),
            ],
            short_day_names: vec![
                "Su".into(),
                "Mo".into(),
                "Tu".into(),
                "We".into(),
                "Th".into(),
                "Fr".into(),
                "Sa".into(),
            ],
            start_of_week: Weekday::Sunday,
            date_format: "yyyy-MM-dd".into(),
            prev_month_label: "Previous Month".into(),
            next_month_label: "Next Month".into(),
            prev_year_label: "Previous Year".into(),
            next_year_label: "Next Year".into(),
            prev_decade_label: "Previous Decade".into(),
            next_decade_label: "Next Decade".into(),
            select_month_label: "Select month".into(),
            select_year_label: "Select year".into(),
            start_date_placeholder: "Start Date".into(),
            end_date_placeholder: "End Date".into(),
            today_label: "Today".into(),
            clear_label: "Clear".into(),
            select_date_placeholder: "Select date".into(),
            select_range_placeholder: "Select range".into(),
            select_dates_placeholder: "Select dates".into(),
            increment_hour_label: "Increment hour".into(),
            decrement_hour_label: "Decrement hour".into(),
            increment_minute_label: "Increment minute".into(),
            decrement_minute_label: "Decrement minute".into(),
            increment_second_label: "Increment second".into(),
            decrement_second_label: "Decrement second".into(),
            toggle_period_label: "Toggle AM/PM".into(),
            week_number_header: "W".into(),
            today_range_label: "Today".into(),
            yesterday_range_label: "Yesterday".into(),
            this_week_label: "This Week".into(),
            last_week_label: "Last Week".into(),
            this_month_label: "This Month".into(),
            last_month_label: "Last Month".into(),
            select_prefix: "Select".into(),
        }
    }
}

impl CalendarLocale {
    /// Gets ordered day names starting from the configured start of week.
    ///
    /// # Returns
    ///
    /// - A vector of abbreviated day name strings in week-start order.
    pub fn ordered_day_names(&self) -> Vec<String> {
        // Determine the starting index based on the configured start of week.
        let start_idx = DateHelper::weekday_to_index(self.start_of_week);

        // Collect day names in the correct order, wrapping around.
        let mut result = Vec::with_capacity(7);
        for i in 0..7 {
            result.push(self.short_day_names[(start_idx + i) % 7].clone());
        }
        result
    }

    /// Gets the full month name for a given month.
    ///
    /// # Parameters
    ///
    /// - `month`: The month to look up.
    ///
    /// # Returns
    ///
    /// - A string slice containing the month name.
    pub fn month_name(&self, month: time::Month) -> &str {
        // Convert the month enum to a zero-based index.
        let idx = DateHelper::month_to_index(month);
        &self.month_names[idx]
    }

    /// Creates a locale from a BCP-47 tag using the browser Intl API.
    ///
    /// Uses `js_sys::Intl::DateTimeFormat` to auto-populate month and day
    /// names from the browser's native internationalization engine. UI
    /// labels (buttons, ARIA text) default to English and should be
    /// overridden manually or combined with a pre-built factory locale.
    ///
    /// # Parameters
    ///
    /// - `tag`: A simple locale tag (e.g., "en", "fr", "de").
    ///
    /// # Returns
    ///
    /// - A `CalendarLocale` with month/day names from the browser Intl API.
    pub fn from_bcp47(tag: &str) -> Self {
        // Start with the default English locale.
        let mut locale = Self::default();

        // Build the locale array argument for Intl.DateTimeFormat.
        let locales = js_sys::Array::new();
        locales.push(&tag.into());

        // Resolve full month names via Intl.DateTimeFormat.
        let long_month_opts = Self::intl_options("month", "long");
        let long_months = Self::format_months(&locales, &long_month_opts);
        if long_months.len() == 12 {
            locale.month_names = long_months;
        }

        // Resolve short month names via Intl.DateTimeFormat.
        let short_month_opts = Self::intl_options("month", "short");
        let short_months = Self::format_months(&locales, &short_month_opts);
        if short_months.len() == 12 {
            locale.short_month_names = short_months;
        }

        // Resolve full day names via Intl.DateTimeFormat.
        let long_day_opts = Self::intl_options("weekday", "long");
        let long_days = Self::format_weekdays(&locales, &long_day_opts);
        if long_days.len() == 7 {
            locale.day_names = long_days;
        }

        // Resolve short day names via Intl.DateTimeFormat.
        let short_day_opts = Self::intl_options("weekday", "short");
        let short_days = Self::format_weekdays(&locales, &short_day_opts);
        if short_days.len() == 7 {
            locale.short_day_names = short_days;
        }

        locale
    }

    /// Builds an Intl.DateTimeFormat options object with a single field.
    ///
    /// # Parameters
    ///
    /// - `field`: The option key (e.g., "month", "weekday").
    /// - `value`: The option value (e.g., "long", "short").
    ///
    /// # Returns
    ///
    /// - A `js_sys::Object` suitable for passing to `Intl.DateTimeFormat`.
    fn intl_options(field: &str, value: &str) -> js_sys::Object {
        // Create a plain JS object.
        let opts = js_sys::Object::new();

        // Set the requested field on the options object.
        let _ = js_sys::Reflect::set(&opts, &field.into(), &value.into());
        opts
    }

    /// Formats a single JS Date using an Intl.DateTimeFormat formatter function.
    ///
    /// The `format()` getter on `DateTimeFormat` returns a JS `Function`.
    /// This helper calls that function with the given date and returns the string.
    ///
    /// # Parameters
    ///
    /// - `format_fn`: The format function obtained from `DateTimeFormat.format()`.
    /// - `date`: The JS Date to format.
    ///
    /// # Returns
    ///
    /// - `Some(String)` if the call succeeds, `None` otherwise.
    fn call_format_fn(format_fn: &js_sys::Function, date: &js_sys::Date) -> Option<String> {
        // Call the format function with the date as the first argument.
        format_fn
            .call1(&wasm_bindgen::JsValue::UNDEFINED, date)
            .ok()?
            .as_string()
    }

    /// Formats month names for all 12 months using Intl.DateTimeFormat.
    ///
    /// # Parameters
    ///
    /// - `locales`: The locale array for Intl.DateTimeFormat.
    /// - `opts`: The options object specifying month format.
    ///
    /// # Returns
    ///
    /// - A vector of 12 formatted month name strings, or fewer on failure.
    fn format_months(locales: &js_sys::Array, opts: &js_sys::Object) -> Vec<String> {
        // Create the Intl.DateTimeFormat formatter and get its format function.
        let formatter = js_sys::Intl::DateTimeFormat::new(locales, opts);
        let format_fn = formatter.format();

        // Format a reference date for each of the 12 months.
        let mut names = Vec::with_capacity(12);
        for month_idx in 0..12i32 {
            // Create a JS Date for the 15th of each month in 2024.
            let date = js_sys::Date::new_with_year_month_day(2024, month_idx, 15);

            // Format the date, capitalize, and extract the month name string.
            if let Some(s) = Self::call_format_fn(&format_fn, &date) {
                names.push(Self::capitalize(&s));
            }
        }
        names
    }

    /// Capitalizes the first character of a string and removes trailing periods.
    ///
    /// # Parameters
    ///
    /// - `s`: The input string.
    ///
    /// # Returns
    ///
    /// - A new string with the first character uppercased and trailing periods removed.
    fn capitalize(s: &str) -> String {
        let trimmed = s.trim_end_matches('.');
        let mut chars = trimmed.chars();
        match chars.next() {
            Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            None => String::new(),
        }
    }

    /// Formats weekday names for Sunday through Saturday using Intl.DateTimeFormat.
    ///
    /// Uses 2024-01-07 (Sunday) through 2024-01-13 (Saturday) as reference dates.
    ///
    /// # Parameters
    ///
    /// - `locales`: The locale array for Intl.DateTimeFormat.
    /// - `opts`: The options object specifying weekday format.
    ///
    /// # Returns
    ///
    /// - A vector of 7 formatted weekday name strings, or fewer on failure.
    fn format_weekdays(locales: &js_sys::Array, opts: &js_sys::Object) -> Vec<String> {
        // Create the Intl.DateTimeFormat formatter and get its format function.
        let formatter = js_sys::Intl::DateTimeFormat::new(locales, opts);
        let format_fn = formatter.format();

        // Format reference dates for Sunday (Jan 7) through Saturday (Jan 13).
        let mut names = Vec::with_capacity(7);
        for day in 7..=13i32 {
            // Create a JS Date for each day in the reference week.
            let date = js_sys::Date::new_with_year_month_day(2024, 0, day);

            // Format the date, capitalize, and extract the weekday name string.
            if let Some(s) = Self::call_format_fn(&format_fn, &date) {
                names.push(Self::capitalize(&s));
            }
        }
        names
    }
}
