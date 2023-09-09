//! Module for holding_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::HoldingSharedContext;
#[allow(unused_imports)]
use leptos::log;
use leptos::StoredValue;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::Holding;
use plus_modeled::ItemGrowth;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// A single holding in an account.
///
///   * **holding_stored_value** - The holding being edited by this component with shared context
///   * **shared_context_stored_value** - Shared context
///   * _return_ - View for holding_component
#[component]
pub fn HoldingComponent(
    /// The holding being edited by this component with shared context
    holding_stored_value: StoredValue<Holding>,
    /// Shared context
    shared_context_stored_value: StoredValue<HoldingSharedContext>,
) -> impl IntoView {
    // α <fn holding_component>

    use crate::to_currency_symbol;
    use crate::DistributionPolicyComponent;
    use crate::ItemGrowthComponent;
    use crate::Modification;
    use crate::NumericInput;
    use crate::SymbolInput;
    use crate::Updatable;
    use crate::YearCurrencyValueInput;
    use leptos::create_rw_signal;
    use leptos::MaybeSignal;
    use leptos::SignalGet;
    use leptos::SignalUpdate;
    use leptos::SignalWith;
    use plus_modeled::Currency;
    use plus_modeled::CurrencyValue;
    use plus_modeled::DistributionPolicy;
    use plus_modeled::DossierItemType;
    use plus_modeled::GrowthAssumption;
    use plus_modeled::GrowthItemMappings;
    use plus_modeled::HoldingType;
    use plus_modeled::NormalSpec;
    use plus_modeled::SystemGrowthId;
    use plus_modeled::SystemId;

    holding_stored_value.with_value(|holding| {
        let quantity = holding.quantity;
        let unit_valuation = holding.unit_valuation;
        let cost_basis = holding.cost_basis;
        let instrument_name = holding.instrument_name.clone();
        let distribution_policy = holding.distribution_policy.clone();

        fn market_value_with_unrealized(holding: &Holding) -> (Option<String>, Option<String>) {
            if let Some(unit_valuation) = holding.unit_valuation.as_ref() {
                let mv = holding.quantity * unit_valuation.value;
                let market_value = CurrencyValue {
                    currency: unit_valuation.currency,
                    value: mv,
                };
                let gain_loss = CurrencyValue {
                    currency: unit_valuation.currency,
                    value: mv - holding.cost_basis,
                };
                (Some(market_value.to_string()), Some(gain_loss.to_string()))
            } else {
                (None, None)
            }
        }

        let (market_value_signal, unrealized_gl_signal) = {
            let (mv, gl) = market_value_with_unrealized(&holding);
            (create_rw_signal(mv), create_rw_signal(gl))
        };

        let update_market_value_and_unrealized = move || {
            holding_stored_value.with_value(|holding| {
                if holding.unit_valuation.is_some() {
                    let (market_value, gain_loss) = market_value_with_unrealized(holding);
                    market_value_signal.update(|mv| *mv = market_value);
                    unrealized_gl_signal.update(|gl| *gl = gain_loss);
                }
            });
        };

        let symbol_updatable = Updatable::new(instrument_name, move |symbol| {
            holding_stored_value.update_value(|holding| {
                holding.instrument_name = symbol.clone();
            });
        });

        let quantity_updatable = Updatable::new(Some(quantity), move |quantity| {
            holding_stored_value.update_value(|holding| {
                if let Some(quantity) = quantity {
                    holding.quantity = *quantity;
                }
            });
            update_market_value_and_unrealized();
        });

        let currency_rw_signal = create_rw_signal(
            to_currency_symbol(
                unit_valuation
                    .map(|ycv| Currency::from_i32(ycv.currency).unwrap())
                    .unwrap_or(Currency::Usd),
            )
            .to_string(),
        );

        let unit_valuation_updatable = Updatable::new(unit_valuation, move |unit_valuation| {
            currency_rw_signal.update(|currency_string| {
                *currency_string = to_currency_symbol(
                    unit_valuation
                        .map(|ycv| Currency::from_i32(ycv.currency).unwrap_or_default())
                        .unwrap_or_default(),
                )
                .to_string()
            });
            holding_stored_value.update_value(|holding| {
                holding.unit_valuation = *unit_valuation;
            });
            update_market_value_and_unrealized();
        });

        let cost_basis_updatable = Updatable::new(Some(cost_basis), move |cost_basis| {
            currency_rw_signal.track();
            holding_stored_value.update_value(|holding| {
                if let Some(cost_basis) = cost_basis {
                    holding.cost_basis = *cost_basis;
                }
            });
            update_market_value_and_unrealized();
        });

        let item_growth_updatable = Updatable::new(
            ItemGrowth {
                system_growth_id: Some(SystemGrowthId {
                    system_id: Some(SystemId::HoldingItemId(HoldingType::UsEquityMarket as u32)),
                }),
                growth_assumption: Some(GrowthAssumption {
                    normal_spec: Some(NormalSpec {
                        mean: 0.11,
                        std_dev: 0.2,
                    }),
                    pinned_growth: None,
                }),
            },
            move |item_growth| log!("ItemGrowth -> {item_growth:?}"),
        );

        let distribution_policy_updatable =
            Updatable::new(distribution_policy, move |distribution_policy| {
                holding_stored_value.update_value(|holding| {
                    holding.distribution_policy = distribution_policy.as_ref().cloned();
                    log!("DistributionPolicy updated {distribution_policy:?}");
                })
            });

        let market_value = move || {
            if let Some(mv) = market_value_signal.get() {
                format!("Market Value: {mv}")
            } else {
                String::default()
            }
        };

        let unrealized_gain_loss = move || {
            if let Some(gl) = unrealized_gl_signal.get() {
                format!("Unrealized G/L: {}", gl)
            } else {
                String::default()
            }
        };

        leptos::on_cleanup(move || log!("CLEANING UP HOLDING"));

        view! {
            <fieldset class="holding" style="margin: 0.5rem;">
                <legend>"Holding"</legend>
                <div class="form">
                    <div class="form-row">
                        <label>"Symbol" <SymbolInput symbol_updatable=symbol_updatable/></label>
                        <label>
                            "Current Price"
                            <div style="display: inline-block; margin-left: 0.45em;">
                                <YearCurrencyValueInput
                                    updatable=unit_valuation_updatable
                                    value_placeholder="price".to_string()
                                />
                            </div>
                        </label>
                    </div>
                    <div class="form-row">
                        <label>"Quantity" <NumericInput updatable=quantity_updatable/></label>
                        <label>
                            "Cost"
                            <NumericInput
                                updatable=cost_basis_updatable
                                modification=Some(
                                    Modification::Prefix(
                                        MaybeSignal::Dynamic(currency_rw_signal.into()),
                                    ),
                                )
                            />

                        </label>
                    </div>
                    <div class="form-row">
                        <div class="info-label">{move || market_value()}</div>
                        <div class="info-label">{move || unrealized_gain_loss()}</div>
                    </div>
                    <div style="grid-column-start: 1; grid-column-end: 2;">
                        <fieldset>
                            <legend>"Growth"</legend>
                            <ItemGrowthComponent
                                updatable=item_growth_updatable
                                dossier_item_type=DossierItemType::Holding
                                growth_item_mappings=&GrowthItemMappings::default()
                            />
                        </fieldset>
                        <fieldset>
                            <legend>"Distributions"</legend>
                            <DistributionPolicyComponent updatable=distribution_policy_updatable/>

                        </fieldset>
                    </div>
                </div>
            </fieldset>
        }
    })

    // ω <fn holding_component>
}

// α <mod-def holding_component>
// ω <mod-def holding_component>
