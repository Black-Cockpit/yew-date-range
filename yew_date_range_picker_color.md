# `yew-date-range` — `DatePicker` cannot set the range fill colour

## Symptom

A `DatePicker` in `SelectionMode::Range` always renders the
selected range (in-range days + start/end edges) in the crate's
default blue `#3d91ff`, regardless of any colour the consumer
supplies. The design-system theme needs a brand-green fill
(`var(--bc-brand-bg)`).

## Root cause (crate v0.2.0)

`DatePicker` builds the ranges it hands to the calendar **purely
from `props.value`** and never threads a colour through.

`src/components/date_picker.rs` (~lines 205-215):

```rust
let ranges_for_calendar: Vec<RangeSelection> = match &props.value {
    SelectionValue::Single(date) =>
        vec![RangeSelection::new("selection").with_dates(*date, *date)],
    SelectionValue::Range { start, end } =>
        vec![RangeSelection::new("selection").with_dates(*start, *end)],
    SelectionValue::Multiple(dates) => dates
        .iter()
        .map(|d| RangeSelection::new("selection").with_dates(Some(*d), Some(*d)))
        .collect(),
};
```

Every `RangeSelection` here is built with `with_dates(...)` only —
`with_color(...)` is never called, so `RangeSelection.color` is
always `None`.

`src/components/calendar_renderer.rs` (~line 396) then falls back
to the hardcoded default:

```rust
let color = state.color.clone().unwrap_or_else(|| "#3d91ff".into());
// ...
<span class="rdrInRange"   style={format!("background: {color}")}></span>
<span class="rdrStartEdge" style={format!("background: {color}")}></span>
<span class="rdrEndEdge"   style={format!("background: {color}")}></span>
```

`DatePickerProps` (`src/components/date_picker.rs`, ~lines 27-146)
has **no colour prop** of any kind. So:

- `DatePicker` has no public way to recolour the range fill.
- The colour is emitted as an inline `style` attribute, which an
  external stylesheet cannot override (without `!important`).
- `DateRangePicker` *does* expose `ranges: Vec<RangeSelection>`
  and therefore honours `RangeSelection.color` — but `DatePicker`
  does not, and `DatePicker` is the component used for single
  pickers and for the app's range filters.

## Requested fix

Give `DatePicker` a way to set the highlight colour, mirroring
what `DateRangePicker` already supports via `RangeSelection`.

Suggested minimal change — add one optional prop and thread it
into the internal range builder:

1. `DatePickerProps` — add:

   ```rust
   /// CSS colour for the selected-range fill (in-range days and
   /// start/end edges). When `None`, the crate default is used.
   #[prop_or_default]
   pub range_color: Option<String>,
   ```

2. In the `ranges_for_calendar` builder, apply it when present —
   e.g. a small helper:

   ```rust
   let apply_color = |range: RangeSelection| match &props.range_color {
       Some(color) => range.with_color(color),
       None => range,
   };
   ```

   and wrap each `RangeSelection::new(...).with_dates(...)` in
   `apply_color(...)`.

`calendar_renderer` already reads `range.color`, so no change is
needed there — the value flows through once the builder sets it.

A CSS colour string is enough; passing `"var(--bc-brand-bg)"`
must keep working (the renderer interpolates it straight into
`style="background: …"`, and CSS custom properties resolve fine
there).

## App-side consumption once the crate is fixed

The app wraps `DatePicker` in
`app/src/components/tables/date_range_filter/date_range_filter.rs`.
After the crate ships `range_color`:

- `DateRangeFilter` forwards a `range_color` prop to `DatePicker`
  (default `Some("var(--bc-brand-bg)".to_string())`), or passes
  it unconditionally.
- The four dead `RangeSelection::new(...).with_color("#34d399")`
  calls — `iam/users/users_list_page.rs`,
  `iam/groups/groups_list_page.rs`,
  `iam/service_accounts/service_accounts_list_page.rs`,
  `login_sessions/session_filter_helpers.rs` — are removed; that
  colour never reached the picker and `#34d399` is a raw hex the
  design system forbids anyway.
- The wizard single-date picker
  (`iam/service_accounts/wizard/account_details.rs`) needs no
  range colour (single mode has no in-range fill).

The `class_name="bc-daterange-calendar"` migration (Rule 1 of
`docs/design_system/yew_date_range.md`) is unaffected and already
in progress.

## Note on `docs/design_system/yew_date_range.md`

That doc's "Rule 2 — pass the range fill colour on
`RangeSelection`" is correct only for `DateRangePicker`. For
`DatePicker` it does nothing today. Once the crate fix lands,
update Rule 2 to say: for `DatePicker`, pass the colour through
the new `range_color` prop.
