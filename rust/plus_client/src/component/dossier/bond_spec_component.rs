//! Module for bond_spec_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::AppContext;
use crate::DateInput;
use crate::NumericInput;
use crate::Updatable;
use leptos::use_context;
use leptos::IntoAttribute;
use leptos::SignalGet;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;
use plus_lookup::I18nBondSpecComponent;
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
    pub const SELF_CLASS: &str = "plus-bsc";
    let lang_selector = use_context::<AppContext>().unwrap().lang_selector;
    let i18n_annual_coupon =
        move || I18nBondSpecComponent::AnnualCoupon(lang_selector.get()).to_string();
    let i18n_maturity_year =
        move || I18nBondSpecComponent::MaturityYear(lang_selector.get()).to_string();
    // α <fn bond_spec_component>

    use crate::PercentInput;
    use leptos::store_value;
    use plus_modeled::RateCurve;
    use plus_modeled::YearValue;

    let bond_spec = updatable.value.as_ref().cloned().unwrap_or_default();
    let updatable_store_value = store_value(updatable);

    // ω <fn bond_spec_component>
    view! {
        <div class=SELF_CLASS>
            // α <plus-bsc-view>
            <div class="form">
                <div class="form-row">
                    <label>
                        {i18n_annual_coupon}
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
                        {i18n_maturity_year} <div style="display: inline-block">
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
        // ω <plus-bsc-view>
        </div>
    }
}

// α <mod-def bond_spec_component>
// ω <mod-def bond_spec_component>
