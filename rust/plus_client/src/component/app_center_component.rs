//! Module for app_center_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Main content
///
///   * _return_ - View for app_center_component
#[component]
pub fn AppCenterComponent() -> impl IntoView {
    // α <fn app_center_component>

    use crate::ComponentDisplayComponent;
    use crate::DossierComponent;

    view! {
        <div class="app-center">
            <DossierComponent/>
            <hr/>
            <div style="height: 40px;"></div>
        // <h3>"Following content is to show features of various components"</h3>
        // <ComponentDisplayComponent/>
        </div>
    }

    // ω <fn app_center_component>
}

// α <mod-def app_center_component>
// ω <mod-def app_center_component>
