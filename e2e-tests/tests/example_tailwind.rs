//! End-to-end browser tests for the Tailwind CSS example application.
//!
//! Verifies rendered DOM structure, dark/light theme toggling, user interactions,
//! and visual state across all 10 example sections using Playwright with Chromium.

mod common;

use common::SharedFixture;
use playwright::api::page::Page;
use std::time::Duration;
use tokio::sync::OnceCell;

type E = Box<dyn std::error::Error + Send + Sync>;
const PAUSE: Duration = Duration::from_millis(350);

static FIXTURE: OnceCell<SharedFixture> = OnceCell::const_new();

/// Creates a fresh browser page navigated to the Tailwind example dist.
async fn page() -> Result<Page, E> {
    common::new_page(&FIXTURE, "example-tailwind/dist").await
}

/// Validates that the page loads with title, description, and all 10 sections.
#[tokio::test]
async fn test_page_heading_and_sections() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the page title.
    let heading = f.text_content("h1", None).await?;
    assert_eq!(heading.as_deref(), Some("yew-date-range"));

    // Verify all 10 sections are present.
    let sections = f.query_selector_all("section").await?;
    assert_eq!(sections.len(), 10, "tailwind example should have 10 sections");

    // Verify each section has an h2 heading.
    let h2s = f.query_selector_all("section h2").await?;
    assert_eq!(h2s.len(), 10, "each section needs a heading");

    // Verify the mode label shows Light or Dark.
    let mode_text = f.text_content(".font-medium", None).await?;
    let mode_str = mode_text.unwrap_or_default();
    assert!(
        mode_str.contains("Light") || mode_str.contains("Dark"),
        "should show current mode label"
    );

    Ok(())
}

/// Validates that the footer includes a GitHub link.
#[tokio::test]
async fn test_footer_present() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the footer text references the project.
    let footer = f.text_content(".text-xs.text-center", None).await?;
    assert!(
        footer.as_deref().unwrap_or("").contains("Tailwind"),
        "footer should mention Tailwind CSS Example"
    );

    Ok(())
}

/// Validates that the dark mode toggle switches the class on the html element.
#[tokio::test]
async fn test_dark_mode_toggle_class_on_html() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the initial state is light.
    let cls = f.get_attribute("html", "class", None).await?;
    assert_eq!(cls.as_deref(), Some("light"), "should start in light mode");

    // Toggle to dark mode.
    f.click_builder("button:has-text('Toggle')").click().await?;
    tokio::time::sleep(PAUSE).await;

    let cls_dark = f.get_attribute("html", "class", None).await?;
    assert_eq!(cls_dark.as_deref(), Some("dark"), "should switch to dark");

    // Toggle back to light mode.
    f.click_builder("button:has-text('Toggle')").click().await?;
    tokio::time::sleep(PAUSE).await;

    let cls_back = f.get_attribute("html", "class", None).await?;
    assert_eq!(cls_back.as_deref(), Some("light"), "should restore light");

    Ok(())
}

/// Validates that the toggle button label changes between "Toggle Dark" and "Toggle Light".
#[tokio::test]
async fn test_dark_mode_toggle_button_label_changes() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the button says "Toggle Dark" in light mode.
    let btn_text = f.text_content("button:has-text('Toggle')", None).await?;
    assert!(
        btn_text.as_deref().unwrap_or("").contains("Dark"),
        "in light mode, button should say Toggle Dark"
    );

    // Switch to dark mode.
    f.click_builder("button:has-text('Toggle')").click().await?;
    tokio::time::sleep(PAUSE).await;

    // Verify the button says "Toggle Light" in dark mode.
    let btn_text_dark = f.text_content("button:has-text('Toggle')", None).await?;
    assert!(
        btn_text_dark.as_deref().unwrap_or("").contains("Light"),
        "in dark mode, button should say Toggle Light"
    );

    Ok(())
}

/// Validates that dark mode applies dark background Tailwind classes to the page.
#[tokio::test]
async fn test_dark_mode_css_overrides_apply() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Switch to dark mode.
    f.click_builder("button:has-text('Toggle')").click().await?;
    tokio::time::sleep(PAUSE).await;

    // Verify the calendar remains visible in dark mode.
    let cal = f.query_selector(".rdrCalendarWrapper").await?;
    assert!(cal.is_some(), "calendar should remain visible in dark mode");

    // Verify dark background Tailwind classes are applied.
    let bg_cls = f.get_attribute("body > div", "class", None).await?;
    let bg_str = bg_cls.unwrap_or_default();
    assert!(
        bg_str.contains("bg-slate-900") || bg_str.contains("dark"),
        "dark mode should apply dark bg classes, got: {bg_str}"
    );

    Ok(())
}

/// Validates that the DateRangePicker (section 1) renders with sidebar, two months, and navigation.
#[tokio::test]
async fn test_daterangepicker_structure() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the wrapper and horizontal layout.
    let wrapper = f.query_selector(".rdrDateRangePickerWrapper").await?;
    assert!(wrapper.is_some(), "DateRangePicker wrapper missing");

    let cls = f.get_attribute(".rdrDateRangePickerWrapper", "class", None).await?;
    assert!(
        cls.as_deref().unwrap_or("").contains("rdrDateRangePickerHorizontal"),
        "should use horizontal layout"
    );

    // Verify the sidebar with 6 predefined ranges.
    let sidebar = f.query_selector(".rdrDefinedRangesWrapper").await?;
    assert!(sidebar.is_some(), "sidebar missing");

    let range_btns = f.query_selector_all(".rdrStaticRange").await?;
    assert_eq!(range_btns.len(), 6, "6 predefined ranges expected");

    // Verify two months and navigation buttons.
    let months = f.query_selector_all(".rdrDateRangePickerWrapper .rdrMonth").await?;
    assert_eq!(months.len(), 2, "should show 2 months");

    let prev = f.query_selector(".rdrPrevButton").await?;
    let next = f.query_selector(".rdrNextButton").await?;
    assert!(prev.is_some() && next.is_some(), "nav buttons missing");

    Ok(())
}

/// Validates that the initial range (Mar 10-20) is highlighted with edge and in-range classes.
#[tokio::test]
async fn test_daterangepicker_initial_range_highlighted() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify start/end edge and in-range classes.
    let start_edges = f
        .query_selector_all(".rdrDateRangePickerWrapper .rdrDayStartEdge")
        .await?;
    assert!(!start_edges.is_empty(), "range start edge missing");

    let end_edges = f
        .query_selector_all(".rdrDateRangePickerWrapper .rdrDayEndEdge")
        .await?;
    assert!(!end_edges.is_empty(), "range end edge missing");

    let in_range = f
        .query_selector_all(".rdrDateRangePickerWrapper .rdrDayInRange")
        .await?;
    assert!(!in_range.is_empty(), "in-range days missing");

    // Verify the selection info displays the initial dates.
    let info = f.text_content("section:nth-of-type(1) strong", None).await?;
    let info_str = info.unwrap_or_default();
    assert!(
        info_str.contains("2026-03-10") && info_str.contains("2026-03-20"),
        "initial selection should show Mar 10-20, got: {info_str}"
    );

    Ok(())
}

/// Validates that clicking the Today sidebar range updates the selection.
#[tokio::test]
async fn test_defined_range_updates_selection() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Record the info before clicking.
    let info_before = f.text_content("section:nth-of-type(1) strong", None).await?;

    // Click the "Today" predefined range.
    f.click_builder(".rdrStaticRange:first-child").click().await?;
    tokio::time::sleep(PAUSE).await;

    // Verify the info changed and the selected class is applied.
    let info_after = f.text_content("section:nth-of-type(1) strong", None).await?;
    assert_ne!(info_before, info_after, "Today should change the selection");

    let selected = f.query_selector(".rdrStaticRangeSelected").await?;
    assert!(selected.is_some(), "clicked range should be marked selected");

    Ok(())
}

/// Validates that the DateRange calendar-only (section 2) renders with date display.
#[tokio::test]
async fn test_daterange_calendar_only_renders() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify standalone DateRange wrappers and date display exist.
    let wrappers = f.query_selector_all(".rdrDateRangeWrapper").await?;
    assert!(!wrappers.is_empty(), "DateRange calendar-only should render");

    let displays = f
        .query_selector_all(".rdrDateRangeWrapper .rdrDateDisplayWrapper")
        .await?;
    assert!(!displays.is_empty(), "calendar-only should show date display");

    Ok(())
}

/// Validates that the vertical layout (section 3) renders with week numbers.
#[tokio::test]
async fn test_vertical_layout_with_week_numbers() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the vertical layout class.
    let vertical = f.query_selector(".rdrMonthsVertical").await?;
    assert!(vertical.is_some(), "should have a vertical months container");

    // Verify week numbers are rendered.
    let week_nums = f.query_selector_all(".rdrWeekNumber").await?;
    assert!(week_nums.len() >= 5, "vertical example should show week numbers");

    Ok(())
}

/// Validates that the inline single-date picker (section 4) renders with Today/Clear buttons.
#[tokio::test]
async fn test_datepicker_inline_renders() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify inline picker and calendar.
    let inline = f.query_selector(".rdrDatePickerInline").await?;
    assert!(inline.is_some(), "inline DatePicker missing");

    let cal = f.query_selector(".rdrDatePickerInline .rdrCalendarWrapper").await?;
    assert!(cal.is_some(), "inline picker should have a calendar");

    // Verify action buttons.
    let today_btn = f.query_selector(".rdrDatePickerTodayButton").await?;
    assert!(today_btn.is_some(), "Today button missing");

    let clear_btn = f.query_selector(".rdrDatePickerClearButton").await?;
    assert!(clear_btn.is_some(), "Clear button missing");

    Ok(())
}

/// Validates that the popup range picker (section 5) has an input with the correct placeholder.
#[tokio::test]
async fn test_popup_range_picker_renders() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the popup pickers exist.
    let popups = f.query_selector_all(".rdrDatePickerPopup").await?;
    assert!(popups.len() >= 2, "should have popup range + popup multiple pickers");

    // Verify the range picker input has the correct placeholder.
    let placeholder = f
        .get_attribute("section:nth-of-type(5) .rdrDatePickerInput", "placeholder", None)
        .await?;
    assert_eq!(
        placeholder.as_deref(),
        Some("Click to select range"),
        "should show custom placeholder"
    );

    // Verify the calendar icon is visible.
    let icon = f.query_selector("section:nth-of-type(5) .rdrDatePickerIcon").await?;
    assert!(icon.is_some(), "popup range picker should show calendar icon");

    Ok(())
}

/// Validates that the popup multiple-date picker (section 6) shows pre-selected dates in the input.
#[tokio::test]
async fn test_popup_multiple_initial_value() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Read the input value via JS property.
    let val_str: String = f
        .evaluate_on_selector(
            "section:nth-of-type(6) .rdrDatePickerInput",
            "el => el.value",
            None::<()>,
        )
        .await?;
    assert!(
        val_str.contains("2026") && val_str.contains(","),
        "input should show comma-separated dates, got: {val_str}"
    );

    Ok(())
}

/// Validates that the 24h time picker (section 7) renders with 3 fields and no AM/PM.
#[tokio::test]
async fn test_time_picker_24h_structure() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the TimePicker is present in section 7.
    let tp = f.query_selector("section:nth-of-type(7) .rdrTimePicker").await?;
    assert!(tp.is_some(), "TimePicker component missing");

    // Verify 3 fields (hours, minutes, seconds) scoped to section 7.
    let fields = f
        .query_selector_all("section:nth-of-type(7) .rdrTimePickerField")
        .await?;
    assert_eq!(fields.len(), 3, "should show hour, minute, second fields");

    // Verify no AM/PM toggle in 24h mode scoped to section 7.
    let period = f
        .query_selector("section:nth-of-type(7) .rdrTimePickerPeriodButton")
        .await?;
    assert!(period.is_none(), "24h mode should not show AM/PM toggle");

    Ok(())
}

/// Validates that the constrained range (section 8) has disabled days from min/max/disabled_ranges.
#[tokio::test]
async fn test_constrained_range_has_disabled_days() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the 8th section has selection info.
    let section_info = f.text_content("section:nth-of-type(8) strong", None).await?;
    assert!(
        section_info.is_some(),
        "constrained range section should have selection info"
    );

    // Verify there are disabled days from the constraints.
    let disabled = f.query_selector_all("section:nth-of-type(8) .rdrDayDisabled").await?;
    assert!(!disabled.is_empty(), "constrained range should have disabled days");

    Ok(())
}

/// Validates that the 12h AM/PM time picker (section 9) shows an AM/PM toggle button.
#[tokio::test]
async fn test_time_picker_12h_ampm_toggle() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the AM/PM period button is present.
    let period_btn = f
        .query_selector("section:nth-of-type(9) .rdrTimePickerPeriodButton")
        .await?;
    assert!(period_btn.is_some(), "12h time picker should have AM/PM toggle button");

    // Verify the info contains AM or PM.
    let info = f.text_content("section:nth-of-type(9) strong", None).await?;
    let info_str = info.unwrap_or_default();
    assert!(
        info_str.contains("AM") || info_str.contains("PM"),
        "12h time picker should show AM or PM, got: {info_str}"
    );

    Ok(())
}

/// Validates that the disabled state (section 10) renders with a read-only label.
#[tokio::test]
async fn test_disabled_state_renders() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the 10th section has an inline picker.
    let disabled_picker = f.query_selector("section:nth-of-type(10) .rdrDatePickerInline").await?;
    assert!(
        disabled_picker.is_some(),
        "disabled state section should have an inline picker"
    );

    // Verify the info mentions read-only.
    let info = f.text_content("section:nth-of-type(10) strong", None).await?;
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
    assert_ne!(month_before, month_fwd, "next should advance month");

    // Navigate backward.
    f.click_builder(".rdrPrevButton").click().await?;
    tokio::time::sleep(PAUSE).await;
    let month_back = f.text_content(".rdrMonthPickerTitle", None).await?;
    assert_eq!(month_before, month_back, "prev should return to original");

    Ok(())
}

/// Validates that day cells have correct weekend, today, and passive state classes.
#[tokio::test]
async fn test_day_cell_states() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify weekend, today, and passive day classes.
    let weekends = f.query_selector_all(".rdrDayWeekend").await?;
    assert!(!weekends.is_empty(), "weekend days should be flagged");

    let today = f.query_selector(".rdrDayToday").await?;
    assert!(today.is_some(), "today should be marked");

    let passive = f.query_selector_all(".rdrDayPassive").await?;
    assert!(!passive.is_empty(), "passive days expected");

    Ok(())
}

/// Validates that day cells have aria-label attributes for accessibility.
#[tokio::test]
async fn test_day_aria_labels() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify day buttons have aria-label.
    let aria = f
        .get_attribute(".rdrDay:not(.rdrDayEmpty):not(.rdrDayPassive)", "aria-label", None)
        .await?;
    assert!(
        aria.is_some() && !aria.as_deref().unwrap_or("").is_empty(),
        "day cells need aria-label"
    );

    Ok(())
}

/// Validates that the library auto-injects CSS into the document head.
#[tokio::test]
async fn test_css_auto_injected() -> Result<(), E> {
    let page = page().await?;
    let f = page.main_frame();

    // Verify the calendar wrapper has non-transparent background from auto-injected CSS.
    let bg: String = f
        .eval("window.getComputedStyle(document.querySelector('.rdrCalendarWrapper')).backgroundColor")
        .await?;
    assert!(
        !bg.is_empty() && bg != "rgba(0, 0, 0, 0)",
        "auto-injected CSS should style the calendar, got: {bg}"
    );

    Ok(())
}
