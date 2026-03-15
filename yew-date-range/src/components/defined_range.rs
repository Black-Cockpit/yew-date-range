use yew::prelude::*;

use yew_date_range_core::models::calendar_locale::CalendarLocale;
use yew_date_range_core::models::range_selection::RangeSelection;
use yew_date_range_core::utils::range_helper::RangeHelper;

/// Properties for the DefinedRange component.
#[derive(Properties, Clone, PartialEq)]
pub struct DefinedRangeProps {
    /// The current ranges.
    #[prop_or_default]
    pub ranges: Vec<RangeSelection>,

    /// Callback when a range is selected.
    #[prop_or_default]
    pub on_select: Callback<RangeSelection>,

    /// Callback when hovering a range.
    #[prop_or_default]
    pub on_preview_change: Callback<Option<(time::Date, time::Date)>>,

    /// Custom class name.
    #[prop_or_default]
    pub class_name: Option<String>,

    /// Header content.
    #[prop_or_default]
    pub header_content: Option<Html>,

    /// Footer content.
    #[prop_or_default]
    pub footer_content: Option<Html>,

    /// Locale configuration.
    #[prop_or_default]
    pub locale: Option<CalendarLocale>,
}

/// The DefinedRange component shows a sidebar with predefined date ranges.
#[function_component(DefinedRange)]
pub fn defined_range(props: &DefinedRangeProps) -> Html {
    // Resolve the locale, falling back to the default English locale.
    let locale = props.locale.clone().unwrap_or_default();

    // Load predefined static ranges using locale labels and week start.
    let static_ranges = RangeHelper::default_static_ranges(&locale);

    // Retrieve the first range from props for selected-state comparison.
    let current_range = props.ranges.first().cloned().unwrap_or_default();

    // Extract optional extra CSS class from props.
    let extra_class = props.class_name.clone().unwrap_or_default();

    html! {
        <div class={classes!("rdrDefinedRangesWrapper", extra_class)}>
            if let Some(ref header) = props.header_content {
                {header.clone()}
            }

            <div class="rdrStaticRanges">
                { for static_ranges.iter().enumerate().map(|(idx, sr)| {
                    let range = sr.get_range();
                    let is_selected = RangeHelper::is_range_selected(&range, &current_range);
                    let label = sr.label.clone();

                    let on_click = {
                        let on_select = props.on_select.clone();
                        let range = range.clone();
                        Callback::from(move |_: MouseEvent| {
                            on_select.emit(range.clone());
                        })
                    };

                    let on_enter = {
                        let on_preview = props.on_preview_change.clone();
                        let range = range.clone();
                        Callback::from(move |_: MouseEvent| {
                            let (s, e) = range.normalized();
                            if let (Some(s), Some(e)) = (s, e) {
                                on_preview.emit(Some((s, e)));
                            }
                        })
                    };

                    let on_leave = {
                        let on_preview = props.on_preview_change.clone();
                        Callback::from(move |_: MouseEvent| {
                            on_preview.emit(None);
                        })
                    };

                    let selected_class = if is_selected { "rdrStaticRangeSelected" } else { "" };

                    html! {
                        <button
                            class={classes!("rdrStaticRange", selected_class)}
                            onclick={on_click}
                            onmouseenter={on_enter}
                            onmouseleave={on_leave}
                            key={idx}
                        >
                            <span class="rdrStaticRangeLabel">{label}</span>
                        </button>
                    }
                })}
            </div>

            if let Some(ref footer) = props.footer_content {
                {footer.clone()}
            }
        </div>
    }
}
