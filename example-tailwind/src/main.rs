use time::macros::date;
use time::Weekday;
use yew::prelude::*;
use yew_date_range::prelude::*;

/// Holds all translated page strings for the tailwind example app.
struct PageText {
    subtitle: &'static str,
    subtitle2: &'static str,
    s1_title: &'static str,
    s2_title: &'static str,
    s3_title: &'static str,
    s4_title: &'static str,
    s5_title: &'static str,
    s6_title: &'static str,
    s7_title: &'static str,
    s8_title: &'static str,
    s9_title: &'static str,
    s10_title: &'static str,
    selected: &'static str,
    read_only: &'static str,
    footer: &'static str,
    click_range: &'static str,
    select_multi: &'static str,
}

/// Returns page text for English.
fn text_en() -> PageText {
    PageText {
        subtitle: "Tailwind CSS example - ",
        subtitle2: "Interactive demos showcasing every feature with dark/light theme support.",
        s1_title: "1. DateRangePicker with Sidebar",
        s2_title: "2. DateRange Calendar Only",
        s3_title: "3. Single Month Vertical with Week Numbers",
        s4_title: "4. Inline Single Date (Sundays Disabled)",
        s5_title: "5. Popup Range Picker (dd/MM/yyyy)",
        s6_title: "6. Popup Multiple Date Selection",
        s7_title: "7. Date + Time (24h with Seconds)",
        s8_title: "8. Constrained Range (Min/Max + Span)",
        s9_title: "9. Date + Time (12h AM/PM)",
        s10_title: "10. Disabled State",
        selected: "Selected",
        read_only: "(read-only)",
        footer: "yew-date-range - Tailwind CSS Example - ",
        click_range: "Click to select range",
        select_multi: "Select multiple dates",
    }
}

/// Returns page text for French.
fn text_fr() -> PageText {
    PageText {
        subtitle: "Exemple Tailwind CSS - ",
        subtitle2: "Exemples interactifs de chaque fonctionnalite avec support des themes sombre et clair.",
        s1_title: "1. Selecteur de plage avec barre laterale",
        s2_title: "2. Calendrier de plage uniquement",
        s3_title: "3. Mois unique vertical avec numeros de semaine",
        s4_title: "4. Date unique en ligne (dimanches desactives)",
        s5_title: "5. Selecteur de plage en popup (jj/MM/aaaa)",
        s6_title: "6. Selection de dates multiples en popup",
        s7_title: "7. Date et heure (format 24h avec secondes)",
        s8_title: "8. Plage avec contraintes (Min/Max et etendue)",
        s9_title: "9. Date et heure (format 12h AM/PM)",
        s10_title: "10. Etat desactive",
        selected: "Selection",
        read_only: "(lecture seule)",
        footer: "yew-date-range - Exemple Tailwind CSS - ",
        click_range: "Cliquer pour selectionner une plage",
        select_multi: "Selectionner plusieurs dates",
    }
}

/// Returns page text for Spanish.
fn text_es() -> PageText {
    PageText {
        subtitle: "Ejemplo Tailwind CSS - ",
        subtitle2: "Ejemplos interactivos de cada funcionalidad con soporte de temas oscuro y claro.",
        s1_title: "1. Selector de rango con barra lateral",
        s2_title: "2. Calendario de rango unicamente",
        s3_title: "3. Mes unico vertical con numeros de semana",
        s4_title: "4. Fecha unica en linea (domingos desactivados)",
        s5_title: "5. Selector de rango en popup (dd/MM/aaaa)",
        s6_title: "6. Seleccion de fechas multiples en popup",
        s7_title: "7. Fecha y hora (formato 24h con segundos)",
        s8_title: "8. Rango con restricciones (Min/Max y extension)",
        s9_title: "9. Fecha y hora (formato 12h AM/PM)",
        s10_title: "10. Estado desactivado",
        selected: "Seleccion",
        read_only: "(solo lectura)",
        footer: "yew-date-range - Ejemplo Tailwind CSS - ",
        click_range: "Clic para seleccionar un rango",
        select_multi: "Seleccionar varias fechas",
    }
}

/// Formats an optional date as YYYY-MM-DD or a dash for None.
fn format_date_opt(d: Option<time::Date>) -> String {
    match d {
        Some(date) => {
            // Extract date components and zero-pad them.
            let (y, m, d) = (date.year(), date.month() as u8, date.day());
            format!("{y:04}-{m:02}-{d:02}")
        }
        None => "-".into(),
    }
}

/// Formats a SelectionValue for display in the info bar.
fn format_selection(value: &SelectionValue) -> String {
    let fmt = DateFormat::default();
    match value {
        SelectionValue::Single(Some(d)) => fmt.format(*d),
        SelectionValue::Single(None) => "-".into(),
        SelectionValue::Range { start, end } => {
            // Format each boundary or show dash.
            format!("{} -> {}", format_date_opt(*start), format_date_opt(*end))
        }
        SelectionValue::Multiple(dates) if !dates.is_empty() => {
            // Join all formatted dates with commas.
            dates.iter().map(|d| fmt.format(*d)).collect::<Vec<_>>().join(", ")
        }
        _ => "-".into(),
    }
}

#[function_component(App)]
fn app() -> Html {
    // Initialize the dark mode toggle state.
    let dark_mode = use_state(|| false);

    // Locale toggle state: 0 = English, 1 = French, 2 = Spanish.
    let locale_idx = use_state(|| 0u8);
    let locale = match *locale_idx {
        1 => LocaleHelper::for_locale("fr"),
        2 => LocaleHelper::for_locale("es"),
        _ => LocaleHelper::default_locale(),
    };
    let locale_label = match *locale_idx {
        1 => "FR",
        2 => "ES",
        _ => "EN",
    };
    let txt = match *locale_idx {
        1 => text_fr(),
        2 => text_es(),
        _ => text_en(),
    };
    let toggle_locale = {
        let locale_idx = locale_idx.clone();
        Callback::from(move |_: MouseEvent| {
            // Cycle through 0 -> 1 -> 2 -> 0.
            locale_idx.set((*locale_idx + 1) % 3);
        })
    };

    // Apply the dark/light class to the document root element.
    {
        let dark_mode = dark_mode.clone();
        use_effect_with(dark_mode.clone(), move |dm| {
            if let Some(window) = web_sys::window()
                && let Some(doc) = window.document()
                && let Some(el) = doc.document_element()
            {
                let cls = if **dm { "dark" } else { "light" };
                let _ = el.set_attribute("class", cls);
            }
            || ()
        });
    }

    // Create the dark mode toggle callback.
    let toggle_dark = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |_: MouseEvent| {
            dark_mode.set(!*dark_mode);
        })
    };

    // 1. DateRangePicker with sidebar.
    let ranges1 = use_state(|| {
        vec![RangeSelection::new("selection")
            .with_dates(Some(date!(2026 - 03 - 10)), Some(date!(2026 - 03 - 20)))
            .with_color("#3b82f6")]
    });
    let on_change1 = {
        let ranges1 = ranges1.clone();
        Callback::from(move |change: RangeChange| { ranges1.set(vec![change.range]); })
    };
    let r1 = ranges1.first().cloned().unwrap_or_default();

    // 2. DateRange calendar-only with green accent.
    let ranges2 = use_state(|| {
        vec![RangeSelection::new("selection").with_color("#10b981").with_dates(None, None)]
    });
    let on_change2 = {
        let ranges2 = ranges2.clone();
        Callback::from(move |change: RangeChange| { ranges2.set(vec![change.range]); })
    };
    let r2 = ranges2.first().cloned().unwrap_or_default();

    // 3. Single month vertical with week numbers.
    let ranges3 = use_state(|| {
        vec![RangeSelection::new("selection").with_color("#8b5cf6").with_dates(None, None)]
    });
    let on_change3 = {
        let ranges3 = ranges3.clone();
        Callback::from(move |change: RangeChange| { ranges3.set(vec![change.range]); })
    };
    let r3 = ranges3.first().cloned().unwrap_or_default();

    // 4. Inline single-date picker with disabled Sundays.
    let single_value = use_state(|| SelectionValue::Single(Some(date!(2026 - 03 - 15))));
    let on_single_change = {
        let single_value = single_value.clone();
        Callback::from(move |val: SelectionValue| { single_value.set(val); })
    };

    // 5. Popup range picker with dd/MM/yyyy format.
    let popup_range = use_state(|| SelectionValue::Range { start: None, end: None });
    let on_popup_range_change = {
        let popup_range = popup_range.clone();
        Callback::from(move |val: SelectionValue| { popup_range.set(val); })
    };

    // 6. Popup multiple-date picker.
    let multi_value = use_state(|| SelectionValue::Multiple(vec![
        date!(2026 - 03 - 05), date!(2026 - 03 - 12), date!(2026 - 03 - 22),
    ]));
    let on_multi_change = {
        let multi_value = multi_value.clone();
        Callback::from(move |val: SelectionValue| { multi_value.set(val); })
    };

    // 7. Inline date + 24h time picker.
    let time_date = use_state(|| SelectionValue::Single(Some(date!(2026 - 03 - 15))));
    let time_value = use_state(|| TimeSelection::new(14, 30, 0));
    let on_time_date = {
        let time_date = time_date.clone();
        Callback::from(move |val: SelectionValue| { time_date.set(val); })
    };
    let on_time = {
        let time_value = time_value.clone();
        Callback::from(move |t: TimeSelection| { time_value.set(t); })
    };

    // 8. Constrained range with min/max dates and span limits.
    let constrained = use_state(|| {
        vec![RangeSelection::new("selection").with_color("#f97316").with_dates(None, None)]
    });
    let on_constrained = {
        let constrained = constrained.clone();
        Callback::from(move |change: RangeChange| { constrained.set(vec![change.range]); })
    };
    let rc = constrained.first().cloned().unwrap_or_default();

    // 9. Inline date + 12h AM/PM time picker.
    let time12_date = use_state(|| SelectionValue::Single(Some(date!(2026 - 03 - 15))));
    let time12_value = use_state(|| TimeSelection::new(9, 15, 30));
    let on_time12_date = {
        let time12_date = time12_date.clone();
        Callback::from(move |val: SelectionValue| { time12_date.set(val); })
    };
    let on_time12 = {
        let time12_value = time12_value.clone();
        Callback::from(move |t: TimeSelection| { time12_value.set(t); })
    };

    // 10. Disabled / read-only state.
    let disabled_value = use_state(|| SelectionValue::Single(Some(date!(2026 - 03 - 15))));

    // Resolve the dark mode flag and labels.
    let is_dark = *dark_mode;
    let mode_label = if is_dark { "Dark" } else { "Light" };
    let toggle_icon = if is_dark { "*" } else { "o" };

    // Reusable Tailwind class helpers.
    let heading = |dark: bool| classes!(
        "text-lg", "font-semibold", "mb-2", "pb-2", "border-b",
        if dark { "border-slate-700" } else { "border-gray-200" }
    );
    let card = |dark: bool| classes!(
        "flex", "justify-center", "p-6", "rounded-xl",
        if dark { "bg-slate-800" } else { "bg-white shadow-sm" }
    );
    let info = |dark: bool| classes!(
        "mt-3", "px-4", "py-3", "rounded-lg", "text-sm", "text-center",
        if dark { "bg-slate-800 text-slate-400" } else { "bg-gray-100 text-gray-500" }
    );
    let desc = |dark: bool| classes!(
        "text-xs", "mb-4",
        if dark { "text-slate-500" } else { "text-gray-400" }
    );
    let accent = if is_dark { "text-blue-400" } else { "text-blue-600" };

    html! {
        <div class={classes!(
            "min-h-screen", "transition-colors", "duration-300",
            if is_dark { "bg-slate-900 text-slate-200" } else { "bg-gray-50 text-gray-900" }
        )}>
            <div class="max-w-5xl mx-auto px-6 py-10">
                // Header with dark mode toggle.
                <div class="flex items-center justify-between mb-10">
                    <div>
                        <h1 class="text-3xl font-bold">{"yew-date-range"}</h1>
                        <p class={classes!("text-sm", "mt-1", if is_dark { "text-slate-400" } else { "text-gray-500" })}>
                            {txt.subtitle}
                            <span class="font-medium">{mode_label}{" Mode"}</span>
                        </p>
                        <p class={classes!("text-xs", "mt-1", if is_dark { "text-slate-500" } else { "text-gray-400" })}>
                            {txt.subtitle2}
                        </p>
                    </div>
                    <div class="flex gap-2">
                        <button
                            onclick={toggle_locale}
                            class={classes!(
                                "px-4", "py-2", "rounded-lg", "border", "text-sm", "font-medium",
                                "transition-all", "duration-200", "cursor-pointer",
                                if is_dark { "bg-slate-800 border-slate-600 hover:bg-slate-700 text-slate-200" }
                                else { "bg-white border-gray-300 hover:bg-gray-100 text-gray-700" }
                            )}
                        >
                            {"Locale: "}{locale_label}
                        </button>
                        <button
                            onclick={toggle_dark}
                            class={classes!(
                                "flex", "items-center", "gap-2", "px-4", "py-2",
                                "rounded-lg", "border", "text-sm", "font-medium",
                                "transition-all", "duration-200", "cursor-pointer",
                                if is_dark { "bg-slate-800 border-slate-600 hover:bg-slate-700 text-slate-200" }
                                else { "bg-white border-gray-300 hover:bg-gray-100 text-gray-700" }
                            )}
                        >
                            <span class="text-lg">{toggle_icon}</span>
                            <span>{"Toggle "}{if is_dark {"Light"} else {"Dark"}}</span>
                        </button>
                    </div>
                </div>

                // 1. Full DateRangePicker with sidebar.
                <section class="mb-12">
                    <h2 class={heading(is_dark)}>{txt.s1_title}</h2>
                    <p class={desc(is_dark)}>{"months=2, direction=Horizontal, show_selection_preview=true"}</p>
                    <div class={card(is_dark)}>
                        <DateRangePicker
                            ranges={(*ranges1).clone()} on_change={on_change1}
                            months={2} direction={CalendarDirection::Horizontal}
                            show_selection_preview={true}
                            locale={locale.clone()}
                        />
                    </div>
                    <div class={info(is_dark)}>
                        {txt.selected}{": "}<strong class={accent}>{format_date_opt(r1.start_date)}{" -> "}{format_date_opt(r1.end_date)}</strong>
                    </div>
                </section>

                // 2. DateRange calendar-only with green accent.
                <section class="mb-12">
                    <h2 class={heading(is_dark)}>{txt.s2_title}</h2>
                    <p class={desc(is_dark)}>{"months=2, with_color(\"#10b981\"), show_date_display=true"}</p>
                    <div class={card(is_dark)}>
                        <DateRange
                            ranges={(*ranges2).clone()} on_change={on_change2}
                            months={2} direction={CalendarDirection::Horizontal}
                            show_selection_preview={true} show_date_display={true}
                            locale={locale.clone()}
                        />
                    </div>
                    <div class={info(is_dark)}>
                        {txt.selected}{": "}<strong class={accent}>{format_date_opt(r2.start_date)}{" -> "}{format_date_opt(r2.end_date)}</strong>
                    </div>
                </section>

                // 3. Single month vertical with week numbers.
                <section class="mb-12">
                    <h2 class={heading(is_dark)}>{txt.s3_title}</h2>
                    <p class={desc(is_dark)}>{"months=1, direction=Vertical, show_week_numbers=true"}</p>
                    <div class={card(is_dark)}>
                        <DateRange
                            ranges={(*ranges3).clone()} on_change={on_change3}
                            months={1} direction={CalendarDirection::Vertical}
                            show_week_numbers={true} show_selection_preview={true}
                            locale={locale.clone()}
                        />
                    </div>
                    <div class={info(is_dark)}>
                        {txt.selected}{": "}<strong class={accent}>{format_date_opt(r3.start_date)}{" -> "}{format_date_opt(r3.end_date)}</strong>
                    </div>
                </section>

                // 4. Inline single-date picker (Sundays disabled).
                <section class="mb-12">
                    <h2 class={heading(is_dark)}>{txt.s4_title}</h2>
                    <p class={desc(is_dark)}>{"selection_mode=Single, display_mode=Inline, disabled_weekdays=[Sunday]"}</p>
                    <div class={card(is_dark)}>
                        <DatePicker
                            selection_mode={SelectionMode::Single}
                            value={(*single_value).clone()} on_change={on_single_change}
                            display_mode={DisplayMode::Inline}
                            show_today_button={true} show_clear_button={true}
                            disabled_weekdays={vec![Weekday::Sunday]}
                            locale={locale.clone()}
                        />
                    </div>
                    <div class={info(is_dark)}>
                        {txt.selected}{": "}<strong class={accent}>{format_selection(&single_value)}</strong>
                    </div>
                </section>

                // 5. Popup range picker with dd/MM/yyyy format.
                <section class="mb-12">
                    <h2 class={heading(is_dark)}>{txt.s5_title}</h2>
                    <p class={desc(is_dark)}>{"display_mode=Popup, date_format=\"dd/MM/yyyy\", show_icon=true"}</p>
                    <div class={card(is_dark)}>
                        <DatePicker
                            selection_mode={SelectionMode::Range}
                            value={(*popup_range).clone()} on_change={on_popup_range_change}
                            display_mode={DisplayMode::Popup} popup_trigger={PopupTrigger::Click}
                            months={2} direction={CalendarDirection::Horizontal}
                            show_icon={true} show_today_button={true} show_clear_button={true}
                            date_format={DateFormat::new("dd/MM/yyyy")}
                            placeholder={txt.click_range}
                            locale={locale.clone()}
                        />
                    </div>
                    <div class={info(is_dark)}>
                        {txt.selected}{": "}<strong class={accent}>{format_selection(&popup_range)}</strong>
                    </div>
                </section>

                // 6. Popup multiple-date picker.
                <section class="mb-12">
                    <h2 class={heading(is_dark)}>{txt.s6_title}</h2>
                    <p class={desc(is_dark)}>{"selection_mode=Multiple, popup_trigger=Both, show_icon=true"}</p>
                    <div class={card(is_dark)}>
                        <DatePicker
                            selection_mode={SelectionMode::Multiple}
                            value={(*multi_value).clone()} on_change={on_multi_change}
                            display_mode={DisplayMode::Popup} popup_trigger={PopupTrigger::Both}
                            show_icon={true} show_clear_button={true}
                            placeholder={txt.select_multi}
                            locale={locale.clone()}
                        />
                    </div>
                    <div class={info(is_dark)}>
                        {txt.selected}{": "}<strong class={accent}>{format_selection(&multi_value)}</strong>
                    </div>
                </section>

                // 7. Inline date + 24h time picker with seconds.
                <section class="mb-12">
                    <h2 class={heading(is_dark)}>{txt.s7_title}</h2>
                    <p class={desc(is_dark)}>{"show_time=true, hour_format=H24, show_seconds=true"}</p>
                    <div class={card(is_dark)}>
                        <DatePicker
                            selection_mode={SelectionMode::Single}
                            value={(*time_date).clone()} on_change={on_time_date}
                            display_mode={DisplayMode::Inline}
                            show_time={true} time_value={Some(*time_value)} on_time_change={on_time}
                            hour_format={HourFormat::H24}
                            time_granularity={TimeGranularity { show_hours: true, show_minutes: true, show_seconds: true }}
                            show_today_button={true}
                            locale={locale.clone()}
                        />
                    </div>
                    <div class={info(is_dark)}>
                        {txt.selected}{": "}<strong class={accent}>{format_selection(&time_date)}{" @ "}{time_value.format_24h_short()}</strong>
                    </div>
                </section>

                // 8. Constrained range (min/max + span).
                <section class="mb-12">
                    <h2 class={heading(is_dark)}>{txt.s8_title}</h2>
                    <p class={desc(is_dark)}>{"min_date, max_date, min_span=2, max_span=10, disabled_ranges"}</p>
                    <div class={card(is_dark)}>
                        <DateRange
                            ranges={(*constrained).clone()} on_change={on_constrained}
                            months={1}
                            min_date={Some(date!(2026 - 03 - 01))} max_date={Some(date!(2026 - 03 - 31))}
                            min_span={Some(2)} max_span={Some(10)}
                            disabled_ranges={vec![(date!(2026 - 03 - 14), date!(2026 - 03 - 16))]}
                            show_selection_preview={true} show_date_display={true}
                            locale={locale.clone()}
                        />
                    </div>
                    <div class={info(is_dark)}>
                        {txt.selected}{": "}<strong class={accent}>{format_date_opt(rc.start_date)}{" -> "}{format_date_opt(rc.end_date)}</strong>
                    </div>
                </section>

                // 9. Inline date + 12h AM/PM time picker.
                <section class="mb-12">
                    <h2 class={heading(is_dark)}>{txt.s9_title}</h2>
                    <p class={desc(is_dark)}>{"show_time=true, hour_format=H12, show_seconds=true"}</p>
                    <div class={card(is_dark)}>
                        <DatePicker
                            selection_mode={SelectionMode::Single}
                            value={(*time12_date).clone()} on_change={on_time12_date}
                            display_mode={DisplayMode::Inline}
                            show_time={true} time_value={Some(*time12_value)} on_time_change={on_time12}
                            hour_format={HourFormat::H12}
                            time_granularity={TimeGranularity { show_hours: true, show_minutes: true, show_seconds: true }}
                            locale={locale.clone()}
                        />
                    </div>
                    <div class={info(is_dark)}>
                        {txt.selected}{": "}<strong class={accent}>{format_selection(&time12_date)}{" @ "}{time12_value.format_12h()}</strong>
                    </div>
                </section>

                // 10. Disabled / read-only state.
                <section class="mb-12">
                    <h2 class={heading(is_dark)}>{txt.s10_title}</h2>
                    <p class={desc(is_dark)}>{"disabled=true, read_only=true"}</p>
                    <div class={card(is_dark)}>
                        <DatePicker
                            selection_mode={SelectionMode::Single}
                            value={(*disabled_value).clone()} on_change={Callback::noop()}
                            display_mode={DisplayMode::Inline}
                            disabled={true} read_only={true}
                            locale={locale.clone()}
                        />
                    </div>
                    <div class={info(is_dark)}>
                        {txt.selected}{": "}<strong class={accent}>{format_selection(&disabled_value)}{" "}{txt.read_only}</strong>
                    </div>
                </section>

                // Footer.
                <div class={classes!(
                    "text-center", "py-6", "text-xs",
                    if is_dark { "text-slate-500" } else { "text-gray-400" }
                )}>
                    {txt.footer}
                    <a href="https://github.com/Black-Cockpit/yew-date-range" class="underline">{"GitHub"}</a>
                </div>
            </div>
        </div>
    }
}

fn main() {
    // Render the Yew application.
    yew::Renderer::<App>::new().render();
}
