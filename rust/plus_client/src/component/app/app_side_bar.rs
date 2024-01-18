//! Module for app_side_bar leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::component;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Top level side bar
///
///   * _return_ - View for app_side_bar
#[component]
pub fn AppSideBar() -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-asb";
    let component_id = crate::component_id!("`AppSideBar`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
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
