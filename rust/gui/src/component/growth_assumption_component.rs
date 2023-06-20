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

    use crate::NormalSpecComponent;
    use crate::RateCurveComponent;

    use plus_modeled::NormalSpec;

    let normal_spec_updatable = Updatable::new(
        Some(NormalSpec::default()),
        |ns| {
            console_log(&format!("NormalSpec updated to {ns:?}"))
        }
    );

    let rate_curve_updatable = Updatable::new(
        updatable.value.pinned_growth.unwrap_or_default(),
        |rc| {
            console_log(&format!("Growth item's rate curve updated -> {rc:?}"))
        }
    );

    view! {
        cx,
        <NormalSpecComponent
            updatable=normal_spec_updatable
        />
        <RateCurveComponent
            updatable=rate_curve_updatable
        />
    }


    // ω <fn growth_assumption_component>
}

// α <mod-def growth_assumption_component>
// ω <mod-def growth_assumption_component>
