////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::utils::html_tag::HtmlTag;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::Element;

// α <mod-def element_sugar>

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

pub fn element_from_event<T: JsCast>(ev: &T) -> Element {
    ev.unchecked_ref::<web_sys::Event>()
        .target()
        .unwrap_throw()
        .unchecked_into::<web_sys::Element>()
}

// ω <mod-def element_sugar>
