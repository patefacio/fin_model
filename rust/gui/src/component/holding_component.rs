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
// --- type aliases ---
////////////////////////////////////////////////////////////////////////////////////
pub(crate) type InstrumentGrowthMappings = HashMap<String, InstrumentGrowth>;

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct InstrumentGrowth {
    /// The modeled growth for the holding
    pub item_growth: ItemGrowth,
    /// Set to false for all instrument names not being edited
    pub only_editable: bool,
}

/// Contains the mapping of {instrument_name -> growth} for all holdings in the balance sheet.
/// This struct is used to keep the growth of all symbols in sync while editing a *single*
/// holding. The growth of symbols can be _shared_ across accounts. If a component is editing
/// the growth of a holding named IBM in a particular account and the user also has that symbol in
/// a different account, it is important that the growth of those two be linked. This is enforced  
/// by the BalanceSheet datatype which stores the growths for all holdings in a map indexed
/// by the instrument name (i.e. String).
///
/// When editing a single Holding we have to allow for the holding to have an _instrument name_
/// change. Suppose they have a holding they are editing named "IBM" and they change the name
/// to "MSFT". Maybe they were confused and did not realize they had already edited the "IBM" holding
/// and decided to change the current holding being edited to "MSFT" - because they also have some
/// "MSFT" in the account. At this point you have to decide how to treat the "IBM" entry in the map.
/// Should it stay or should it be deleted? If there were another holding besides the one being
/// edited that had been modeled then the "IBM" entry should stay. If however, there were no other
/// "IBM" named holding in the balance sheet, then it should be removed. So, to satisfy this
/// requirement of only removing a mapping if no other holding has that name we need to snapshot
/// the set of holding names of all holdings **that are not being edited**.
///
/// This is achieved by keeping the set of symbol names for all holdings excluding the one
/// being edited.
#[derive(Debug, Clone)]
pub struct InstrumentGrowthSync {
    /// The mapping of **all** instrument names to their current growths.
    pub instrument_growth_map: InstrumentGrowthMappings,
}

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

    use crate::CurrencyValueInput;
    use crate::EnumSelect;
    use crate::NormalSpecComponent;
    use crate::NumericInput;
    use crate::OkCancel;
    use crate::OkCancelComponent;
    use crate::SymbolInput;
    use crate::YearCurrencyValueInput;
    use crate::YearInput;
    use leptos::create_signal;
    use leptos::store_value;
    use leptos::SignalGet;
    use leptos_dom::console_log;
    use plus_modeled::{
        core_enums::HoldingType,
        growth::{system_growth_id::SystemId, SystemGrowthId},
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    let mut on_cancel = on_cancel;
    let holding = &updatable.value;
    let quantity = holding.quantity;
    let unit_valuation = holding.unit_valuation;
    let cost_basis = holding.cost_basis;
    let instrument_name = holding.instrument_name.clone();

    log!("Showing HOLDING {holding:?} with yvc {unit_valuation:?}");

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
                log!("Updating holding -> {holding:?}");
                if let Some(cost_basis) = cost_basis {
                    log!("Updating holding cost_basis -> {cost_basis:?}");
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

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl InstrumentGrowthSync {
    /// Create new sync object
    ///
    ///   * **balance_sheet** - The balance sheet.
    ///   * **edit_holding_index** - Index of holding being edited
    ///   * _return_ - The new sync instance.
    pub fn new(balance_sheet: &BalanceSheet, edit_holding_index: DossierHoldingIndex) -> Self {
        // α <fn InstrumentGrowthSync::new>
        todo!("Implement `new`")
        // ω <fn InstrumentGrowthSync::new>
    }
}

/// Unit tests for `holding_component`
#[cfg(test)]
pub mod unit_tests {

    /// Test type InstrumentGrowthSync
    mod test_instrument_growth_sync {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn new() {
            // α <fn test InstrumentGrowthSync::new>
            todo!("Test new")
            // ω <fn test InstrumentGrowthSync::new>
        }

        // α <mod-def test_instrument_growth_sync>
        // ω <mod-def test_instrument_growth_sync>
    }

    // α <mod-def unit_tests>
    // ω <mod-def unit_tests>
}

// α <mod-def holding_component>
// ω <mod-def holding_component>
