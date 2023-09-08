//! Module for integer_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::NumericInput;
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::ReadSignal;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use std::ops::RangeInclusive;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a single integer
///
///   * **cx** - Context
///   * **updatable** - Called when input is updated.
///   * **input_class** - Class to decorate input element for styling
///   * **placeholder** - Placeholder shown if entry is empty.
///   * **size** - The size attribute, which one hopes would make the size of the
/// input field roughly that number of characters. But YMMV.
///   * **max_len** - Maximum length (digit count including any commas).
///   * **range** - Range of valid values for input.
///   * **non_negative** - If set, negative values are disallowed.
///   * **on_enter** - Called if user hits enter, passes current input value.
///   * **clear_input** - Signal requesting to clear the input.
///   * **align_left** - If set, numeric text aligned to left.
///   * **disabled** - Signal allowing the disabling of the input.
///   * **validator** - Called on update to check if value is valid.
///   * _return_ - View for integer_input
#[component]
pub fn IntegerInput(
    /// Context
    cx: Scope,
    /// Called when input is updated.
    updatable: Updatable<Option<u32>>,
    /// Class to decorate input element for styling
    #[prop(default=None)]
    input_class: Option<String>,
    /// Placeholder shown if entry is empty.
    #[prop(default=None)]
    placeholder: Option<String>,
    /// The size attribute, which one hopes would make the size of the
    /// input field roughly that number of characters. But YMMV.
    #[prop(default = 8)]
    size: u32,
    /// Maximum length (digit count including any commas).
    #[prop(default = 8)]
    max_len: u32,
    /// Range of valid values for input.
    #[prop(default=None)]
    range: Option<RangeInclusive<u32>>,
    /// If set, negative values are disallowed.
    #[prop(default = false)]
    non_negative: bool,
    /// Called if user hits enter, passes current input value.
    #[prop(default=None)]
    on_enter: Option<Box<dyn FnMut(String)>>,
    /// Signal requesting to clear the input.
    #[prop(default=None)]
    clear_input: Option<ReadSignal<()>>,
    /// If set, numeric text aligned to left.
    #[prop(default = false)]
    align_left: bool,
    /// Signal allowing the disabling of the input.
    #[prop(default=None)]
    disabled: Option<ReadSignal<bool>>,
    /// Called on update to check if value is valid.
    #[prop(default=None)]
    validator: Option<Box<dyn FnMut(i32) -> bool>>,
) -> impl IntoView {
    // α <fn integer_input>

    let float_value = updatable.value.map(|value| value as f64);
    let mut updatable = updatable;
    let numeric_updatable = Updatable::new(float_value, move |new_float_value| {
        let actual_value = new_float_value.map(|v| v);
        if actual_value.is_some() {
            updatable.update_and_then_signal(|current_value| {
                *current_value = actual_value.map(|v| v as u32)
            });
        };
    });

    let casting_validator = validator.map(|mut validator| {
        let new_validator = move |v: f64| validator(v as i32);
        Box::new(new_validator) as Box<dyn FnMut(f64) -> bool>
    });

    view! { cx,
        <NumericInput
            input_class=input_class
            updatable=numeric_updatable
            on_enter=on_enter
            non_negative=non_negative
            placeholder=placeholder
            max_len=max_len
            clear_input=clear_input
            size=size
            range=range.map(|range| *range.start() as f64..=*range.end() as f64)
            no_decimal=true
            align_left=align_left
            validator=casting_validator
            disabled=disabled
        />
    }

    // ω <fn integer_input>
}

// α <mod-def integer_input>
// ω <mod-def integer_input>
