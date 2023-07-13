//! Module for value_flow_spec_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::ValueFlowSpec;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a set of cash flows as an *ordered* list of [YearValue] instances.
///
///   * **cx** - Context
///   * **updatable** - The [ValueFlowSpec] being edited
///   * _return_ - View for value_flow_spec_component
#[component]
pub fn ValueFlowSpecComponent(
    /// Context
    cx: Scope,
    /// The [ValueFlowSpec] being edited
    updatable: Updatable<Option<ValueFlowSpec>>,
) -> impl IntoView {
    // α <fn value_flow_spec_component>

    view! { cx, <h3>"TODO ValueFlowSpecComponent"</h3> }

    // ω <fn value_flow_spec_component>
}

// α <mod-def value_flow_spec_component>
// ω <mod-def value_flow_spec_component>
