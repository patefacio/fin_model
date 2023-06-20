//! Module for dossier_correlation_entry_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::DossierCorrelationEntry;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a correlation value between _dossier items_.
///
///   * **cx** - Context
///   * **updatable** - The [DossierCorrelationEntry] being edited
///   * _return_ - View for dossier_correlation_entry_component
#[component]
pub fn DossierCorrelationEntryComponent(
    /// Context
    cx: Scope,
    /// The [DossierCorrelationEntry] being edited
    updatable: Updatable<Option<DossierCorrelationEntry>>,
) -> impl IntoView {
    // α <fn dossier_correlation_entry_component>

    view! {
        cx,
        <h3>"TODO DossierCorrelationEntryComponent"</h3>
    }

    // ω <fn dossier_correlation_entry_component>
}

// α <mod-def dossier_correlation_entry_component>
// ω <mod-def dossier_correlation_entry_component>
