use time::{Date, Month, Weekday};
use yew::prelude::*;

use yew_date_range_core::models::calendar_locale::CalendarLocale;
use yew_date_range_core::models::day_state::DayState;
use yew_date_range_core::models::month_data::MonthData;
use yew_date_range_core::models::range_selection::RangeSelection;
use yew_date_range_core::utils::date_helper::DateHelper;
use yew_date_range_core::utils::range_helper::RangeHelper;

/// Internal rendering utilities for the calendar component.
///
/// Provides static methods for rendering month grids, day cells,
/// month picker grids, and year picker grids used by the calendar.
pub(crate) struct CalendarRenderer;

impl CalendarRenderer {
    /// Renders a 3x4 grid of month names for the month picker view.
    ///
    /// # Parameters
    ///
    /// - `_locale`: The calendar locale (reserved for future localization).
    /// - `current_month`: The currently selected month (1-based).
    /// - `on_click`: Callback emitting the zero-based month index on click.
    ///
    /// # Returns
    ///
    /// - The rendered month picker grid HTML.
    pub(crate) fn render_month_picker_grid(
        locale: &CalendarLocale,
        current_month: u8,
        on_click: &Callback<u8>,
    ) -> Html {
        // Use locale short month names for the grid cells.
        let month_short_names = &locale.short_month_names;
        let select_prefix = &locale.select_prefix;

        html! {
            <div class="rdrMonthPickerGrid">
                { for (0u8..12).map(|i| {
                    // Convert to 1-based for comparison with current_month.
                    let month_1 = i + 1;
                    let is_selected = month_1 == current_month;
                    let on_click = on_click.clone();
                    let label = month_short_names.get(i as usize).map(|s| s.as_str()).unwrap_or("?");

                    // Build the aria label using the locale select prefix.
                    let aria = format!("{} {}", select_prefix, label);

                    // Create the click handler emitting the zero-based index.
                    let onclick = Callback::from(move |_: MouseEvent| {
                        on_click.emit(i);
                    });

                    // Apply the selected class if this is the current month.
                    let class = if is_selected {
                        "rdrMonthPickerCell rdrMonthPickerCellSelected"
                    } else {
                        "rdrMonthPickerCell"
                    };

                    html! {
                        <button
                            class={class}
                            onclick={onclick}
                            aria-label={aria}
                        >
                            <span>{label}</span>
                        </button>
                    }
                })}
            </div>
        }
    }

    /// Renders a 2x5 grid of years for the year picker view.
    ///
    /// # Parameters
    ///
    /// - `decade_start`: The first year of the decade.
    /// - `current_year`: The currently selected year.
    /// - `on_click`: Callback emitting the selected year on click.
    ///
    /// # Returns
    ///
    /// - The rendered year picker grid HTML.
    pub(crate) fn render_year_picker_grid(
        locale: &CalendarLocale,
        decade_start: i32,
        current_year: i32,
        on_click: &Callback<i32>,
    ) -> Html {
        // Use the locale select prefix for aria labels.
        let select_prefix = &locale.select_prefix;

        html! {
            <div class="rdrYearPickerGrid">
                { for (0..10).map(|i| {
                    // Calculate the year for this cell.
                    let year = decade_start + i;
                    let is_selected = year == current_year;
                    let on_click = on_click.clone();

                    // Build the aria label using the locale select prefix.
                    let aria = format!("{} {}", select_prefix, year);

                    // Create the click handler emitting the year.
                    let onclick = Callback::from(move |_: MouseEvent| {
                        on_click.emit(year);
                    });

                    // Apply the selected class if this is the current year.
                    let class = if is_selected {
                        "rdrYearPickerCell rdrYearPickerCellSelected"
                    } else {
                        "rdrYearPickerCell"
                    };

                    html! {
                        <button
                            class={class}
                            onclick={onclick}
                            aria-label={aria}
                        >
                            <span>{year.to_string()}</span>
                        </button>
                    }
                })}
            </div>
        }
    }

    /// Renders a single month grid with day cells.
    ///
    /// # Parameters
    ///
    /// - `month_data`: The month data containing week rows.
    /// - `locale`: The calendar locale for day names.
    /// - `ranges`: The current range selections.
    /// - `preview`: The optional preview range.
    /// - `min_date`: The optional minimum date constraint.
    /// - `max_date`: The optional maximum date constraint.
    /// - `disabled_dates`: Specifically disabled dates.
    /// - `disabled_weekdays`: Disabled weekdays.
    /// - `disabled_ranges`: Disabled date ranges.
    /// - `is_date_disabled`: Optional custom disabled callback.
    /// - `show_week_numbers`: Whether to show week numbers.
    /// - `on_day_click`: Callback when a day is clicked.
    /// - `on_day_hover`: Callback when hovering over a day.
    /// - `on_day_hover_end`: Callback when mouse leaves.
    /// - `day_content_renderer`: Optional custom day content renderer.
    /// - `today_date`: Today's date for highlighting.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn render_month(
        month_data: &MonthData,
        locale: &CalendarLocale,
        ranges: &[RangeSelection],
        preview: Option<(Date, Date)>,
        min_date: Option<Date>,
        max_date: Option<Date>,
        disabled_dates: &[Date],
        disabled_weekdays: &[Weekday],
        disabled_ranges: &[(Date, Date)],
        is_date_disabled: &Option<Callback<Date, bool>>,
        show_week_numbers: bool,
        on_day_click: &Callback<Date>,
        on_day_hover: &Callback<Date>,
        on_day_hover_end: &Callback<()>,
        day_content_renderer: &Option<Callback<DayState, Html>>,
        today_date: Date,
    ) -> Html {
        // Get ordered day name headers.
        let day_names = locale.ordered_day_names();

        // Create the mouse leave handler.
        let on_mouse_leave = {
            let cb = on_day_hover_end.clone();
            Callback::from(move |_: MouseEvent| cb.emit(()))
        };

        html! {
            <div class="rdrMonth" onmouseleave={on_mouse_leave}>
                // Day of week headers.
                <div class="rdrWeekDays">
                    if show_week_numbers {
                        <span class="rdrWeekDay rdrWeekNumber">{locale.week_number_header.clone()}</span>
                    }
                    { for day_names.iter().map(|name| {
                        html! { <span class="rdrWeekDay">{name}</span> }
                    })}
                </div>

                // Week rows.
                <div class="rdrDays">
                    { for month_data.weeks.iter().map(|week| {
                        html! {
                            <div class="rdrWeek">
                                if show_week_numbers {
                                    <span class="rdrWeekNumber">
                                        {week.week_number.map(|w| w.to_string()).unwrap_or_default()}
                                    </span>
                                }
                                { for week.days.iter().map(|day_opt| {
                                    match day_opt {
                                        Some(date) => {
                                            // Build the day state for this cell.
                                            let state = Self::build_day_state(
                                                *date,
                                                month_data.month,
                                                ranges,
                                                preview,
                                                min_date,
                                                max_date,
                                                disabled_dates,
                                                disabled_weekdays,
                                                disabled_ranges,
                                                is_date_disabled,
                                                locale.start_of_week,
                                                today_date,
                                            );
                                            // Render the day cell.
                                            Self::render_day(
                                                &state,
                                                on_day_click,
                                                on_day_hover,
                                                day_content_renderer,
                                            )
                                        }
                                        None => html! { <span class="rdrDay rdrDayEmpty"></span> },
                                    }
                                })}
                            </div>
                        }
                    })}
                </div>
            </div>
        }
    }

    /// Builds the visual state for a single day cell.
    ///
    /// # Parameters
    ///
    /// - `date`: The date for this cell.
    /// - `display_month`: The month being displayed.
    /// - `ranges`: The current range selections.
    /// - `preview`: The optional preview range.
    /// - `min_date`: The optional minimum date constraint.
    /// - `max_date`: The optional maximum date constraint.
    /// - `disabled_dates`: Specifically disabled dates.
    /// - `disabled_weekdays`: Disabled weekdays.
    /// - `disabled_ranges`: Disabled date ranges.
    /// - `is_date_disabled_cb`: Optional custom disabled callback.
    /// - `week_start`: The configured first day of the week.
    /// - `today_date`: Today's date.
    ///
    /// # Returns
    ///
    /// - A `DayState` with all flags computed.
    #[allow(clippy::too_many_arguments)]
    fn build_day_state(
        date: Date,
        display_month: Month,
        ranges: &[RangeSelection],
        preview: Option<(Date, Date)>,
        min_date: Option<Date>,
        max_date: Option<Date>,
        disabled_dates: &[Date],
        disabled_weekdays: &[Weekday],
        disabled_ranges: &[(Date, Date)],
        is_date_disabled_cb: &Option<Callback<Date, bool>>,
        week_start: Weekday,
        today_date: Date,
    ) -> DayState {
        // Initialize the day state with default flags.
        let mut state = DayState::new(date);
        state.is_today = date == today_date;
        state.is_passive = date.month() != display_month;
        state.is_weekend = DateHelper::is_weekend(date);

        // Build a custom disabled function from the callback if provided.
        let custom_disabled_fn: Option<Box<dyn Fn(Date) -> bool>> = is_date_disabled_cb
            .as_ref()
            .map(|cb| {
                let cb = cb.clone();
                Box::new(move |d: Date| cb.emit(d)) as Box<dyn Fn(Date) -> bool>
            });

        // Check all disabled constraints.
        state.is_disabled = RangeHelper::is_date_disabled_full(
            date,
            min_date,
            max_date,
            disabled_dates,
            disabled_weekdays,
            disabled_ranges,
            custom_disabled_fn.as_deref(),
        );

        // Compute month and week boundary flags.
        let som = DateHelper::start_of_month(date);
        let eom = DateHelper::end_of_month(date);
        state.is_start_of_month = date == som;
        state.is_end_of_month = date == eom;

        let sow = DateHelper::start_of_week(date, week_start);
        let eow_date = DateHelper::end_of_week(date, week_start);
        state.is_start_of_week = date == sow;
        state.is_end_of_week = date == eow_date;

        // Check range membership and edge flags.
        for range in ranges {
            let (start, end) = range.normalized();
            if let (Some(s), Some(e)) = (start, end) {
                if date >= s && date <= e {
                    state.is_in_range = true;
                    state.color = range.color.clone();
                }
                if date == s {
                    state.is_start_edge = true;
                    state.is_selected = true;
                }
                if date == e {
                    state.is_end_edge = true;
                    state.is_selected = true;
                }
            } else if let Some(s) = start {
                if date == s {
                    state.is_selected = true;
                    state.is_start_edge = true;
                    state.is_end_edge = true;
                }
            }
        }

        // Check preview range membership.
        if let Some((ps, pe)) = preview {
            if date >= ps && date <= pe {
                state.is_preview = true;
            }
            if date == ps {
                state.is_preview_start = true;
            }
            if date == pe {
                state.is_preview_end = true;
            }
        }

        state
    }

    /// Renders a single day cell button.
    ///
    /// # Parameters
    ///
    /// - `state`: The computed day state.
    /// - `on_day_click`: Callback when the day is clicked.
    /// - `on_day_hover`: Callback when hovering over the day.
    /// - `day_content_renderer`: Optional custom content renderer.
    ///
    /// # Returns
    ///
    /// - The rendered day cell HTML.
    fn render_day(
        state: &DayState,
        on_day_click: &Callback<Date>,
        on_day_hover: &Callback<Date>,
        day_content_renderer: &Option<Callback<DayState, Html>>,
    ) -> Html {
        // Join all CSS classes for this day cell.
        let classes = state.css_classes().join(" ");
        let date = state.date;

        // Create the click handler with disabled check.
        let onclick = {
            let cb = on_day_click.clone();
            let disabled = state.is_disabled;
            Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                if !disabled {
                    cb.emit(date);
                }
            })
        };

        // Create the hover handler.
        let onmouseenter = {
            let cb = on_day_hover.clone();
            Callback::from(move |_: MouseEvent| {
                cb.emit(date);
            })
        };

        // Extract the day number for display.
        let day_number = state.date.day();

        // Determine the selection/range highlight color.
        let color = state.color.clone().unwrap_or_else(|| "#3d91ff".into());

        // Render custom or default day content.
        let content = if let Some(renderer) = day_content_renderer {
            renderer.emit(state.clone())
        } else {
            html! {
                <span class="rdrDayNumber">
                    <span>{day_number.to_string()}</span>
                </span>
            }
        };

        html! {
            <button
                class={classes}
                onclick={onclick}
                onmouseenter={onmouseenter}
                aria-label={DateHelper::format_date(date)}
                disabled={state.is_disabled}
            >
                if state.is_in_range || state.is_start_edge || state.is_end_edge {
                    <span class="rdrInRange" style={format!("background: {color}")}></span>
                }
                if state.is_start_edge {
                    <span class="rdrStartEdge" style={format!("background: {color}")}></span>
                }
                if state.is_end_edge {
                    <span class="rdrEndEdge" style={format!("background: {color}")}></span>
                }
                <span class="rdrDayPreview">
                    <span class="rdrDayStartPreview"></span>
                    <span class="rdrDayEndPreview"></span>
                </span>
                {content}
                if state.is_today {
                    <span class="rdrDayTodayDot"></span>
                }
            </button>
        }
    }
}
