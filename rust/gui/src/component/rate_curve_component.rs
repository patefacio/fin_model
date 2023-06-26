//! Module for rate_curve_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::RateCurve;
use plus_modeled::YearValue;
use crate::YearValueInput;
use leptos_dom::html::{Button, Div};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, KeyboardEvent, MouseEvent};

use std::fmt::Display;


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

    enum RateCurveTableTools {
        Edit,
        Remove,
        AddNewYearValue
    }


    let plus_button = create_node_ref::<Button>(cx);
    let minus_button = create_node_ref::<Button>(cx);
    let edit_button = create_node_ref::<Button>(cx);

    let mut year_value_vec = Vec::new();
    let mut year_value_vec = vec![YearValue];
    







    let year_value_row = YearValue::new;//I don't think this actually works 

    //fn add (onclickofplussign)
   // fn remove(onclickofminussign) {year_value_vec.remove(year_value_row)}
    //use year_value_vec.push(next_year_value)
    //year_value_vec.remove(clicked minus sign)
//create a variable that is like year_value_row, so you can add them, or subtract them
//how to make year_value_row an element that includes YearValues
<Button::plus_button>
on::click
    />

}



pub fn is_empty(&self) -> bool {
    self.0.is_empty()
}

pub fn add(&mut self, AddNewYearValue: RateCurveTableTools) {
    year_value_vec.0.push(year_value_row);
}

 pub fn remove(&mut self, id: Uuid) {
     year_value_vec.retain(|year_value_row| year_value_row.id != id);}
     


 let mut year_value_vec = YearValue{year: 1, value: 42.0};
      view! {
        cx,
         <div>"Rate Curve TODO"</div>
        <main>
        
         </main>

                

        







    
    }





    

     // ω <fn rate_curve_component>








// α <mod-def rate_curve_component>
// ω <mod-def rate_curve_component>




