# yew-date-range

[![CI](https://github.com/Black-Cockpit/yew-date-range/workflows/CI/badge.svg)](https://github.com/Black-Cockpit/yew-date-range/actions)
[![Crates.io](https://img.shields.io/crates/v/yew-date-range.svg)](https://crates.io/crates/yew-date-range)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A date range picker component library for [Yew](https://yew.rs/) applications, inspired by [react-date-range](https://github.com/hypeserver/react-date-range) and [PrimeReact Calendar](https://primereact.org/calendar/). Provides a full-featured calendar with range selection, single date picking, multiple date selection, time picking, and drill-down navigation -- all in idiomatic Rust and WebAssembly.

## Live Demos

- [**Default Example**](https://black-cockpit.github.io/yew-date-range/example/) -- CSS-based styling with EN/FR/ES locale toggle
- [**Tailwind Example**](https://black-cockpit.github.io/yew-date-range/example-tailwind/) -- Tailwind CSS with dark mode and EN/FR/ES locale toggle

## Overview

yew-date-range separates the headless core engine (`yew-date-range-core`) from the Yew UI layer (`yew-date-range`), letting you use the models and utilities standalone or with the provided Yew components.

- **Headless Core** -- Models, date helpers, range utilities with zero UI dependencies
- **Full Components** -- `DateRangePicker`, `DateRange`, `DatePicker`, `Calendar`, `TimePicker`
- **Drill-Down Navigation** -- Days, Months, and Years picker views
- **Internationalization** -- Built-in i18n via the browser `Intl.DateTimeFormat` API, with locale-driven labels for all UI strings
- **Auto CSS Injection** -- Default styles injected automatically, override with your own
- **Responsive** -- Mobile-friendly layout with touch targets and bottom-sheet popup on small screens
- **100% Safe Rust** -- No `unsafe` blocks, no `unwrap()` or `expect()` anywhere

## Features

| Category | Capabilities |
|----------|-------------|
| **Selection Modes** | Single date, date range, multiple dates |
| **Display Modes** | Inline (embedded), popup overlay with click/focus triggers |
| **Range Picker** | Two-click range, predefined sidebar (Today, This Week, etc.), hover preview |
| **Time Picker** | 12h/24h formats, hours/minutes/seconds, AM/PM toggle |
| **Constraints** | Min/max dates, disabled dates, disabled weekdays, disabled ranges, min/max span |
| **Navigation** | Month prev/next, month picker grid, year picker decade grid |
| **i18n** | `CalendarLocale` with all UI labels, `Intl.DateTimeFormat` for month/day names, `LocaleHelper::for_locale("fr")` |
| **Customization** | Custom colors, date format patterns, day content renderer, CSS overrides |
| **Accessibility** | ARIA labels on all interactive elements, keyboard Escape to close popup |
| **Responsive** | 768px breakpoint, touch-friendly day cells (44px), mobile bottom-sheet overlay |

## Installation

```toml
[dependencies]
yew-date-range = "0.1"
```

For the headless core only:

```toml
[dependencies]
yew-date-range-core = "0.1"
```

## Quick Start

```rust
use yew::prelude::*;
use yew_date_range::prelude::*;
use time::macros::date;

#[function_component(App)]
fn app() -> Html {
    let ranges = use_state(|| vec![
        RangeSelection::new("selection")
            .with_dates(Some(date!(2026 - 01 - 01)), Some(date!(2026 - 01 - 15)))
    ]);

    let on_change = {
        let ranges = ranges.clone();
        Callback::from(move |change: RangeChange| {
            ranges.set(vec![change.range]);
        })
    };

    html! {
        <DateRangePicker
            ranges={(*ranges).clone()}
            on_change={on_change}
            months={2}
            direction={CalendarDirection::Horizontal}
        />
    }
}
```

## Components

### DateRangePicker

Full picker with a predefined range sidebar and a multi-month calendar. The sidebar labels, month names, and day names all respond to the `locale` prop.

```rust
<DateRangePicker
    ranges={ranges}
    on_change={on_change}
    months={2}
    direction={CalendarDirection::Horizontal}
    show_selection_preview={true}
    locale={LocaleHelper::for_locale("fr")}
/>
```

### DatePicker

Versatile date picker supporting single, range, and multiple selection modes with inline or popup display. Button labels (Today, Clear) and input placeholders are locale-aware.

```rust
<DatePicker
    selection_mode={SelectionMode::Single}
    value={value}
    on_change={on_change}
    display_mode={DisplayMode::Popup}
    show_today_button={true}
    show_clear_button={true}
    show_icon={true}
    locale={LocaleHelper::for_locale("es")}
/>
```

### DateRange

Standalone calendar with range selection (no sidebar).

```rust
<DateRange
    ranges={ranges}
    on_change={on_change}
    months={1}
    show_week_numbers={true}
    locale={locale}
/>
```

### TimePicker

Time spinner supporting 12h/24h formats with configurable granularity. ARIA labels on all buttons are locale-aware.

```rust
<DatePicker
    selection_mode={SelectionMode::Single}
    value={value}
    on_change={on_change}
    show_time={true}
    hour_format={HourFormat::H12}
    time_granularity={TimeGranularity { show_hours: true, show_minutes: true, show_seconds: true }}
    locale={locale}
/>
```

## Internationalization (i18n)

Every visible string in the library is driven by `CalendarLocale`. Pass a `locale` prop to any component to translate the entire UI -- month names, day names, button labels, sidebar ranges, ARIA text, and input placeholders.

### Using the Browser Intl API

The recommended approach uses `LocaleHelper::for_locale()` which calls the browser's native `Intl.DateTimeFormat` API to resolve month and day names automatically. Pass a simple BCP-47 tag like `"en"`, `"fr"`, `"de"`, or `"es"`:

```rust
// French locale -- month/day names from browser Intl, Monday week start
let locale = LocaleHelper::for_locale("fr");

// Spanish locale
let locale = LocaleHelper::for_locale("es");

// German locale
let locale = LocaleHelper::for_locale("de");
```

### Using CalendarLocale::from_bcp47

For finer control, use `CalendarLocale::from_bcp47()` directly. This populates month and day names from the browser Intl API but leaves UI labels (buttons, ARIA) at their English defaults. Override any field manually:

```rust
let mut locale = CalendarLocale::from_bcp47("de");
locale.today_label = "Heute".into();
locale.clear_label = "Loeschen".into();
locale.today_range_label = "Heute".into();
```

### Custom Locale from Scratch

Override any field using struct update syntax:

```rust
let locale = CalendarLocale {
    today_label: "Custom Today".into(),
    clear_label: "Custom Clear".into(),
    start_date_placeholder: "From".into(),
    end_date_placeholder: "To".into(),
    ..CalendarLocale::default()
};
```

### Locale Keys Reference

All keys follow the `yew_date_range.*` namespace:

| Namespace | Keys |
|-----------|------|
| `yew_date_range.nav.*` | `prev_month_label`, `next_month_label`, `prev_year_label`, `next_year_label`, `prev_decade_label`, `next_decade_label`, `select_month_label`, `select_year_label` |
| `yew_date_range.display.*` | `start_date_placeholder`, `end_date_placeholder` |
| `yew_date_range.action.*` | `today_label`, `clear_label` |
| `yew_date_range.input.*` | `select_date_placeholder`, `select_range_placeholder`, `select_dates_placeholder` |
| `yew_date_range.time.*` | `increment_hour_label`, `decrement_hour_label`, `increment_minute_label`, `decrement_minute_label`, `increment_second_label`, `decrement_second_label`, `toggle_period_label` |
| `yew_date_range.range.*` | `today_range_label`, `yesterday_range_label`, `this_week_label`, `last_week_label`, `this_month_label`, `last_month_label` |
| `yew_date_range.week.*` | `week_number_header` |
| `yew_date_range.select.*` | `select_prefix` |

## Custom Styles

The library auto-injects default CSS via `StyleInjector` on first render. No manual stylesheet setup is needed.

### Overriding Styles

Override specific `.rdr*` classes in your own stylesheet. The auto-injected styles are inserted first, so your rules take precedence:

```css
/* Custom accent color */
.rdrStartEdge, .rdrEndEdge { background: #10b981; }
.rdrStaticRangeSelected { color: #10b981; }
.rdrDayTodayDot { background: #10b981; }
```

### Programmatic Injection

Use `StyleInjector::inject_custom_css()` to inject additional CSS at runtime without duplicates:

```rust
StyleInjector::inject_custom_css(
    ".rdrDayToday .rdrDayNumber span { color: #e74c3c; }",
    "my-custom-theme",
);
```

### Dark Mode

See [`example-tailwind/dark-overrides.css`](example-tailwind/dark-overrides.css) for a complete dark theme that overrides all `.rdr*` classes under `html.dark`.

### Key CSS Classes

| Class | Element |
|-------|---------|
| `.rdrCalendarWrapper` | Calendar container |
| `.rdrDateRangePickerWrapper` | Full picker with sidebar |
| `.rdrDefinedRangesWrapper` | Sidebar container |
| `.rdrMonth` | Single month grid |
| `.rdrDay` | Day cell button |
| `.rdrDayToday` | Today highlight |
| `.rdrDaySelected` | Selected day |
| `.rdrStartEdge` / `.rdrEndEdge` | Range edge highlights |
| `.rdrInRange` | Range body highlight |
| `.rdrOverlay` | Popup overlay |
| `.rdrTimePicker` | Time picker container |
| `.rdrDatePickerInput` | Popup input field |
| `.rdrMonthPickerGrid` | Month drill-down grid |
| `.rdrYearPickerGrid` | Year drill-down grid |

## Architecture

The library follows a two-crate design:

**`yew-date-range-core`** -- The headless engine with zero UI dependencies. Contains:
- **Models** -- `RangeSelection`, `SelectionValue`, `DayState`, `CalendarLocale`, `DateFormat`, `TimeSelection`, `StaticRange`, and 15+ other types (one per file)
- **Utilities** -- `DateHelper`, `RangeHelper`, `LocaleHelper` structs with static methods for date arithmetic, range validation, locale configuration, and Intl API integration

**`yew-date-range`** -- The Yew integration layer. Contains:
- **Components** -- `Calendar`, `DatePicker`, `DateRange`, `DateRangePicker`, `DefinedRange`, `Overlay`, `TimePicker`
- **Styles** -- `StyleInjector` for automatic CSS injection, responsive `date_range.css`
- **Prelude** -- All public types from both crates via `yew_date_range::prelude::*`

## Examples

Both example applications include a **locale toggle** (EN / FR / ES) that translates the entire page -- section titles, descriptions, picker labels, sidebar ranges, button text, and info displays.

| # | Use Case | Key Props |
|---|----------|-----------|
| 1 | DateRangePicker with sidebar | `months=2`, `direction=Horizontal`, `locale` |
| 2 | DateRange calendar-only | Custom color, `show_date_display=true`, `locale` |
| 3 | Single month vertical | `show_week_numbers=true`, `direction=Vertical`, `locale` |
| 4 | Inline single date | `disabled_weekdays=[Sunday]`, `locale` |
| 5 | Popup range picker | `date_format="dd/MM/yyyy"`, `show_icon=true`, `locale` |
| 6 | Popup multiple dates | `popup_trigger=Both`, `locale` |
| 7 | Date + time (24h) | `show_time=true`, `hour_format=H24`, `locale` |
| 8 | Constrained range | `min_date`, `max_date`, `min_span`, `max_span`, `disabled_ranges`, `locale` |
| 9 | Date + time (12h) | `hour_format=H12`, AM/PM toggle, `locale` |
| 10 | Disabled state | `disabled=true`, `read_only=true`, `locale` |

### Running Examples

```bash
# Default CSS example (with EN/FR/ES locale toggle)
cd example
trunk serve

# Tailwind CSS example (with dark mode and EN/FR/ES locale toggle)
cd example-tailwind
trunk serve
```

## Testing

```bash
# Core unit tests (125 tests)
cargo test --package yew-date-range-core --target x86_64-unknown-linux-gnu

# E2E browser tests (requires built examples)
cargo test --package e2e-tests
```

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) before submitting a pull request. The project enforces strict source code rules including one type per file, comprehensive documentation.

## License

Distributed under the [MIT](LICENSE) license.
