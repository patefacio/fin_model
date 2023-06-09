//! Module for currency_select leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::Currency;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a currency select of a set of currencies
///
///   * **cx** - Context
///   * **updatable** - Currency update callback
///   * _return_ - View for currency_select
#[component]
pub fn CurrencySelect(
    /// Context
    cx: Scope,
    /// Currency update callback
    updatable: Updatable<Currency>,
) -> impl IntoView {
    // α <fn currency_select>

    use crate::{InitialValue, MultiColumnSelect, SelectOption};

    let mut updatable = updatable;
    let initial_currency = updatable.value;

    let options: Vec<_> = (0..std::mem::variant_count::<Currency>())
        .map(|i| {
            let currency = Currency::from_i32(i as i32).expect("Valid currency");
            let label = currency.as_str_name().to_string();
            let key = to_currency_symbol(currency).to_string();
            SelectOption::KeyLabel { key, label }
        })
        .collect();

    let menu_selection = move |value: String| {
        let selected_currency = currency_from_symbol(&value).unwrap_or_default();
        log!("About to signal currency -> {:?}", updatable.value);
        updatable.update_and_then_signal(|currency| {
            *currency = selected_currency
        });
    };

    view! { cx,
        <MultiColumnSelect
            options=options
            initial_value=Some(InitialValue::SelectionIndex(initial_currency as i32 as usize))
            on_select=menu_selection
        />
    }

    // ω <fn currency_select>
}

/// Map the currency into its currency symbol
///
///   * **currency** - The currency to convert to symbol
///   * _return_ - The mapped currency symbol
#[inline]
pub fn to_currency_symbol(currency: Currency) -> &'static str {
    // α <fn to_currency_symbol>

    match currency {
        Currency::Usd => "$",
        Currency::Eur => "€",
        Currency::Jpy => "¥",
        Currency::Gbp => "£",
        Currency::Aud => "A$",
        Currency::Cad => "C$",
        Currency::Chf => "CHF",
        Currency::Cny => "¥",
        Currency::Hkd => "HK$",
        Currency::Nzd => "NZ$",
        Currency::Crc => "₡",
        Currency::Rub => "₽",
        Currency::Krw => "₩",
        Currency::Sek => "kr",
    }

    // ω <fn to_currency_symbol>
}

/// Map the currency symbol to its Currency enum variant
///
///   * **currency_symbol** - The currency symbol
///   * _return_ - The mapped currency
#[inline]
pub fn currency_from_symbol(currency_symbol: &str) -> Option<Currency> {
    // α <fn currency_from_symbol>
    match currency_symbol {
        "$" => Some(Currency::Usd),
        "€" => Some(Currency::Eur),
        "¥" => Some(Currency::Jpy),
        "£" => Some(Currency::Gbp),
        "A$" => Some(Currency::Aud),
        "C$" => Some(Currency::Cad),
        "CHF" => Some(Currency::Chf),
        "¥" => Some(Currency::Cny),
        "HK$" => Some(Currency::Hkd),
        "NZ$" => Some(Currency::Nzd),
        "₡" => Some(Currency::Crc),
        "₽" => Some(Currency::Rub),
        "₩" => Some(Currency::Krw),
        "kr" => Some(Currency::Sek),
        _ => None,
    }
    // ω <fn currency_from_symbol>
}

// α <mod-def currency_select>
// ω <mod-def currency_select>
