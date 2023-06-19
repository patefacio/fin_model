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
///   * **date_placeholder** - Placeholder for the asOf field
///   * _return_ - View for year_currency_value_input
#[component]
pub fn YearCurrencyValueInput(
    /// Context
    cx: Scope,
    /// Initial value and callback
    updatable: Updatable<YearCurrencyValue>,
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

    use leptos_dom::{html::Input};
    use leptos::IntoAttribute;
    use crate::CurrencySelect;
    use crate::Updatable;
    use plus_modeled::Currency;

    view! { cx,

        <fieldset class="year-currency-value">
        <legend>"Currency/Value/Year"</legend>
        
        <CurrencySelect
            updatable = Updatable::new(
                Currency::Eur,
                |currency| {
                    console_log(&format!("Currency updated to {currency:?}"))
                }
            )
        />
        
        <input 
            type="text"
            placeholder=value_placeholder
        />

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
