use wasm_bindgen::JsCast;

/// Default CSS for the date range picker.
pub const DEFAULT_CSS: &str = include_str!("date_range.css");

/// Identifier for the default style element in the DOM.
const STYLE_ELEMENT_ID: &str = "yew-date-range-default-styles";

/// CSS style injection utilities for the date range picker.
///
/// Provides static methods for injecting default and custom CSS
/// into the document head. Default styles are automatically injected
/// by all top-level components on first render.
pub struct StyleInjector;

impl StyleInjector {
    /// Injects the default CSS into the document head if not already present.
    ///
    /// Called automatically by all top-level components (Calendar, DatePicker,
    /// DateRange, DateRangePicker). Developers do NOT need to call this manually.
    ///
    /// To customize styles, either:
    /// - Override specific `.rdr*` classes in your own stylesheet
    /// - Call `inject_custom_css()` with your own CSS after injection
    pub fn inject_default_styles() {
        // Obtain the browser window object.
        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };

        // Obtain the document object.
        let document = match window.document() {
            Some(d) => d,
            None => return,
        };

        // Check if styles are already injected.
        if document.get_element_by_id(STYLE_ELEMENT_ID).is_some() {
            return;
        }

        // Create a style element.
        let style = match document.create_element("style") {
            Ok(el) => el,
            Err(_) => return,
        };

        // Set the element ID and CSS content.
        style.set_id(STYLE_ELEMENT_ID);
        style.set_text_content(Some(DEFAULT_CSS));

        // Cast to HtmlElement and append to the document head.
        if let Ok(html_el) = style.dyn_into::<web_sys::HtmlElement>() {
            let _ = html_el.set_attribute("data-source", "yew-date-range");
            if let Some(head) = document.head() {
                let _ = head.append_child(&html_el);
            }
        }
    }

    /// Injects custom CSS into the document head.
    ///
    /// Useful for theme overrides without replacing default styles.
    ///
    /// # Parameters
    ///
    /// - `css`: The CSS string to inject.
    /// - `id`: A unique ID for the style element to prevent duplicates.
    pub fn inject_custom_css(css: &str, id: &str) {
        // Obtain the browser window object.
        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };

        // Obtain the document object.
        let document = match window.document() {
            Some(d) => d,
            None => return,
        };

        // Check if this custom style is already injected.
        if document.get_element_by_id(id).is_some() {
            return;
        }

        // Create a style element with the custom CSS.
        let style = match document.create_element("style") {
            Ok(el) => el,
            Err(_) => return,
        };

        // Set the element ID and CSS content.
        style.set_id(id);
        style.set_text_content(Some(css));

        // Append to the document head.
        if let Some(head) = document.head() {
            let _ = head.append_child(&style);
        }
    }
}
