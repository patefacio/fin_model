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
    todo!("Implement `integer_input`")
    // ω <fn integer_input>
}

// α <mod-def integer_input>
// ω <mod-def integer_input>
