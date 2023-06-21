//! Module for flow_spec_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::FlowSpec;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a flow spec which is either a [GrowingFlowSpec](plus_modeled::GrowingFlowSpec)
/// or a [ValueFlowSpec](plus_modeled::ValueFlowSpec)
/// .
/// Also models any links from this flow to _funding sources_ or _investment targets_.
///
///   * **cx** - Context
///   * **updatable** - The [FlowSpec] being edited
///   * _return_ - View for flow_spec_component
#[component]
pub fn FlowSpecComponent(
    /// Context
    cx: Scope,
    /// The [FlowSpec] being edited
    updatable: Updatable<Option<FlowSpec>>,
) -> impl IntoView {
    // α <fn flow_spec_component>

    view! {
        cx,
        <h3>"TODO FlowSpecComponent"</h3>
    }

    // ω <fn flow_spec_component>
}

// α <mod-def flow_spec_component>
// ω <mod-def flow_spec_component>
