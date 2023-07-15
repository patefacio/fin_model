//! Module for historic_risk_return_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
#[allow(unused_imports)]
use leptos::log;
use leptos::MaybeSignal;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::NormalSpec;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Given a [NormalSpec] provides an SVG plot showing the `(risk/return)` along side
/// various historic `(risk/return)` values for equities, bonds, ...
///
///   * **cx** - Context
///   * **normal_spec** - The normal to plot
///   * _return_ - View for historic_risk_return_component
#[component]
pub fn HistoricRiskReturnComponent(
    /// Context
    cx: Scope,
    /// The normal to plot
    normal_spec: MaybeSignal<NormalSpec>,
) -> impl IntoView {
    // α <fn historic_risk_return_component>
    todo!("Implement `historic_risk_return_component`")
    // ω <fn historic_risk_return_component>
}

// α <mod-def historic_risk_return_component>
// ω <mod-def historic_risk_return_component>
