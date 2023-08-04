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
    use crate::Modification;
    use crate::NormalSpecComponent;
    use crate::NumericInput;
    use crate::OkCancel;
    use crate::OkCancelComponent;
    use crate::SymbolInput;
    use crate::YearCurrencyValueInput;
    use crate::UpdatePairType;
    use leptos::create_rw_signal;
    use leptos::create_signal;
    use leptos::store_value;
    use leptos::MaybeSignal;
    use leptos::SignalGet;
    use leptos::SignalUpdate;
    use leptos::SignalWith;
    use leptos_dom::console_log;
    use plus_modeled::Currency;
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

    log!("Showing HOLDING {cx:?} {holding:?} with yvc {unit_valuation:?}");

    let updatable = store_value(cx, updatable);

    let symbol_updatable = Updatable::new(instrument_name, move |symbol| {
        updatable.update_value(|updatable| {
            updatable.update(|(holding, shared_context)| {
                holding.instrument_name = symbol.clone();
                crate::UpdatePairType::UpdateFirst
            })
        });
    });

    let quantity_updatable = Updatable::new(Some(quantity), move |quantity| {
        updatable.update_value(|updatable| {
            updatable.update(|(holding, shared_context)| {
                if let Some(quantity) = quantity {
                    holding.quantity = *quantity;
                    UpdatePairType::UpdateFirst
                } else {
                    UpdatePairType::UpdateNone
                }
            })
        })
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
        updatable.update_value(|updatable| {
            updatable.update(|(holding, shared_context)| {
                holding.unit_valuation = *unit_valuation;
                UpdatePairType::UpdateFirst
            }) 
        });
    });

    let cost_basis_updatable = Updatable::new(Some(cost_basis), move |cost_basis| {
        log!("Cost basis updated -> {cost_basis:?}");
        currency_rw_signal.track();
        updatable.update_value(|updatable| {
            updatable.update(|(holding, shared_context)| {
                if let Some(cost_basis) = cost_basis {
                    holding.cost_basis = *cost_basis;
                    UpdatePairType::UpdateFirst
                } else {
                    UpdatePairType::UpdateNone
                }
            })
        });
    });

    // Get actual initial holding type from instrument mapping in balance sheet
    let initial_holding_type = HoldingType::UsEquityMarket;
    let holding_type_updatable = Updatable::new(initial_holding_type, |_| println!("Updated!"));

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

    view! { cx,
        <fieldset class="holding" style="margin: 0.5rem;">
            <legend>"Holding"</legend>
            <div class="form">
                <div class="form-row">
                    <div>
                        <label for="symbol">"Symbol"</label>
                        <SymbolInput symbol_updatable=symbol_updatable/>
                    </div>
                    <div>
                        <label>"Holding Type"</label>
                        <EnumSelect updatable=holding_type_updatable/>
                    </div>
                </div>
                <div class="form-row">
                    <div>
                        <label for="quantity">"Quantity"</label>
                        <NumericInput updatable=quantity_updatable/>
                    </div>
                    <div>
                        <label for="unit-value">"Price"</label>
                        <YearCurrencyValueInput
                            updatable=unit_valuation_updatable
                            value_placeholder="price".to_string()
                        />
                    </div>
                    <div>
                        <label for="cost">"Cost"</label>
                        <NumericInput
                            updatable=cost_basis_updatable
                            modification=Some(Modification::Prefix(MaybeSignal::Dynamic(currency_rw_signal.into())))
                        />
                    </div>
                </div>
                <div class="form-row">
                    <div>
                        <label>"Growth"</label>
                        <label>"N(10.31%, 20.32%)"</label>
                    </div>
                    <div>
                        <label>"Override (Advanced)"</label>
                        <button>"🚀"</button>
                    </div>
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
