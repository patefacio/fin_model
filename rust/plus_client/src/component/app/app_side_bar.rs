//! Module for app_side_bar leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::IntoAttribute;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Top level side bar
///
///   * _return_ - View for app_side_bar
#[component]
pub fn AppSideBar() -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-asb";
    crate::log_component!("`AppSideBar`");
    // α <fn app_side_bar>

    // ω <fn app_side_bar>
    view! {
        <div class=SELF_CLASS>
            // α <plus-asb-view>

            <div></div>

        // ω <plus-asb-view>
        </div>
    }
}

// α <mod-def app_side_bar>
// ω <mod-def app_side_bar>
