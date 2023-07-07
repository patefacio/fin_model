//! Module for year_currency_value_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::core::{YearCurrencyValue, YearRange};

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
    #[prop(default=YearRange{ start: 1900, end: 2400 })]
    year_range: YearRange,
    /// Placeholder for the value field
    #[prop(default="value".to_string())]
    value_placeholder: String,
    /// Placeholder for the year field
    #[prop(default="year".to_string())]
    year_placeholder: String,
) -> impl IntoView {
    // α <fn year_currency_value_input>

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
            .update_and_then_signal(|ycv| {
                if let Some(ycv) = ycv {
                    console_log(&format!("Setting year on ycv -> {new_currency:?}"));
                    ycv.currency = *new_currency as i32;
                } else {
                    console_log(&format!(
                        "Setting empty ycv on first change of currency -> {new_currency:?}"
                    ));
                    *ycv = Some(YearCurrencyValue {
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
                .update_and_then_signal(|ycv| {
                    if let Some(ycv) = ycv {
                        console_log(&format!("Setting value on ycv -> {new_input:?}"));
                        ycv.value = new_input
                    } else {
                        console_log(&format!(
                            "Setting empty YVC on first change of value -> {new_input:?}"
                        ));

                        *ycv = Some(YearCurrencyValue {
                            year: 1900,
                            value: new_input,
                            currency: Currency::Usd as i32,
                        })
                    }
                });
        }
        console_log(&format!("New value -> {new_input:?}"));
    });

    view! { cx,
        <div class="ycv" style="display: flex">
            <div class="ycv-currency">
                <CurrencySelect updatable=currency_select_updatable/>
            </div>
            <div class="ycv-value">
                <NumericInput updatable=value_updatable placeholder=Some(value_placeholder)/>
            </div>
            <div class="ycv-as-of">"As Of"</div>
            <div class="ycv-year">
                <YearInput
                    updatable=year_updatable
                    year_range=year_range
                    placeholder=Some(year_placeholder)
                />
            </div>
        </div>
    }

    // ω <fn year_currency_value_input>
}

// α <mod-def year_currency_value_input>
// ω <mod-def year_currency_value_input>
