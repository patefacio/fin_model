//! Module for holding_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::HoldingSharedContext;
use crate::SymbolGrowthMap;
use crate::Updatable;
use crate::UpdatablePair;
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::BalanceSheet;
use plus_modeled::DossierHoldingIndex;
use plus_modeled::Holding;
use plus_modeled::ItemGrowth;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// A single holding in an account.
///
///
///   * **cx** - Context
///   * **updatable** - The holding being edited by this component with shared context
///   * **on_cancel** - Called if edit is canceled.
///   * _return_ - View for holding_component
#[component]
pub fn HoldingComponent<F>(
    /// Context
    cx: Scope,
    /// The holding being edited by this component with shared context
    updatable: UpdatablePair<Holding, HoldingSharedContext>,
    /// Called if edit is canceled.
    on_cancel: F,
) -> impl IntoView
where
    F: 'static + FnMut(),
{
    // α <fn holding_component>

    use crate::to_currency_symbol;
    use crate::EnumSelect;
    use crate::ItemGrowthComponent;
    use crate::Modification;
    use crate::NumericInput;
    use crate::OkCancel;
    use crate::OkCancelComponent;
    use crate::SymbolInput;
    use crate::UpdatePairType;
    use crate::YearCurrencyValueInput;
    use leptos::create_rw_signal;
    use leptos::create_signal;
    use leptos::store_value;
    use leptos::MaybeSignal;
    use leptos::RwSignal;
    use leptos::SignalGet;
    use leptos::SignalUpdate;
    use leptos::SignalWith;
    use plus_modeled::Currency;
    use plus_modeled::CurrencyValue;
    use plus_modeled::DossierItemType;
    use plus_modeled::GrowthAssumption;
    use plus_modeled::GrowthItemMappings;
    use plus_modeled::NormalSpec;
    use plus_modeled::{
        core_enums::HoldingType,
        growth::{system_growth_id::SystemId, SystemGrowthId},
    };

    let mut on_cancel = on_cancel;
    let holding = &updatable.first_value;
    let quantity = holding.quantity;
    let unit_valuation = holding.unit_valuation;
    let cost_basis = holding.cost_basis;
    let instrument_name = holding.instrument_name.clone();

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
        let (mv, gl) = market_value_with_unrealized(&updatable.first_value);
        (create_rw_signal(cx, mv), create_rw_signal(cx, gl))
    };

    let updatable = store_value(cx, updatable);

    let update_market_value_and_unrealized = move || {
        log!("Rerunning market value and unrealized");
        updatable.with_value(move |updatable| {
            let holding = &updatable.first_value;
            if holding.unit_valuation.is_some() {
                let (market_value, gain_loss) = market_value_with_unrealized(holding);
                market_value_signal.update(|mv| *mv = market_value);
                unrealized_gl_signal.update(|gl| *gl = gain_loss);
            }
        })
    };

    let symbol_updatable = Updatable::new(instrument_name, move |symbol| {
        updatable.update_value(|updatable| {
            updatable.update(|(holding, _shared_context)| {
                holding.instrument_name = symbol.clone();
                crate::UpdatePairType::UpdateFirst
            })
        });
    });

    let quantity_updatable = Updatable::new(Some(quantity), move |quantity| {
        updatable.update_value(|updatable| {
            updatable.update(|(holding, _shared_context)| {
                if let Some(quantity) = quantity {
                    holding.quantity = *quantity;
                    UpdatePairType::UpdateFirst
                } else {
                    UpdatePairType::UpdateNone
                }
            })
        });
        update_market_value_and_unrealized();
    });

    let currency_rw_signal = create_rw_signal(
        cx,
        to_currency_symbol(
            unit_valuation
                .map(|ycv| Currency::from_i32(ycv.currency).unwrap())
                .unwrap_or(Currency::Usd),
        )
        .to_string(),
    );

    let unit_valuation_updatable = Updatable::new(unit_valuation, move |unit_valuation| {
        log!("Unit value updated {unit_valuation:?}");
        currency_rw_signal.update(|currency_string| {
            log!("UPDATING UV -> {unit_valuation:?}");
            *currency_string = to_currency_symbol(
                unit_valuation
                    .map(|ycv| Currency::from_i32(ycv.currency).unwrap_or_default())
                    .unwrap_or_default(),
            )
            .to_string()
        });
        log!("Updating main updatable with borrow");
        updatable.update_value(|updatable| {
            updatable.update(|(holding, _shared_context)| {
                holding.unit_valuation = *unit_valuation;
                UpdatePairType::UpdateFirst
            })
        });
        update_market_value_and_unrealized();
    });

    let cost_basis_updatable = Updatable::new(Some(cost_basis), move |cost_basis| {
        log!("Cost basis updated -> {cost_basis:?}");
        currency_rw_signal.track();
        updatable.update_value(|updatable| {
            updatable.update(|(holding, _shared_context)| {
                if let Some(cost_basis) = cost_basis {
                    holding.cost_basis = *cost_basis;
                    UpdatePairType::UpdateFirst
                } else {
                    UpdatePairType::UpdateNone
                }
            })
        });
        update_market_value_and_unrealized();
    });

    let on_ok_cancel = move |ok_cancel| {
        log!("Ok/Cancel holding -> {ok_cancel:?}");
        match ok_cancel {
            OkCancel::Ok => {
                updatable.update_value(|updatable| updatable.signal());
                log!("Ok!");
            }
            OkCancel::Cancel => {
                on_cancel();
                log!("Cancel");
            }
        }
    };

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

    let market_value = move || {
        log!("Market value reactive updated!");
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

    view! { cx,
        <fieldset class="holding" style="margin: 0.5rem;">
            <legend>"Holding"</legend>
            <div class="form">
                <div class="form-row">
                    <label>"Symbol" <SymbolInput symbol_updatable=symbol_updatable/></label>
                    <label>
                        "Current Price" <div style="display: inline-block; margin-left: 0.45em;">
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
                    <span>
                        <strong>
                            <em>{move || market_value()}</em>
                        </strong>
                    </span>
                    <span>
                        <strong>
                            <em>{move || unrealized_gain_loss()}</em>
                        </strong>
                    </span>

                </div>
                <div class="form-row">
                    // <div style="grid-column-start: 1; grid-column-end: 2;">
                    <fieldset>
                        <legend>
                            <strong>"Growth"</strong>
                        </legend>
                        <ItemGrowthComponent
                            updatable=item_growth_updatable
                            dossier_item_type=DossierItemType::Holding
                            growth_item_mappings=&GrowthItemMappings::default()
                        />
                    </fieldset>
                    <fieldset>
                        <legend>
                            <strong>"Distributions"</strong>
                        </legend>
                        <div>"TODO"</div>
                    </fieldset>
                </div>
                <div style="display: flex; justify-content: center; background-color: lightgray;">
                    <div class="display: grid; grid-template-columns: 1fr 1fr; ">
                        <OkCancelComponent on_ok_cancel=on_ok_cancel/>
                    </div>
                </div>
            </div>
        </fieldset>
    }
    // ω <fn holding_component>
}

// α <mod-def holding_component>
// ω <mod-def holding_component>
