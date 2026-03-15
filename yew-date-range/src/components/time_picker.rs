use yew::prelude::*;

use yew_date_range_core::models::calendar_locale::CalendarLocale;
use yew_date_range_core::models::hour_format::HourFormat;
use yew_date_range_core::models::time_granularity::TimeGranularity;
use yew_date_range_core::models::time_selection::TimeSelection;

/// Properties for the TimePicker component.
#[derive(Properties, Clone, PartialEq)]
pub struct TimePickerProps {
    /// Current time value.
    #[prop_or_default]
    pub value: TimeSelection,

    /// Callback when time changes.
    #[prop_or_default]
    pub on_change: Callback<TimeSelection>,

    /// Hour display format (12h or 24h).
    #[prop_or_default]
    pub hour_format: HourFormat,

    /// Which fields to show.
    #[prop_or_default]
    pub granularity: TimeGranularity,

    /// Whether the picker is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS class.
    #[prop_or_default]
    pub class_name: Option<String>,

    /// Locale configuration.
    #[prop_or_default]
    pub locale: Option<CalendarLocale>,
}

/// TimePicker component for selecting hours, minutes, and seconds.
#[function_component(TimePicker)]
pub fn time_picker(props: &TimePickerProps) -> Html {
    // Extract the current time value and configuration from props.
    let value = props.value;
    let on_change = props.on_change.clone();
    let disabled = props.disabled;
    let hour_format = props.hour_format;

    // Resolve the locale, falling back to the default English locale.
    let locale = props.locale.clone().unwrap_or_default();

    // Create the callback for incrementing the hour.
    let on_hour_up = {
        let on_change = on_change.clone();
        Callback::from(move |_: MouseEvent| {
            if !disabled {
                on_change.emit(value.increment_hour());
            }
        })
    };

    // Create the callback for decrementing the hour.
    let on_hour_down = {
        let on_change = on_change.clone();
        Callback::from(move |_: MouseEvent| {
            if !disabled {
                on_change.emit(value.decrement_hour());
            }
        })
    };

    // Create the callback for incrementing the minute.
    let on_minute_up = {
        let on_change = on_change.clone();
        Callback::from(move |_: MouseEvent| {
            if !disabled {
                on_change.emit(value.increment_minute());
            }
        })
    };

    // Create the callback for decrementing the minute.
    let on_minute_down = {
        let on_change = on_change.clone();
        Callback::from(move |_: MouseEvent| {
            if !disabled {
                on_change.emit(value.decrement_minute());
            }
        })
    };

    // Create the callback for incrementing the second.
    let on_second_up = {
        let on_change = on_change.clone();
        Callback::from(move |_: MouseEvent| {
            if !disabled {
                on_change.emit(value.increment_second());
            }
        })
    };

    // Create the callback for decrementing the second.
    let on_second_down = {
        let on_change = on_change.clone();
        Callback::from(move |_: MouseEvent| {
            if !disabled {
                on_change.emit(value.decrement_second());
            }
        })
    };

    // Create the callback for toggling between AM and PM.
    let on_period_toggle = {
        let on_change = on_change.clone();
        Callback::from(move |_: MouseEvent| {
            if !disabled {
                on_change.emit(value.toggle_period());
            }
        })
    };

    // Format the hour display value based on the configured format.
    let hour_display = match hour_format {
        HourFormat::H24 => format!("{:02}", value.hour),
        HourFormat::H12 => format!("{:02}", value.hour_12()),
    };

    // Extract optional extra CSS class and disabled state class.
    let extra_class = props.class_name.clone().unwrap_or_default();
    let disabled_class = if disabled { "rdrTimePickerDisabled" } else { "" };

    html! {
        <div class={classes!("rdrTimePicker", disabled_class, extra_class)}>
            if props.granularity.show_hours {
                <div class="rdrTimePickerField">
                    <button
                        class="rdrTimePickerUp"
                        onclick={on_hour_up}
                        disabled={disabled}
                        aria-label={locale.increment_hour_label.clone()}
                    >
                        <span class="rdrTimePickerArrow rdrTimePickerArrowUp">{"▲"}</span>
                    </button>
                    <span class="rdrTimePickerValue">{hour_display}</span>
                    <button
                        class="rdrTimePickerDown"
                        onclick={on_hour_down}
                        disabled={disabled}
                        aria-label={locale.decrement_hour_label.clone()}
                    >
                        <span class="rdrTimePickerArrow rdrTimePickerArrowDown">{"▼"}</span>
                    </button>
                </div>
            }

            if props.granularity.show_hours && props.granularity.show_minutes {
                <span class="rdrTimePickerSeparator">{":"}</span>
            }

            if props.granularity.show_minutes {
                <div class="rdrTimePickerField">
                    <button
                        class="rdrTimePickerUp"
                        onclick={on_minute_up}
                        disabled={disabled}
                        aria-label={locale.increment_minute_label.clone()}
                    >
                        <span class="rdrTimePickerArrow rdrTimePickerArrowUp">{"▲"}</span>
                    </button>
                    <span class="rdrTimePickerValue">{format!("{:02}", value.minute)}</span>
                    <button
                        class="rdrTimePickerDown"
                        onclick={on_minute_down}
                        disabled={disabled}
                        aria-label={locale.decrement_minute_label.clone()}
                    >
                        <span class="rdrTimePickerArrow rdrTimePickerArrowDown">{"▼"}</span>
                    </button>
                </div>
            }

            if props.granularity.show_minutes && props.granularity.show_seconds {
                <span class="rdrTimePickerSeparator">{":"}</span>
            }

            if props.granularity.show_seconds {
                <div class="rdrTimePickerField">
                    <button
                        class="rdrTimePickerUp"
                        onclick={on_second_up}
                        disabled={disabled}
                        aria-label={locale.increment_second_label.clone()}
                    >
                        <span class="rdrTimePickerArrow rdrTimePickerArrowUp">{"▲"}</span>
                    </button>
                    <span class="rdrTimePickerValue">{format!("{:02}", value.second)}</span>
                    <button
                        class="rdrTimePickerDown"
                        onclick={on_second_down}
                        disabled={disabled}
                        aria-label={locale.decrement_second_label.clone()}
                    >
                        <span class="rdrTimePickerArrow rdrTimePickerArrowDown">{"▼"}</span>
                    </button>
                </div>
            }

            if hour_format == HourFormat::H12 {
                <div class="rdrTimePickerField rdrTimePickerPeriod">
                    <button
                        class="rdrTimePickerPeriodButton"
                        onclick={on_period_toggle}
                        disabled={disabled}
                        aria-label={locale.toggle_period_label.clone()}
                    >
                        <span class="rdrTimePickerValue">{value.period().to_string()}</span>
                    </button>
                </div>
            }
        </div>
    }
}
