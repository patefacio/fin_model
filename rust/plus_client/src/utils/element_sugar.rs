//! Helper functions for dealing with elements

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::HtmlTag;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::Element;

// α <mod-def element_sugar>

/// Search up the dom from `element` for specified `html_tag`
pub fn find_element_up(element: Element, html_tag: HtmlTag) -> Option<Element> {
    let mut optional_element = Some(element);
    while let Some(element) = optional_element.as_ref() {
        if element.tag_name() == html_tag.as_str() {
            break;
        } else {
            optional_element = element.parent_element();
        }
    }
    optional_element
}

/// Given an event, return the element the event targeted
pub fn element_from_event<T: JsCast>(ev: &T) -> Element {
    ev.unchecked_ref::<web_sys::Event>()
        .target()
        .unwrap_throw()
        .unchecked_into::<web_sys::Element>()
}

// ω <mod-def element_sugar>
