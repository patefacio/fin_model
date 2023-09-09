//! Module for holdings_grid leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::CollectionGrid;
use crate::CollectionGridEditType;
use crate::CollectionGridState;
use crate::SymbolGrowthMap;
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::StoredValue;
use leptos::WriteSignal;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::console_log;
use leptos_dom::View;
use plus_modeled::Holding;
use std::collections::HashSet;

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// The shared context required to edit a single holding in a collection of holdings
/// within an account. The [Holding](plus_modeled::Holding) contains a symbol
/// name. which may be shared among many holdings in different accounts. In fact,
/// the [Holding] does not contain the growth of the symbol it refers to.
/// Rather the symbol name is the key to the growth that is stored in the
/// [BalanceSheet](plus_modeled::BalanceSheet). This relationship implies that
/// when editing the growth of a holding that it is really editing the growth
/// of the symbol it refers to (which may be shared). In order to edit a holding
/// **and** allow for the symbol growth of that holding's symbol to be updated
/// this context needs to have that mapping of all symbols. In addition, we want
/// to prevent creating a new holding with the same symbol name in the same account.
/// The `symbol_names` set enables the component to prevent either creating a new
/// holding that is already present or changing the name of holding being edited
/// to another symbol.
#[derive(Debug, Clone, Default)]
pub struct HoldingSharedContext {
    /// Map of symbols to their growths
    pub symbol_growth_map: SymbolGrowthMap,
    /// Set of already taken symbol names
    pub symbol_names: HashSet<String>,
}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Display and edit support for list of holdings in an account
///
///   * **holdings_updatable** - Holdings to edit
///   * **shared_context_updatable** - The shared context
///   * _return_ - View for holdings_grid
#[component]
pub fn HoldingsGrid(
    /// Holdings to edit
    holdings_updatable: Updatable<Vec<Holding>>,
    /// The shared context
    shared_context_updatable: Updatable<HoldingSharedContext>,
) -> impl IntoView {
    // α <fn holdings_grid>

    use crate::CollectionGridComponent;

    view! {
        <CollectionGridComponent
            header=vec![
                "Symbol".to_string(), "Market Value".to_string(), "Cost Basis".to_string(),
                "Unrealized (G/L)".to_string(),
            ]

            rows_updatable=holdings_updatable
            shared_context_updatable=shared_context_updatable
            add_item_label="Add New Holding".to_string()
        />
    }

    // ω <fn holdings_grid>
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl CollectionGrid for Holding {
    type SharedContext = HoldingSharedContext;
    /// Get the display fields for the element.
    ///
    ///   * _return_ - The fields as elements
    fn get_fields(&self) -> Vec<View> {
        // α <fn CollectionGrid::get_fields for Holding>

        use leptos::IntoStyle;
        use plus_modeled::CurrencyValue;
        use plus_modeled::YearCurrencyValue;

        let (market_value, cost_basis, gain_loss) =
            if let Some(unit_valuation) = self.unit_valuation {
                let year = unit_valuation.year;
                let currency = unit_valuation.currency;
                let market_value = unit_valuation.value * self.quantity;
                (
                    format!(
                        "{}",
                        YearCurrencyValue {
                            year,
                            currency,
                            value: market_value
                        }
                    ),
                    format!(
                        "{}",
                        CurrencyValue {
                            currency,
                            value: self.cost_basis
                        }
                    ),
                    format!(
                        "{}",
                        CurrencyValue {
                            currency,
                            value: market_value - self.cost_basis
                        }
                    ),
                )
            } else {
                (
                    String::default(),
                    format!("{:.2}", self.cost_basis),
                    String::default(),
                )
            };

        vec![
            view! {
                <div class="cgc-header-cell"
                style:text-align="right">{self.instrument_name.clone()}</div>
            }
            .into_view(),
            view! {
                <div class="cgc-header-cell"
                style:text-align="right">{market_value}</div>
            }
            .into_view(),
            view! {
                <div class="cgc-header-cell"
                style:text-align="right">{cost_basis}</div>
            }
            .into_view(),
            view! {
                <div
                class="cgc-header-cell"
                style:text-align="right">{gain_loss}</div>
            }
            .into_view(),
        ]

        // ω <fn CollectionGrid::get_fields for Holding>
    }

    /// Get key that uniquely identifies the element.
    ///
    ///   * _return_ - The key for the element
    fn get_key(&self) -> String {
        // α <fn CollectionGrid::get_key for Holding>
        self.instrument_name.clone()
        // ω <fn CollectionGrid::get_key for Holding>
    }

    /// Create new element to edit
    ///
    ///   * _return_ - New element
    fn new() -> Self {
        // α <fn CollectionGrid::new for Holding>
        Holding::default()
        // ω <fn CollectionGrid::new for Holding>
    }

    /// Create a view to edit the row
    ///
    ///   * **edit_type** - Type of edit
    ///   * **row_stored_value** - Row to edit.
    ///   * **shared_context_stored_value** - Updatable containing the shared context.
    ///   * _return_ - The edit view
    fn edit_row(
        edit_type: CollectionGridEditType,
        row_stored_value: StoredValue<Self>,
        shared_context_stored_value: StoredValue<Self::SharedContext>,
    ) -> View {
        // α <fn CollectionGrid::edit_row for Holding>

        use crate::HoldingComponent;

        view! {
            <HoldingComponent
                holding_stored_value=row_stored_value
                shared_context_stored_value=shared_context_stored_value
            />
        }

        // ω <fn CollectionGrid::edit_row for Holding>
    }

    /// Return true if row edit satisfies any shared context constraints
    ///
    ///   * **edited_row** - The edited row
    ///   * **shared_context** - The current shared context
    ///   * _return_ - An error message if not acceptable change, None otherwise
    fn accept_row_edit(
        edited_row: &Self,
        shared_context: &mut Self::SharedContext,
    ) -> Option<String> {
        // α <fn CollectionGrid::accept_row_edit for Holding>
        todo!("Implement `accept_row_edit`")
        // ω <fn CollectionGrid::accept_row_edit for Holding>
    }
}

// α <mod-def holdings_grid>
// ω <mod-def holdings_grid>
