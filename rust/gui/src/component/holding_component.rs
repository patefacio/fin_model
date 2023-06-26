//! Module for holding_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
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
///   * **holding_updatable** - The holding being edited by this component
///   * _return_ - View for holding_component
#[component]
pub fn HoldingComponent(
    /// Context
    cx: Scope,
    /// The holding being edited by this component
    holding_updatable: Updatable<Holding>,
) -> impl IntoView {
    // α <fn holding_component>

    use crate::EnumSelect;
    use crate::NormalSpecComponent;
    use crate::{Modification, NumericInput};
    use crate::SymbolInput;
    use crate::YearInput;
    use plus_modeled::{
        core_enums::HoldingType,
        growth::{system_growth_id::SystemId, SystemGrowthId},
    };
    use leptos::create_signal;
    use leptos_dom::console_log;
    use std::cell::RefCell;
    use std::rc::Rc;

    let unit_valuation = holding_updatable.value.unit_valuation;
    let instrument_name = holding_updatable.value.instrument_name.clone();
    let holding_updatable = Rc::new(RefCell::new(holding_updatable));

    let symbol_updatable = Updatable::new(instrument_name.clone(), |symbol| {
        console_log(&format!("Symbol is now {symbol:?}"));
    });

    let (currency_symbol, set_currency_symbol) = create_signal(cx, String::from("$"));
    let share_price_placeholder = Some(format!("e.g. {}50.00", currency_symbol()));

    let holding_updatable_for_price = holding_updatable.clone();

    let price_updatable = Updatable::new(
        unit_valuation
            // Map the YearValueCurrency to just the value part
            .map(|year_value_currency| year_value_currency.value),
        move |price: &Option<f64>| {
            // No need to use it until ok/cancel
        },
    );

    let holding_updatable_for_quantity = holding_updatable.clone();
    let on_quantity_updated = move |quantity: f64| {
        let mut current = holding_updatable_for_quantity.borrow_mut();
        current.update(|h| {
            h.quantity = quantity;
        });
    };

    let cost_basis_updatable = Updatable::new(0.0, |year| {
        leptos_dom::console_log(&format!("Cost basis updated to {year}"));
    });

    let year_updatable = Updatable::new(1929, |year| {
        leptos_dom::console_log(&format!("Year updated to {year}"));
    });

    /*

    let mappings_updatable_for_holding_type = mappings_updatable.clone();
    let holdings_updatable_for_holding_type = holding_updatable.clone();
    let on_holding_type_updated = move |holding_type: &HoldingType| {
        let mut current_mappings = mappings_updatable_for_holding_type.borrow_mut();
        let current_holding = holdings_updatable_for_holding_type.borrow();
        let instrument_name = &current_holding.value.instrument_name;
        current_mappings.update(|mappings| {
            mappings
                .instrument_growth_map
                .entry(instrument_name.clone())
                .and_modify(|instrument_growth| {
                    instrument_growth.item_growth.system_growth_id = Some(SystemGrowthId {
                        system_id: Some(SystemId::HoldingItemId(*holding_type as i32 as u32)),
                    });
                });
        })
    };
    */

    let initial_holding_type = HoldingType::UsEquityMarket;
    

    let holding_type_updatable =
        Updatable::new(initial_holding_type, |_| println!("Updated!"));

    view! { cx,
        <fieldset class="holding">
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
                        <label>"Growth"</label>
                        <label>"N(10.31%, 20.32%)"</label>
                    </div>
                    <div>
                        <label>"Override"</label>
                    </div>
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

        debug_assert!(edit_holding_index.holding_index.is_some());
        use std::collections::hash_map::Entry;

        let edit_holding_within_account = edit_holding_index
            .holding_index
            .expect("Valid holding index") as usize;

        // Get the holding associated with the edit_holding_index
        let edit_holding = &balance_sheet.accounts[edit_holding_index.account_index as usize]
            .holdings[edit_holding_within_account];

        // The balance sheet holds the real instrument growth mappings which we will pull from
        let bs_mappings = &balance_sheet.instrument_growth_mappings;

        let mut instrument_growth_map = InstrumentGrowthMappings::new();

        for (account_index, account) in balance_sheet.accounts.iter().enumerate() {
            for (holding_index, holding) in account.holdings.iter().enumerate() {
                // Is this holding the edit_holding?
                let is_edit_holding = edit_holding_index.account_index == account_index as u32
                    && edit_holding_within_account == holding_index;
                // Get the entry for growth for this holding's key
                let entry = instrument_growth_map.entry(holding.instrument_name.clone());

                match entry {
                    Entry::Occupied(entry) => (),
                    Entry::Vacant(entry) => (),
                }

                let is_same_instrument = holding.instrument_name == edit_holding.instrument_name;
            }
        }

        InstrumentGrowthSync {
            instrument_growth_map,
        }

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
