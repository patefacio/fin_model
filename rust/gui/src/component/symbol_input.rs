//! Module for symbol_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::utils::updatable::Updatable;
use leptos::{component, view, IntoView, Scope};

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models an input field that accepts a valid symbol (aka Instrument Name)
///
///   * **cx** - Context
///   * **symbol_updatable** - The symbol name.
///   * _return_ - View for symbol_input
#[component]
pub fn SymbolInput(
    /// Context
    cx: Scope,
    /// The symbol name.
    symbol_updatable: Updatable<String>,
) -> impl IntoView {
    // α <fn symbol_input>

    use leptos::IntoAttribute;

    // Get the initial value for the year if provided. Set to empty string if
    // not provided.
    let initial_value = symbol_updatable.value.clone();
    let mut symbol_updatable = symbol_updatable;
    use leptos::create_node_ref;
    use leptos_dom::{console_log, html::Input};

    let input_ref = create_node_ref::<Input>(cx);

    let on_input = move |_| {
        console_log("Got keydown for symbol!");
        let input_ref = input_ref.get().expect("Input node exists");
        symbol_updatable.update(move |symbol_name| *symbol_name = input_ref.value());
    };

    let on_keydown = move |_| {
        console_log("Got keydown");
    };

    view! {
        cx,

        <input
            node_ref=input_ref
            on:keydown=on_keydown
            on:input=on_input
            placeholder="SYMBOL"
            style="text-transform:uppercase"
            type="text"
            value=initial_value
        />

    }

    // ω <fn symbol_input>
}

// α <mod-def symbol_input>
// ω <mod-def symbol_input>
