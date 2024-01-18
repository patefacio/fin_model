//! Module for toggle_image_button leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::ButtonData;
use crate::ToggleState;
use leptos::component;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::RwSignal;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Button control with an image and label that as selected/deselected state
///
///   * **button_data** - Models the button to display and contains the state
///   * **rw_signal** - Writer for click driven change - reads only for initial state
///   * _return_ - View for toggle_image_button
#[component]
pub fn ToggleImageButton(
    /// Models the button to display and contains the state
    button_data: ButtonData,
    /// Writer for click driven change - reads only for initial state
    rw_signal: RwSignal<ToggleState>,
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-tib";
    let component_id = crate::component_id!("`ToggleImageButton`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn toggle_image_button>

    use crate::ToggleImageButtonControl;
    use leptos::create_rw_signal;
    use leptos::store_value;
    use leptos::SignalGetUntracked;
    use leptos::SignalSet;

    let initial_state = rw_signal.get_untracked();
    let toggle_state_stored_value = store_value(initial_state);

    let reader = move || toggle_state_stored_value.get_value();

    let writer = move |toggle_state| {
        toggle_state_stored_value.set_value(toggle_state);
        rw_signal.set(toggle_state);
    };

    let rw_signal = create_rw_signal(());

    // ω <fn toggle_image_button>
    view! {
        <div class=SELF_CLASS>
            // α <plus-tib-view>

            <ToggleImageButtonControl button_data writer rw_signal reader/>

        // ω <plus-tib-view>
        </div>
    }
}

// α <mod-def toggle_image_button>
// ω <mod-def toggle_image_button>
