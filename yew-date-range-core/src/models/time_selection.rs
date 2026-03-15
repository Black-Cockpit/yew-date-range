use crate::models::time_period::TimePeriod;

/// Represents a time-of-day selection.
///
/// Stores hour, minute, and second values with methods for
/// formatting, incrementing, decrementing, and converting
/// between 12-hour and 24-hour formats.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TimeSelection {
    /// The hour component (0-23).
    pub hour: u8,

    /// The minute component (0-59).
    pub minute: u8,

    /// The second component (0-59).
    pub second: u8,
}

impl TimeSelection {
    /// Creates a new time selection with clamped values.
    ///
    /// # Parameters
    ///
    /// - `hour`: The hour value, clamped to 0-23.
    /// - `minute`: The minute value, clamped to 0-59.
    /// - `second`: The second value, clamped to 0-59.
    ///
    /// # Returns
    ///
    /// - A `TimeSelection` with valid clamped values.
    pub fn new(hour: u8, minute: u8, second: u8) -> Self {
        Self {
            hour: hour.min(23),
            minute: minute.min(59),
            second: second.min(59),
        }
    }

    /// Gets the hour in 12-hour format (1-12).
    ///
    /// # Returns
    ///
    /// - The hour value in 12-hour format.
    pub fn hour_12(&self) -> u8 {
        // Convert 24h to 12h, treating 0 and 12 as 12.
        match self.hour % 12 {
            0 => 12,
            h => h,
        }
    }

    /// Gets the AM/PM period for the current hour.
    ///
    /// # Returns
    ///
    /// - `TimePeriod::AM` for hours 0-11, `TimePeriod::PM` for 12-23.
    pub fn period(&self) -> TimePeriod {
        // Determine the period based on the 24h hour value.
        if self.hour < 12 { TimePeriod::AM } else { TimePeriod::PM }
    }

    /// Creates a time selection from 12-hour format values.
    ///
    /// # Parameters
    ///
    /// - `hour_12`: The hour in 12-hour format (1-12).
    /// - `period`: The AM/PM period.
    ///
    /// # Returns
    ///
    /// - A `TimeSelection` with the converted 24-hour value.
    pub fn with_12h(hour_12: u8, period: TimePeriod) -> Self {
        // Convert 12h + period to 24h format.
        let h = match (hour_12, period) {
            (12, TimePeriod::AM) => 0,
            (12, TimePeriod::PM) => 12,
            (h, TimePeriod::PM) => h + 12,
            (h, TimePeriod::AM) => h,
        };
        Self {
            hour: h.min(23),
            ..Default::default()
        }
    }

    /// Formats the time as HH:MM:SS in 24-hour format.
    ///
    /// # Returns
    ///
    /// - A formatted string like "14:30:00".
    pub fn format_24h(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }

    /// Formats the time as HH:MM in 24-hour format.
    ///
    /// # Returns
    ///
    /// - A formatted string like "14:30".
    pub fn format_24h_short(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }

    /// Formats the time as hh:mm:ss AM/PM in 12-hour format.
    ///
    /// # Returns
    ///
    /// - A formatted string like "02:30:00 PM".
    pub fn format_12h(&self) -> String {
        format!(
            "{:02}:{:02}:{:02} {}",
            self.hour_12(),
            self.minute,
            self.second,
            self.period()
        )
    }

    /// Formats the time as hh:mm AM/PM in 12-hour format.
    ///
    /// # Returns
    ///
    /// - A formatted string like "02:30 PM".
    pub fn format_12h_short(&self) -> String {
        format!("{:02}:{:02} {}", self.hour_12(), self.minute, self.period())
    }

    /// Increments the hour, wrapping from 23 to 0.
    ///
    /// # Returns
    ///
    /// - The modified `TimeSelection`.
    pub fn increment_hour(mut self) -> Self {
        self.hour = (self.hour + 1) % 24;
        self
    }

    /// Decrements the hour, wrapping from 0 to 23.
    ///
    /// # Returns
    ///
    /// - The modified `TimeSelection`.
    pub fn decrement_hour(mut self) -> Self {
        self.hour = if self.hour == 0 { 23 } else { self.hour - 1 };
        self
    }

    /// Increments the minute, wrapping from 59 to 0.
    ///
    /// # Returns
    ///
    /// - The modified `TimeSelection`.
    pub fn increment_minute(mut self) -> Self {
        self.minute = (self.minute + 1) % 60;
        self
    }

    /// Decrements the minute, wrapping from 0 to 59.
    ///
    /// # Returns
    ///
    /// - The modified `TimeSelection`.
    pub fn decrement_minute(mut self) -> Self {
        self.minute = if self.minute == 0 { 59 } else { self.minute - 1 };
        self
    }

    /// Increments the second, wrapping from 59 to 0.
    ///
    /// # Returns
    ///
    /// - The modified `TimeSelection`.
    pub fn increment_second(mut self) -> Self {
        self.second = (self.second + 1) % 60;
        self
    }

    /// Decrements the second, wrapping from 0 to 59.
    ///
    /// # Returns
    ///
    /// - The modified `TimeSelection`.
    pub fn decrement_second(mut self) -> Self {
        self.second = if self.second == 0 { 59 } else { self.second - 1 };
        self
    }

    /// Toggles between AM and PM by adding/subtracting 12 hours.
    ///
    /// # Returns
    ///
    /// - The modified `TimeSelection`.
    pub fn toggle_period(mut self) -> Self {
        self.hour = (self.hour + 12) % 24;
        self
    }
}
