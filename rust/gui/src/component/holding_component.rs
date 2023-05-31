//! Module for holding_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::utils::updatable::Updatable;
use fin_model::account::Holding;
use leptos::{component, tracing, view, IntoView, Scope};

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// A single holding in an account.
///
///
///   * **cx** - Context
///   * **updatable** - The holding being edited by this component
///   * _return_ - View for holding_component
#[component]
pub fn HoldingComponent<F>(
    /// Context
    cx: Scope,
    /// The holding being edited by this component
    updatable: Updatable<Holding, F>,
) -> impl IntoView
where
    F: FnMut(&Holding) + 'static,
{
    // α <fn holding_component>
    view! {
        cx,
        <div>"This is a holding"</div>
    }
    // ω <fn holding_component>
}

// α <mod-def holding_component>
// ω <mod-def holding_component>
