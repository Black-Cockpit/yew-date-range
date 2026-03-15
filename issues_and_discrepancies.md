# Issues, Bugs & Discrepancies

Comprehensive audit of the `yew-date-range` codebase performed on 2026-03-15.

---

## 1. Dead Code in `yew-date-range` UI Crate

**Severity: Medium**

The UI crate (`yew-date-range/src/`) still contains full duplicate copies of models and utilities that were moved to `yew-date-range-core` during the refactor. These files are **not referenced** from `lib.rs` or `mod.rs`, so they are effectively dead code, but they still exist on disk and could confuse contributors.

| Dead file | Core equivalent |
|---|---|
| `yew-date-range/src/models/date_format.rs` | `yew-date-range-core/src/models/date_format.rs` |
| `yew-date-range/src/models/day.rs` | `yew-date-range-core/src/models/day_state.rs` |
| `yew-date-range/src/models/month.rs` | `yew-date-range-core/src/models/month_data.rs` + `calendar_locale.rs` + `calendar_direction.rs` + `navigation_action.rs` |
| `yew-date-range/src/models/range.rs` | `yew-date-range-core/src/models/range_selection.rs` + `static_range.rs` + `input_range.rs` + `range_focus.rs` + `range_change.rs` + `range_change_source.rs` |
| `yew-date-range/src/models/selection.rs` | `yew-date-range-core/src/models/selection_value.rs` + `selection_mode.rs` + `display_mode.rs` + `popup_trigger.rs` |
| `yew-date-range/src/models/time_selection.rs` | `yew-date-range-core/src/models/time_selection.rs` + `time_period.rs` + `hour_format.rs` + `time_granularity.rs` |
| `yew-date-range/src/utils/date_utils.rs` | `yew-date-range-core/src/utils/date_helper.rs` |
| `yew-date-range/src/utils/locale_utils.rs` | `yew-date-range-core/src/utils/locale_helper.rs` |
| `yew-date-range/src/utils/range_utils.rs` | `yew-date-range-core/src/utils/range_helper.rs` |

The dead files still contain inline `#[cfg(test)] mod tests` blocks (e.g., `selection.rs`, `time_selection.rs`, `date_format.rs`, `date_utils.rs`, `range_utils.rs`). Since the files are never compiled, **those tests are silently never run**.

**Recommendation:** Delete all dead files and their empty `mod.rs` parents (which already contain comments pointing to core). Remove the `models/` and `utils/` directories from the UI crate entirely.

---

## 2. `unreachable!()` in Dead Code

**Severity: Low (dead code) / High (if ever compiled)**

`yew-date-range/src/utils/date_utils.rs:191` contains `_ => unreachable!()` in `index_to_month`. The core equivalent correctly uses `_ => Month::December` (safe fallback). If the dead file were ever re-enabled, this would be a panic risk.

---

## 3. `unwrap_or('-')` in `DateFormat::parse` (Core)

**Severity: Low**

`yew-date-range-core/src/models/date_format.rs:89`:
```rust
let sep = pattern
    .chars()
    .find(|c| !c.is_alphanumeric())
    .unwrap_or('-');
```

While `unwrap_or` is safe (not `unwrap()`), the fallback to `'-'` means a pattern like `"yyyyMMdd"` (no separator) will silently try to split on `'-'` and fail to parse. This is not a crash but a silent logic error — the parser cannot handle non-separated patterns. This should either be documented as a limitation or handled explicitly.

---

## 4. `format_date_display` Panics on Short Month Names

**Severity: Medium**

`yew-date-range-core/src/utils/date_helper.rs:502`:
```rust
format!("{} {}, {}", &month_name[..3], date.day(), date.year())
```

If a consumer provides a `CalendarLocale` with month names shorter than 3 characters (e.g., single-character abbreviations for some locales), the `[..3]` slice will panic at runtime. This should use `.get(..3).unwrap_or(month_name)` or similar safe slicing.

---

## 5. `DefinedRange` Ignores `static_ranges` Prop

**Severity: Medium**

`yew-date-range/src/components/defined_range.rs:43`:
```rust
let static_ranges = RangeHelper::default_static_ranges();
```

The `DefinedRangeProps` struct accepts an optional `static_ranges: Option<Vec<StaticRangeItem>>` prop (line 16), but the component **always** uses the default static ranges and **never reads** the prop value. Custom predefined ranges passed by the user are silently ignored.

---

## 6. Hardcoded English Strings in UI Components

**Severity: Medium**

Multiple components contain hardcoded English text that cannot be overridden:

| Location | String |
|---|---|
| `date_picker.rs:495` | `"Today"` |
| `date_picker.rs:504` | `"Clear"` |
| `date_picker.rs:525` | `"Select date"` |
| `date_picker.rs:526` | `"Select range"` |
| `date_picker.rs:527` | `"Select dates"` |
| `calendar.rs:351` | `"Start Date"` |
| `calendar.rs:354` | `"End Date"` |
| `calendar.rs:289` | `"Select month"` |
| `calendar.rs:296` | `"Select year"` |
| `calendar.rs:333-340` | `"Previous Month"`, `"Next Month"`, etc. |
| `calendar_renderer.rs:67` | `"Select Jan"`, `"Select Feb"`, etc. |
| `calendar_renderer.rs:117` | `"Select 2020"`, etc. |
| `time_picker.rs:132-206` | `"Increment hour"`, `"Decrement hour"`, `"Toggle AM/PM"`, etc. |
| `calendar_renderer.rs:180` | `"W"` (week number header) |

All of these bypass the `CalendarLocale` and cannot be translated without forking the component code.

---

## 7. Hardcoded English Labels in Static Ranges

**Severity: Medium**

`yew-date-range-core/src/utils/range_helper.rs:22-54`:

The default static range labels (`"Today"`, `"Yesterday"`, `"This Week"`, `"Last Week"`, `"This Month"`, `"Last Month"`) are hardcoded in English. There is no mechanism to localize them.

---

## 8. Hardcoded `Sunday` as Week Start in Static Ranges

**Severity: Low**

`yew-date-range-core/src/utils/range_helper.rs:32-33`:
```rust
let start = DateHelper::start_of_week(t, time::Weekday::Sunday);
let end = DateHelper::end_of_week(t, time::Weekday::Sunday);
```

The "This Week" and "Last Week" static ranges always use `Sunday` as the week start, even though the `CalendarLocale` may be configured with `Monday` as `start_of_week`. This means the sidebar ranges will be inconsistent with the calendar grid when using a Monday-start locale.

---

## 9. `CalendarLocale.date_format` Field Inconsistency

**Severity: Low**

`CalendarLocale` has a `date_format: String` field defaulting to `"YYYY-MM-DD"` (uppercase tokens), while `DateFormat` uses lowercase tokens like `"yyyy-MM-dd"`. The locale field is never actually consumed by any component — it appears to be vestigial. The `DatePicker` uses a separate `date_format: Option<DateFormat>` prop instead.

---

## 10. `StaticRangeItem` is Unused

**Severity: Low**

`StaticRangeItem` (in core) is defined and exported in the prelude, and `DefinedRangeProps` accepts `Option<Vec<StaticRangeItem>>`, but:
- The `DefinedRange` component ignores the prop (see issue #5).
- `StaticRangeItem` has no `range_fn` — it only has `label` and a private `id` field, making it functionally useless for producing date ranges.
- No code in the project creates `StaticRangeItem` instances.

---

## 11. `MonthData` and `WeekData` Lack `Default` Derives

**Severity: Low**

`MonthData` and `WeekData` in core (`month_data.rs`, `week_data.rs`) do not derive `Default`. While not strictly a bug, this limits ergonomics for consumers building test fixtures or custom month data.

---

## 12. Overlay Positioning Is Always Below Input

**Severity: Medium**

`yew-date-range/src/styles/date_range.css:703-707`:
```css
.rdrOverlay {
  position: absolute;
  top: 100%;
  left: 0;
}
```

The popup overlay always opens **below** the input. When the input is near the bottom of the viewport, the popup will overflow off-screen. There is no viewport-aware positioning logic (flip to above when insufficient space below).

---

## 13. No Keyboard Navigation for Day Cells

**Severity: Medium (Accessibility)**

Day cells are `<button>` elements but lack `onkeydown` handlers for arrow-key navigation between days. Users can Tab to individual day cells (native button behavior), but cannot use arrow keys to move between days, which is a standard calendar accessibility pattern (WAI-ARIA date picker pattern).

---

## 14. `shown_date` State Not Synced with Props on Update

**Severity: Low**

In `Calendar`, `DateRange`, and `DatePicker`, the `shown_date` is initialized from props via `use_state(|| ...)` which only runs once. If the parent component changes the `shown_date` prop after initial render, the internal state will **not** update to reflect the new prop value. The component will continue showing the month from the first render.

---

## 15. CSS Class Typo: `rdrPprevButton`

**Severity: Low (Cosmetic)**

The "previous" button uses class `rdrPprevButton` (double `p`). This is consistent across all references (CSS, Rust components, and e2e tests), so it works, but it is a typo that deviates from the otherwise clean naming convention (`rdrNextButton` uses single `N`).

---

## 16. Tailwind Example Uses CDN Script Tag

**Severity: Not an issue**

`example-tailwind/index.html:8`:
```html
<script src="https://cdn.tailwindcss.com"></script>
```

The Tailwind example loads the full Tailwind CDN script at runtime. This is **acceptable** since it is only a demo/example application, not production code. No action needed.

---

## 17. `wasm_logger` Missing from Example-Tailwind

**Severity: Low**

The default `example/src/main.rs` calls `wasm_logger::init(...)` for browser console logging, but `example-tailwind/src/main.rs` does not. This is inconsistent — console logs won't appear when debugging the Tailwind example.

---

## 18. Docs Page Example Links Use Relative Paths

**Severity: Low**

`docs/index.html:93-98`:
```html
<a class="btn" href="./example/">Open Default Example →</a>
<a class="btn secondary" href="./example-tailwind/">Open Tailwind Example →</a>
```

These relative paths only work when the docs are deployed alongside the example directories at the same level (as on GitHub Pages). If the docs page is served independently, the links will 404.

---

## 19. No `max-width` on Calendar in Horizontal Multi-Month Layout

**Severity: Low (Responsiveness)**

When `months=2` with `direction=Horizontal`, the calendar renders two months side-by-side, each with `min-width: 280px`. Combined with the sidebar (`min-width: 180px`), the total minimum width is ~740px. On viewports narrower than this, the component overflows horizontally. The CSS media query at `@media (max-width: 640px)` switches to column layout, but the range 640-740px is uncovered.

---

## 20. `year_range` Prop Is Declared but Unused

**Severity: Low**

`CalendarProps.year_range: Option<(i32, i32)>` (line 108 of `calendar.rs`) is documented as "unused now, decade navigation is automatic" but still appears in the public API. This is confusing for consumers who may try to use it expecting it to constrain the year picker range.

---

## 21. `log` Crate and All Logging Should Be Removed

**Severity: Medium**

The `log` crate is listed as a dependency in:
- `Cargo.toml` (workspace level): `log = "0.4"`
- `yew-date-range-core/Cargo.toml`: `log = "0.4"`
- `yew-date-range/Cargo.toml`: `log = "0.4"`

The `log` crate is not used anywhere in the source code — no `log::info!()`, `log::debug!()`, etc. calls exist. It is a dead dependency that adds unnecessary weight. The `wasm_logger` dependency in the example apps (`example/Cargo.toml`) should also be removed along with the `wasm_logger::init(...)` call.

**Recommendation:** Remove `log` from all `Cargo.toml` files. Remove `wasm_logger` from example `Cargo.toml` files and the `wasm_logger::init(...)` call from `example/src/main.rs`.

---

## Summary

| # | Issue | Severity | Category |
|---|---|---|---|
| 1 | Dead code files in UI crate | Medium | Code hygiene |
| 2 | `unreachable!()` in dead code | Low | Safety |
| 3 | `unwrap_or('-')` silent failure | Low | Logic |
| 4 | `format_date_display` slice panic | Medium | Bug |
| 5 | `static_ranges` prop ignored | Medium | Bug |
| 6 | Hardcoded English strings in UI | Medium | i18n |
| 7 | Hardcoded English static range labels | Medium | i18n |
| 8 | Hardcoded Sunday week start in static ranges | Low | Logic |
| 9 | `CalendarLocale.date_format` vestigial | Low | API |
| 10 | `StaticRangeItem` unused | Low | Dead code |
| 11 | Missing `Default` on `MonthData`/`WeekData` | Low | Ergonomics |
| 12 | Overlay always below input | Medium | UX |
| 13 | No arrow-key day navigation | Medium | Accessibility |
| 14 | `shown_date` not synced on prop update | Low | Bug |
| 15 | `rdrPprevButton` typo | Low | Cosmetic |
| 16 | ~~Tailwind CDN script in example~~ | N/A | Acceptable |
| 17 | Missing `wasm_logger` in tailwind example | Low | Consistency |
| 18 | Relative paths in docs page | Low | Deployment |
| 19 | No responsive handling for 640-740px range | Low | Responsiveness |
| 20 | `year_range` prop unused but public | Low | API |
| 21 | `log` crate and all logging should be removed | Medium | Code hygiene |
