//! Module for year_currency_value_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;
use plus_modeled::core::{YearCurrencyValue, YearRange};

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Input for combined (year, currency, value).
///
///   * **updatable** - Initial value and callback
///   * **year_range** - Range of valid years.
///   * **value_placeholder** - Placeholder for the value field
///   * **year_placeholder** - Placeholder for the year field
///   * _return_ - View for year_currency_value_input
#[component]
pub fn YearCurrencyValueInput(
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

    use crate::to_currency_symbol;
    use crate::CurrencySelect;
    use crate::Modification;
    use crate::NumericInput;
    use crate::Updatable;
    use crate::YearInput;
    use leptos::create_signal;
    use leptos::store_value;
    use leptos::MaybeSignal;
    use leptos::SignalUpdate;
    use plus_modeled::Currency;
    use plus_utils::this_year;

    let initial_year = updatable
        .value
        .as_ref()
        .map(|ycv| ycv.year)
        .or_else(|| Some(this_year()));

    let initial_value = updatable.value.as_ref().map(|ycv| ycv.value);

    let initial_currency = updatable
        .value
        .as_ref()
        .and_then(|ycv| Currency::from_i32(ycv.currency))
        .unwrap_or_default();

    let updatable = store_value(updatable);

    let (currency_prefix, set_currency_prefix) =
        create_signal(to_currency_symbol(initial_currency).to_string());

    let currency_select_updatable = Updatable::new(initial_currency, move |new_currency| {
        set_currency_prefix.update(|currency_prefix| {
            *currency_prefix = to_currency_symbol(*new_currency).to_string();
        });
        updatable.update_value(|updatable| {
            updatable.update_and_then_signal(|ycv| {
                if let Some(ycv) = ycv {
                    ycv.currency = *new_currency as i32;
                } else {
                    *ycv = Some(YearCurrencyValue {
                        year: year_range.start,
                        value: 0.0,
                        currency: *new_currency as i32,
                    })
                };
            });
        });
    });

    let year_updatable = Updatable::new(initial_year, move |new_year| {
        if let Some(new_year) = new_year.clone() {
            updatable.update_value(|updatable| {
                updatable.update_and_then_signal(|ycv| {
                    if let Some(ycv) = ycv {
                        ycv.year = new_year
                    } else {
                        *ycv = Some(YearCurrencyValue {
                            year: new_year,
                            value: 0.0,
                            currency: Currency::Usd as i32,
                        })
                    }
                })
            })
        }
    });

    let value_updatable = Updatable::new(initial_value, move |new_value| {
        if let Some(new_input) = new_value.clone() {
            updatable.update_value(|updatable| {
                updatable.update_and_then_signal(|ycv| {
                    if let Some(ycv) = ycv {
                        ycv.value = new_input;
                    } else {
                        *ycv = Some(YearCurrencyValue {
                            year: 1900,
                            value: new_input,
                            currency: Currency::Usd as i32,
                        });
                    };
                });
            });
        }
    });

    let modification = Some(Modification::Prefix(MaybeSignal::Dynamic(
        currency_prefix.into(),
    )));

    view! {
        <div class="ycv" style="display: flex">
            <div class="ycv-currency">
                <CurrencySelect updatable=currency_select_updatable/>
            </div>
            <div class="ycv-value">
                <NumericInput
                    updatable=value_updatable
                    placeholder=value_placeholder
                    modification=modification
                />
            </div>
            <div class="ycv-as-of">"As Of"</div>
            <div class="ycv-year">
                <YearInput
                    updatable=year_updatable
                    year_range=year_range
                    placeholder=year_placeholder
                />
            </div>
        </div>
    }

    // ω <fn year_currency_value_input>
}

// α <mod-def year_currency_value_input>
// ω <mod-def year_currency_value_input>
