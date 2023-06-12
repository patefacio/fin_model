//! Module for currency_value_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use fin_model::core::CurrencyValue;
use leptos::create_node_ref;
use leptos::{component, view, IntoView, Scope};
use leptos_dom::{console_log, html::Input};

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// A component for specifying a currency and value
///
///   * **cx** - Context
///   * **updatable** - TODO Document Param(updatable)
///   * _return_ - View for currency_value_input
#[component]
pub fn CurrencyValueInput(
    /// Context
    cx: Scope,
    /// TODO Document Param(updatable)
    updatable: Updatable<CurrencyValue>,
) -> impl IntoView {
    // α <fn currency_value_input>
    todo!("Implement `currency_value_input`")
    // ω <fn currency_value_input>
}

// α <mod-def currency_value_input>
// ω <mod-def currency_value_input>
