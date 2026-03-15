use time::{Date, Weekday};
use yew::prelude::*;

use yew_date_range_core::models::calendar_direction::CalendarDirection;
use yew_date_range_core::models::calendar_locale::CalendarLocale;
use yew_date_range_core::models::day_state::DayState;
use yew_date_range_core::models::range_change::RangeChange;
use yew_date_range_core::models::range_change_source::RangeChangeSource;
use yew_date_range_core::models::range_focus::RangeFocus;
use yew_date_range_core::models::range_selection::RangeSelection;
use yew_date_range_core::utils::date_helper::DateHelper;
use yew_date_range_core::utils::range_helper::RangeHelper;

use crate::components::calendar::Calendar;
use crate::styles::style_injector::StyleInjector;

/// Properties for the DateRange component.
#[derive(Properties, Clone, PartialEq)]
pub struct DateRangeProps {
    /// The current date ranges.
    #[prop_or_default]
    pub ranges: Vec<RangeSelection>,

    /// Callback when ranges change.
    #[prop_or_default]
    pub on_change: Callback<RangeChange>,

    /// Number of months to show.
    #[prop_or(1)]
    pub months: usize,

    /// Layout direction.
    #[prop_or_default]
    pub direction: CalendarDirection,

    /// Minimum selectable date.
    #[prop_or_default]
    pub min_date: Option<Date>,

    /// Maximum selectable date.
    #[prop_or_default]
    pub max_date: Option<Date>,

    /// Disabled dates.
    #[prop_or_default]
    pub disabled_dates: Vec<Date>,

    /// Show selection preview on hover.
    #[prop_or(true)]
    pub show_selection_preview: bool,

    /// Move range on first selection.
    #[prop_or(false)]
    pub move_range_on_first_selection: bool,

    /// Retain end date on first selection.
    #[prop_or(false)]
    pub retain_end_date_on_first_selection: bool,

    /// Show week numbers.
    #[prop_or(false)]
    pub show_week_numbers: bool,

    /// Show date display header.
    #[prop_or(true)]
    pub show_date_display: bool,

    /// Locale configuration.
    #[prop_or_default]
    pub locale: Option<CalendarLocale>,

    /// Custom class name.
    #[prop_or_default]
    pub class_name: Option<String>,

    /// Preview override.
    #[prop_or_default]
    pub preview: Option<(Date, Date)>,

    /// Custom day content renderer.
    #[prop_or_default]
    pub day_content_renderer: Option<Callback<DayState, Html>>,

    /// Show month and year pickers.
    #[prop_or(true)]
    pub show_month_and_year_pickers: bool,

    /// Initial shown date.
    #[prop_or_default]
    pub shown_date: Option<Date>,

    /// Disabled weekdays.
    #[prop_or_default]
    pub disabled_weekdays: Vec<Weekday>,

    /// Disabled date ranges (contiguous blocks).
    #[prop_or_default]
    pub disabled_ranges: Vec<(Date, Date)>,

    /// Custom callback to determine if a date is disabled.
    #[prop_or_default]
    pub is_date_disabled: Option<Callback<Date, bool>>,

    /// Maximum number of days allowed in a selection (inclusive).
    #[prop_or_default]
    pub max_span: Option<i64>,

    /// Minimum number of days required in a selection (inclusive).
    #[prop_or_default]
    pub min_span: Option<i64>,
}

/// The DateRange component provides a calendar with range selection.
#[function_component(DateRange)]
pub fn date_range(props: &DateRangeProps) -> Html {
    // Inject the default CSS into the document head on first render.
    StyleInjector::inject_default_styles();

    // Initialize state for tracking which range part is being edited.
    let focused_range_idx = use_state(|| 0usize);
    let range_focus = use_state(|| RangeFocus::Start);
    let hover_date = use_state(|| None::<Date>);
    let is_selecting = use_state(|| false);

    // Initialize the shown date from props or the first range's start date.
    let shown_date = use_state(|| {
        props.shown_date.unwrap_or_else(|| {
            props
                .ranges
                .first()
                .and_then(|r| r.start_date)
                .unwrap_or_else(DateHelper::today)
        })
    });

    // Compute the hover preview range only during active mid-selection.
    let preview = if props.show_selection_preview {
        if let Some(p) = props.preview {
            Some(p)
        } else {
            RangeHelper::compute_preview(
                *hover_date,
                &props.ranges,
                *focused_range_idx,
                *range_focus == RangeFocus::Start,
                *is_selecting,
            )
        }
    } else {
        None
    };

    // Create the day click handler that updates the range based on focus state.
    let on_day_click = {
        let ranges = props.ranges.clone();
        let on_change = props.on_change.clone();
        let focused_range_idx = focused_range_idx.clone();
        let range_focus = range_focus.clone();
        let is_selecting = is_selecting.clone();
        let move_on_first = props.move_range_on_first_selection;
        let retain_end = props.retain_end_date_on_first_selection;
        let max_span = props.max_span;
        let min_span = props.min_span;

        Callback::from(move |date: Date| {
            let idx = *focused_range_idx;
            if let Some(range) = ranges.get(idx) {
                let mut new_range = range.clone();
                let current_focus = *range_focus;

                match current_focus {
                    RangeFocus::Start => {
                        new_range.start_date = Some(date);
                        if move_on_first {
                            new_range.end_date = Some(date);
                        }
                        if !retain_end {
                            if let Some(end) = new_range.end_date {
                                if date > end {
                                    new_range.end_date = Some(date);
                                }
                            }
                        }
                        range_focus.set(RangeFocus::End);
                        is_selecting.set(true);
                    }
                    RangeFocus::End => {
                        new_range.end_date = Some(date);
                        // Normalize the range order so start is before end.
                        if let Some(start) = new_range.start_date {
                            if date < start {
                                new_range.end_date = new_range.start_date;
                                new_range.start_date = Some(date);
                            }
                        }
                        // Enforce the minimum and maximum span constraints.
                        if let (Some(s), Some(e)) = (new_range.start_date, new_range.end_date) {
                            if !RangeHelper::is_span_valid(s, e, min_span, max_span) {
                                return; // reject selection silently
                            }
                        }
                        range_focus.set(RangeFocus::Start);
                        is_selecting.set(false);
                    }
                }

                on_change.emit(RangeChange {
                    range: new_range,
                    source: RangeChangeSource::Click,
                });
            }
        })
    };

    // Create the hover handler that tracks the currently hovered date.
    let on_day_hover = {
        let hover_date = hover_date.clone();
        Callback::from(move |date: Date| {
            hover_date.set(Some(date));
        })
    };

    let on_day_hover_end = {
        let hover_date = hover_date.clone();
        Callback::from(move |_: ()| {
            hover_date.set(None);
        })
    };

    // Create the navigation callback that updates the displayed month.
    let on_shown_date_change = {
        let shown_date = shown_date.clone();
        Callback::from(move |date: Date| {
            shown_date.set(date);
        })
    };

    // Extract the optional extra CSS class from props.
    let extra_class = props.class_name.clone().unwrap_or_default();

    html! {
        <div class={classes!("rdrDateRangeWrapper", extra_class)}>
            <Calendar
                shown_date={Some(*shown_date)}
                months={props.months}
                direction={props.direction}
                min_date={props.min_date}
                max_date={props.max_date}
                disabled_dates={props.disabled_dates.clone()}
                disabled_weekdays={props.disabled_weekdays.clone()}
                disabled_ranges={props.disabled_ranges.clone()}
                is_date_disabled={props.is_date_disabled.clone()}
                ranges={props.ranges.clone()}
                focused_range={*focused_range_idx}
                range_focus_start={*range_focus == RangeFocus::Start}
                on_day_click={on_day_click}
                on_day_hover={on_day_hover}
                on_day_hover_end={on_day_hover_end}
                on_shown_date_change={on_shown_date_change}
                preview={preview}
                show_week_numbers={props.show_week_numbers}
                show_date_display={props.show_date_display}
                locale={props.locale.clone()}
                day_content_renderer={props.day_content_renderer.clone()}
                show_month_and_year_pickers={props.show_month_and_year_pickers}
            />
        </div>
    }
}
