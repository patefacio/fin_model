//! Module for integer_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;

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
) -> impl IntoView {
    // α <fn integer_input>

    use leptos::create_node_ref;
    use leptos::html::Input;
    use leptos::IntoAttribute;

    let node_ref = create_node_ref::<Input>(cx);
    let mut updatable = updatable;

    let initial_value = if let Some(initial_value) = updatable.value.as_ref() {
        initial_value.to_string()
    } else {
        String::default()
    };

    let mut update_value = move || {
        let input_ref = node_ref.get().expect("Integer input node");
        let mut value = input_ref.value();
        //let error = &format!(" ");

        let last_letter = value.chars().last().unwrap_or(' ');
        if last_letter == 'k' {
            value.pop();
            value.push_str("000");
        }
        if last_letter == 'm' {
            value.pop();
            value.push_str("000000");
        }
        if last_letter == 'b' {
            value.pop();
            value.push_str("000000000");
        }

        if value.len() > max_len as usize { //let error = &format!("Input must be {max_len} chars long");
        }

        value = value.chars().filter(|c| c.is_ascii_digit()).collect();

        value = value[0..std::cmp::min(value.len(), max_len as usize)].to_string();

        let raw_num = value.parse::<u32>().unwrap_or(0);

        if include_comma {
            for i in (4..value.len() * 5 / 4 + 1).step_by(4) {
                //Multiply by 5/4 to account for commas
                value.insert(value.len() - i + 1, ',');
                console_log(&format!("{}-{}", value.len(), i));
            }
        }

        if value.is_empty() {
            updatable.update_and_then_signal(|int| *int = None);
            leptos_dom::console_log(&format!("IntegerInput: setting value to -> None"));
            input_ref.set_value("");
        } else {
            updatable.update_and_then_signal(|int| *int = Some(raw_num));
            input_ref.set_value(&value);
            leptos_dom::console_log(&format!("IntegerInput: setting value to -> {value:?}"));
        }
    };

    let error = &format!("Input must be {max_len} chars long");

    view! { cx,
        <input
            class=input_class
            node_ref=node_ref
            on:input=move |_| update_value()
            placeholder=placeholder.unwrap_or_default()
            value=initial_value
            size=max_len + 2
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
