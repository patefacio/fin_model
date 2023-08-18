//! Module for holdings_grid leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::CollectionGrid;
use crate::CollectionGridState;
use crate::SymbolGrowthMap;
use crate::Updatable;
use crate::UpdatablePair;
#[allow(unused_imports)]
use leptos::log;
use leptos::WriteSignal;
use leptos::{component, view, IntoView, Scope};
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
///   * **cx** - Context
///   * **updatable_pair** - The account to edit with shared context
///   * **on_state_change** - Enables parent to track state changes.
/// For example, parent may want different behavior when editing an entry
/// versus just displaying the rows.
///   * _return_ - View for holdings_grid
#[component]
pub fn HoldingsGrid(
    /// Context
    cx: Scope,
    /// The account to edit with shared context
    updatable_pair: UpdatablePair<Vec<Holding>, HoldingSharedContext>,
    /// Enables parent to track state changes.
    /// For example, parent may want different behavior when editing an entry
    /// versus just displaying the rows.
    #[prop(default=None)]
    on_state_change: Option<WriteSignal<CollectionGridState>>,
) -> impl IntoView {
    // α <fn holdings_grid>

    use crate::CollectionGridComponent;

    view! { cx,
        <CollectionGridComponent
            header=vec![
                "Symbol".to_string(), "Market Value".to_string(), "Cost Basis".to_string(),
                "Unrealized (G/L)".to_string(),
            ]

            updatable=updatable_pair
            add_item_label="Add New Holding".to_string()
            on_state_change=on_state_change
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
    ///   * **cx** - The context for the fields
    ///   * _return_ - The fields as elements
    fn get_fields(&self, cx: Scope) -> Vec<View> {
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
            view! { cx,

                <div class="holding-header-cell"
                style:text-align="right">{self.instrument_name.clone()}</div>
            }
            .into_view(cx),
            view! { cx,
                <div class="holding-header-cell"
                style:text-align="right">{market_value}</div>
            }
            .into_view(cx),
            view! { cx,
                <div class="holding-header-cell"
                style:text-align="right">{cost_basis}</div>
            }
            .into_view(cx),
            view! { cx,
                <div
                class="holding-header-cell"
                style:text-align="right">{gain_loss}</div>
            }
            .into_view(cx),
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

    /// Create a view to edit the element
    ///
    ///   * **cx** - Context
    ///   * **updatable** - Updatable containing the element to edit.
    /// This component will update the vector whenever the element is signaled
    /// by finding the proper element in the vector and replacing it with the update.
    ///   * **on_cancel** - Called if edit is canceled.
    ///   * _return_ - The edit view
    fn edit_element<F>(
        cx: Scope,
        updatable: UpdatablePair<Self, Self::SharedContext>,
        on_cancel: F,
    ) -> View
    where
        F: 'static + FnMut(&str),
    {
        // α <fn CollectionGrid::edit_element for Holding>

        use crate::HoldingComponent;
        let key = updatable.first_value.get_key();
        let mut on_cancel = on_cancel;
        let on_cancel = move || {
            on_cancel(&key);
        };

        leptos::log!(
            "EDITING element -> `{:?}` with shared: -> {:?}",
            updatable.first_value,
            updatable.second_value
        );

        view! { cx, <HoldingComponent updatable=updatable on_cancel=on_cancel/> }.into_view(cx)

        // ω <fn CollectionGrid::edit_element for Holding>
    }
}

// α <mod-def holdings_grid>
// ω <mod-def holdings_grid>
