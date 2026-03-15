use time::{Date, Weekday};
use yew::prelude::*;

use yew_date_range_core::models::calendar_direction::CalendarDirection;
use yew_date_range_core::models::calendar_locale::CalendarLocale;
use yew_date_range_core::models::date_format::DateFormat;
use yew_date_range_core::models::day_state::DayState;
use yew_date_range_core::models::display_mode::DisplayMode;
use yew_date_range_core::models::hour_format::HourFormat;
use yew_date_range_core::models::popup_trigger::PopupTrigger;
use yew_date_range_core::models::range_focus::RangeFocus;
use yew_date_range_core::models::range_selection::RangeSelection;
use yew_date_range_core::models::selection_mode::SelectionMode;
use yew_date_range_core::models::selection_value::SelectionValue;
use yew_date_range_core::models::time_granularity::TimeGranularity;
use yew_date_range_core::models::time_selection::TimeSelection;
use yew_date_range_core::utils::date_helper::DateHelper;
use yew_date_range_core::utils::range_helper::RangeHelper;

use crate::components::calendar::Calendar;
use crate::components::overlay::Overlay;
use crate::components::time_picker::TimePicker;
use crate::styles::style_injector::StyleInjector;

/// Properties for the DatePicker component.
#[derive(Properties, Clone, PartialEq)]
pub struct DatePickerProps {
    /// Selection mode.
    #[prop_or_default]
    pub selection_mode: SelectionMode,

    /// Current selection value.
    #[prop_or_default]
    pub value: SelectionValue,

    /// Callback when the selection changes.
    #[prop_or_default]
    pub on_change: Callback<SelectionValue>,

    /// Inline or popup display mode.
    #[prop_or_default]
    pub display_mode: DisplayMode,

    /// What triggers the popup (click, focus, both).
    #[prop_or_default]
    pub popup_trigger: PopupTrigger,

    /// Number of months to display.
    #[prop_or(1)]
    pub months: usize,

    /// Layout direction.
    #[prop_or_default]
    pub direction: CalendarDirection,

    /// Show the date display header.
    #[prop_or(true)]
    pub show_date_display: bool,

    /// Show week numbers.
    #[prop_or(false)]
    pub show_week_numbers: bool,

    /// Show month and year pickers.
    #[prop_or(true)]
    pub show_month_and_year_pickers: bool,

    /// Minimum selectable date.
    #[prop_or_default]
    pub min_date: Option<Date>,

    /// Maximum selectable date.
    #[prop_or_default]
    pub max_date: Option<Date>,

    /// Disabled specific dates.
    #[prop_or_default]
    pub disabled_dates: Vec<Date>,

    /// Disabled weekdays.
    #[prop_or_default]
    pub disabled_weekdays: Vec<Weekday>,

    /// Disabled date ranges (contiguous blocks).
    #[prop_or_default]
    pub disabled_ranges: Vec<(Date, Date)>,

    /// Custom callback to determine if a date is disabled.
    #[prop_or_default]
    pub is_date_disabled: Option<Callback<Date, bool>>,

    /// Maximum number of days allowed in a range selection (inclusive).
    #[prop_or_default]
    pub max_span: Option<i64>,

    /// Minimum number of days required in a range selection (inclusive).
    #[prop_or_default]
    pub min_span: Option<i64>,

    /// Date format for input display and parsing.
    #[prop_or_default]
    pub date_format: Option<DateFormat>,

    /// Placeholder text for the input.
    #[prop_or_default]
    pub placeholder: Option<String>,

    /// Enable time selection.
    #[prop_or(false)]
    pub show_time: bool,

    /// Current time value.
    #[prop_or_default]
    pub time_value: Option<TimeSelection>,

    /// Callback when time changes.
    #[prop_or_default]
    pub on_time_change: Callback<TimeSelection>,

    /// Hour format.
    #[prop_or_default]
    pub hour_format: HourFormat,

    /// Time granularity (which fields to show).
    #[prop_or_default]
    pub time_granularity: TimeGranularity,

    /// Show a "Today" button.
    #[prop_or(false)]
    pub show_today_button: bool,

    /// Show a "Clear" button.
    #[prop_or(false)]
    pub show_clear_button: bool,

    /// Show an icon in the input.
    #[prop_or(false)]
    pub show_icon: bool,

    /// Whether the input is read-only (no manual typing).
    #[prop_or(false)]
    pub read_only: bool,

    /// Whether the entire picker is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// Locale configuration.
    #[prop_or_default]
    pub locale: Option<CalendarLocale>,

    /// Custom class name.
    #[prop_or_default]
    pub class_name: Option<String>,

    /// Custom day content renderer.
    #[prop_or_default]
    pub day_content_renderer: Option<Callback<DayState, Html>>,

    /// Initial shown date (the month to display initially).
    #[prop_or_default]
    pub shown_date: Option<Date>,
}

/// The DatePicker component: a PrimeReact-style date picker with inline and popup modes.
#[function_component(DatePicker)]
pub fn date_picker(props: &DatePickerProps) -> Html {
    // Inject the default CSS into the document head on first render.
    StyleInjector::inject_default_styles();

    // Initialize component state for popup visibility, focus tracking, and hover.
    let popup_visible = use_state(|| false);
    let input_ref = use_node_ref();
    let range_focus = use_state(|| RangeFocus::Start);
    let hover_date = use_state(|| None::<Date>);
    let is_selecting = use_state(|| false);

    // Resolve the date format from props or use the default pattern.
    let fmt = props.date_format.clone().unwrap_or_default();

    // Resolve the locale, falling back to the default English locale.
    let locale = props.locale.clone().unwrap_or_default();

    // Initialize the shown date from props or the first selected date.
    let shown_date = use_state(|| {
        props.shown_date.unwrap_or_else(|| match &props.value {
            SelectionValue::Single(Some(d)) => *d,
            SelectionValue::Range { start: Some(s), .. } => *s,
            SelectionValue::Multiple(dates) if !dates.is_empty() => dates[0],
            _ => DateHelper::today(),
        })
    });

    // Build the formatted text for the input field based on the current selection.
    let input_text = {
        match &props.value {
            SelectionValue::Single(Some(d)) => fmt.format(*d),
            SelectionValue::Range { start, end } => fmt.format_range(*start, *end),
            SelectionValue::Multiple(dates) => fmt.format_multiple(dates, ", "),
            _ => String::new(),
        }
    };

    // Convert the selection value into a vector of range selections for the calendar.
    let ranges_for_calendar: Vec<RangeSelection> = match &props.value {
        SelectionValue::Single(date) => {
            vec![RangeSelection::new("selection").with_dates(*date, *date)]
        }
        SelectionValue::Range { start, end } => {
            vec![RangeSelection::new("selection").with_dates(*start, *end)]
        }
        SelectionValue::Multiple(dates) => dates
            .iter()
            .map(|d| RangeSelection::new("selection").with_dates(Some(*d), Some(*d)))
            .collect(),
    };

    // Compute the hover preview range only during active mid-selection.
    let preview = if props.selection_mode == SelectionMode::Range {
        RangeHelper::compute_preview(
            *hover_date,
            &ranges_for_calendar,
            0,
            *range_focus == RangeFocus::Start,
            *is_selecting,
        )
    } else {
        None
    };

    // Create the day click handler that updates the selection based on the current mode.
    let on_day_click = {
        let on_change = props.on_change.clone();
        let value = props.value.clone();
        let mode = props.selection_mode;
        let range_focus = range_focus.clone();
        let is_selecting = is_selecting.clone();
        let popup_visible = popup_visible.clone();
        let display_mode = props.display_mode;

        Callback::from(move |date: Date| {
            let new_value = match mode {
                SelectionMode::Single => {
                    if display_mode == DisplayMode::Popup {
                        popup_visible.set(false);
                    }
                    SelectionValue::Single(Some(date))
                }
                SelectionMode::Range => {
                    let current_focus = *range_focus;
                    match current_focus {
                        RangeFocus::Start => {
                            range_focus.set(RangeFocus::End);
                            is_selecting.set(true);
                            SelectionValue::Range {
                                start: Some(date),
                                end: None,
                            }
                        }
                        RangeFocus::End => {
                            range_focus.set(RangeFocus::Start);
                            is_selecting.set(false);
                            let start = match &value {
                                SelectionValue::Range { start, .. } => *start,
                                _ => None,
                            };
                            // Normalize order
                            let (s, e) = match start {
                                Some(s) if date < s => (Some(date), Some(s)),
                                Some(s) => (Some(s), Some(date)),
                                None => (Some(date), Some(date)),
                            };
                            if display_mode == DisplayMode::Popup {
                                popup_visible.set(false);
                            }
                            SelectionValue::Range { start: s, end: e }
                        }
                    }
                }
                SelectionMode::Multiple => {
                    let mut dates = match &value {
                        SelectionValue::Multiple(d) => d.clone(),
                        _ => Vec::new(),
                    };
                    if let Some(pos) = dates.iter().position(|d| *d == date) {
                        dates.remove(pos);
                    } else {
                        dates.push(date);
                        dates.sort();
                    }
                    SelectionValue::Multiple(dates)
                }
            };
            on_change.emit(new_value);
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

    // Create the popup open callback that respects the disabled state.
    let open_popup = {
        let popup_visible = popup_visible.clone();
        let disabled = props.disabled;
        Callback::from(move |_: ()| {
            if !disabled {
                popup_visible.set(true);
            }
        })
    };

    let close_popup = {
        let popup_visible = popup_visible.clone();
        Callback::from(move |_: ()| {
            popup_visible.set(false);
        })
    };

    let on_input_click = {
        let open_popup = open_popup.clone();
        let trigger = props.popup_trigger;
        Callback::from(move |_: MouseEvent| {
            if trigger == PopupTrigger::Click || trigger == PopupTrigger::Both {
                open_popup.emit(());
            }
        })
    };

    let on_input_focus = {
        let open_popup = open_popup.clone();
        let trigger = props.popup_trigger;
        Callback::from(move |_: FocusEvent| {
            if trigger == PopupTrigger::Focus || trigger == PopupTrigger::Both {
                open_popup.emit(());
            }
        })
    };

    // Create the text input change handler that parses typed dates.
    let on_input_change = {
        let on_change = props.on_change.clone();
        let mode = props.selection_mode;
        let fmt = fmt.clone();
        let min_date = props.min_date;
        let max_date = props.max_date;

        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let text = input.value();

            match mode {
                SelectionMode::Single => {
                    if let Some(date) = fmt.parse(&text) {
                        if DateHelper::validate_against_constraints(date, min_date, max_date) {
                            on_change.emit(SelectionValue::Single(Some(date)));
                        }
                    }
                }
                SelectionMode::Range => {
                    let parts: Vec<&str> = text.split(" - ").collect();
                    if parts.len() == 2 {
                        let start = fmt.parse(parts[0].trim());
                        let end = fmt.parse(parts[1].trim());
                        if let (Some(s), Some(e)) = (start, end) {
                            if DateHelper::validate_against_constraints(s, min_date, max_date)
                                && DateHelper::validate_against_constraints(e, min_date, max_date)
                            {
                                on_change.emit(SelectionValue::Range {
                                    start: Some(s),
                                    end: Some(e),
                                });
                            }
                        }
                    }
                }
                SelectionMode::Multiple => {
                    let dates: Vec<Date> = text
                        .split(", ")
                        .filter_map(|s| fmt.parse(s.trim()))
                        .filter(|d| DateHelper::validate_against_constraints(*d, min_date, max_date))
                        .collect();
                    if !dates.is_empty() {
                        on_change.emit(SelectionValue::Multiple(dates));
                    }
                }
            }
        })
    };

    // Create the today button handler that selects the current date.
    let on_today = {
        let on_change = props.on_change.clone();
        let mode = props.selection_mode;
        let shown_date = shown_date.clone();

        Callback::from(move |_: MouseEvent| {
            let t = DateHelper::today();
            shown_date.set(t);
            match mode {
                SelectionMode::Single => on_change.emit(SelectionValue::Single(Some(t))),
                SelectionMode::Range => on_change.emit(SelectionValue::Range {
                    start: Some(t),
                    end: Some(t),
                }),
                SelectionMode::Multiple => on_change.emit(SelectionValue::Multiple(vec![t])),
            }
        })
    };

    // Create the clear button handler that resets the selection.
    let on_clear = {
        let on_change = props.on_change.clone();
        let mode = props.selection_mode;

        Callback::from(move |_: MouseEvent| match mode {
            SelectionMode::Single => on_change.emit(SelectionValue::Single(None)),
            SelectionMode::Range => on_change.emit(SelectionValue::Range { start: None, end: None }),
            SelectionMode::Multiple => on_change.emit(SelectionValue::Multiple(Vec::new())),
        })
    };

    // Extract optional extra CSS class and disabled state class.
    let extra_class = props.class_name.clone().unwrap_or_default();
    let disabled_class = if props.disabled { "rdrDatePickerDisabled" } else { "" };

    // Build the calendar content shared between inline and popup modes.
    let calendar_content = html! {
        <div class="rdrDatePickerCalendarWrapper">
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
                ranges={ranges_for_calendar}
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

            if props.show_time {
                <TimePicker
                    value={props.time_value.unwrap_or_default()}
                    on_change={props.on_time_change.clone()}
                    hour_format={props.hour_format}
                    granularity={props.time_granularity}
                    disabled={props.disabled}
                />
            }

            if props.show_today_button || props.show_clear_button {
                <div class="rdrDatePickerActions">
                    if props.show_today_button {
                        <button
                            class="rdrDatePickerTodayButton"
                            onclick={on_today}
                            disabled={props.disabled}
                        >
                            {locale.today_label.clone()}
                        </button>
                    }
                    if props.show_clear_button {
                        <button
                            class="rdrDatePickerClearButton"
                            onclick={on_clear}
                            disabled={props.disabled}
                        >
                            {locale.clear_label.clone()}
                        </button>
                    }
                </div>
            }
        </div>
    };

    match props.display_mode {
        DisplayMode::Inline => {
            html! {
                <div class={classes!("rdrDatePicker", "rdrDatePickerInline", disabled_class, extra_class)}>
                    {calendar_content}
                </div>
            }
        }
        DisplayMode::Popup => {
            let placeholder = props.placeholder.clone().unwrap_or_else(|| match props.selection_mode {
                SelectionMode::Single => locale.select_date_placeholder.clone(),
                SelectionMode::Range => locale.select_range_placeholder.clone(),
                SelectionMode::Multiple => locale.select_dates_placeholder.clone(),
            });

            html! {
                <div class={classes!("rdrDatePicker", "rdrDatePickerPopup", disabled_class, extra_class)}>
                    <div class="rdrDatePickerInputWrapper" ref={input_ref.clone()}>
                        if props.show_icon {
                            <span class="rdrDatePickerIcon">
                                <svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor">
                                    <path d="M19 3h-1V1h-2v2H8V1H6v2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V8h14v11zM7 10h5v5H7z"/>
                                </svg>
                            </span>
                        }
                        <input
                            type="text"
                            class="rdrDatePickerInput"
                            value={input_text}
                            placeholder={placeholder}
                            onclick={on_input_click}
                            onfocus={on_input_focus}
                            oninput={on_input_change}
                            readonly={props.read_only}
                            disabled={props.disabled}
                            aria-label="Date input"
                        />
                    </div>
                    <Overlay
                        visible={*popup_visible}
                        on_close={close_popup}
                        target_ref={input_ref}
                        close_on_outside_click={true}
                        close_on_escape={true}
                    >
                        {calendar_content}
                    </Overlay>
                </div>
            }
        }
    }
}
