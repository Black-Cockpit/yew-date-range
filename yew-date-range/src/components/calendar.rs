use time::{Date, Weekday};
use yew::prelude::*;

use yew_date_range_core::models::calendar_direction::CalendarDirection;
use yew_date_range_core::models::calendar_locale::CalendarLocale;
use yew_date_range_core::models::day_state::DayState;
use yew_date_range_core::models::navigation_action::NavigationAction;
use yew_date_range_core::models::range_selection::RangeSelection;
use yew_date_range_core::utils::date_helper::DateHelper;

use crate::components::calendar_renderer::CalendarRenderer;
use crate::components::calendar_view::CalendarView;
use crate::styles::style_injector::StyleInjector;

/// Properties for the Calendar component.
#[derive(Properties, Clone, PartialEq)]
pub struct CalendarProps {
    /// Currently displayed month/year (controlled).
    #[prop_or_default]
    pub shown_date: Option<Date>,

    /// Number of months to display.
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

    /// Disabled specific dates.
    #[prop_or_default]
    pub disabled_dates: Vec<Date>,

    /// Disabled weekdays (e.g. disable all Sundays).
    #[prop_or_default]
    pub disabled_weekdays: Vec<Weekday>,

    /// Currently selected ranges.
    #[prop_or_default]
    pub ranges: Vec<RangeSelection>,

    /// Index of the focused range.
    #[prop_or(0)]
    pub focused_range: usize,

    /// Whether focusing on start (true) or end (false) of range.
    #[prop_or(false)]
    pub range_focus_start: bool,

    /// Callback when a day is clicked.
    #[prop_or_default]
    pub on_day_click: Callback<Date>,

    /// Callback when hovering over a day.
    #[prop_or_default]
    pub on_day_hover: Callback<Date>,

    /// Callback when mouse leaves the calendar.
    #[prop_or_default]
    pub on_day_hover_end: Callback<()>,

    /// Callback when navigating months.
    #[prop_or_default]
    pub on_navigate: Callback<NavigationAction>,

    /// Callback when the displayed month changes.
    #[prop_or_default]
    pub on_shown_date_change: Callback<Date>,

    /// Preview range (hover preview).
    #[prop_or_default]
    pub preview: Option<(Date, Date)>,

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

    /// Custom day content renderer.
    #[prop_or_default]
    pub day_content_renderer: Option<Callback<DayState, Html>>,

    /// Show month and year pickers.
    #[prop_or(true)]
    pub show_month_and_year_pickers: bool,

    /// Disabled date ranges (contiguous blocks).
    #[prop_or_default]
    pub disabled_ranges: Vec<(Date, Date)>,

    /// Custom callback to determine if a date is disabled.
    #[prop_or_default]
    pub is_date_disabled: Option<Callback<Date, bool>>,
}

/// The Calendar component displays one or more months with day cells.
/// Features PrimeReact-style drill-down navigation: Days → Months → Years.
#[function_component(Calendar)]
pub fn calendar(props: &CalendarProps) -> Html {
    // Inject the default CSS into the document head on first render.
    StyleInjector::inject_default_styles();

    // Resolve the locale, falling back to the default English locale.
    let locale = props.locale.clone().unwrap_or_default();

    // Retrieve today's date for highlighting the current day.
    let t = DateHelper::today();

    // Initialize the internal shown date state when not externally controlled.
    let internal_shown_date = use_state(|| {
        props.shown_date.unwrap_or(t)
    });

    // Sync internal state when the shown_date prop changes from the parent.
    {
        let internal_shown_date = internal_shown_date.clone();
        let prop_shown_date = props.shown_date;
        use_effect_with(prop_shown_date, move |sd| {
            if let Some(d) = *sd {
                internal_shown_date.set(d);
            }
            || ()
        });
    }

    // Resolve the effective shown date from props or internal state.
    let shown_date = props.shown_date.unwrap_or(*internal_shown_date);

    // Initialize the drill-down view state to the day grid.
    let view = use_state(|| CalendarView::Days);

    // Create a helper closure that updates both internal state and parent callback.
    let update_shown_date = {
        let internal_shown_date = internal_shown_date.clone();
        let on_shown_date_change = props.on_shown_date_change.clone();
        move |new_date: Date| {
            internal_shown_date.set(new_date);
            on_shown_date_change.emit(new_date);
        }
    };

    // Create the month navigation handler for prev/next actions.
    let on_nav = {
        let on_navigate = props.on_navigate.clone();
        let update_shown_date = update_shown_date.clone();

        Callback::from(move |action: NavigationAction| {
            let new_date = match action {
                NavigationAction::PrevMonth => DateHelper::sub_months(shown_date, 1),
                NavigationAction::NextMonth => DateHelper::add_months(shown_date, 1),
                NavigationAction::PrevYear => DateHelper::sub_months(shown_date, 12),
                NavigationAction::NextYear => DateHelper::add_months(shown_date, 12),
            };
            update_shown_date(new_date);
            on_navigate.emit(action);
        })
    };

    // Create the previous navigation button callback per current view.
    let on_prev_click = {
        let view = view.clone();
        let on_nav = on_nav.clone();
        let update_shown_date = update_shown_date.clone();
        Callback::from(move |_: MouseEvent| {
            match *view {
                CalendarView::Days => on_nav.emit(NavigationAction::PrevMonth),
                CalendarView::Months => {
                    let new_date = DateHelper::sub_months(shown_date, 12);
                    update_shown_date(new_date);
                }
                CalendarView::Years => {
                    let new_date = DateHelper::sub_months(shown_date, 120);
                    update_shown_date(new_date);
                }
            }
        })
    };

    // Create the next navigation button callback per current view.
    let on_next_click = {
        let view = view.clone();
        let on_nav = on_nav.clone();
        let update_shown_date = update_shown_date.clone();
        Callback::from(move |_: MouseEvent| {
            match *view {
                CalendarView::Days => on_nav.emit(NavigationAction::NextMonth),
                CalendarView::Months => {
                    let new_date = DateHelper::add_months(shown_date, 12);
                    update_shown_date(new_date);
                }
                CalendarView::Years => {
                    let new_date = DateHelper::add_months(shown_date, 120);
                    update_shown_date(new_date);
                }
            }
        })
    };

    // Create the callback for switching to the month picker view.
    let on_month_title_click = {
        let view = view.clone();
        Callback::from(move |_: MouseEvent| {
            view.set(CalendarView::Months);
        })
    };

    // Create the callback for switching to the year picker view.
    let on_year_title_click = {
        let view = view.clone();
        Callback::from(move |_: MouseEvent| {
            view.set(CalendarView::Years);
        })
    };

    // Create the callback for selecting a month and returning to the day view.
    let on_month_cell_click = {
        let view = view.clone();
        let update_shown_date = update_shown_date.clone();
        Callback::from(move |month_index: u8| {
            let month = DateHelper::index_to_month(month_index as usize);
            let max_day = DateHelper::days_in_month(shown_date.year(), month);
            let day = shown_date.day().min(max_day);
            if let Ok(new_date) = Date::from_calendar_date(shown_date.year(), month, day) {
                update_shown_date(new_date);
            }
            view.set(CalendarView::Days);
        })
    };

    // Create the callback for selecting a year and switching to the month view.
    let on_year_cell_click = {
        let view = view.clone();
        let update_shown_date = update_shown_date.clone();
        Callback::from(move |year: i32| {
            let max_day = DateHelper::days_in_month(year, shown_date.month());
            let day = shown_date.day().min(max_day);
            if let Ok(new_date) = Date::from_calendar_date(year, shown_date.month(), day) {
                update_shown_date(new_date);
            }
            view.set(CalendarView::Months);
        })
    };

    // Build the month grid data for all displayed months.
    let months_data = DateHelper::get_months_to_display(
        shown_date.year(),
        shown_date.month(),
        props.months,
        locale.start_of_week,
    );

    // Determine the CSS class for the layout direction.
    let direction_class = match props.direction {
        CalendarDirection::Horizontal => "rdrMonthsHorizontal",
        CalendarDirection::Vertical => "rdrMonthsVertical",
    };

    // Extract optional extra CSS class from props.
    let extra_class = props.class_name.clone().unwrap_or_default();
    let show_pickers = props.show_month_and_year_pickers;
    let current_view = *view;

    // Compute the decade range for the year picker view.
    let decade_start = (shown_date.year() / 10) * 10;
    let decade_end = decade_start + 9;

    // Build the navigation title HTML based on the current drill-down view.
    let nav_title_html = if show_pickers {
        match current_view {
            CalendarView::Days => {
                let month_name = locale.month_name(shown_date.month());
                let year = shown_date.year();
                html! {
                    <span class="rdrMonthAndYearPickers">
                        <button
                            class="rdrMonthPickerTitle"
                            onclick={on_month_title_click}
                            aria-label={locale.select_month_label.clone()}
                        >
                            {month_name}
                        </button>
                        <button
                            class="rdrYearPickerTitle"
                            onclick={on_year_title_click}
                            aria-label={locale.select_year_label.clone()}
                        >
                            {year.to_string()}
                        </button>
                    </span>
                }
            }
            CalendarView::Months => {
                let year = shown_date.year();
                html! {
                    <span class="rdrMonthAndYearPickers">
                        <button
                            class="rdrYearPickerTitle"
                            onclick={on_year_title_click}
                            aria-label={locale.select_year_label.clone()}
                        >
                            {year.to_string()}
                        </button>
                    </span>
                }
            }
            CalendarView::Years => {
                html! {
                    <span class="rdrMonthAndYearPickers">
                        <span class="rdrDecadeTitle">
                            {format!("{} - {}", decade_start, decade_end)}
                        </span>
                    </span>
                }
            }
        }
    } else {
        html! {}
    };

    // Determine the accessibility labels for prev/next navigation buttons.
    let prev_label = match current_view {
        CalendarView::Days => locale.prev_month_label.clone(),
        CalendarView::Months => locale.prev_year_label.clone(),
        CalendarView::Years => locale.prev_decade_label.clone(),
    };
    let next_label = match current_view {
        CalendarView::Days => locale.next_month_label.clone(),
        CalendarView::Months => locale.next_year_label.clone(),
        CalendarView::Years => locale.next_decade_label.clone(),
    };

    html! {
        <div class={classes!("rdrCalendarWrapper", extra_class)}>
            // Date display header
            if props.show_date_display {
                <div class="rdrDateDisplayWrapper">
                    { for props.ranges.iter().map(|range| {
                        let start_str = range.start_date
                            .map(|d| DateHelper::format_date_display(d, &locale.month_names))
                            .unwrap_or_else(|| locale.start_date_placeholder.clone());
                        let end_str = range.end_date
                            .map(|d| DateHelper::format_date_display(d, &locale.month_names))
                            .unwrap_or_else(|| locale.end_date_placeholder.clone());
                        let color = range.color.clone().unwrap_or_else(|| "#3d91ff".into());

                        html! {
                            <div class="rdrDateDisplay" style={format!("color: {color}")}>
                                <span class="rdrDateDisplayItem">
                                    <input
                                        readonly=true
                                        value={start_str}
                                        class="rdrDateInput"
                                    />
                                </span>
                                <span class="rdrDateDisplayItemSeparator">{" -> "}</span>
                                <span class="rdrDateDisplayItem">
                                    <input
                                        readonly=true
                                        value={end_str}
                                        class="rdrDateInput"
                                    />
                                </span>
                            </div>
                        }
                    })}
                </div>
            }

            // Navigation header
            <div class="rdrMonthAndYearWrapper">
                <button
                    class="rdrPrevButton rdrNextPrevButton"
                    onclick={on_prev_click}
                    aria-label={prev_label}
                >
                    <i class="rdrPrevButtonIcon"></i>
                </button>

                {nav_title_html}

                <button
                    class="rdrNextButton rdrNextPrevButton"
                    onclick={on_next_click}
                    aria-label={next_label}
                >
                    <i class="rdrNextButtonIcon"></i>
                </button>
            </div>

            // Body: depends on view
            if current_view == CalendarView::Days {
                <div class={classes!("rdrMonths", direction_class)}>
                    { for months_data.iter().map(|month_data| {
                        CalendarRenderer::render_month(
                            month_data,
                            &locale,
                            &props.ranges,
                            props.preview,
                            props.min_date,
                            props.max_date,
                            &props.disabled_dates,
                            &props.disabled_weekdays,
                            &props.disabled_ranges,
                            &props.is_date_disabled,
                            props.show_week_numbers,
                            &props.on_day_click,
                            &props.on_day_hover,
                            &props.on_day_hover_end,
                            &props.day_content_renderer,
                            t,
                        )
                    })}
                </div>
            }

            if current_view == CalendarView::Months {
                {CalendarRenderer::render_month_picker_grid(
                    &locale,
                    shown_date.month() as u8,
                    &on_month_cell_click,
                )}
            }

            if current_view == CalendarView::Years {
                {CalendarRenderer::render_year_picker_grid(
                    &locale,
                    decade_start,
                    shown_date.year(),
                    &on_year_cell_click,
                )}
            }
        </div>
    }
}

