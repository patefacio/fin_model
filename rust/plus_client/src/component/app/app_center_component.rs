//! Module for app_center_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::IntoAttribute;
use leptos::{component, view, IntoView};

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Main content
///
///   * _return_ - View for app_center_component
#[component]
pub fn AppCenterComponent() -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-acc";
    crate::log_component!("`AppCenterComponent`");
    // α <fn app_center_component>

    use crate::CoreComponentDisplay;

    // ω <fn app_center_component>
    view! {
        <div class=SELF_CLASS>
            // α <plus-acc-view>

            <CoreComponentDisplay/>

        // ω <plus-acc-view>
        </div>
    }
}

// α <mod-def app_center_component>
// ω <mod-def app_center_component>
