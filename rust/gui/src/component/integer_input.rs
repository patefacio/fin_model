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
) -> impl IntoView {
    // α <fn integer_input>

    use crate::utils::commify_number;
    use leptos::create_node_ref;
    use leptos::create_signal;
    use leptos::html::Input;
    use leptos::store_value;
    use leptos::IntoAttribute;
    use leptos::IntoClass;
    use leptos::IntoStyle;
    use leptos::SignalGet;
    use leptos::SignalSet;

    let node_ref = create_node_ref::<Input>(cx);

    let initial_value = if let Some(initial_value) = updatable.value {
        initial_value.to_string()
    } else {
        String::default()
    };

    let updatable = store_value(cx, updatable);
    let update_value = move || {
        if let Some(input_ref) = node_ref.get() {
            let mut value = input_ref.value();

            if value.is_empty() {
                updatable.update_value(|updatable| updatable.update_and_then_signal(|n| *n = None));
            } else {
                if let Some(last_letter) = value.chars().last() {
                    match last_letter {
                        'k' | 'm' | 'b' => {
                            value.pop();
                            match last_letter {
                                'k' => value.push_str("000"),
                                'm' => value.push_str("000000"),
                                'b' => value.push_str("000000000"),
                                _ => unreachable!(),
                            }
                        }
                        _ => (),
                    }
                }

                value = value.chars().filter(|c| c.is_ascii_digit()).collect();

                if let Ok(raw_num) = value.parse::<u32>() {
                    value = commify_number(raw_num);
                    updatable.update_value(|updatable| {
                        updatable.update_and_then_signal(|n| *n = Some(raw_num));
                    })
                }

                input_ref.set_value(&value);
            }
        }
    };

    view! { cx,
        <input
            class=move || {
                input_class.as_ref().cloned().unwrap_or_default()
            }
            node_ref=node_ref
            on:input=move |_| update_value()
            placeholder=placeholder.as_ref().cloned().unwrap_or_default()
            value=initial_value
            size=max_len + 2
            maxlength=max_len
            style:text-align=move || { if align_left { "left" } else { "right" } }
            type="text"
        />
    }

    // ω <fn integer_input>
}

// α <mod-def integer_input>
// ω <mod-def integer_input>
