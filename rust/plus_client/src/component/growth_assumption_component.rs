//! Module for growth_assumption_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView};
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
///   * **updatable** - The [GrowthAssumption] being edited
///   * _return_ - View for growth_assumption_component
#[component]
pub fn GrowthAssumptionComponent(
    /// The [GrowthAssumption] being edited
    updatable: Updatable<GrowthAssumption>,
) -> impl IntoView {
    // α <fn growth_assumption_component>

    use crate::NormalSpecComponent;
    use crate::OneOfComponent;
    use crate::YearValueSeriesComponent;
    use leptos::store_value;
    use plus_modeled::RateCurve;

    #[derive(Debug, Clone, Copy, PartialEq, EnumVariantNames, EnumIter)]
    enum GrowthType {
        UseNormal,
        UsePinned,
    }

    let initial_growth_type = if updatable.value.normal_spec.is_some() {
        GrowthType::UseNormal
    } else if updatable.value.pinned_growth.is_some() {
        GrowthType::UsePinned
    } else {
        // Prefer normal if neither set yet
        GrowthType::UseNormal
    };

    let updatable = store_value(updatable);

    let normal_spec_updatable = move || {
        Updatable::new(
            updatable.with_value(|updatable| updatable.value.normal_spec.as_ref().cloned()),
            move |ns| updatable.update_value(|updatable| updatable.value.normal_spec = *ns),
        )
    };

    let rate_curve_updatable = move || {
        Updatable::new(
            updatable.with_value(|updatable| {
                updatable
                    .value
                    .pinned_growth
                    .as_ref()
                    .map(|rc| rc.curve.clone())
                    .unwrap_or_default()
            }),
            move |pinned| {
                updatable.update_value(|updatable| {
                    updatable.value.pinned_growth = Some(RateCurve {
                        curve: pinned.clone(),
                    })
                })
            },
        )
    };

    let show_normal_spec =
        move || view! { <NormalSpecComponent updatable=normal_spec_updatable()/> }.into_view();

    let show_pinned_curve =
        move || view! { <YearValueSeriesComponent updatable=rate_curve_updatable()/> }.into_view();

    let views = move |growth_type: &GrowthType| match growth_type {
        GrowthType::UseNormal => show_normal_spec(),
        GrowthType::UsePinned => show_pinned_curve(),
    };

    let label_maker = move |growth_type: &GrowthType| match growth_type {
        GrowthType::UseNormal => String::from("Normal Spec"),
        GrowthType::UsePinned => String::from("Fixed Rate Curve"),
    };

    view! {
        <OneOfComponent
            selection=initial_growth_type
            name="system-growth".to_string()
            views=views
            labels=Some(label_maker)
        />
    }

    // ω <fn growth_assumption_component>
}

// α <mod-def growth_assumption_component>
// ω <mod-def growth_assumption_component>
