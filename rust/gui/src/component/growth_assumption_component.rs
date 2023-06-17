//! Module for growth_assumption_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::GrowthAssumption;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a growth assumption [GrowthAssumption](plus_modeled::GrowthAssumption) which may be:
///
///  # Values Modeled
///  - A [NormalSpec](plus_modeled::NormalSpec)
///  - A [RateCurve](plus_modeled::RateCurve) - referred to as a _pinned curve_
///  - A [NormalSpec](plus_modeled::NormalSpec) **and** a [RateCurve](plus_modeled::RateCurve)
///
/// ---
/// The component enables the specification of growth in either form
/// but does not _currently_ indicate a preference for use. The idea is the
/// forecaster will configure the forecast for a preference for [NormalSpec](plus_modeled::NormalSpec)
/// or [RateCurve](plus_modeled::RateCurve) and for a given forecast the appropriate
/// growth will be selected.
///
///   * **cx** - Context
///   * **updatable** - The [GrowthAssumption] being edited
///   * _return_ - View for growth_assumption_component
#[component]
pub fn GrowthAssumptionComponent(
    /// Context
    cx: Scope,
    /// The [GrowthAssumption] being edited
    updatable: Updatable<GrowthAssumption>,
) -> impl IntoView {
    // α <fn growth_assumption_component>
    todo!("Implement `growth_assumption_component`")
    // ω <fn growth_assumption_component>
}

// α <mod-def growth_assumption_component>
// ω <mod-def growth_assumption_component>
