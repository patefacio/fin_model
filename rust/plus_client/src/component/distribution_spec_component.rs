//! Module for distribution_spec_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::DistributionSpec;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models the distributions streams associated with a symbol or holding set
///
///   * **updatable** - The [AgeAssumptions] being edited.
///   * _return_ - View for distribution_spec_component
#[component]
pub fn DistributionSpecComponent(
    /// The [AgeAssumptions] being edited.
    updatable: Updatable<Option<DistributionSpec>>,
) -> impl IntoView {
    // α <fn distribution_spec_component>

    use crate::PercentInput;
    use leptos::store_value;
    use plus_modeled::RateCurve;
    use plus_modeled::YearValue;

    let distribution_spec = updatable.value.clone().unwrap();

    fn first_rate(rate_curve: &Option<RateCurve>) -> Option<f64> {
        rate_curve
            .as_ref()
            .and_then(|rc| rc.curve.first().map(|yv| yv.value))
            .or_else(|| Some(0.0))
    }

    fn make_single_entry_curve(rate: &Option<f64>) -> Option<RateCurve> {
        rate.map(|r| RateCurve {
            curve: vec![YearValue {
                year: 1900,
                value: r,
            }],
        })
    }

    let updatable_store_value = store_value(updatable);

    view! {
        <div class="form">
            <div class="form-row">
                <label>
                    "Qualified Div." <div style="display: inline-block;">
                        <PercentInput
                            updatable=Updatable::new(
                                first_rate(&distribution_spec.qualified_dividend),
                                move |rate| {
                                    updatable_store_value
                                        .update_value(|updatable| {
                                            updatable
                                                .update_and_then_signal(|ds| {
                                                    let new_curve = make_single_entry_curve(rate);
                                                    ds
                                                        .get_or_insert_with(|| DistributionSpec::default())
                                                        .qualified_dividend = new_curve;
                                                })
                                        });
                                },
                            )

                            placeholder="annual pct"
                        />
                    </div>
                </label>
                <label>
                    "Unqualified Div." <div style="display: inline-block;">
                        <PercentInput
                            updatable=Updatable::new(
                                first_rate(&distribution_spec.unqualified_dividend),
                                move |rate| {
                                    updatable_store_value
                                        .update_value(|updatable| {
                                            updatable
                                                .update_and_then_signal(|ds| {
                                                    ds
                                                        .get_or_insert_with(|| DistributionSpec::default())
                                                        .unqualified_dividend = make_single_entry_curve(rate)
                                                })
                                        });
                                },
                            )

                            placeholder="annual pct"
                        />
                    </div>
                </label>
            </div>
            <div class="form-row">
                <label>
                    "Capital Gain" <div style="display: inline-block;">
                        <PercentInput
                            updatable=Updatable::new(
                                first_rate(&distribution_spec.capital_gain),
                                move |rate| {
                                    updatable_store_value
                                        .update_value(|updatable| {
                                            updatable
                                                .update_and_then_signal(|ds| {
                                                    ds
                                                        .get_or_insert_with(|| DistributionSpec::default())
                                                        .capital_gain = make_single_entry_curve(rate)
                                                })
                                        });
                                },
                            )

                            placeholder="annual pct"
                        />
                    </div>
                </label>
                <label>
                    "Interest" <div style="display: inline-block;">
                        <PercentInput
                            updatable=Updatable::new(
                                first_rate(&distribution_spec.interest),
                                move |rate| {
                                    updatable_store_value
                                        .update_value(|updatable| {
                                            updatable
                                                .update_and_then_signal(|ds| {
                                                    ds
                                                        .get_or_insert_with(|| DistributionSpec::default())
                                                        .qualified_dividend = make_single_entry_curve(rate)
                                                })
                                        });
                                },
                            )

                            placeholder="annual pct"
                        />
                    </div>
                </label>
            </div>
        </div>
    }

    // ω <fn distribution_spec_component>
}

// α <mod-def distribution_spec_component>
// ω <mod-def distribution_spec_component>