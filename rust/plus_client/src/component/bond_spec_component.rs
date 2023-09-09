//! Module for bond_spec_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::DateInput;
use crate::NumericInput;
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::BondSpec;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models bond specification
///
///   * **updatable** - The [BondSpec] being edited.
///   * _return_ - View for bond_spec_component
#[component]
pub fn BondSpecComponent(
    /// The [BondSpec] being edited.
    updatable: Updatable<Option<BondSpec>>,
) -> impl IntoView {
    // α <fn bond_spec_component>

    use crate::PercentInput;
    use leptos::store_value;
    use plus_modeled::RateCurve;
    use plus_modeled::YearValue;

    let bond_spec = updatable.value.as_ref().cloned().unwrap_or_default();
    let updatable_store_value = store_value(updatable);

    log!("Creating bond spec component for {bond_spec}");

    view! {
        <div class="form">
            <div class="form-row">
                <label>
                    "Annual Coupon"
                    <PercentInput updatable=Updatable::new(
                        bond_spec
                            .annual_coupon
                            .as_ref()
                            .and_then(|rc| rc.curve.first())
                            .map(|yv| yv.value),
                        move |new_value| {
                            if let Some(new_value) = new_value {
                                updatable_store_value
                                    .update_value(|updatable| {
                                        updatable
                                            .update_and_then_signal(|bs| {
                                                bs
                                                    .get_or_insert_with(|| BondSpec::default())
                                                    .annual_coupon = Some(RateCurve {
                                                    curve: vec![YearValue { year : 1900, value : * new_value }],
                                                })
                                            })
                                    });
                            }
                        },
                    )/>
                </label>

                <label>
                    "Maturity Year" <div style="display: inline-block">
                        <DateInput updatable=Updatable::new(
                            bond_spec.maturity,
                            move |new_maturity| {
                                updatable_store_value
                                    .update_value(|updatable| {
                                        updatable
                                            .update_and_then_signal(|maturity| {
                                                maturity
                                                    .get_or_insert_with(|| BondSpec::default())
                                                    .maturity = new_maturity.as_ref().cloned()
                                            })
                                    })
                            },
                        )/>
                    </div>
                </label>
            </div>
        </div>
    }

    // ω <fn bond_spec_component>
}

// α <mod-def bond_spec_component>
// ω <mod-def bond_spec_component>
