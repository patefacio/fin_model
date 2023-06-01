//! Module for holding_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::component::InstrumentGrowthMappings;
use crate::utils::updatable::Updatable;
use fin_model::account::Holding;
use leptos::{component, tracing, view, IntoView, Scope};

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// A single holding in an account.
///
///
///   * **cx** - Context
///   * **updatable** - The holding being edited by this component
///   * **updatable_mappings** - The mappings that this holding may impact if its name is shared among other
/// holdings in the balance sheet
///   * _return_ - View for holding_component
#[component]
pub fn HoldingComponent<F, M>(
    /// Context
    cx: Scope,
    /// The holding being edited by this component
    updatable: Updatable<Holding, F>,
    /// The mappings that this holding may impact if its name is shared among other
    /// holdings in the balance sheet
    updatable_mappings: Updatable<InstrumentGrowthMappings, M>,
) -> impl IntoView
where
    F: FnMut(&Holding) + 'static,
    M: FnMut(&InstrumentGrowthMappings),
{
    // α <fn holding_component>

    use crate::component::numeric_input::{NumericInput, Modification};
    use crate::component::enum_select::EnumSelect;
    use crate::component::normal_spec_growth::NormalSpecGrowth;
    use fin_model::core_enums::HoldingType;
    use std::rc::Rc;
    use std::cell::RefCell;
    use leptos::create_signal;

    let holding_updatable = Rc::new(RefCell::new(updatable));

    let (currency_symbol, set_currency_symbol) = create_signal(cx, String::from("$"));
    let share_price_placeholder = Some(format!("e.g. {}50.00", currency_symbol()));

    let holding_updatable_for_price = holding_updatable.clone();
    let on_price_updated = move |price: f64| {
        let mut current = holding_updatable_for_price.borrow_mut();
        current.update(|h| {
            if let Some(ycv) = h.unit_valuation.as_mut() {
                ycv.value = price;
            }
        });
    };

    let holding_updatable_for_quantity = holding_updatable.clone();
    let on_quantity_updated = move |quantity: f64| {
        let mut current = holding_updatable_for_quantity.borrow_mut();
        current.update(|h| {
            h.quantity = quantity;
        });
    };

    let holding_updatable_for_holding_type = holding_updatable.clone();
    let on_holding_type_updated = move |holding_type: HoldingType| {
        let mut current = holding_updatable_for_holding_type.borrow_mut();
        current.update(|h| {
            //h.
        })
    };
   
   // let holding_type_updateable = Updatable::new(updatable.value., on_update)

    view! { cx,
        <fieldset class="holding">
        <legend>"Holding"</legend>

        <div class="form">

            <div class="form-row">
                <div>
                    <label>"Holding Type"</label>
      //TODO              <EnumSelect enumeration=HoldingType on_selection=on_holding_type_updated />
                </div>
                <div>
                    <label for="symbol">"Symbol"</label>
                    <input id="symbol" placeholder="Symbol" style="text-transform:uppercase"/>
                </div>
            </div>

            <div class="form-row">
            <div>
                <label>"Growth"</label>
                <label>"N(10.31%, 20.32%)"</label>
            </div>
            <div>
                <label>"Override"</label>
           // TODO     <NormalSpecGrowth />
            </div>
        </div>


            <div class="form-row">
                <div class="price">
                <div class="button-and-price">
                    <div class="currency-button">
        //TODO                <CurrencyInput on_selection=on_select_currency/>
                    </div>
                    <label>"Price"</label>
                </div>
                    <NumericInput
                        input_class=Some("input-share-price".into())
                        on_update=on_price_updated
                        placeholder=share_price_placeholder
                        modification=Some(Modification::ReactivePrefix(currency_symbol))
                    />
                </div>

                <div>
                    <label for="quantity">"Quantity"</label>
                    <NumericInput 
                        input_class=Some("input-share-count".into()) 
                        on_update=on_quantity_updated
                        placeholder=Some("Share Count".to_string())/>
                </div>
            </div>

/*
            <div class="form-row">
                <div class="cost-basis-wrapper">
                    <div>
                            <label>"Cost Basis"</label>
                            <NumericInput
                                input_class=Some("cost-basis".into())
                                on_update=number_updated
                                placeholder="Total Cost".into()
                                modification=Some(Modification::ReactivePrefix(currency_symbol))
                            />
                            //<div class="cost-basis"><div class="currency-symbol">{currency_symbol}</div><input type="text"/></div>
                    </div>
                    <div class="as-of-group">
                            <label>"As Of"</label>
                            <NumericInput 
                                input_class=Some("as-of".into()) 
                                on_update=number_updated
                                placeholder="Year".into()/>
                    </div>
                </div>
            </div>
*/
        </div>

        </fieldset>
    }
    // ω <fn holding_component>
}

// α <mod-def holding_component>
// ω <mod-def holding_component>
