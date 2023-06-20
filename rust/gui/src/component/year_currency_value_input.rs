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
///   * **year_placeholder** - Placeholder for the year field
///   * _return_ - View for year_currency_value_input
#[component]
pub fn YearCurrencyValueInput(
    /// Context
    cx: Scope,
    /// Initial value and callback
    updatable: Updatable<Option<YearCurrencyValue>>,
    /// Range of valid years.
    #[prop(default=YearRange{ start: 1900, end: 2300 })]
    year_range: YearRange,
    /// Placeholder for the value field
    #[prop(default="value".to_string())]
    value_placeholder: String,
    /// Placeholder for the year field
    #[prop(default="year".to_string())]
    year_placeholder: String,
) -> impl IntoView {
    // α <fn year_currency_value_input>
    
    let mut updatable = updatable;

    use std::cell::RefCell;
    use std::rc::Rc;

    let updatable = Rc::new(RefCell::new(updatable));

    use crate::CurrencySelect;
    use crate::NumericInput;
    use crate::Updatable;
    use crate::YearInput;
    use plus_modeled::Currency;

    let initial_year = updatable
        .as_ref()
        .borrow()
        .value
        .as_ref()
        .map(|ycv| ycv.year);

    let initial_value = updatable
        .as_ref()
        .borrow()
        .value
        .as_ref()
        .map(|ycv| ycv.value);

    let initial_currency = updatable
        .as_ref()
        .borrow()
        .value
        .as_ref()
        .and_then(|ycv| Currency::from_i32(ycv.currency))
        .unwrap_or_default();

    let updatable_for_currency = Rc::clone(&updatable);
    let currency_select_updatable = Updatable::new(initial_currency, move |new_currency| {
        console_log(&format!("Currency updated to {new_currency:?}"));
        updatable_for_currency
            .as_ref()
            .borrow_mut()
            .update_and_then_signal(|yvc| {
                if let Some(yvc) = yvc {
                    console_log(&format!("Setting year on YVC -> {new_currency:?}"));
                    yvc.currency = *new_currency as i32;
                } else {
                    console_log(&format!(
                        "Setting empty YVC on first change of currency -> {new_currency:?}"
                    ));
                    *yvc = Some(YearCurrencyValue {
                        year: year_range.start,
                        value: 0.0,
                        currency: *new_currency as i32,
                    })
                }

            })
    });

    let updatable_for_year = Rc::clone(&updatable);
    let year_updatable = Updatable::new(initial_year, move |new_year| {
        if let Some(new_year) = new_year.clone() {
            console_log(&format!("The year is -> {new_year:?}"));
            updatable_for_year
                .as_ref()
                .borrow_mut()
                .update_and_then_signal(|yvc| {
                    if let Some(yvc) = yvc {
                        console_log(&format!("Setting year on YVC -> {new_year:?}"));
                        yvc.year = new_year
                    } else {
                        console_log(&format!(
                            "Setting empty YVC on first change of value -> {new_year:?}"
                        ));
    
                        *yvc = Some(YearCurrencyValue {
                            year: new_year,
                            value: 0.0,
                            currency: Currency::Usd as i32,
                        })
                    }
                });
        }
        console_log(&format!("New value -> {new_year:?}"));
    });

    let updatable_for_value = Rc::clone(&updatable);
    let value_updatable = Updatable::new(initial_value, move |new_input| {
        if let Some(new_input) = new_input.clone() {
            updatable_for_value
            .as_ref()
            .borrow_mut()
            .update_and_then_signal(|yvc| {
                if let Some(yvc) = yvc {
                    console_log(&format!("Setting value on YVC -> {new_input:?}"));
                    yvc.value = new_input
                } else {
                    console_log(&format!(
                        "Setting empty YVC on first change of value -> {new_input:?}"
                    ));

                    *yvc = Some(YearCurrencyValue {
                        year: 1900,
                        value: new_input,
                        currency: Currency::Usd as i32,
                    })
                }
            });
        }
        console_log(&format!("New value -> {new_input:?}"));
    });

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

        <div style="display: inline-flex" >

            <CurrencySelect
                updatable = currency_select_updatable
            />

            <NumericInput
                updatable=value_updatable
                placeholder=Some(value_placeholder)
            />

            <div style="vertical-align: bottom;">"As Of"</div>

            <YearInput
                updatable=year_updatable
                year_range=year_range
                placeholder=Some(year_placeholder)
            />

        </div>

        </fieldset>

    }

    // ω <fn year_currency_value_input>
}

// α <mod-def year_currency_value_input>
// ω <mod-def year_currency_value_input>
