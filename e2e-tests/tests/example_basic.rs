//! End-to-end browser tests for the default (CSS) example application.
//!
//! Verifies rendered DOM structure, user interactions, and visual state
//! across all 10 example sections using Playwright with Chromium.

mod common;

use common::SharedFixture;
use playwright::api::page::Page;
use std::time::Duration;
use tokio::sync::OnceCell;

type E = Box<dyn std::error::Error + Send + Sync>;
const PAUSE: Duration = Duration::from_millis(350);

static FIXTURE: OnceCell<SharedFixture> = OnceCell::const_new();

/// Creates a fresh browser page navigated to the default example dist.
async fn page() -> Result<Page, E> {
    common::new_page(&FIXTURE, "example/dist").await
}

/// Validates that the page loads with title, subtitle, and all 10 example sections.
#[tokio::test]
async fn test_page_loads_with_all_example_sections() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the page title.
    let title = f.text_content("h1", None).await?;
    assert_eq!(title.as_deref(), Some("yew-date-range"));

    // Verify the subtitle mentions the library purpose.
    let subtitle = f.text_content("p.subtitle", None).await?;
    assert!(
        subtitle.as_deref().unwrap_or("").contains("date range picker"),
        "subtitle should mention date range picker"
    );

    // All 10 example sections should be present.
    let sections = f.query_selector_all(".example-section").await?;
    assert_eq!(sections.len(), 10, "page should contain all 10 example sections");

    // Each section should have an h2 heading.
    let headings = f.query_selector_all(".example-section h2").await?;
    assert_eq!(headings.len(), 10, "each section should have a heading");

    // Each section should have a selection-info display.
    let infos = f.query_selector_all(".selection-info").await?;
    assert_eq!(infos.len(), 10, "each section should have selection info");

    Ok(())
}

/// Validates that the DateRangePicker (example 1) renders with sidebar, two months, and navigation.
#[tokio::test]
async fn test_daterangepicker_structure() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the wrapper uses horizontal layout.
    let wrapper = f.query_selector(".rdrDateRangePickerWrapper").await?;
    assert!(wrapper.is_some(), "DateRangePicker wrapper missing");

    let cls = f.get_attribute(".rdrDateRangePickerWrapper", "class", None).await?;
    assert!(
        cls.as_deref().unwrap_or("").contains("rdrDateRangePickerHorizontal"),
        "should use horizontal layout"
    );

    // Verify the sidebar has 6 predefined ranges.
    let sidebar = f.query_selector(".rdrDefinedRangesWrapper").await?;
    assert!(sidebar.is_some(), "DefinedRange sidebar missing");

    let range_btns = f.query_selector_all(".rdrStaticRange").await?;
    assert_eq!(range_btns.len(), 6, "should have 6 predefined ranges");

    // Verify two months are displayed.
    let months = f.query_selector_all(".rdrDateRangePickerWrapper .rdrMonth").await?;
    assert_eq!(months.len(), 2, "should show 2 months side by side");

    // Verify the date display header with start/end inputs.
    let date_display = f.query_selector(".rdrDateRangePickerWrapper .rdrDateDisplayWrapper").await?;
    assert!(date_display.is_some(), "date display header missing");

    let date_inputs = f.query_selector_all(".rdrDateRangePickerWrapper .rdrDateInput").await?;
    assert_eq!(date_inputs.len(), 2, "should have start + end date inputs");

    // Verify navigation controls.
    let prev_btn = f.query_selector(".rdrDateRangePickerWrapper .rdrPrevButton").await?;
    assert!(prev_btn.is_some(), "prev month button missing");
    let next_btn = f.query_selector(".rdrDateRangePickerWrapper .rdrNextButton").await?;
    assert!(next_btn.is_some(), "next month button missing");

    Ok(())
}

/// Validates that the DateRangePicker initial selection (Mar 10-20) is displayed with edge classes.
#[tokio::test]
async fn test_daterangepicker_initial_selection_displayed() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the selection info shows the initial March 2026 dates.
    let info = f.text_content(".example-section:nth-of-type(1) .selection-info", None).await?;
    let info_str = info.unwrap_or_default();
    assert!(
        info_str.contains("2026") && info_str.contains("03"),
        "initial selection should show March 2026 dates, got: {info_str}"
    );

    // Verify start/end edge and in-range classes are present.
    let start_edges = f.query_selector_all(".rdrDateRangePickerWrapper .rdrDayStartEdge").await?;
    assert!(!start_edges.is_empty(), "should highlight range start edge");

    let end_edges = f.query_selector_all(".rdrDateRangePickerWrapper .rdrDayEndEdge").await?;
    assert!(!end_edges.is_empty(), "should highlight range end edge");

    let in_range = f.query_selector_all(".rdrDateRangePickerWrapper .rdrDayInRange").await?;
    assert!(!in_range.is_empty(), "should have in-range day cells");

    Ok(())
}

/// Validates that clicking the Today sidebar range updates the selection info.
#[tokio::test]
async fn test_defined_range_today_click_updates_selection() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Record the info before clicking.
    let info_before = f.text_content(".example-section:nth-of-type(1) .selection-info", None).await?;

    // Click the "Today" predefined range.
    f.click_builder(".rdrStaticRange:first-child").click().await?;
    tokio::time::sleep(PAUSE).await;

    // Verify the info changed.
    let info_after = f.text_content(".example-section:nth-of-type(1) .selection-info", None).await?;
    assert_ne!(info_before, info_after, "clicking Today should update the selection info");

    // Verify the selected class is applied.
    let selected = f.query_selector(".rdrStaticRangeSelected").await?;
    assert!(selected.is_some(), "clicked range should be marked as selected");

    Ok(())
}

/// Validates that the two-click range selection flow works: first click sets start, second sets end.
#[tokio::test]
async fn test_range_selection_two_click_flow() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Click a day to start range selection.
    let day_sel = ".rdrDateRangePickerWrapper .rdrDay:not(.rdrDayPassive):not(.rdrDayDisabled):not(.rdrDayEmpty)";
    f.click_builder(day_sel).click().await?;
    tokio::time::sleep(PAUSE).await;

    // Verify the info shows a partial selection.
    let info_mid = f.text_content(".example-section:nth-of-type(1) .selection-info", None).await?;
    let info_mid_str = info_mid.unwrap_or_default();
    assert!(info_mid_str.contains("Selected"), "info should show selection in progress");

    // Click a later day to complete the range.
    let days = f.query_selector_all(day_sel).await?;
    assert!(days.len() > 5, "should have multiple clickable days");
    let target_idx = days.len().min(10) - 1;
    days[target_idx].click_builder().click().await?;
    tokio::time::sleep(PAUSE).await;

    // Verify the info changed after the second click.
    let info_after = f.text_content(".example-section:nth-of-type(1) .selection-info", None).await?;
    assert_ne!(
        info_mid_str,
        info_after.unwrap_or_default(),
        "second click should complete the range"
    );

    Ok(())
}

/// Validates that the DateRange calendar-only (example 2) renders with date display.
#[tokio::test]
async fn test_daterange_calendar_only_renders() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify standalone DateRange wrappers exist.
    let wrappers = f.query_selector_all(".rdrDateRangeWrapper").await?;
    assert!(!wrappers.is_empty(), "should have at least one standalone DateRange");

    // Verify the date display header is visible.
    let displays = f.query_selector_all(".rdrDateRangeWrapper .rdrDateDisplayWrapper").await?;
    assert!(!displays.is_empty(), "calendar-only example should show date display");

    Ok(())
}

/// Validates that the vertical layout (example 3) renders with week numbers.
#[tokio::test]
async fn test_vertical_layout_with_week_numbers() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the vertical layout class is present.
    let vertical = f.query_selector(".rdrMonthsVertical").await?;
    assert!(vertical.is_some(), "should have a vertical months container");

    // Verify week numbers are rendered.
    let week_nums = f.query_selector_all(".rdrWeekNumber").await?;
    assert!(
        week_nums.len() >= 5,
        "vertical example should show week numbers for each week row"
    );

    Ok(())
}

/// Validates that the inline single-date picker (example 4) renders with Today/Clear buttons.
#[tokio::test]
async fn test_datepicker_inline_renders_calendar_with_actions() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify inline picker is present.
    let inline_pickers = f.query_selector_all(".rdrDatePickerInline").await?;
    assert!(!inline_pickers.is_empty(), "should have at least one inline DatePicker");

    // Verify the calendar is inside.
    let cal = f.query_selector(".rdrDatePickerInline .rdrCalendarWrapper").await?;
    assert!(cal.is_some(), "inline picker should contain a Calendar");

    // Verify action buttons are present.
    let today_btns = f.query_selector_all(".rdrDatePickerTodayButton").await?;
    assert!(!today_btns.is_empty(), "Today button missing");

    let clear_btns = f.query_selector_all(".rdrDatePickerClearButton").await?;
    assert!(!clear_btns.is_empty(), "Clear button missing");

    Ok(())
}

/// Validates that the Today button selects today's date after clearing.
#[tokio::test]
async fn test_datepicker_today_button_selects_today() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Click Clear to reset the selection.
    f.click_builder(".rdrDatePickerClearButton").click().await?;
    tokio::time::sleep(PAUSE).await;

    // Click Today to select today's date.
    f.click_builder(".rdrDatePickerTodayButton").click().await?;
    tokio::time::sleep(PAUSE).await;

    // Verify the today dot indicator is visible.
    let today_dot = f.query_selector(".rdrDayTodayDot").await?;
    assert!(today_dot.is_some(), "today dot indicator should be visible");

    // Verify the info shows a selected date.
    let info_after = f.text_content(".example-section:nth-of-type(4) .selection-info", None).await?;
    assert!(
        info_after.as_deref().unwrap_or("").contains("Selected"),
        "Today button should set a date"
    );

    Ok(())
}

/// Validates that the popup range picker (example 5) has input, icon, overlay, and two months.
#[tokio::test]
async fn test_popup_range_input_and_overlay() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify popup pickers exist.
    let popups = f.query_selector_all(".rdrDatePickerPopup").await?;
    assert!(popups.len() >= 2, "should have popup range + popup multiple pickers");

    // Verify the input has the correct placeholder and ARIA label.
    let placeholder = f.get_attribute(
        ".example-section:nth-of-type(5) .rdrDatePickerInput",
        "placeholder",
        None,
    ).await?;
    assert_eq!(placeholder.as_deref(), Some("Click to select range"), "popup range should show custom placeholder");

    // Verify the calendar icon is visible.
    let icon = f.query_selector(".rdrDatePickerPopup .rdrDatePickerIcon").await?;
    assert!(icon.is_some(), "popup range picker should show calendar icon");

    // Verify the overlay is hidden initially.
    let overlay = f.query_selector(".rdrOverlay").await?;
    assert!(overlay.is_none(), "overlay hidden before user interaction");

    // Click the input to open the overlay.
    f.click_builder(".rdrDatePickerPopup .rdrDatePickerInput").click().await?;
    f.wait_for_selector_builder(".rdrOverlay").wait_for_selector().await?;

    // Verify the overlay contains a calendar with two months.
    let popup_months = f.query_selector_all(".rdrOverlay .rdrMonth").await?;
    assert_eq!(popup_months.len(), 2, "popup range should show 2 months");

    Ok(())
}

/// Validates that pressing Escape closes the popup overlay.
#[tokio::test]
async fn test_popup_escape_to_close() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Open the popup overlay.
    f.click_builder(".rdrDatePickerPopup .rdrDatePickerInput").click().await?;
    f.wait_for_selector_builder(".rdrOverlay").wait_for_selector().await?;

    // Press Escape via JS dispatch.
    f.eval::<serde_json::Value>("document.dispatchEvent(new KeyboardEvent('keydown', {key: 'Escape', bubbles: true}))").await?;
    tokio::time::sleep(PAUSE).await;

    // Verify the overlay is closed.
    let overlay = f.query_selector(".rdrOverlay").await?;
    assert!(overlay.is_none(), "Escape should close the overlay");

    Ok(())
}

/// Validates that clicking outside the popup overlay closes it.
#[tokio::test]
async fn test_popup_outside_click_closes() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Open the popup overlay.
    f.click_builder(".rdrDatePickerPopup .rdrDatePickerInput").click().await?;
    f.wait_for_selector_builder(".rdrOverlay").wait_for_selector().await?;

    // Click outside the overlay on the page heading.
    f.click_builder("h1").click().await?;
    tokio::time::sleep(PAUSE).await;

    // Verify the overlay is closed.
    let overlay = f.query_selector(".rdrOverlay").await?;
    assert!(overlay.is_none(), "outside click should close the overlay");

    Ok(())
}

/// Validates that the popup multiple-date picker (example 6) shows pre-selected dates in the input.
#[tokio::test]
async fn test_popup_multiple_initial_value_in_input() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Read the input value via JS property.
    let val_str: String = f.evaluate_on_selector(
        ".example-section:nth-of-type(6) .rdrDatePickerInput",
        "el => el.value",
        None::<()>,
    ).await?;
    assert!(
        val_str.contains("2026") && val_str.contains(","),
        "multiple popup input should show comma-separated dates, got: {val_str}"
    );

    Ok(())
}

/// Validates that clicking a day in the multiple popup toggles it in the selection.
#[tokio::test]
async fn test_popup_multiple_toggle_date() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Open the multiple date popup.
    f.click_builder(".example-section:nth-of-type(6) .rdrDatePickerInput").click().await?;
    f.wait_for_selector_builder(".rdrOverlay").wait_for_selector().await?;

    // Record the selection info before clicking.
    let info_before = f.text_content(".example-section:nth-of-type(6) .selection-info", None).await?;

    // Click a non-selected day to toggle it.
    f.click_builder(".rdrOverlay .rdrDay:not(.rdrDayPassive):not(.rdrDayDisabled):not(.rdrDayEmpty):not(.rdrDaySelected)")
        .click()
        .await?;
    tokio::time::sleep(PAUSE).await;

    // Verify the selection info changed.
    let info_after = f.text_content(".example-section:nth-of-type(6) .selection-info", None).await?;
    assert_ne!(info_before, info_after, "clicking a day should toggle it in multiple mode");

    Ok(())
}

/// Validates that the 24h time picker (example 7) renders with 3 fields, separators, and no AM/PM.
#[tokio::test]
async fn test_time_picker_24h_structure() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the TimePicker is present in the 7th section.
    let tp = f.query_selector(".example-section:nth-of-type(7) .rdrTimePicker").await?;
    assert!(tp.is_some(), "TimePicker component missing");

    // Verify 3 fields (hours, minutes, seconds) scoped to section 7.
    let fields = f.query_selector_all(".example-section:nth-of-type(7) .rdrTimePickerField").await?;
    assert_eq!(fields.len(), 3, "should show hour, minute, second fields");

    // Verify 2 separators between h:m:s scoped to section 7.
    let seps = f.query_selector_all(".example-section:nth-of-type(7) .rdrTimePickerSeparator").await?;
    assert_eq!(seps.len(), 2, "2 colon separators between h:m:s");

    // Verify no AM/PM toggle in 24h mode scoped to section 7.
    let period = f.query_selector(".example-section:nth-of-type(7) .rdrTimePickerPeriodButton").await?;
    assert!(period.is_none(), "24h mode should not show AM/PM toggle");

    Ok(())
}

/// Validates that clicking the hour up button increments the displayed hour.
#[tokio::test]
async fn test_time_picker_increment_hour() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the initial time is 14:30.
    let info_before = f.text_content(".example-section:nth-of-type(7) .selection-info", None).await?;
    let before_str = info_before.unwrap_or_default();
    assert!(before_str.contains("14:30"), "initial time should be 14:30, got: {before_str}");

    // Click the hour up button.
    f.click_builder(".example-section:nth-of-type(7) .rdrTimePickerUp").click().await?;
    tokio::time::sleep(PAUSE).await;

    // Verify the hour incremented to 15.
    let info_after = f.text_content(".example-section:nth-of-type(7) .selection-info", None).await?;
    let after_str = info_after.unwrap_or_default();
    assert!(
        after_str.contains("15:30"),
        "hour should increment from 14 to 15, got: {after_str}"
    );

    Ok(())
}

/// Validates that the constrained range (example 8) limits selection to March 2026 with disabled date ranges.
#[tokio::test]
async fn test_constrained_range_renders() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the 8th section has a DateRange component.
    let section_info = f.text_content(".example-section:nth-of-type(8) .selection-info", None).await?;
    assert!(
        section_info.is_some(),
        "constrained range section should have selection info"
    );

    // Verify there are disabled days in the constrained range (disabled_ranges spans Mar 14-16).
    let disabled = f.query_selector_all(".example-section:nth-of-type(8) .rdrDayDisabled").await?;
    assert!(
        !disabled.is_empty(),
        "constrained range should have disabled days from min_date, max_date, or disabled_ranges"
    );

    Ok(())
}

/// Validates that the 12h AM/PM time picker (example 9) shows an AM/PM toggle button.
#[tokio::test]
async fn test_time_picker_12h_ampm_toggle() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the 9th section has a time picker with an AM/PM period button.
    let period_btn = f.query_selector(".example-section:nth-of-type(9) .rdrTimePickerPeriodButton").await?;
    assert!(period_btn.is_some(), "12h time picker should have AM/PM toggle button");

    // Verify the initial time info contains AM (9:15:30 AM).
    let info = f.text_content(".example-section:nth-of-type(9) .selection-info", None).await?;
    let info_str = info.unwrap_or_default();
    assert!(
        info_str.contains("AM") || info_str.contains("PM"),
        "12h time picker should show AM or PM, got: {info_str}"
    );

    Ok(())
}

/// Validates that the disabled state (example 10) renders a non-interactive read-only picker.
#[tokio::test]
async fn test_disabled_state_renders() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the 10th section has an inline picker.
    let disabled_picker = f.query_selector(".example-section:nth-of-type(10) .rdrDatePickerInline").await?;
    assert!(disabled_picker.is_some(), "disabled state section should have an inline picker");

    // Verify the info mentions read-only.
    let info = f.text_content(".example-section:nth-of-type(10) .selection-info", None).await?;
    let info_str = info.unwrap_or_default();
    assert!(
        info_str.contains("read-only"),
        "disabled state info should mention read-only, got: {info_str}"
    );

    Ok(())
}

/// Validates that prev/next navigation buttons change the displayed month.
#[tokio::test]
async fn test_month_navigation_prev_next() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Record the current month title.
    let month_before = f.text_content(".rdrMonthPickerTitle", None).await?;

    // Navigate forward.
    f.click_builder(".rdrNextButton").click().await?;
    tokio::time::sleep(PAUSE).await;
    let month_fwd = f.text_content(".rdrMonthPickerTitle", None).await?;
    assert_ne!(month_before, month_fwd, "next button should advance month");

    // Navigate backward.
    f.click_builder(".rdrPrevButton").click().await?;
    tokio::time::sleep(PAUSE).await;
    let month_back = f.text_content(".rdrMonthPickerTitle", None).await?;
    assert_eq!(month_before, month_back, "prev button should return to original month");

    Ok(())
}

/// Validates that the month picker drill-down shows 12 month cells and returns to day view on click.
#[tokio::test]
async fn test_month_picker_drilldown_and_select() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Click the month title to open the month grid.
    f.click_builder(".rdrMonthPickerTitle").click().await?;
    f.wait_for_selector_builder(".rdrMonthPickerGrid").wait_for_selector().await?;

    // Verify 12 month cells with one selected.
    let cells = f.query_selector_all(".rdrMonthPickerCell").await?;
    assert_eq!(cells.len(), 12, "month grid should show all 12 months");

    let selected = f.query_selector(".rdrMonthPickerCellSelected").await?;
    assert!(selected.is_some(), "current month should be highlighted");

    // Click a month to return to the day view.
    f.click_builder(".rdrMonthPickerCell:first-child").click().await?;
    tokio::time::sleep(PAUSE).await;

    let days = f.query_selector(".rdrDays").await?;
    assert!(days.is_some(), "clicking a month should return to day view");

    Ok(())
}

/// Validates that the year picker drill-down shows a decade of years and navigates to month view.
#[tokio::test]
async fn test_year_picker_drilldown_and_select() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Click the year title to open the year grid.
    f.click_builder(".rdrYearPickerTitle").click().await?;
    f.wait_for_selector_builder(".rdrYearPickerGrid").wait_for_selector().await?;

    // Verify 10 year cells with one selected.
    let cells = f.query_selector_all(".rdrYearPickerCell").await?;
    assert_eq!(cells.len(), 10, "year grid should show a decade (10 years)");

    let decade = f.query_selector(".rdrDecadeTitle").await?;
    assert!(decade.is_some(), "decade range title should be visible");

    // Click a year to navigate to the month picker view.
    f.click_builder(".rdrYearPickerCell:first-child").click().await?;
    tokio::time::sleep(PAUSE).await;

    let month_grid = f.query_selector(".rdrMonthPickerGrid").await?;
    assert!(month_grid.is_some(), "clicking a year should go to month picker view");

    Ok(())
}

/// Validates that day cells have correct state classes (weekend, today, passive).
#[tokio::test]
async fn test_day_cell_css_state_classes() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify weekend day cells.
    let weekends = f.query_selector_all(".rdrDayWeekend").await?;
    assert!(!weekends.is_empty(), "weekend days should have rdrDayWeekend class");

    // Verify today marker.
    let today = f.query_selector(".rdrDayToday").await?;
    assert!(today.is_some(), "today should have rdrDayToday class");

    let today_dot = f.query_selector(".rdrDayTodayDot").await?;
    assert!(today_dot.is_some(), "today should have a dot marker");

    // Verify passive days from adjacent months.
    let passive = f.query_selector_all(".rdrDayPassive").await?;
    assert!(!passive.is_empty(), "should have passive days from adjacent months");

    Ok(())
}

/// Validates that day cells have aria-label attributes with date information.
#[tokio::test]
async fn test_day_cells_have_aria_labels() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify day buttons have aria-label.
    let aria = f.get_attribute(
        ".rdrDay:not(.rdrDayEmpty):not(.rdrDayPassive)",
        "aria-label",
        None,
    ).await?;
    assert!(
        aria.is_some() && !aria.as_deref().unwrap_or("").is_empty(),
        "day cells should have aria-label with date"
    );

    Ok(())
}

/// Validates that hover during mid-selection shows a preview range.
#[tokio::test]
async fn test_hover_preview_during_range_selection() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Click a day to enter mid-selection state.
    let day_sel = ".rdrDateRangePickerWrapper .rdrDay:not(.rdrDayPassive):not(.rdrDayDisabled):not(.rdrDayEmpty)";
    f.click_builder(day_sel).click().await?;
    tokio::time::sleep(PAUSE).await;

    // Hover over another day to trigger preview.
    let days = f.query_selector_all(day_sel).await?;
    if days.len() > 8 {
        days[8].hover_builder().goto().await?;
        tokio::time::sleep(PAUSE).await;

        // Verify preview elements appear.
        let preview = f.query_selector(".rdrDayPreview").await?;
        assert!(preview.is_some(), "hover during mid-selection should show preview");
    }

    Ok(())
}

/// Validates that the library auto-injects CSS into the document head.
#[tokio::test]
async fn test_default_css_auto_injected() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the calendar wrapper has a non-transparent background from auto-injected CSS.
    let bg = f.eval::<String>(
        "window.getComputedStyle(document.querySelector('.rdrCalendarWrapper')).backgroundColor"
    ).await?;
    assert!(
        !bg.is_empty() && bg != "rgba(0, 0, 0, 0)",
        "auto-injected CSS should apply background to calendar"
    );

    Ok(())
}
