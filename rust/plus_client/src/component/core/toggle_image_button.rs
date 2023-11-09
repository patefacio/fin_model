//! Module for toggle_image_button leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::component;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// The state of a toggle, either selected or deselected
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum ToggleState {
    #[default]
    /// Indicates the toggle state is selected
    Selected,
    /// Indicates the toggle state is deselected
    Deselected,
}

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Represents a single toggle button selection among many.
/// Includes the reference to the image and manages the selected/deselected
/// state of the button.
#[derive(Debug, Default, Clone)]
pub struct ButtonSelection {
    /// Reference to the image
    pub image_ref: String,
    /// Label for the button
    pub label: String,
    /// The toggle state of the button
    pub toggle_state: ToggleState,
}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Button with an image and label that as selected/deselected state
///
///   * **updatable** - Models the button to display and contains the state
///   * _return_ - View for toggle_image_button
#[component]
pub fn ToggleImageButton(
    /// Models the button to display and contains the state
    updatable: Updatable<ButtonSelection>,
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-tib";
    crate::log_component!("`ToggleImageButton`");
    // α <fn toggle_image_button>

    use crate::CssClasses;
    use crate::ToggleState;
    use leptos::create_signal;
    use leptos::SignalUpdate;
    use leptos::SignalWith;

    let img = updatable.value.image_ref.clone();
    let label = updatable.value.label.clone();
    let (state_read, state_write) = create_signal(updatable);

    let state_indicator = move || {
        {
            state_read.with(|updatable| {
                if updatable.value.toggle_state == ToggleState::Deselected {
                    CssClasses::TibOff
                } else {
                    CssClasses::TibOn
                }
            })
        }
        .to_string()
    };

    let click_handler = move |_| {
        state_write.update(|updatable| {
            updatable.update_and_then_signal(|button_selection| {
                button_selection.toggle_state =
                    if button_selection.toggle_state == ToggleState::Selected {
                        ToggleState::Deselected
                    } else {
                        ToggleState::Selected
                    }
            })
        })
    };

    // ω <fn toggle_image_button>
    view! {
        <div class=SELF_CLASS>
            // α <plus-tib-view>

            <button on:click=click_handler>
                <img class=CssClasses::TibImg.as_str() src=img/>
            </button>
            <div class=state_indicator></div>
            <label class=CssClasses::TibLbl.as_str()>{label}</label>

        // ω <plus-tib-view>
        </div>
    }
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl ButtonSelection {
    /// Initializer
    ///
    ///   * **image_ref** - Reference to the image
    ///   * **label** - Label for the button
    ///   * **toggle_state** - The toggle state of the button
    ///   * _return_ - The constructed instance
    pub fn new(image_ref: String, label: String, toggle_state: ToggleState) -> Self {
        Self {
            image_ref,
            label,
            toggle_state,
        }
    }
}

impl ToggleState {
    /// Toggles the state
    #[inline]
    pub fn toggle(&mut self) {
        // α <fn ToggleState::toggle>
        *self = match &self {
            ToggleState::Deselected => ToggleState::Selected,
            ToggleState::Selected => ToggleState::Deselected,
        }
        // ω <fn ToggleState::toggle>
    }
}

/// Unit tests for `toggle_image_button`
#[cfg(test)]
pub mod unit_tests {

    /// Test type ToggleState
    mod test_toggle_state {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn toggle() {
            // α <fn test ToggleState::toggle>

            let mut ts = ToggleState::Selected;
            ts.toggle();
            assert_eq!(ToggleState::Deselected, ts);
            ts.toggle();
            assert_eq!(ToggleState::Selected, ts);

            // ω <fn test ToggleState::toggle>
        }

        // α <mod-def test_toggle_state>
        use super::*;
        // ω <mod-def test_toggle_state>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def toggle_image_button>
// ω <mod-def toggle_image_button>
