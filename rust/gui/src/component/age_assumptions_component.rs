//! Module for age_assumptions_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::AgeAssumptions;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models age of various stages, eg retirement, death...
///
///   * **cx** - Context
///   * **updatable** - The [AgeAssumptions] being edited.
///   * _return_ - View for age_assumptions_component
#[component]
pub fn AgeAssumptionsComponent(
    /// Context
    cx: Scope,
    /// The [AgeAssumptions] being edited.
    updatable: Updatable<Option<AgeAssumptions>>,
) -> impl IntoView {
    // α <fn age_assumptions_component>
    todo!("Implement `age_assumptions_component`")
    // ω <fn age_assumptions_component>
}

// α <mod-def age_assumptions_component>
// ω <mod-def age_assumptions_component>
