//! # yew-date-range
//!
//! A date range picker component library for Yew, inspired by react-date-range.
//!
//! ## Components
//!
//! - [`DateRangePicker`] - Full picker with sidebar and calendar
//! - [`DateRange`] - Calendar with range selection
//! - [`Calendar`] - Base calendar component
//! - [`DefinedRange`] - Sidebar with predefined ranges
//!
//! ## Example
//!
//! ```rust,ignore
//! use yew::prelude::*;
//! use yew_date_range::prelude::*;
//! use time::macros::date;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let ranges = use_state(|| vec![
//!         RangeSelection::new("selection")
//!             .with_dates(
//!                 Some(date!(2024 - 01 - 01)),
//!                 Some(date!(2024 - 01 - 05)),
//!             )
//!     ]);
//!
//!     let on_change = {
//!         let ranges = ranges.clone();
//!         Callback::from(move |change: RangeChange| {
//!             ranges.set(vec![change.range]);
//!         })
//!     };
//!
//!     html! {
//!         <DateRangePicker
//!             ranges={(*ranges).clone()}
//!             on_change={on_change}
//!             months={2}
//!             direction={CalendarDirection::Horizontal}
//!         />
//!     }
//! }
//! ```

/// Yew components for the date range picker.
///
/// Provides the calendar, date picker, date range, overlay,
/// and time picker components for building date selection UIs.
pub mod components;

/// CSS style injection utilities.
///
/// Provides the style injector for automatically inserting
/// default and custom CSS into the document head.
pub mod styles;

/// Re-exports for convenient access to all public types.
///
/// Import this module to get access to the most frequently used
/// types from both the core engine and UI components.
pub mod prelude;

/// Re-export the core crate for direct access to all types.
pub use yew_date_range_core as core;
