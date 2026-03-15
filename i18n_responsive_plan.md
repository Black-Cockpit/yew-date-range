# Plan: Multi-Language (i18n) Support & Responsive Design

Comprehensive plan for adding internationalization and responsive improvements to `yew-date-range`.

---

## Part A: Internationalization (i18n)

### A.1 Design Approach

**Strategy: Locale-driven labels passed externally, defaulting to English, with Intl auto-detection via `js-sys`.**

The plan combines two approaches:

1. **Label-bag pattern** (similar to PrimeReact's `locale` prop) — all translatable strings live in `CalendarLocale`. Components read from the locale with zero hardcoded English.
2. **`Intl.DateTimeFormat` via `js-sys`/`wasm-bindgen`** — use the browser's native `Intl` API to auto-resolve month names and day names from a BCP-47 locale tag. This gives automatic, accurate translations for calendar data without bundling translation tables.

**Key naming convention:** All locale keys use the `yew_date_range.` namespace prefix to avoid collisions when consumers aggregate multiple component locales.

Steps:
1. Extend `CalendarLocale` in the core crate with all translatable strings (prefixed keys).
2. All components read labels from the locale — zero hardcoded English strings in rendering code.
3. Provide a `CalendarLocale::from_bcp47()` constructor that uses `js_sys::Intl::DateTimeFormat` to auto-populate month/day names.
4. Provide `LocaleHelper` factory methods for common locales (at least English and French).
5. Consumers can pass a fully custom `CalendarLocale` for any language not pre-built.
6. Examples demonstrate at least two languages (English + French).
7. The docs page documents i18n usage and custom styling.

### A.2 Phase 1: Extend `CalendarLocale` with All UI Labels

**File:** `yew-date-range-core/src/models/calendar_locale.rs`

Add the following fields to `CalendarLocale`, all defaulting to English:

All keys use the `yew_date_range.` namespace convention in their semantic meaning, though the Rust field names omit the prefix for ergonomics:

```rust
pub struct CalendarLocale {
    // --- Existing fields (yew_date_range.month_names, etc.) ---
    pub month_names: Vec<String>,
    pub short_month_names: Vec<String>,
    pub day_names: Vec<String>,
    pub short_day_names: Vec<String>,
    pub start_of_week: Weekday,
    pub date_format: String,

    // --- yew_date_range.nav.* ---
    pub prev_month_label: String,       // "Previous Month"
    pub next_month_label: String,       // "Next Month"
    pub prev_year_label: String,        // "Previous Year"
    pub next_year_label: String,        // "Next Year"
    pub prev_decade_label: String,      // "Previous Decade"
    pub next_decade_label: String,      // "Next Decade"
    pub select_month_label: String,     // "Select month"
    pub select_year_label: String,      // "Select year"

    // --- yew_date_range.display.* ---
    pub start_date_placeholder: String, // "Start Date"
    pub end_date_placeholder: String,   // "End Date"

    // --- yew_date_range.action.* ---
    pub today_label: String,            // "Today"
    pub clear_label: String,            // "Clear"

    // --- yew_date_range.input.* ---
    pub select_date_placeholder: String,   // "Select date"
    pub select_range_placeholder: String,  // "Select range"
    pub select_dates_placeholder: String,  // "Select dates"

    // --- yew_date_range.time.* ---
    pub increment_hour_label: String,   // "Increment hour"
    pub decrement_hour_label: String,   // "Decrement hour"
    pub increment_minute_label: String, // "Increment minute"
    pub decrement_minute_label: String, // "Decrement minute"
    pub increment_second_label: String, // "Increment second"
    pub decrement_second_label: String, // "Decrement second"
    pub toggle_period_label: String,    // "Toggle AM/PM"

    // --- yew_date_range.week.* ---
    pub week_number_header: String,     // "W"

    // --- yew_date_range.range.* ---
    pub today_range_label: String,      // "Today"
    pub yesterday_range_label: String,  // "Yesterday"
    pub this_week_label: String,        // "This Week"
    pub last_week_label: String,        // "Last Week"
    pub this_month_label: String,       // "This Month"
    pub last_month_label: String,       // "Last Month"

    // --- yew_date_range.select.* ---
    pub select_prefix: String,          // "Select" (used as "{select_prefix} Jan")
}
```

All new fields default to their current English hardcoded values in the `Default` impl. **This is fully backward-compatible** — existing consumers who don't pass a locale get the same English UI they have today.

### A.3 Phase 2: Wire Locale Through All Components

For each component, replace every hardcoded English string with a read from the `CalendarLocale`:

| Component | Current hardcoded | Replacement |
|---|---|---|
| `calendar.rs` | `"Start Date"`, `"End Date"` | `locale.start_date_placeholder`, `locale.end_date_placeholder` |
| `calendar.rs` | `"Select month"`, `"Select year"` | `locale.select_month_label`, `locale.select_year_label` |
| `calendar.rs` | `"Previous Month"`, etc. | `locale.prev_month_label`, etc. |
| `calendar_renderer.rs` | `"W"` | `locale.week_number_header` |
| `calendar_renderer.rs` | `"Select {month}"` | `format!("{} {}", locale.select_prefix, label)` |
| `calendar_renderer.rs` | `"Select {year}"` | `format!("{} {}", locale.select_prefix, year)` |
| `date_picker.rs` | `"Today"`, `"Clear"` | `locale.today_label`, `locale.clear_label` |
| `date_picker.rs` | `"Select date"`, `"Select range"`, `"Select dates"` | `locale.select_date_placeholder`, etc. |
| `time_picker.rs` | `"Increment hour"`, etc. | `locale.increment_hour_label`, etc. |

**Also pass the locale down** from `DatePicker` → `Calendar` → `CalendarRenderer` and from `DateRangePicker` → `DefinedRange`.

The `CalendarRenderer::render_month_picker_grid` currently has a hardcoded `month_short_names` array. Replace it with `locale.short_month_names`.

### A.4 Phase 3: Localize Static Ranges

**File:** `yew-date-range-core/src/utils/range_helper.rs`

Change `default_static_ranges()` to accept a `&CalendarLocale` parameter:

```rust
pub fn default_static_ranges(locale: &CalendarLocale) -> Vec<StaticRange> {
    vec![
        StaticRange::new(&locale.today_range_label, || { ... }),
        StaticRange::new(&locale.yesterday_range_label, || { ... }),
        StaticRange::new(&locale.this_week_label, || { ... }),
        // ...
    ]
}
```

Also fix the hardcoded `Sunday` week start in "This Week"/"Last Week" by accepting the locale's `start_of_week`:

```rust
pub fn default_static_ranges(locale: &CalendarLocale) -> Vec<StaticRange> {
    let week_start = locale.start_of_week;
    // ...
    StaticRange::new(&locale.this_week_label, move || {
        let t = DateHelper::today();
        let start = DateHelper::start_of_week(t, week_start);
        let end = DateHelper::end_of_week(t, week_start);
        RangeSelection::new("selection").with_dates(Some(start), Some(end))
    }),
    // ...
}
```

**Note:** This changes the signature of `default_static_ranges()`. Since the `DefinedRange` component currently calls this internally, it will need to pass its locale prop through.

### A.5 Phase 4: Fix `DefinedRange` to Use `static_ranges` Prop

The `DefinedRange` component currently ignores its `static_ranges` prop and always loads defaults. Fix this:

```rust
let static_ranges = match &props.static_ranges {
    Some(custom) => { /* convert StaticRangeItem to display items */ },
    None => RangeHelper::default_static_ranges(&locale),
};
```

This also requires redesigning `StaticRangeItem` to be useful (add a `range_fn` or change the prop type to `Vec<StaticRange>`).

### A.6 Phase 5: Add `LocaleHelper` Factory Methods for Common Locales

**File:** `yew-date-range-core/src/utils/locale_helper.rs`

Add pre-built locale factories:

```rust
impl LocaleHelper {
    pub fn default_locale() -> CalendarLocale { ... }        // English, Sunday start
    pub fn monday_start_locale() -> CalendarLocale { ... }   // English, Monday start
    pub fn french() -> CalendarLocale { ... }                 // French labels, Monday start
    pub fn german() -> CalendarLocale { ... }                 // German labels, Monday start
    pub fn spanish() -> CalendarLocale { ... }                // Spanish labels, Monday start
    pub fn arabic() -> CalendarLocale { ... }                 // Arabic labels, Saturday start
    pub fn japanese() -> CalendarLocale { ... }               // Japanese labels, Sunday start
    pub fn chinese() -> CalendarLocale { ... }                // Chinese labels, Monday start
}
```

Each factory returns a fully populated `CalendarLocale` with translated month names, day names, button labels, and correct `start_of_week`.

### A.7 Phase 6: `Intl`-Based Auto-Locale via `js-sys` and `wasm-bindgen`

Add a constructor that uses the browser's native `Intl.DateTimeFormat` API to auto-resolve month and day names from a BCP-47 locale tag. This is **not optional** — it is the recommended way to get accurate calendar translations.

**Implementation approach:** Use `js_sys::eval()` or `wasm_bindgen` FFI to call:
```javascript
new Intl.DateTimeFormat(locale, { month: 'long' }).format(date)
new Intl.DateTimeFormat(locale, { month: 'short' }).format(date)
new Intl.DateTimeFormat(locale, { weekday: 'long' }).format(date)
new Intl.DateTimeFormat(locale, { weekday: 'short' }).format(date)
```

For each of the 12 months and 7 weekdays, format a known reference date to extract the localized name.

```rust
impl CalendarLocale {
    /// Creates a locale from a BCP-47 tag (e.g., "fr-FR", "de-DE").
    ///
    /// Uses `js_sys::Intl::DateTimeFormat` via the browser's native
    /// internationalization API to auto-populate month and day names.
    /// UI labels (buttons, ARIA text) default to English — combine
    /// with a pre-built factory or override manually for full translation.
    pub fn from_bcp47(tag: &str) -> Self { ... }
}
```

This populates `month_names`, `short_month_names`, `day_names`, `short_day_names` automatically from the browser. UI labels (buttons, ARIA text) default to English unless explicitly overridden or combined with a factory locale.

### A.8 Phase 7: Update Tests

- Add unit tests in `yew-date-range-core/tests/` for each new factory method.
- Add tests validating that `default_static_ranges` respects the locale's `start_of_week`.
- Add an e2e test that renders a picker with a French locale and verifies month/day names.

### A.9 Phase 8: Examples with Two Languages

Both `example/` and `example-tailwind/` must demonstrate at least two languages:

- Add a locale toggle (e.g., English / French) to both examples.
- When switched, the entire picker UI (month names, day names, button labels, static range labels) updates to the selected language.
- The default example should use `LocaleHelper::french()` for the French option.
- The Tailwind example should use `CalendarLocale::from_bcp47("fr-FR")` to demonstrate the Intl-based approach.

### A.10 Phase 9: Document i18n in Docs Page

Add a dedicated "Internationalization" section to `docs/index.html` covering:

- How to pass a `locale` prop to any component.
- The `CalendarLocale` struct and all available keys (with `yew_date_range.` namespace).
- Pre-built locales via `LocaleHelper` factories.
- Using `CalendarLocale::from_bcp47()` for browser-native translations.
- Example code for English, French, and custom locales.

### A.11 Phase 10: Document Custom Styles in Docs Page

Add a dedicated "Custom Styles" section to `docs/index.html` covering:

- How the auto-injected CSS works (`StyleInjector`).
- How to override specific `.rdr*` classes.
- How to use `StyleInjector::inject_custom_css()` for theme overrides.
- Dark mode theming approach (as demonstrated in `dark-overrides.css`).
- CSS custom properties (if added) for easy color theming.
- Complete list of all `.rdr*` CSS class names and their purposes.

---

## Part B: Responsive Design

### B.1 Current State

The library CSS (`date_range.css`) already has a `@media (max-width: 640px)` block that:
- Switches `rdrDateRangePickerHorizontal` from `flex-direction: row` to `column`
- Converts sidebar ranges to horizontal pill layout
- Stacks horizontal months vertically

**Gaps identified:**
1. The 640–740px range is not covered — two months + sidebar overflow between these widths.
2. The overlay/popup has no responsive positioning (always below, never flips above).
3. The examples and docs page have limited mobile optimization.
4. Touch targets on day cells and time picker buttons may be too small on mobile.
5. No landscape-mode optimization for the full `DateRangePicker`.

### B.2 Phase 1: CSS Improvements in `date_range.css`

#### B.2.1 Widen the Media Query Breakpoint

Change the breakpoint from `640px` to `768px` to cover the overflow gap:

```css
@media (max-width: 768px) {
    .rdrDateRangePickerHorizontal { flex-direction: column; }
    .rdrMonthsHorizontal { flex-direction: column; }
    /* ... existing rules ... */
}
```

#### B.2.2 Add Touch-Friendly Sizing

```css
@media (max-width: 768px) {
    .rdrDay { height: 44px; min-width: 44px; }  /* WCAG 2.5.8: 44px minimum */
    .rdrTimePickerUp, .rdrTimePickerDown { width: 36px; height: 28px; }
    .rdrNextPrevButton { width: 40px; height: 40px; }
    .rdrMonth { padding: 8px 8px 12px; min-width: auto; }
}
```

#### B.2.3 Popup Overflow Protection

Add viewport-aware overflow handling:

```css
@media (max-width: 768px) {
    .rdrOverlay {
        position: fixed;
        top: auto;
        bottom: 0;
        left: 0;
        right: 0;
        margin-top: 0;
        border-radius: 16px 16px 0 0;
        max-height: 80vh;
        overflow-y: auto;
        z-index: 1000;
    }
}
```

On mobile, the overlay becomes a bottom sheet instead of an absolutely-positioned dropdown. This is a common mobile pattern (e.g., iOS date picker, Google date picker).

#### B.2.4 Improve Month Separator for Vertical Stacking

When months stack vertically on mobile, the left-border separator should switch to a top border:

```css
@media (max-width: 768px) {
    .rdrMonth + .rdrMonth {
        border-left: none;
        border-top: 1px solid #e4e7eb;
    }
}
```

(Already partially present, but should be under the wider breakpoint.)

### B.3 Phase 2: Responsive Example — Default (CSS)

**File:** `example/styles.css`

Add responsive rules:

```css
@media (max-width: 768px) {
    .page-header { padding: 32px 16px 24px; }
    .page-header h1 { font-size: 24px; }
    .examples { padding: 0 12px 32px; }
    .example-section { margin-bottom: 32px; }
    .example-container {
        padding: 12px;
        overflow-x: auto;
    }
}
```

### B.4 Phase 3: Responsive Example — Tailwind

**File:** `example-tailwind/src/main.rs`

The Tailwind example already uses responsive Tailwind utilities (`max-w-5xl`, `px-6`, `py-10`). Additional improvements:

- Use `sm:px-6 px-3` for tighter padding on small screens.
- Add `overflow-x-auto` to card containers holding wide pickers.
- Use `flex-col sm:flex-row` for the header layout with the dark mode toggle.

### B.5 Phase 4: Responsive Docs Page

**File:** `docs/index.html`

The docs page already has `@media (max-width: 640px)` for the examples grid. Additional improvements:

```css
@media (max-width: 640px) {
    header { padding: 40px 16px 32px; }
    header h1 { font-size: 1.8rem; }
    main { padding: 0 16px 48px; }
    .card { padding: 20px; }
    .badge-row { flex-wrap: wrap; }
    table { font-size: 0.8rem; }
    th, td { padding: 6px 8px; }
    .card a.btn { display: block; text-align: center; margin-bottom: 8px; }
    .card a.btn.secondary { margin-left: 0; }
}
```

### B.6 Phase 5: Dark Mode Override Responsive Handling

**File:** `example-tailwind/dark-overrides.css`

Ensure all dark mode overrides also apply correctly when the layout shifts on mobile. Add:

```css
@media (max-width: 768px) {
    html.dark .rdrDefinedRangesWrapper {
        border-right: none;
        border-bottom: 1px solid #334155;
    }
    html.dark .rdrMonth + .rdrMonth {
        border-left: none;
        border-top: 1px solid #334155;
    }
}
```

### B.7 Phase 6: Component-Level Responsive Prop (Optional)

Add an optional `compact` prop to `Calendar` and `DatePicker` that forces a more compact layout regardless of viewport:

```rust
/// Force compact/mobile layout.
#[prop_or(false)]
pub compact: bool,
```

When `compact` is true, add a `rdrCompact` CSS class to the wrapper, and mirror the mobile CSS rules:

```css
.rdrCompact .rdrMonthsHorizontal { flex-direction: column; }
.rdrCompact .rdrDateRangePickerHorizontal { flex-direction: column; }
/* etc. */
```

This allows consumers to render a compact picker in a sidebar or narrow container without relying on viewport width.

---

## Part C: Implementation Order & Effort Estimates

| Phase | Description | Effort | Files touched |
|---|---|---|---|
| **A.1** | Extend `CalendarLocale` with all UI labels | Small | `calendar_locale.rs` |
| **A.2** | Wire locale through all components | Medium | `calendar.rs`, `calendar_renderer.rs`, `date_picker.rs`, `time_picker.rs` |
| **A.3** | Localize static ranges + fix week start | Small | `range_helper.rs`, `defined_range.rs` |
| **A.4** | Fix `DefinedRange` `static_ranges` prop | Small | `defined_range.rs`, `static_range_item.rs` |
| **A.5** | Add locale factories (French, German, etc.) | Medium | `locale_helper.rs` |
| **A.6** | Optional `Intl`-based auto-locale | Small | `calendar_locale.rs` |
| **A.7** | i18n tests | Small | `tests/` |
| **B.1** | CSS responsive improvements | Small | `date_range.css` |
| **B.2** | Default example responsive | Small | `example/styles.css` |
| **B.3** | Tailwind example responsive | Small | `example-tailwind/src/main.rs` |
| **B.4** | Docs page responsive | Small | `docs/index.html` |
| **B.5** | Dark mode responsive overrides | Small | `dark-overrides.css` |
| **B.6** | Optional `compact` prop | Small | `calendar.rs`, `date_range.css` |

**Recommended order:** A.1 → A.2 → A.3 → A.4 → B.1 → B.2 → B.3 → B.4 → B.5 → A.5 → A.7 → A.6 → B.6

---

## Part D: Migration & Backward Compatibility

- All new `CalendarLocale` fields have English defaults → **zero breaking changes**.
- `default_static_ranges()` signature change (now takes `&CalendarLocale`) is an internal-only change for the `DefinedRange` component. External consumers who call `RangeHelper::default_static_ranges()` directly will need to pass a locale. Provide a zero-arg convenience that calls `default_static_ranges(&CalendarLocale::default())` for backward compat.
- The responsive CSS changes only add rules — no existing selectors are removed or renamed.
- The `compact` prop defaults to `false`, so it is opt-in.

---

## Part E: Consumer Usage Example (After Implementation)

```rust
use yew_date_range::prelude::*;

// Option 1: Pre-built French locale (all yew_date_range.* keys translated)
let locale = LocaleHelper::french();

// Option 2: Auto-resolve month/day names from browser Intl API
// (UI labels default to English, month/day names from Intl)
let locale = CalendarLocale::from_bcp47("fr-FR");

// Option 3: Custom locale with yew_date_range.* keys
let locale = CalendarLocale {
    // yew_date_range.action.today
    today_label: "Aujourd'hui".into(),
    // yew_date_range.action.clear
    clear_label: "Effacer".into(),
    // yew_date_range.month_names
    month_names: vec!["Janvier".into(), /* ... */],
    ..CalendarLocale::default()
};

// Option 4: Combine Intl auto-detection with manual UI label overrides
let mut locale = CalendarLocale::from_bcp47("de-DE");
locale.today_label = "Heute".into();
locale.clear_label = "Loeschen".into();

html! {
    <DateRangePicker
        ranges={ranges}
        on_change={on_change}
        locale={locale}
        months={2}
    />
}
```
