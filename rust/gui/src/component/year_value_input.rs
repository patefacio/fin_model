//! Module for year_value_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
use crate::component::numeric_input::{Modification, NumericInput};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::YearRange;
use plus_modeled::YearValue;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a _year_ combined with a _value_.
///
///   * **cx** - Context
///   * **updatable** - The [YearValue] being edited
///   * _return_ - View for year_value_input
#[component]
pub fn YearValueInput(
    /// Context
    cx: Scope,
    /// The [YearValue] being edited
    updatable: Updatable<Option<YearValue>>,//changed this from <YearRange>
) -> impl IntoView {

    // α <fn year_value_input>
    use crate::NumericInput;
    use crate::YearInput;

    use std::cell::RefCell;
    use std::rc::Rc;

    let updatable = Rc::new(RefCell::new(updatable));

    let initial_year = updatable
        .as_ref()
        .borrow()
        .value
        .as_ref()
        .map(|option_of_initial_year| option_of_initial_year.year);

        let initial_value = updatable
        .as_ref()
        .borrow()
        .value
        .as_ref()
        .map(|option_of_initial_value| option_of_initial_value.value);

        let updatable_for_year = Rc::clone(&updatable);
        let year_updatable = Updatable::new(initial_year, move |initial_year| {
            if let Some(initial_year) = initial_year.clone() {
                updatable_for_year
                    .as_ref()
                    .borrow_mut()
                    .update_and_then_signal(|year| {
                        if let Some(year) = year {
                            year.year = initial_year;
                            
                        }; 
                    });
            }
        });

        let updatable_for_value = Rc::clone(&updatable);
        let value_updatable = Updatable::new(initial_value, move |initial_value| {
            if let Some(initial_value) = initial_value.clone() {
                updatable_for_value
                .as_ref()
                .borrow_mut()
                .update_and_then_signal(|value| {
                    if let Some(value) = value{
                        value.value = initial_value;
                    }
                })
            }
        });



    view! {
        cx,
        <h3>"TODO YearValue"</h3>

        
        <fieldset class="nsg">
        <legend>"Year Value"</legend>
        <div class="form">


            <div style="display: inline-flex" >

                
                    <YearInput
                    placeholder=Some("year".to_string())
                    updatable=year_updatable
                    />
                ","
                    <NumericInput
                        placeholder=Some("value".to_string())
                        updatable=value_updatable
                        non_negative=true
                        modification=Some(Modification::Prefix(("$".into())))
                    />

            </div>

        </div>
        </fieldset>

    }
    // ω <fn year_value_input>
}

// α <mod-def year_value_input>
// ω <mod-def year_value_input>
