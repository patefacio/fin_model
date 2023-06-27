//! Module for growing_flow_spec_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::GrowingFlowSpec;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a set of cash flows with an initial value and an associated growth.
///
///   * **cx** - Context
///   * **updatable** - The [GrowingFlowSpec] being edited
///   * _return_ - View for growing_flow_spec_component
#[component]
pub fn GrowingFlowSpecComponent(
    /// Context
    cx: Scope,
    /// The [GrowingFlowSpec] being edited
    updatable: Updatable<Option<GrowingFlowSpec>>,
) -> impl IntoView {
    // α <fn growing_flow_spec_component>

    view! { cx, <h3>"TODO GrowingFlowSpecComponent"</h3> }

    // ω <fn growing_flow_spec_component>
}

// α <mod-def growing_flow_spec_component>
// ω <mod-def growing_flow_spec_component>
