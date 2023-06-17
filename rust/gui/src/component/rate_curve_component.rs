//! Module for rate_curve_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::RateCurve;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models an **ordered** series of [YearValue](plus_modeled::YearValue) pairs that together constitute a
/// piece-wise step function. The component ensures the ordering (i.e. the years in the
/// vector of [YearValue](plus_modeled::YearValue) are strictly increasing)
///
///
///   * **cx** - Context
///   * **updatable** - The [RateCurve] being edited
///   * _return_ - View for rate_curve_component
#[component]
pub fn RateCurveComponent(
    /// Context
    cx: Scope,
    /// The [RateCurve] being edited
    updatable: Updatable<RateCurve>,
) -> impl IntoView {
    // α <fn rate_curve_component>

    view! {
        cx,

        <div>"Rate Curve TODO"</div>
    }

    // ω <fn rate_curve_component>
}

// α <mod-def rate_curve_component>
// ω <mod-def rate_curve_component>
