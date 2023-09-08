//! Module for app_side_bar leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Top level side bar
///
///   * **cx** - Context
///   * _return_ - View for app_side_bar
#[component]
pub fn AppSideBar(
    /// Context
    cx: Scope,
) -> impl IntoView {
    // α <fn app_side_bar>

    view! {
        cx,
        <div class="app-side-bar">
        "SIDE"
        </div>
    }

    // ω <fn app_side_bar>
}

// α <mod-def app_side_bar>
// ω <mod-def app_side_bar>
