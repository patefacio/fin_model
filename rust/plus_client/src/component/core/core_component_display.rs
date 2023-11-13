//! Module for core_component_display leptos function/component

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
/// Component that displays various core components.
/// Useful for seeing how they work and testing.
///
///   * _return_ - View for core_component_display
#[component]
pub fn CoreComponentDisplay() -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-ccd";
    crate::log_component!("`CoreComponentDisplay`");
    // α <fn core_component_display>

    use crate::CcdMisc;
    use crate::CcdMultiButton;
    use crate::CcdNumbers;
    use crate::CcdOneOf;
    use crate::CcdSelectLists;
    use crate::CcdYearsAndDate;
    use leptos::create_signal;

    let (last_update_read, last_update_write) = create_signal(String::default());

    // ω <fn core_component_display>
    view! {
        <div class=SELF_CLASS>
            // α <plus-ccd-view>

            <div class="ccd-top-notify">
                <h4>"Last Update"</h4>
                <p>{last_update_read}</p>
            </div>

            <div class="ccd-ctnr">

                <CcdMultiButton show_update=last_update_write/>

                <CcdOneOf show_update=last_update_write/>

                <CcdYearsAndDate show_update=last_update_write/>

                <CcdNumbers show_update=last_update_write/>

                <CcdSelectLists show_update=last_update_write/>

                <CcdMisc show_update=last_update_write/>
            </div>

        // ω <plus-ccd-view>
        </div>
    }
}

// α <mod-def core_component_display>
// ω <mod-def core_component_display>
