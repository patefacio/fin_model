//! Module for integer_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::IntegerClamp;
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
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
///   * **updatable** - Signal updated integer value
///   * **input_class** - Class to decorate input element for styling
///   * **placeholder** - Placeholder shown if entry is empty.
///   * **max_len** - Maximum length (digit count including any commas).
///   * **include_comma** - If true commifies the number
///   * **range** - Range of valid values for input.
///   * **live_clamp** - If set will force values to be within the range as they are typed.
///   * _return_ - View for integer_input
#[component]
pub fn IntegerInput(
    /// Context
    cx: Scope,
    /// Signal updated integer value
    updatable: Updatable<Option<u32>>,
    /// Class to decorate input element for styling
    #[prop(default=None)]
    input_class: Option<String>,
    /// Placeholder shown if entry is empty.
    #[prop(default=None)]
    placeholder: Option<String>,
    /// Maximum length (digit count including any commas).
    #[prop(default = 8)]
    max_len: u32,
    /// If true commifies the number
    #[prop(default = true)]
    include_comma: bool,
    /// Range of valid values for input.
    #[prop(default=None)]
    range: Option<RangeInclusive<u32>>,
    /// If set will force values to be within the range as they are typed.
    #[prop(default = false)]
    live_clamp: bool,
) -> impl IntoView {
    // α <fn integer_input>
    
    use crate::ParsedNum;
    use crate::utils::commify_number;
    use leptos::create_node_ref;
    use leptos::create_signal;
    use leptos::html::Input;
    use leptos::IntoAttribute;
    use leptos::IntoClass;
    use leptos::SignalGet;
    use leptos::SignalSet;
    use leptos::*;

    let is_in_range = true;

    let integer_clamp = if let Some(range) = range.as_ref().cloned() {
        if live_clamp {
            Some(IntegerClamp::new(range))
        } else {
            None
        }
    } else {
        None
    };

    struct IntegerInputData {
        updatable: Updatable<Option<u32>>,
        input_class: Option<String>,
        placeholder: Option<String>,
        range: Option<RangeInclusive<u32>>,
    }

    let integer_input_data = store_value(
        cx,
        IntegerInputData {
            updatable,
            input_class,
            placeholder,
            range,
        },
    );

    let node_ref = create_node_ref::<Input>(cx);

    let initial_value = if let Some(initial_value) =
        integer_input_data.with_value(|integer_input_data| integer_input_data.updatable.value)
    {
        initial_value.to_string()
    } else {
        String::default()
    };

    let (is_in_range, set_is_in_range) = create_signal(cx, is_in_range);

    let update_value = move || {
        let input_ref = node_ref.get().expect("Integer input node");
        let mut value = input_ref.value();

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

        let clamped = integer_clamp
                .as_ref()
                .map(|integer_clamp| integer_clamp.clamp(&value))
                .unwrap_or_else(|| ParsedNum::from_str(&value));

        value = clamped.as_string;
        if value.is_empty() {
            integer_input_data.update_value(|integer_input_data| {
                integer_input_data
                    .updatable
                    .update_and_then_signal(|int| *int = None)
            });
            input_ref.set_value("");
        } else {
            let raw_num = clamped.as_u32;
            if include_comma {
                value = commify_number(raw_num);
            }

            integer_input_data.update_value(|integer_input_data| {
                integer_input_data
                    .updatable
                    .update_and_then_signal(|int| *int = Some(raw_num))
            });
            input_ref.set_value(&value);
            set_is_in_range.set(integer_input_data.with_value(|integer_input_range| {
                integer_input_range
                    .range
                    .as_ref()
                    .map(move |range| range.contains(&raw_num))
                    .unwrap_or(true)
            }));
        }
    };

    let error = &format!("Input must be {max_len} chars long");

    view! { cx,
        <input
            class=move || {
                integer_input_data
                    .with_value(|integer_input_data| {
                        integer_input_data.input_class.as_ref().cloned().unwrap_or_default()
                    })
            }
            class:invalid=move || { !is_in_range.get() }
            node_ref=node_ref
            on:input=move |_| update_value()
            placeholder=move || {
                integer_input_data
                    .with_value(|integer_input_data| {
                        integer_input_data.placeholder.as_ref().cloned().unwrap_or_default()
                    })
            }
            value=initial_value
            size=max_len + 2
            maxlength=max_len
            type="text"
        />
        <div style="
        position: -webkit-sticky; 
        top:0; 
        right: 0; 
        background-color: #bbb; 
        padding: 6px;
        border-style: inset solid;
        border-color: red;
        border-width: 6px;
        z-index: 2;
        ">
            <h4>"IntegerInput"</h4>
            <p>{error}</p>
        </div>
    }
    // ω <fn integer_input>
}

// α <mod-def integer_input>
// ω <mod-def integer_input>
