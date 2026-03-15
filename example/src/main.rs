use time::macros::date;
use time::Weekday;
use yew::prelude::*;
use yew_date_range::prelude::*;

/// Holds all translated page strings for the example app.
struct PageText {
    subtitle: &'static str,
    subtitle2: &'static str,
    toggle_label: &'static str,
    s1_title: &'static str,
    s1_desc: &'static str,
    s2_title: &'static str,
    s2_desc: &'static str,
    s3_title: &'static str,
    s3_desc: &'static str,
    s4_title: &'static str,
    s4_desc: &'static str,
    s5_title: &'static str,
    s5_desc: &'static str,
    s6_title: &'static str,
    s6_desc: &'static str,
    s7_title: &'static str,
    s7_desc: &'static str,
    s8_title: &'static str,
    s8_desc: &'static str,
    s9_title: &'static str,
    s9_desc: &'static str,
    s10_title: &'static str,
    s10_desc: &'static str,
    selected: &'static str,
    no_range: &'static str,
    no_date: &'static str,
    no_dates: &'static str,
    dates_label: &'static str,
    read_only: &'static str,
    none_label: &'static str,
    footer: &'static str,
    click_range: &'static str,
    select_multi: &'static str,
}

/// Returns page text for English.
fn text_en() -> PageText {
    PageText {
        subtitle: "A date range picker component library for Yew, inspired by react-date-range and PrimeReact.",
        subtitle2: "Below are interactive examples showcasing every feature. Each section explains the props used.",
        toggle_label: "click to toggle",
        s1_title: "1. DateRangePicker with Sidebar",
        s1_desc: "Combines a predefined range sidebar (Today, This Week, etc.) with a two-month calendar.",
        s2_title: "2. DateRange Calendar Only",
        s2_desc: "A standalone range calendar without the sidebar. Custom green accent color.",
        s3_title: "3. Single Month Vertical with Week Numbers",
        s3_desc: "A compact single-month vertical layout with ISO week numbers enabled. Purple accent.",
        s4_title: "4. Inline Single Date (Sundays Disabled)",
        s4_desc: "An inline single-date picker with Today/Clear buttons. All Sundays are disabled.",
        s5_title: "5. Popup Range Picker (dd/MM/yyyy)",
        s5_desc: "A popup-triggered range picker with a calendar icon, custom date format, and action buttons.",
        s6_title: "6. Popup Multiple Date Selection",
        s6_desc: "Select multiple individual dates with a popup that opens on both click and focus.",
        s7_title: "7. Date + Time (24h with Seconds)",
        s7_desc: "Inline date picker with a time spinner in 24-hour format, showing hours, minutes, and seconds.",
        s8_title: "8. Constrained Range (Min/Max + Span)",
        s8_desc: "Range selection limited to March 2026, with a maximum span of 10 days and minimum span of 2 days.",
        s9_title: "9. Date + Time (12h AM/PM)",
        s9_desc: "Inline date picker with a time spinner in 12-hour AM/PM format, including seconds.",
        s10_title: "10. Disabled State",
        s10_desc: "A fully disabled date picker that shows a read-only value. Useful for display-only contexts.",
        selected: "Selected",
        no_range: "No range selected",
        no_date: "No date selected",
        no_dates: "No dates selected",
        dates_label: "dates",
        read_only: "(read-only)",
        none_label: "none",
        footer: "yew-date-range - MIT License - ",
        click_range: "Click to select range",
        select_multi: "Select multiple dates",
    }
}

/// Returns page text for French.
fn text_fr() -> PageText {
    PageText {
        subtitle: "Composants de calendrier et de plage de dates pour Yew, inspires de react-date-range et PrimeReact.",
        subtitle2: "Exemples interactifs de chaque fonctionnalite. Chaque section decrit les proprietes utilisees.",
        toggle_label: "cliquer pour changer",
        s1_title: "1. Selecteur de plage avec barre laterale",
        s1_desc: "Barre laterale de plages predefinies (Aujourd'hui, Cette semaine, etc.) avec calendrier sur deux mois.",
        s2_title: "2. Calendrier de plage uniquement",
        s2_desc: "Calendrier de plage autonome sans barre laterale. Accent vert personnalise.",
        s3_title: "3. Mois unique vertical avec numeros de semaine",
        s3_desc: "Calendrier compact a un mois en disposition verticale avec numeros de semaine ISO. Accent violet.",
        s4_title: "4. Date unique en ligne (dimanches desactives)",
        s4_desc: "Selecteur de date en ligne avec boutons Aujourd'hui et Effacer. Tous les dimanches sont desactives.",
        s5_title: "5. Selecteur de plage en popup (jj/MM/aaaa)",
        s5_desc: "Selecteur de plage en popup avec icone de calendrier, format de date personnalise et boutons d'action.",
        s6_title: "6. Selection de dates multiples en popup",
        s6_desc: "Selectionnez plusieurs dates individuelles via un popup qui s'ouvre au clic ou au focus.",
        s7_title: "7. Date et heure (format 24h avec secondes)",
        s7_desc: "Selecteur de date en ligne avec compteur horaire au format 24 heures, minutes et secondes.",
        s8_title: "8. Plage avec contraintes (Min/Max et etendue)",
        s8_desc: "Plage limitee a mars 2026, etendue maximale de 10 jours et minimale de 2 jours. Plages desactivees.",
        s9_title: "9. Date et heure (format 12h AM/PM)",
        s9_desc: "Selecteur de date en ligne avec compteur horaire au format 12 heures AM/PM, secondes incluses.",
        s10_title: "10. Etat desactive",
        s10_desc: "Selecteur de date entierement desactive et en lecture seule. Utile pour les contextes d'affichage.",
        selected: "Selection",
        no_range: "Aucune plage selectionnee",
        no_date: "Aucune date selectionnee",
        no_dates: "Aucune date selectionnee",
        dates_label: "dates",
        read_only: "(lecture seule)",
        none_label: "aucune",
        footer: "yew-date-range - Licence MIT - ",
        click_range: "Cliquer pour selectionner une plage",
        select_multi: "Selectionner plusieurs dates",
    }
}

/// Returns page text for Spanish.
fn text_es() -> PageText {
    PageText {
        subtitle: "Componentes de calendario y rango de fechas para Yew, inspirados en react-date-range y PrimeReact.",
        subtitle2: "Ejemplos interactivos de cada funcionalidad. Cada seccion describe las propiedades utilizadas.",
        toggle_label: "clic para cambiar",
        s1_title: "1. Selector de rango con barra lateral",
        s1_desc: "Barra lateral de rangos predefinidos (Hoy, Esta semana, etc.) con calendario de dos meses.",
        s2_title: "2. Calendario de rango unicamente",
        s2_desc: "Calendario de rango independiente sin barra lateral. Acento verde personalizado.",
        s3_title: "3. Mes unico vertical con numeros de semana",
        s3_desc: "Calendario compacto de un mes en disposicion vertical con numeros de semana ISO. Acento morado.",
        s4_title: "4. Fecha unica en linea (domingos desactivados)",
        s4_desc: "Selector de fecha en linea con botones Hoy y Borrar. Todos los domingos estan desactivados.",
        s5_title: "5. Selector de rango en popup (dd/MM/aaaa)",
        s5_desc: "Selector de rango en popup con icono de calendario, formato de fecha personalizado y botones de accion.",
        s6_title: "6. Seleccion de fechas multiples en popup",
        s6_desc: "Seleccione varias fechas individuales mediante un popup que se abre al hacer clic o al enfocar.",
        s7_title: "7. Fecha y hora (formato 24h con segundos)",
        s7_desc: "Selector de fecha en linea con contador de tiempo en formato 24 horas, minutos y segundos.",
        s8_title: "8. Rango con restricciones (Min/Max y extension)",
        s8_desc: "Rango limitado a marzo 2026, extension maxima de 10 dias y minima de 2 dias. Rangos desactivados.",
        s9_title: "9. Fecha y hora (formato 12h AM/PM)",
        s9_desc: "Selector de fecha en linea con contador de tiempo en formato 12 horas AM/PM, segundos incluidos.",
        s10_title: "10. Estado desactivado",
        s10_desc: "Selector de fecha completamente desactivado y en modo solo lectura. Util para contextos de visualizacion.",
        selected: "Seleccion",
        no_range: "Ningun rango seleccionado",
        no_date: "Ninguna fecha seleccionada",
        no_dates: "Ninguna fecha seleccionada",
        dates_label: "fechas",
        read_only: "(solo lectura)",
        none_label: "ninguno",
        footer: "yew-date-range - Licencia MIT - ",
        click_range: "Clic para seleccionar un rango",
        select_multi: "Seleccionar varias fechas",
    }
}

#[function_component(App)]
fn app() -> Html {
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
    let t = match *locale_idx {
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

    // Configure the full DateRangePicker with predefined sidebar ranges.
    let ranges1 = use_state(|| {
        vec![RangeSelection::new("selection").with_dates(
            Some(date!(2026 - 03 - 10)),
            Some(date!(2026 - 03 - 20)),
        )]
    });
    let on_change1 = {
        let ranges1 = ranges1.clone();
        Callback::from(move |change: RangeChange| {
            ranges1.set(vec![change.range]);
        })
    };
    let info1 = format_range_info((*ranges1).first(), &t);

    // Configure the DateRange calendar-only component with a green accent.
    let ranges2 = use_state(|| {
        vec![RangeSelection::new("selection")
            .with_color("#00b894")
            .with_dates(None, None)]
    });
    let on_change2 = {
        let ranges2 = ranges2.clone();
        Callback::from(move |change: RangeChange| {
            ranges2.set(vec![change.range]);
        })
    };
    let info2 = format_range_info((*ranges2).first(), &t);

    // Configure a single month vertical layout with week numbers.
    let ranges3 = use_state(|| {
        vec![RangeSelection::new("selection")
            .with_color("#6c5ce7")
            .with_dates(None, None)]
    });
    let on_change3 = {
        let ranges3 = ranges3.clone();
        Callback::from(move |change: RangeChange| {
            ranges3.set(vec![change.range]);
        })
    };
    let info3 = format_range_info((*ranges3).first(), &t);

    // Configure the inline single-date DatePicker with disabled Sundays.
    let single_value = use_state(|| SelectionValue::Single(Some(date!(2026 - 03 - 15))));
    let on_single_change = {
        let single_value = single_value.clone();
        Callback::from(move |val: SelectionValue| {
            single_value.set(val);
        })
    };
    let single_info = format_selection_info(&single_value, &t);

    // Configure the popup range-mode DatePicker with dd/MM/yyyy format.
    let popup_range = use_state(|| SelectionValue::Range { start: None, end: None });
    let on_popup_range_change = {
        let popup_range = popup_range.clone();
        Callback::from(move |val: SelectionValue| {
            popup_range.set(val);
        })
    };
    let popup_range_info = format_selection_info(&popup_range, &t);

    // Configure the popup multiple-date DatePicker.
    let multi_value = use_state(|| SelectionValue::Multiple(vec![
        date!(2026 - 03 - 05),
        date!(2026 - 03 - 12),
        date!(2026 - 03 - 19),
    ]));
    let on_multi_change = {
        let multi_value = multi_value.clone();
        Callback::from(move |val: SelectionValue| {
            multi_value.set(val);
        })
    };
    let multi_info = format_selection_info(&multi_value, &t);

    // Configure the inline DatePicker with 24-hour time selection.
    let time_date = use_state(|| SelectionValue::Single(Some(date!(2026 - 03 - 15))));
    let time_value = use_state(|| TimeSelection::new(14, 30, 0));
    let on_time_date_change = {
        let time_date = time_date.clone();
        Callback::from(move |val: SelectionValue| {
            time_date.set(val);
        })
    };
    let on_time_change = {
        let time_value = time_value.clone();
        Callback::from(move |t: TimeSelection| {
            time_value.set(t);
        })
    };
    let time_info = {
        let date_str = format_selection_info(&time_date, &t);
        let tv = *time_value;
        format!("{} @ {}", date_str, tv.format_24h_short())
    };

    // Configure the DateRange with min/max date constraints and span limits.
    let constrained_ranges = use_state(|| {
        vec![RangeSelection::new("selection")
            .with_color("#e17055")
            .with_dates(None, None)]
    });
    let on_constrained_change = {
        let constrained_ranges = constrained_ranges.clone();
        Callback::from(move |change: RangeChange| {
            constrained_ranges.set(vec![change.range]);
        })
    };
    let constrained_info = format_range_info((*constrained_ranges).first(), &t);

    // Configure the inline DatePicker with 12-hour time and seconds.
    let time12_date = use_state(|| SelectionValue::Single(Some(date!(2026 - 03 - 15))));
    let time12_value = use_state(|| TimeSelection::new(9, 15, 30));
    let on_time12_date_change = {
        let time12_date = time12_date.clone();
        Callback::from(move |val: SelectionValue| {
            time12_date.set(val);
        })
    };
    let on_time12_change = {
        let time12_value = time12_value.clone();
        Callback::from(move |t: TimeSelection| {
            time12_value.set(t);
        })
    };
    let time12_info = {
        let date_str = format_selection_info(&time12_date, &t);
        let tv = *time12_value;
        format!("{} @ {}", date_str, tv.format_12h())
    };

    // Configure a disabled DatePicker for the read-only demo.
    let disabled_value = use_state(|| SelectionValue::Single(Some(date!(2026 - 03 - 15))));
    let disabled_info = format_selection_info(&disabled_value, &t);

    html! {
        <>
            <header class="page-header">
                <h1>{"yew-date-range"}</h1>
                <p class="subtitle">{t.subtitle}</p>
                <p class="subtitle">{t.subtitle2}</p>
                <button class="locale-toggle" onclick={toggle_locale}>
                    {"Locale: "}{locale_label}{" ("}{t.toggle_label}{")"}
                </button>
            </header>

            <main class="examples">
                // Full DateRangePicker with sidebar.
                <section class="example-section">
                    <h2>{t.s1_title}</h2>
                    <p class="example-desc">
                        {t.s1_desc}{" "}
                        {"Props: "}
                        <code>{"months=2, direction=Horizontal, show_selection_preview=true"}</code>
                    </p>
                    <div class="example-container">
                        <DateRangePicker
                            ranges={(*ranges1).clone()}
                            on_change={on_change1}
                            show_selection_preview={true}
                            move_range_on_first_selection={false}
                            months={2}
                            direction={CalendarDirection::Horizontal}
                            locale={locale.clone()}
                        />
                    </div>
                    <div class="selection-info">{info1}</div>
                </section>

                // DateRange calendar-only with green accent.
                <section class="example-section">
                    <h2>{t.s2_title}</h2>
                    <p class="example-desc">
                        {t.s2_desc}{" "}
                        {"Props: "}
                        <code>{"months=2, with_color(\"#00b894\"), show_date_display=true"}</code>
                    </p>
                    <div class="example-container">
                        <DateRange
                            ranges={(*ranges2).clone()}
                            on_change={on_change2}
                            months={2}
                            direction={CalendarDirection::Horizontal}
                            show_selection_preview={true}
                            show_date_display={true}
                            locale={locale.clone()}
                        />
                    </div>
                    <div class="selection-info">{info2}</div>
                </section>

                // Single month vertical with week numbers.
                <section class="example-section">
                    <h2>{t.s3_title}</h2>
                    <p class="example-desc">
                        {t.s3_desc}{" "}
                        {"Props: "}
                        <code>{"months=1, direction=Vertical, show_week_numbers=true"}</code>
                    </p>
                    <div class="example-container">
                        <DateRange
                            ranges={(*ranges3).clone()}
                            on_change={on_change3}
                            months={1}
                            direction={CalendarDirection::Vertical}
                            show_week_numbers={true}
                            show_selection_preview={true}
                            locale={locale.clone()}
                        />
                    </div>
                    <div class="selection-info">{info3}</div>
                </section>

                // Inline single-date picker with disabled weekdays.
                <section class="example-section">
                    <h2>{t.s4_title}</h2>
                    <p class="example-desc">
                        {t.s4_desc}{" "}
                        {"Props: "}
                        <code>{"selection_mode=Single, display_mode=Inline, disabled_weekdays=[Sunday]"}</code>
                    </p>
                    <div class="example-container">
                        <DatePicker
                            selection_mode={SelectionMode::Single}
                            value={(*single_value).clone()}
                            on_change={on_single_change}
                            display_mode={DisplayMode::Inline}
                            show_today_button={true}
                            show_clear_button={true}
                            disabled_weekdays={vec![Weekday::Sunday]}
                            locale={locale.clone()}
                        />
                    </div>
                    <div class="selection-info">{single_info}</div>
                </section>

                // Popup range picker with custom date format.
                <section class="example-section">
                    <h2>{t.s5_title}</h2>
                    <p class="example-desc">
                        {t.s5_desc}{" "}
                        {"Props: "}
                        <code>{"display_mode=Popup, popup_trigger=Click, date_format=\"dd/MM/yyyy\", show_icon=true"}</code>
                    </p>
                    <div class="example-container">
                        <DatePicker
                            selection_mode={SelectionMode::Range}
                            value={(*popup_range).clone()}
                            on_change={on_popup_range_change}
                            display_mode={DisplayMode::Popup}
                            popup_trigger={PopupTrigger::Click}
                            months={2}
                            direction={CalendarDirection::Horizontal}
                            show_icon={true}
                            show_today_button={true}
                            show_clear_button={true}
                            date_format={DateFormat::new("dd/MM/yyyy")}
                            placeholder={t.click_range}
                            locale={locale.clone()}
                        />
                    </div>
                    <div class="selection-info">{popup_range_info}</div>
                </section>

                // Popup multiple-date picker.
                <section class="example-section">
                    <h2>{t.s6_title}</h2>
                    <p class="example-desc">
                        {t.s6_desc}{" "}
                        {"Props: "}
                        <code>{"selection_mode=Multiple, popup_trigger=Both, show_icon=true"}</code>
                    </p>
                    <div class="example-container">
                        <DatePicker
                            selection_mode={SelectionMode::Multiple}
                            value={(*multi_value).clone()}
                            on_change={on_multi_change}
                            display_mode={DisplayMode::Popup}
                            popup_trigger={PopupTrigger::Both}
                            show_icon={true}
                            show_clear_button={true}
                            placeholder={t.select_multi}
                            locale={locale.clone()}
                        />
                    </div>
                    <div class="selection-info">{multi_info}</div>
                </section>

                // Inline date + 24h time picker.
                <section class="example-section">
                    <h2>{t.s7_title}</h2>
                    <p class="example-desc">
                        {t.s7_desc}{" "}
                        {"Props: "}
                        <code>{"show_time=true, hour_format=H24, show_seconds=true"}</code>
                    </p>
                    <div class="example-container">
                        <DatePicker
                            selection_mode={SelectionMode::Single}
                            value={(*time_date).clone()}
                            on_change={on_time_date_change}
                            display_mode={DisplayMode::Inline}
                            show_time={true}
                            time_value={Some(*time_value)}
                            on_time_change={on_time_change}
                            hour_format={HourFormat::H24}
                            time_granularity={TimeGranularity { show_hours: true, show_minutes: true, show_seconds: true }}
                            show_today_button={true}
                            locale={locale.clone()}
                        />
                    </div>
                    <div class="selection-info">{time_info}</div>
                </section>

                // Constrained range with min/max dates and span limits.
                <section class="example-section">
                    <h2>{t.s8_title}</h2>
                    <p class="example-desc">
                        {t.s8_desc}{" "}
                        {"Props: "}
                        <code>{"min_date, max_date, min_span=2, max_span=10, disabled_ranges"}</code>
                    </p>
                    <div class="example-container">
                        <DateRange
                            ranges={(*constrained_ranges).clone()}
                            on_change={on_constrained_change}
                            months={1}
                            min_date={Some(date!(2026 - 03 - 01))}
                            max_date={Some(date!(2026 - 03 - 31))}
                            min_span={Some(2)}
                            max_span={Some(10)}
                            disabled_ranges={vec![(date!(2026 - 03 - 14), date!(2026 - 03 - 16))]}
                            show_selection_preview={true}
                            show_date_display={true}
                            locale={locale.clone()}
                        />
                    </div>
                    <div class="selection-info">{constrained_info}</div>
                </section>

                // Inline date + 12-hour AM/PM time picker.
                <section class="example-section">
                    <h2>{t.s9_title}</h2>
                    <p class="example-desc">
                        {t.s9_desc}{" "}
                        {"Props: "}
                        <code>{"show_time=true, hour_format=H12, show_seconds=true"}</code>
                    </p>
                    <div class="example-container">
                        <DatePicker
                            selection_mode={SelectionMode::Single}
                            value={(*time12_date).clone()}
                            on_change={on_time12_date_change}
                            display_mode={DisplayMode::Inline}
                            show_time={true}
                            time_value={Some(*time12_value)}
                            on_time_change={on_time12_change}
                            hour_format={HourFormat::H12}
                            time_granularity={TimeGranularity { show_hours: true, show_minutes: true, show_seconds: true }}
                            locale={locale.clone()}
                        />
                    </div>
                    <div class="selection-info">{time12_info}</div>
                </section>

                // Disabled / read-only state.
                <section class="example-section">
                    <h2>{t.s10_title}</h2>
                    <p class="example-desc">
                        {t.s10_desc}{" "}
                        {"Props: "}
                        <code>{"disabled=true, read_only=true"}</code>
                    </p>
                    <div class="example-container">
                        <DatePicker
                            selection_mode={SelectionMode::Single}
                            value={(*disabled_value).clone()}
                            on_change={Callback::noop()}
                            display_mode={DisplayMode::Inline}
                            disabled={true}
                            read_only={true}
                            locale={locale.clone()}
                        />
                    </div>
                    <div class="selection-info">{disabled_info}{" "}{t.read_only}</div>
                </section>
            </main>

            <footer class="page-footer">
                <p>{t.footer}
                    <a href="https://github.com/Black-Cockpit/yew-date-range">{"GitHub"}</a>
                </p>
            </footer>
        </>
    }
}

fn format_range_info(range: Option<&RangeSelection>, t: &PageText) -> String {
    match range {
        Some(r) => {
            // Format each boundary, falling back to translated "none".
            let start = r
                .start_date
                .map(DateHelper::format_date)
                .unwrap_or_else(|| t.none_label.into());
            let end = r
                .end_date
                .map(DateHelper::format_date)
                .unwrap_or_else(|| t.none_label.into());
            format!("{}: {} -> {}", t.selected, start, end)
        }
        None => t.no_range.into(),
    }
}

fn format_selection_info(value: &SelectionValue, t: &PageText) -> String {
    // Use the default date format for display.
    let fmt = DateFormat::default();
    match value {
        SelectionValue::Single(Some(d)) => format!("{}: {}", t.selected, fmt.format(*d)),
        SelectionValue::Single(None) => t.no_date.into(),
        SelectionValue::Range { start, end } => fmt.format_range(*start, *end),
        SelectionValue::Multiple(dates) => {
            if dates.is_empty() {
                t.no_dates.into()
            } else {
                format!("{} {} {}: {}", t.selected, dates.len(), t.dates_label, fmt.format_multiple(dates, ", "))
            }
        }
    }
}

fn main() {
    // Render the Yew application.
    yew::Renderer::<App>::new().render();
}
