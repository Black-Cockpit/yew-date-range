//! # yew-date-range-core
//!
//! Headless core library for the yew-date-range date picker.
//! Contains models, utilities, and date logic without any UI dependencies.
//!
//! ## Architecture
//!
//! The library is organized into:
//! - `models`: Data types for dates, ranges, selections, and configuration
//! - `utils`: Helper structs for date manipulation, locale, and range operations
//!
//! ## Example
//!
//! ```rust,ignore
//! use yew_date_range_core::prelude::*;
//!
//! let range = RangeSelection::new("selection")
//!     .with_dates(Some(DateHelper::today()), None);
//! ```

/// Data types for dates, ranges, selections, and configuration.
///
/// Contains all the model structs and enums used by the date range picker,
/// including day state, month data, range selections, and time selections.
pub mod models;

/// Helper structs for date manipulation, locale, and range operations.
///
/// Provides utility structs with static methods for common date operations,
/// locale configuration, and range validation logic.
pub mod utils;

/// Re-exports for convenient access to all public types.
///
/// Import this module to get access to the most frequently used
/// types, enums, and utility structs in the library.
pub mod prelude;
