//! Module for distribution_spec_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::RateCurveComponent;
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::DistributionSpec;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models the distributions streams associated with a symbol or holding set
///
///   * **cx** - Context
///   * **updatable** - The [AgeAssumptions] being edited.
///   * _return_ - View for distribution_spec_component
#[component]
pub fn DistributionSpecComponent(
    /// Context
    cx: Scope,
    /// The [AgeAssumptions] being edited.
    updatable: Updatable<Option<DistributionSpec>>,
) -> impl IntoView {
    // α <fn distribution_spec_component>

    use crate::ExpandableRateComponent;

    let i = updatable.value.unwrap();

    view! { cx,
        <div class="form">
            <div class="form-row">
                <label>
                    "Qualified Div." <div style="display: inline-block;">
                        <ExpandableRateComponent updatable=Updatable::new(
                            i.qualified_dividend.unwrap_or_default(),
                            move |rc| {
                                log!("Rate Curve -> {rc:?}");
                            },
                        )/>
                    </div>
                </label>
                <label>
                    "Unqualified Div." <div style="display: inline-block;">
                        <ExpandableRateComponent updatable=Updatable::new(
                            i.unqualified_dividend.unwrap_or_default(),
                            move |rc| {
                                log!("Rate Curve -> {rc:?}");
                            },
                        )/>
                    </div>
                </label>
            </div>
            <div class="form-row">
                <label>
                    "Capital Gain" <div style="display: inline-block;">
                        <ExpandableRateComponent updatable=Updatable::new(
                            i.capital_gain.unwrap_or_default(),
                            move |rc| {
                                log!("Rate Curve -> {rc:?}");
                            },
                        )/>
                    </div>
                </label>
                <label>
                    "Interest" <div style="display: inline-block;">
                        <ExpandableRateComponent updatable=Updatable::new(
                            i.interest.unwrap_or_default(),
                            move |rc| {
                                log!("Rate Curve -> {rc:?}");
                            },
                        )/>
                    </div>
                </label>
            </div>
        </div>
    }

    // ω <fn distribution_spec_component>
}

// α <mod-def distribution_spec_component>
// ω <mod-def distribution_spec_component>
