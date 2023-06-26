//! Module for rate_curve_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use crate::Year;
use crate::YearClamp;
use crate::YearValueInput;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use leptos_dom::html::{Button, Div};
use plus_modeled::RateCurve;
use plus_modeled::YearValue;
use std::fmt::Display;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, KeyboardEvent, MouseEvent};

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models an **ordered** series of [YearValue](plus_modeled::YearValue) pairs that together constitute a
/// piece-wise step function. The component ensures the ordering (i.e. the years in the
/// vector of [YearValue](plus_modeled::YearValue) are strictly increasing)
///
///
///   * **cx** - Context
///   * **updatable** - The [RateCurve] being edited
///   * _return_ - View for rate_curve_component
#[component]
pub fn RateCurveComponent(
    /// Context
    cx: Scope,
    /// The [RateCurve] being edited
    updatable: Updatable<RateCurve>,
) -> impl IntoView {
    // α <fn rate_curve_component>
    use crate::Modification;
    use crate::NumericInput;
    use crate::PercentInput;
    use crate::Updatable;
    use crate::YearInput;
    use crate::YearRangeInput;
    use leptos::create_signal;
    use leptos::For;
    use leptos::*;
    use plus_modeled::YearValue;

    let (rate_curve, set_rate_curve) = create_signal(cx, updatable.value);

    /*let plus_button = create_node_ref::<Button>(cx);
    let minus_button = create_node_ref::<Button>(cx);
    let edit_button = create_node_ref::<Button>(cx);

    //This kind of thing was on the leptos TODO program. It was how he added new todos or got rid of deleted ones

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn add(&mut self, AddNewYearValue: RateCurveTableTools) {
        year_value_vec.0.push(year_value_row);
    }

    pub fn remove(&mut self, id: Uuid) {
        year_value_vec.retain(|year_value_row| year_value_row.id != id);
    }*/
    view! {
    cx,

    <div>
                     <div style="display: inline-flex;">
                         <div></div>
                         <div></div>
                         <div><h6>"Year"</h6></div>
                         <div><h6>"Rate"</h6></div>
                    </div>
    </div>




             <For

                 each=move || {rate_curve.get().curve.into_iter().enumerate()}
                 key=|entry| { entry.1.year }
                 view=|cx, element| view! {
                     cx,
                     <div>
                     <div style="display: inline-flex;">
                         <button>"🗑"</button>
                         <button>"✎"</button>
                         <YearInput
                             updatable=Updatable::new(Some(element.1.year), |y| {console_log("Year is updating")})
                             placeholder=Some("year".to_string())

                         />
                         <PercentInput
                             updatable=Updatable::new(Some(element.1.value), |p|{

                             console_log(&format!("Percent is updating for index {p:?}" ));
                             //console_log(&format!("Percent is updating for index {}", element.clone().0));
                            })
                             placeholder=Some("rate".to_string())

                         />

                     </div>
                     </div>
                     //make an updatable that finds the year; years are unique, so this will
                     //tell you which value it is that is being updated
                     //find index of year in vector to update the changed value
                     //figure out sorting by year

                 }
             />
             <div>
             <div style="display: inline-flex;">
                 <div></div>
                 <button>"+"</button>
                 <YearInput
                     updatable=Updatable::new(None, |y| {console_log("Year is updating")})
                     placeholder=Some("year".to_string())
                     
                 />
                 <PercentInput
                     updatable=Updatable::new(None, |p|
                     {console_log(&format!("Percent is updating => {p:?}"))})
                     placeholder=Some("rate".to_string())

                 />

             </div>
             </div>



             <div>"Rate Curve TODO"</div>
         }
}

// ω <fn rate_curve_component>

// α <mod-def rate_curve_component>
// ω <mod-def rate_curve_component>
