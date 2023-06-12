//! Module for year_currency_value_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use fin_model::core::{YearCurrencyValue, YearRange};
use leptos::{component, view, IntoView, Scope};

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

    view! { cx,
        
        <input 
            type="text"
            placeholder=value_placeholder
        />

        <div>"As Of"</div>

        <input
            type="text"
            placeholder=date_placeholder
        />
        
    }

    // ω <fn year_currency_value_input>
}

// α <mod-def year_currency_value_input>
// ω <mod-def year_currency_value_input>
