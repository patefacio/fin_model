//! Module for symbol_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models an input field that accepts a valid symbol (aka Instrument Name)
///
///   * **symbol_updatable** - The symbol name.
///   * **size** - The size attribute, which one hopes would make the size of the
/// input field roughly that number of characters. But YMMV.
///
///   * **max_len** - The maximum number of characters for the input.
///
///   * _return_ - View for symbol_input
#[component]
pub fn SymbolInput(
    /// The symbol name.
    symbol_updatable: Updatable<String>,
    /// The size attribute, which one hopes would make the size of the
    /// input field roughly that number of characters. But YMMV.
    #[prop(default = 9)]
    size: u32,
    /// The maximum number of characters for the input.
    #[prop(default = 9)]
    max_len: u32,
) -> impl IntoView {
    // α <fn symbol_input>
    use leptos::IntoAttribute;

    // Get the initial value for the year if provided. Set to empty string if
    // not provided.
    let initial_value = symbol_updatable.value.clone();
    let mut symbol_updatable = symbol_updatable;
    use leptos::create_node_ref;
    use leptos_dom::html::Input;

    let input_ref = create_node_ref::<Input>();

    let on_input = move |_| {
        let input_ref = input_ref.get().expect("Input node exists");
        symbol_updatable
            .update_and_then_signal(move |symbol_name| *symbol_name = input_ref.value());
    };

    let on_keydown = move |_| {
        log!("Got keydown");
    };

    view! {
        <input
            node_ref=input_ref
            on:keydown=on_keydown
            on:input=on_input
            placeholder="SYMBOL"
            style="text-transform:uppercase"
            type="text"
            value=initial_value
            size=size
            maxlength=max_len
        />
    }

    // ω <fn symbol_input>
}

// α <mod-def symbol_input>
// ω <mod-def symbol_input>
