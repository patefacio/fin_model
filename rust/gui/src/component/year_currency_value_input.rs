//! Module for year_currency_value_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use crate::component::numeric_input::{Modification, NumericInput};
use leptos::{component, view, IntoView, Scope};
use crate::YearClamp;
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::core::{YearCurrencyValue, YearRange};
use plus_modeled::Currency;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Input for combined (year, currency, value).
///
///   * **cx** - Context
///   * **updatable** - Initial value and callback
///   * **year_range** - Range of valid years.
///   * **value_placeholder** - Placeholder for the value field
///   * **date_placeholder** - Placeholder for the asOf field
///   * _return_ - View for year_currency_value_input
#[component]
pub fn YearCurrencyValueInput(
    /// Context
    cx: Scope,
    /// Initial value and callback
    updatable: Updatable<YearCurrencyValue>,
    //updatable: Updatable<Option<f64>>,
    /// Range of valid years.
    #[prop(default=YearRange{ start: 1900, end: 2300 })]
    year_range: YearRange,
    /// Placeholder for the value field
    #[prop(default="Value".to_string())]
    value_placeholder: String,
    /// Placeholder for the asOf field
    #[prop(default="AsOf".to_string())]
    date_placeholder: String,
) -> impl IntoView {
    // α <fn year_currency_value_input>
    
    let mut updatable = updatable;

    use leptos_dom::{html::Input};

    use leptos::IntoAttribute;

    let initial_value = updatable.value.clone();

    use leptos::create_node_ref;
    //use leptos_dom::{console_log, html::Input};

    let input_ref = create_node_ref::<Input>(cx);
    let node_ref = create_node_ref::<Input>(cx);

    let year_clamp = YearClamp::new(year_range);
    
    use std::cell::RefCell;
    use std::rc::Rc;

    let updates = Vec::<String>::new();
    let updates = Rc::new(RefCell::new(updates));
    let updates_to_move = Rc::clone(&updates);


    


    let mut update_value = move || {

        // First get the HtmlElement<Input>, think of it as a handle that provides
        // access to the input element in the DOM that holds our number.
        let input_ref = node_ref.get().expect("Year input node");

        // From the input element get the current value, which is just the
        // text the user has typed in the input field so far.
        let mut value = input_ref.value();

        // First cull any non numeric characters. Years must be all ascii digits,
        // no negative or decimal allowed. (Could this be faster?)
        value = value.chars().filter(|c| c.is_ascii_digit()).collect();

        leptos_dom::console_log(&format!("YearInput: filtered -> {value:?}"));

        if value.is_empty() && false {
            // No characters in the input are valid digits - the value is now None
            //updatable.update(|year| *year = None);
            //leptos_dom::console_log(&format!("YearInput: setting value to -> None"));
            //input_ref.set_value("");
            ;
        } else {
            // We have valid digits, pass to clamp to ensure value is in range.
            // Update the input by setting the value to the clamped_year. This may
            // be off-putting and we should discuss. Imagine typing "2303" and on
            // entering that last 3 what gets displayed is "2300". It will certainly
            // ensure the value is in range but maybe the approach is heavy handed.
            let clamped  = year_clamp.clamp(&value);
            updatable.update(|year| *year = YearCurrencyValue{year: clamped.as_u32, currency:0, value:0.0});
            input_ref.set_value(&clamped.as_string);
            leptos_dom::console_log(&format!("YearInput: setting value to -> {clamped:?}"));
        }
    };

    //let _u1 = Updatable::new(Some(0.3), update_value());
    
    use crate::CurrencySelect;
    use crate::Updatable;



    view! { cx,

        <fieldset class="year-currency-value">
        <legend>"Currency/Value/Year"</legend>
        
        <input 
            node_ref=input_ref
            on:input=move |_| update_value()
            type="text"
            placeholder="lol".to_string()+&value_placeholder
        />

        <CurrencySelect
            updatable=Updatable::new(
                Currency::Usd,
                |currency|{
                    console_log(&format!("Currency Update"))
                }

            )
        />

        //<NumericInput
        //            updatable=updatable
        //            modification=Some(Modification::Suffix("%".into()))
        //            non_negative=true
        //        />


        
        
        <div>"As Of"</div>

        <input
            type="text"
            placeholder=date_placeholder
        />

        </fieldset>
        
    }

    // ω <fn year_currency_value_input>
}

// α <mod-def year_currency_value_input>
// ω <mod-def year_currency_value_input>
