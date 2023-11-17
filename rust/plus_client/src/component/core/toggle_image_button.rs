//! Module for toggle_image_button leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::component;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::MaybeSignal;
use leptos::RwSignal;

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
pub struct ButtonData {
    /// Reference to the image
    pub image_ref: String,
    /// Label for the button
    pub label: MaybeSignal<String>,
}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Button with an image and label that as selected/deselected state
///
///   * **button_data** - Models the button to display and contains the state
///   * **writer** - Writer for click driven change
///   * **reader** - Reader for current state managed in parent
///   * **rw_signal** - Signal indicating new read required
///   * _return_ - View for toggle_image_button
#[component]
pub fn ToggleImageButton<W, R>(
    /// Models the button to display and contains the state
    button_data: ButtonData,
    /// Writer for click driven change
    writer: W,
    /// Reader for current state managed in parent
    reader: R,
    /// Signal indicating new read required
    rw_signal: RwSignal<()>,
) -> impl IntoView
where
    W: FnMut(ToggleState) + 'static,
    R: Fn() -> ToggleState + Clone + 'static,
{
    pub const SELF_CLASS: &str = "plus-tib";
    crate::log_component!("`ToggleImageButton`");
    // α <fn toggle_image_button>

    use crate::CssClasses;
    use crate::ToggleState;
    use leptos::SignalSet;
    use leptos::SignalWith;

    let mut writer = writer;
    let img_ref = button_data.image_ref.clone();

    let state_indicator_reader = reader.clone();
    let state_indicator = move || {
        rw_signal.track();
        let indicator_class = match state_indicator_reader() {
            ToggleState::Deselected => CssClasses::TibOff,
            ToggleState::Selected => CssClasses::TibOn,
        }
        .to_string();
        indicator_class
    };

    let click_handler = move |_| {
        let mut current = reader();
        current.toggle();
        writer(current);
        rw_signal.set(());
    };

    // ω <fn toggle_image_button>
    view! {
        <div class=SELF_CLASS>
            // α <plus-tib-view>

            <button on:click=click_handler>
                <img class=CssClasses::TibImg.as_str() src=img_ref/>
            </button>
            <div class=state_indicator></div>
            <label class=CssClasses::TibLbl.as_str()>{button_data.label}</label>

        // ω <plus-tib-view>
        </div>
    }
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl ButtonData {
    /// Initializer
    ///
    ///   * **image_ref** - Reference to the image
    ///   * **label** - Label for the button
    ///   * _return_ - The constructed instance
    pub fn new(image_ref: String, label: MaybeSignal<String>) -> Self {
        // α <new initialization>
        // ω <new initialization>
        Self { image_ref, label }
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
