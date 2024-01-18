//! Module for slider_with_numeric_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Modification;
use crate::Updatable;
use leptos::component;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::MaybeSignal;
use leptos::ReadSignal;
use std::ops::RangeInclusive;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Combines a slider with a [NumericInput](crate::NumericInput) or
/// [IntegerInput](crate::IntegerInput) that provides for a single
/// number controllable with a convenient slider or an easy target
/// for paste operation or granular up/down movement.
///
///   * **updatable** - Signal updated as numeric input is updated.
///   * **label** - Label for the number
///   * **slider_id** - Id for the slider
///   * **range** - Range of valid values for input.
///   * **step** - Step for the slider
///   * **numeric_input_class** - Class to decorate input element for styling
///   * **slider_input_class** - Class to decorate input element for styling
///   * **modification** - Optional modification (e.g. suffix/prefix)
///   * **non_negative** - If set, negative values are disallowed.
///   * **align_left** - If set, numeric text aligned to left.
///   * **placeholder** - Placeholder shown if entry is empty.
///   * **size** - The size attribute, which one hopes would make the size of the
/// input field roughly that number of characters. But YMMV.
///   * **max_len** - The maximum number of characters for the input.
///   * **on_enter** - Called if user hits enter, passes current input value.
///   * **clear_input** - Signal requesting to clear the input.
///   * **no_decimal** - Indicates decimals disallowed.
///   * **disabled** - Signal allowing the disabling of the select button.
///   * **validator** - Called on update to check if value is valid.
///   * _return_ - View for slider_with_numeric_input
#[component]
pub fn SliderWithNumericInput(
    /// Signal updated as numeric input is updated.
    updatable: Updatable<f64>,
    /// Label for the number
    #[prop(default=MaybeSignal::Static(String::default()))]
    label: MaybeSignal<String>,
    /// Id for the slider
    slider_id: String,
    /// Range of valid values for input.
    #[prop(default=1.0..=100.0)]
    range: RangeInclusive<f64>,
    /// Step for the slider
    #[prop(default = 1.0)]
    step: f64,
    /// Class to decorate input element for styling
    #[prop(default=None)]
    numeric_input_class: Option<String>,
    /// Class to decorate input element for styling
    #[prop(default=None)]
    slider_input_class: Option<String>,
    /// Optional modification (e.g. suffix/prefix)
    #[prop(default=None)]
    modification: Option<Modification>,
    /// If set, negative values are disallowed.
    #[prop(default = false)]
    non_negative: bool,
    /// If set, numeric text aligned to left.
    #[prop(default = false)]
    align_left: bool,
    /// Placeholder shown if entry is empty.
    #[prop(default=MaybeSignal::Static(String::from("value")), into)]
    placeholder: MaybeSignal<String>,
    /// The size attribute, which one hopes would make the size of the
    /// input field roughly that number of characters. But YMMV.
    #[prop(default = 9)]
    size: u32,
    /// The maximum number of characters for the input.
    #[prop(default = 12)]
    max_len: u32,
    /// Called if user hits enter, passes current input value.
    #[prop(default=None)]
    on_enter: Option<Box<dyn FnMut(String)>>,
    /// Signal requesting to clear the input.
    #[prop(default=None)]
    clear_input: Option<ReadSignal<()>>,
    /// Indicates decimals disallowed.
    #[prop(default = false)]
    no_decimal: bool,
    /// Signal allowing the disabling of the select button.
    #[prop(into, optional)]
    disabled: MaybeSignal<bool>,
    /// Called on update to check if value is valid.
    #[prop(default=None)]
    validator: Option<Box<dyn FnMut(f64) -> bool>>,
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-swni";
    let component_id = crate::component_id!("`SliderWithNumericInput`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn slider_with_numeric_input>

    use crate::ClientCssClasses;
    use crate::NumericInput;
    use leptos::create_node_ref;
    use leptos::create_signal;
    use leptos::store_value;
    use leptos::SignalSet;
    use leptos::SignalWith;
    use leptos_dom::html::Input;

    let slider_input_node_ref = create_node_ref::<Input>();
    let initial_value = updatable.value;
    let (slider_number_read, slider_number_write) = create_signal(initial_value);
    let (update_read, update_write) = create_signal(());
    let updatable_stored_value = store_value(updatable);

    // ω <fn slider_with_numeric_input>
    view! {
        <div class=SELF_CLASS>
            // α <plus-swni-view>

            <label>
                <input
                    node_ref=slider_input_node_ref
                    class=ClientCssClasses::SwniSlider.as_str()
                    class=slider_input_class
                    type="range"
                    id=slider_id.clone()
                    name=slider_id.clone()
                    min=range.start().to_string()
                    max=range.end().to_string()
                    step=step.to_string()
                    on:input=move |_| {
                        let mut new_number = None;
                        if let Some(input_ref) = slider_input_node_ref.get() {
                            new_number = input_ref.value().parse::<f64>().ok();
                        }
                        if let Some(new_number) = new_number {
                            slider_number_write.set(new_number);
                        }
                    }

                    value=initial_value
                    prop:value=move || {
                        slider_number_read.track();
                        update_read.track();
                        updatable_stored_value.with_value(|updatable| updatable.value)
                    }
                />

                {label}
                <NumericInput
                    updatable=Updatable::new(
                        Some(initial_value),
                        move |new_number| {
                            let new_number = new_number.unwrap_or_default();
                            updatable_stored_value
                                .update_value(|updatable| {
                                    updatable.update_and_then_signal(|value| *value = new_number)
                                });
                            update_write.set(());
                        },
                    )

                    input_class=numeric_input_class
                    modification=modification
                    non_negative=non_negative
                    align_left=align_left
                    placeholder=placeholder
                    size=size
                    max_len=max_len
                    range=Some(range)
                    on_enter=on_enter
                    clear_input=clear_input
                    no_decimal=no_decimal
                    disabled=disabled
                    validator=validator
                    parent_override=Some(slider_number_read)
                />
            </label>

        // ω <plus-swni-view>
        </div>
    }
}

// α <mod-def slider_with_numeric_input>
// ω <mod-def slider_with_numeric_input>
