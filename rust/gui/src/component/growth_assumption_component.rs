//! Module for growth_assumption_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
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
    use leptos::create_rw_signal;
    use leptos::store_value;
    use leptos::IntoAttribute;
    use leptos::Show;
    use leptos::SignalGet;
    use leptos::SignalUpdate;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    let growth_type = create_rw_signal(cx, initial_growth_type);

    let updatable = store_value(cx, updatable);

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
                    .cloned()
                    .unwrap_or_default()
            }),
            move |pinned| {
                updatable
                    .update_value(|updatable| updatable.value.pinned_growth = Some(pinned.clone()))
            },
        )
    };

    let show_normal_spec = move || {
        view! { cx,
            <Show when=move || { growth_type.get() == GrowthType::UseNormal } fallback=|_| ()>
                <NormalSpecComponent updatable=normal_spec_updatable()/>
            </Show>
        }
    };

    let show_pinned_curve = move || {
        view! { cx,
            <Show when=move || { growth_type.get() == GrowthType::UsePinned } fallback=|_| ()>
                <RateCurveComponent updatable=rate_curve_updatable()/>
            </Show>
        }
    };

    view! { cx,
        <fieldset style="margin: 0.5rem;">
            <div style="display: grid; grid-template-columns: 1fr 1fr; margin: 3px">
                <div style="display: flex; grid-column-start: 1; grid-column-end: 2;">
                    <div>
                        <input
                            id="normal"
                            on:click=move |_| {
                                growth_type
                                    .update(|growth_type| *growth_type = GrowthType::UseNormal)
                            }

                            name="override-select"
                            type="radio"
                            value="normal"
                            checked=move || { growth_type.get() == GrowthType::UseNormal }
                        />
                    </div>
                    <div>
                        <label for="normal">"Normal Spec"</label>
                    </div>
                </div>
                <div style="display: inline-flex; grid-column-start: 2; grid-column-end: 3;">
                    <div>
                        <input
                            id="pinned"
                            on:click=move |_| {
                                growth_type
                                    .update(|growth_type| *growth_type = GrowthType::UsePinned)
                            }

                            name="override-select"
                            type="radio"
                            value="pinned"
                            checked=move || { growth_type.get() == GrowthType::UsePinned }
                        />
                    </div>
                    <div>
                        <label for="pinned">"Pinned Curve"</label>
                    </div>
                </div>
                <div style="grid-column-start: 1; grid-column-end: 3">
                    {move || show_normal_spec()}
                </div>
                <div style="grid-column-start: 2; grid-column-end: 3">
                    {move || show_pinned_curve()}
                </div>
            </div>
        </fieldset>
    }

    // ω <fn growth_assumption_component>
}

// α <mod-def growth_assumption_component>
// ω <mod-def growth_assumption_component>
