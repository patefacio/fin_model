//! Module for app_center_component leptos function/component

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
/// Main content
///
///   * _return_ - View for app_center_component
#[component]
pub fn AppCenterComponent() -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-acc";
    let component_id = crate::component_id!("`AppCenterComponent`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
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
