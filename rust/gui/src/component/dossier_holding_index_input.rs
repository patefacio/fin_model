//! Module for dossier_holding_index_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::DossierHoldingIndex;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a composite index referring to the account (0-indexed) and
/// _optionally_ the holding within the account (0-indexed).
///
///   * **cx** - Context
///   * **updatable** - The [DossierHoldingIndex] being edited
///   * _return_ - View for dossier_holding_index_input
#[component]
pub fn DossierHoldingIndexInput(
    /// Context
    cx: Scope,
    /// The [DossierHoldingIndex] being edited
    updatable: Updatable<Option<DossierHoldingIndex>>,
) -> impl IntoView {
    // α <fn dossier_holding_index_input>
    view! {
        cx,
        <h3>"TODO DossierHoldingIndex"</h3>
    }
    // ω <fn dossier_holding_index_input>
}

// α <mod-def dossier_holding_index_input>
// ω <mod-def dossier_holding_index_input>
