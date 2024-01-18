//! Module for toggle_image_button_control leptos function/component

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
/// Enumerates button image support for [ToggleImageButton]
#[derive(Debug, Clone)]
pub enum ButtonImage {
    /// Reference to image
    ImageRef(String),
    /// String of unicode
    ImageUnicode(String),
    /// Pair of references for `on` and `off` images
    ImageRefPair {
        /// Reference to _selected_ image
        selected: String,
        /// Reference to _deselected_ image
        deselected: String,
    },
}

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
    /// The selected image(s)
    pub button_image: ButtonImage,
    /// Label for the button
    pub label: MaybeSignal<String>,
}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Button control with an image and label that as selected/deselected state
/// providing user control over the state.
///
///   * **button_data** - Models the button to display and contains the state
///   * **writer** - Writer for click driven change
///   * **reader** - Reader for current state managed in parent
///   * **rw_signal** - Signal indicating new read required
///   * _return_ - View for toggle_image_button_control
#[component]
pub fn ToggleImageButtonControl<W, R>(
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
    pub const SELF_CLASS: &str = "plus-tibc";
    let component_id = crate::component_id!("`ToggleImageButtonControl`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn toggle_image_button_control>

    use crate::ClientCssClasses;
    use crate::ToggleState;
    use leptos::store_value;
    use leptos::SignalSet;
    use leptos::SignalWith;
    use leptos::SignalWithUntracked;

    let mut writer = writer;

    let has_label = button_data.label.with_untracked(|label| !label.is_empty());

    let state_indicator_reader = reader.clone();
    let state_indicator = move || {
        rw_signal.track();
        let indicator_class = match state_indicator_reader() {
            ToggleState::Deselected => ClientCssClasses::TibOff,
            ToggleState::Selected => ClientCssClasses::TibOn,
        }
        .to_string();
        indicator_class
    };

    let reader_stored_value = store_value(reader);

    let click_handler = move |_| {
        let mut current_toggle_state = reader_stored_value.with_value(|reader| reader());
        current_toggle_state.toggle();
        writer(current_toggle_state);
        rw_signal.set(());
    };

    let image = move || {
        let current_toggle_state = reader_stored_value.with_value(|reader| reader());
        match &button_data.button_image {
            ButtonImage::ImageRef(image_ref) => {
                view! { <img class=ClientCssClasses::TibImg.as_str() src=image_ref.clone()/> }
                    .into_view()
            }
            ButtonImage::ImageUnicode(image_unicode) => view! { {image_unicode} }.into_view(),
            ButtonImage::ImageRefPair {
                selected,
                deselected,
            } => match current_toggle_state {
                ToggleState::Selected => {
                    view! { <img class=ClientCssClasses::TibImg.as_str() src=selected.clone()/> }
                        .into_view()
                }
                ToggleState::Deselected => {
                    view! { <img class=ClientCssClasses::TibImg.as_str() src=deselected.clone()/> }
                        .into_view()
                }
            },
        }
    };

    let label = {
        if has_label {
            view! { <label class=ClientCssClasses::TibLbl.as_str()>{button_data.label}</label> }
                .into_view()
        } else {
            ().into_view()
        }
    };

    // ω <fn toggle_image_button_control>
    view! {
        <div class=SELF_CLASS>
            // α <plus-tibc-view>

            <div>
                <button on:click=click_handler>{image}</button>
                <div class=state_indicator></div>
            </div>

            {label}

        // ω <plus-tibc-view>
        </div>
    }
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl ButtonData {
    /// Initializer
    ///
    ///   * **button_image** - The selected image(s)
    ///   * **label** - Label for the button
    ///   * _return_ - The constructed instance
    pub fn new(button_image: ButtonImage, label: MaybeSignal<String>) -> Self {
        Self {
            button_image,
            label,
        }
    }
}

impl ButtonImage {
    /// Create from an image reference
    ///
    ///   * **image_ref** - Reference to image
    ///   * _return_ - The image to display
    #[inline]
    pub fn from_image_ref(image_ref: &str) -> Self {
        // α <fn ButtonImage::from_image_ref>
        ButtonImage::ImageRef(image_ref.into())
        // ω <fn ButtonImage::from_image_ref>
    }

    /// Create from a pair of _selected_ and _deselected_ image references
    ///
    ///   * **selected** - Reference to image in _selected_ state
    ///   * **deselected** - Reference to image in _deselected_ state
    ///   * _return_ - The images to display
    #[inline]
    pub fn from_image_ref_pair(selected: &str, deselected: &str) -> Self {
        // α <fn ButtonImage::from_image_ref_pair>
        ButtonImage::ImageRefPair {
            selected: selected.into(),
            deselected: deselected.into(),
        }
        // ω <fn ButtonImage::from_image_ref_pair>
    }

    /// Create from a pair of _selected_ and _deselected_ image references
    ///
    ///   * **image_unicode** - Reference _image_ which may simply be a string with unicode/emoji(s)
    ///   * _return_ - The image to display
    #[inline]
    pub fn from_unicode(image_unicode: &str) -> Self {
        // α <fn ButtonImage::from_unicode>
        ButtonImage::ImageUnicode(image_unicode.into())
        // ω <fn ButtonImage::from_unicode>
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

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl Default for ButtonImage {
    /// A trait for giving a type a useful default value.
    ///
    ///   * _return_ - The new default instance
    fn default() -> Self {
        // α <fn Default::default for ButtonImage>
        use plus_lookup::SystemUnicodes;
        ButtonImage::ImageUnicode(SystemUnicodes::PlusauriUser.as_unicode().to_string())
        // ω <fn Default::default for ButtonImage>
    }
}

/// Unit tests for `toggle_image_button_control`
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

// α <mod-def toggle_image_button_control>
// ω <mod-def toggle_image_button_control>
