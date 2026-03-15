use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;

/// Properties for the Overlay component.
#[derive(Properties, Clone, PartialEq)]
pub struct OverlayProps {
    /// Whether the overlay is visible.
    #[prop_or(false)]
    pub visible: bool,

    /// Callback when the overlay should close.
    #[prop_or_default]
    pub on_close: Callback<()>,

    /// Reference element to position relative to (the input container).
    #[prop_or_default]
    pub target_ref: NodeRef,

    /// Child content to render inside the overlay.
    #[prop_or_default]
    pub children: Children,

    /// Additional CSS class.
    #[prop_or_default]
    pub class_name: Option<String>,

    /// Close on outside click.
    #[prop_or(true)]
    pub close_on_outside_click: bool,

    /// Close on escape key.
    #[prop_or(true)]
    pub close_on_escape: bool,
}

/// Overlay component that renders content in a positioned popup.
#[function_component(Overlay)]
pub fn overlay(props: &OverlayProps) -> Html {
    // Create a ref for the overlay element to detect outside clicks.
    let overlay_ref = use_node_ref();
    let on_close = props.on_close.clone();
    let close_on_escape = props.close_on_escape;
    let close_on_outside = props.close_on_outside_click;

    // Register the outside click handler via a browser event listener.
    {
        let overlay_ref = overlay_ref.clone();
        let on_close = on_close.clone();
        let visible = props.visible;

        use_effect_with(visible, move |visible| {
            let mut stored_closure: Option<Closure<dyn Fn(web_sys::MouseEvent)>> = None;

            if *visible && close_on_outside {
                let overlay_ref = overlay_ref.clone();
                let on_close = on_close.clone();

                let closure = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
                    if let Some(target) = e.target() {
                        if let Some(target_el) = target.dyn_ref::<Element>() {
                            if let Some(overlay_el) = overlay_ref.cast::<Element>() {
                                if !overlay_el.contains(Some(target_el)) {
                                    on_close.emit(());
                                }
                            }
                        }
                    }
                });

                if let Some(win) = web_sys::window() {
                    let _ = win.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref());
                }

                stored_closure = Some(closure);
            }

            move || {
                if let Some(closure) = stored_closure.take() {
                    if let Some(win) = web_sys::window() {
                        let _ = win.remove_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref());
                    }
                }
            }
        });
    }

    // Register the escape key handler via a browser event listener.
    {
        let on_close = on_close.clone();
        let visible = props.visible;

        use_effect_with(visible, move |visible| {
            let mut stored_closure: Option<Closure<dyn Fn(web_sys::KeyboardEvent)>> = None;

            if *visible && close_on_escape {
                let on_close = on_close.clone();

                let closure = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
                    if e.key() == "Escape" {
                        on_close.emit(());
                    }
                });

                if let Some(win) = web_sys::window() {
                    let _ = win.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
                }

                stored_closure = Some(closure);
            }

            move || {
                if let Some(closure) = stored_closure.take() {
                    if let Some(win) = web_sys::window() {
                        let _ = win.remove_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
                    }
                }
            }
        });
    }

    // Return an empty fragment when the overlay is hidden.
    if !props.visible {
        return html! {};
    }

    // Extract optional extra CSS class from props.
    let extra_class = props.class_name.clone().unwrap_or_default();

    html! {
        <div
            ref={overlay_ref}
            class={classes!("rdrOverlay", extra_class)}
        >
            <div class="rdrOverlayContent">
                { for props.children.iter() }
            </div>
        </div>
    }
}
