use time::Weekday;

use crate::models::calendar_locale::CalendarLocale;

/// Locale configuration utilities.
///
/// Provides static helper methods for creating pre-configured
/// calendar locale instances for different regional settings.
pub struct LocaleHelper;

impl LocaleHelper {
    /// Creates a default English locale with Sunday as start of week.
    ///
    /// # Returns
    ///
    /// - A `CalendarLocale` with English names and Sunday start.
    pub fn default_locale() -> CalendarLocale {
        CalendarLocale::default()
    }

    /// Creates a locale with Monday as start of week (common in Europe).
    ///
    /// # Returns
    ///
    /// - A `CalendarLocale` with English names and Monday start.
    pub fn monday_start_locale() -> CalendarLocale {
        CalendarLocale {
            start_of_week: Weekday::Monday,
            short_day_names: vec![
                "Mo".into(), "Tu".into(), "We".into(),
                "Th".into(), "Fr".into(), "Sa".into(),
                "Su".into(),
            ],
            day_names: vec![
                "Monday".into(), "Tuesday".into(), "Wednesday".into(),
                "Thursday".into(), "Friday".into(), "Saturday".into(),
                "Sunday".into(),
            ],
            ..Default::default()
        }
    }

    /// Creates a locale for a given simple language tag using the browser Intl API.
    ///
    /// Accepts simple tags such as "en", "fr", "de", "es", "ja", "ar", "zh".
    /// Month and day names are resolved via `Intl.DateTimeFormat`.
    /// UI labels remain English by default. The week start day is inferred
    /// from common regional conventions.
    ///
    /// # Parameters
    ///
    /// - `tag`: A simple BCP-47 language tag (e.g., "fr", "de").
    ///
    /// # Returns
    ///
    /// - A `CalendarLocale` with Intl-resolved month/day names and inferred week start.
    pub fn for_locale(tag: &str) -> CalendarLocale {
        // Resolve month and day names from the browser Intl API.
        let mut locale = CalendarLocale::from_bcp47(tag);

        // Infer the week start day from common regional conventions.
        locale.start_of_week = Self::infer_week_start(tag);

        locale
    }

    /// Infers the first day of the week from a language tag.
    ///
    /// Most of the world uses Monday. The US, Canada, Japan, and a few
    /// others use Sunday. Some Middle Eastern locales use Saturday.
    ///
    /// # Parameters
    ///
    /// - `tag`: A BCP-47 language tag.
    ///
    /// # Returns
    ///
    /// - The inferred `Weekday` for the start of the week.
    fn infer_week_start(tag: &str) -> Weekday {
        // Extract the primary subtag before any hyphen or underscore.
        let primary = tag.split(['-', '_']).next().unwrap_or(tag);

        // Map common language tags to their conventional week start.
        match primary {
            "ar" | "fa" | "he" => Weekday::Saturday,
            "en" | "ja" | "ko" | "zh" => Weekday::Sunday,
            _ => Weekday::Monday,
        }
    }
}
