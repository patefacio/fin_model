//! Module for holding_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
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
///   * **updatable** - The holding being edited by this component
///   * **on_cancel** - Called if edit is canceled.
///   * _return_ - View for holding_component
#[component]
pub fn HoldingComponent<F>(
    /// Context
    cx: Scope,
    /// The holding being edited by this component
    updatable: Updatable<Holding>,
    /// Called if edit is canceled.
    on_cancel: F,
) -> impl IntoView
where
    F: 'static + FnMut(),
{
    // α <fn holding_component>

    use crate::EnumSelect;
    use crate::NormalSpecComponent;
    use crate::NumericInput;
    use crate::OkCancel;
    use crate::OkCancelComponent;
    use crate::SymbolInput;
    use crate::YearCurrencyValueInput;
    use leptos::create_signal;
    use leptos::store_value;
    use leptos::SignalGet;
    use leptos_dom::console_log;
    use plus_modeled::{
        core_enums::HoldingType,
        growth::{system_growth_id::SystemId, SystemGrowthId},
    };

    let mut on_cancel = on_cancel;
    let holding = &updatable.value;
    let quantity = holding.quantity;
    let unit_valuation = holding.unit_valuation;
    let cost_basis = holding.cost_basis;
    let instrument_name = holding.instrument_name.clone();

    log!("Showing HOLDING {cx:?} {holding:?} with yvc {unit_valuation:?}");

    let updatable = store_value(cx, updatable);

    let symbol_updatable = Updatable::new(instrument_name, move |symbol| {
        updatable.update_value(|updatable| {
            updatable.update(|holding| holding.instrument_name = symbol.clone())
        });
    });

    let quantity_updatable = Updatable::new(Some(quantity), move |quantity| {
        updatable.update_value(|updatable| {
            updatable.update(|holding| {
                if let Some(quantity) = quantity {
                    holding.quantity = *quantity;
                }
            })
        })
    });

    let unit_valuation_updatable = Updatable::new(unit_valuation, move |unit_valuation| {
        log!("Unit value updated {unit_valuation:?}");
        updatable.update_value(|updatable| {
            updatable.update(|holding| holding.unit_valuation = *unit_valuation)
        });
    });

    let (currency_symbol, set_currency_symbol) = create_signal(cx, String::from("$"));

    let cost_basis_updatable = Updatable::new(Some(cost_basis), move |cost_basis| {
        log!("Cost basis updated -> {cost_basis:?}");
        updatable.update_value(|updatable| {
            updatable.update(|holding| {
                if let Some(cost_basis) = cost_basis {
                    holding.cost_basis = *cost_basis;
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
                        <NumericInput updatable=cost_basis_updatable/>
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
                <div class="form-row">
                    <OkCancelComponent on_ok_cancel=on_ok_cancel/>
                </div>
            </div>
        </fieldset>
    }
    // ω <fn holding_component>
}

// α <mod-def holding_component>
// ω <mod-def holding_component>
