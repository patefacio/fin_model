//! Module for string_input leptos function/component

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
/// Models a single string
///
///   * **cx** - Context
///   * **updatable** - Signals updates of the text input
///   * **input_class** - Class to decorate input element for styling
///   * **placeholder** - Placeholder shown if entry is empty.
///   * **max_len** - Maximum length of the string.
///   * _return_ - View for string_input
#[component]
pub fn StringInput(
    /// Context
    cx: Scope,
    /// Signals updates of the text input
    updatable: Updatable<String>,
    /// Class to decorate input element for styling
    #[prop(default=None)]
    input_class: Option<String>,
    /// Placeholder shown if entry is empty.
    #[prop(default=None)]
    placeholder: Option<String>,
    /// Maximum length of the string.
    #[prop(default = 16)]
    max_len: u32,
) -> impl IntoView {
    // α <fn string_input>

    use leptos::create_node_ref;
    use leptos_dom::html::Input;
    use leptos::IntoAttribute;

    let node_ref = create_node_ref::<Input>(cx);
    let initial_value = updatable.value.clone();

    view! {
        cx,

        <input
            class=input_class
            node_ref=node_ref
            //on:input = move |_| update_value.update_value(|update_value| update_value())
            placeholder=placeholder.unwrap_or_default()
            value=initial_value
            size=max_len
            maxlength=max_len
            type="text"
        />

    }

    // ω <fn string_input>
}

// α <mod-def string_input>
// ω <mod-def string_input>
