use time::{Date, Weekday};
use yew::prelude::*;

use yew_date_range_core::models::calendar_direction::CalendarDirection;
use yew_date_range_core::models::calendar_locale::CalendarLocale;
use yew_date_range_core::models::day_state::DayState;
use yew_date_range_core::models::range_change::RangeChange;
use yew_date_range_core::models::range_change_source::RangeChangeSource;
use yew_date_range_core::models::range_selection::RangeSelection;

use crate::components::date_range::DateRange;
use crate::components::defined_range::DefinedRange;
use crate::styles::style_injector::StyleInjector;

/// Properties for the DateRangePicker component.
#[derive(Properties, Clone, PartialEq)]
pub struct DateRangePickerProps {
    /// The current date ranges.
    #[prop_or_default]
    pub ranges: Vec<RangeSelection>,

    /// Callback when ranges change.
    #[prop_or_default]
    pub on_change: Callback<RangeChange>,

    /// Number of months to show.
    #[prop_or(2)]
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

    /// Show the defined ranges sidebar.
    #[prop_or(true)]
    pub show_defined_ranges: bool,

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

    /// Initial shown date.
    #[prop_or_default]
    pub shown_date: Option<Date>,

    /// Defined range header content.
    #[prop_or_default]
    pub defined_range_header: Option<Html>,

    /// Defined range footer content.
    #[prop_or_default]
    pub defined_range_footer: Option<Html>,

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

/// The DateRangePicker combines a DefinedRange sidebar with a DateRange calendar.
#[function_component(DateRangePicker)]
pub fn date_range_picker(props: &DateRangePickerProps) -> Html {
    // Inject the default CSS into the document head on first render.
    StyleInjector::inject_default_styles();

    // Initialize state for tracking the hover preview range.
    let preview = use_state(|| None::<(Date, Date)>);

    // Create the callback for handling predefined range selections from the sidebar.
    let on_defined_select = {
        let on_change = props.on_change.clone();
        Callback::from(move |range: RangeSelection| {
            on_change.emit(RangeChange {
                range,
                source: RangeChangeSource::DefinedRange,
            });
        })
    };

    // Create the callback for updating the preview when hovering sidebar ranges.
    let on_preview_change = {
        let preview = preview.clone();
        Callback::from(move |p: Option<(Date, Date)>| {
            preview.set(p);
        })
    };

    // Extract optional extra CSS class from props.
    let extra_class = props.class_name.clone().unwrap_or_default();

    // Determine the CSS class for the layout direction.
    let direction_class = match props.direction {
        CalendarDirection::Horizontal => "rdrDateRangePickerHorizontal",
        CalendarDirection::Vertical => "rdrDateRangePickerVertical",
    };

    html! {
        <div class={classes!("rdrDateRangePickerWrapper", direction_class, extra_class)}>
            if props.show_defined_ranges {
                <DefinedRange
                    ranges={props.ranges.clone()}
                    on_select={on_defined_select}
                    on_preview_change={on_preview_change}
                    header_content={props.defined_range_header.clone()}
                    footer_content={props.defined_range_footer.clone()}
                    locale={props.locale.clone()}
                />
            }
            <DateRange
                ranges={props.ranges.clone()}
                on_change={props.on_change.clone()}
                months={props.months}
                direction={props.direction}
                min_date={props.min_date}
                max_date={props.max_date}
                disabled_dates={props.disabled_dates.clone()}
                disabled_weekdays={props.disabled_weekdays.clone()}
                disabled_ranges={props.disabled_ranges.clone()}
                is_date_disabled={props.is_date_disabled.clone()}
                show_selection_preview={props.show_selection_preview}
                move_range_on_first_selection={props.move_range_on_first_selection}
                retain_end_date_on_first_selection={props.retain_end_date_on_first_selection}
                show_week_numbers={props.show_week_numbers}
                show_date_display={props.show_date_display}
                locale={props.locale.clone()}
                preview={*preview}
                day_content_renderer={props.day_content_renderer.clone()}
                show_month_and_year_pickers={props.show_month_and_year_pickers}
                shown_date={props.shown_date}
                max_span={props.max_span}
                min_span={props.min_span}
            />
        </div>
    }
}
