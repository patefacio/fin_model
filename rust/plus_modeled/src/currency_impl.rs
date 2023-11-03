//! An impl for enum currency

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Currency;

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl Currency {
    /// Get string representation of value.
    ///
    ///   * **value** - Value to get as money string.
    ///   * _return_ - String representation of the value with currency symbol.
    #[inline]
    pub fn as_money(&self, value: f64) -> String {
        // α <fn Currency::as_money>

        use num_format::{Locale, ToFormattedString};
        let currency_symbol = self.to_currency_symbol();
        if value < -10.0 {
            format!(
                "{}({})",
                currency_symbol,
                (-(value.round()) as i64).to_formatted_string(&Locale::en)
            )
        } else if value.is_sign_negative() {
            format!("{}({:.2})", currency_symbol, -value)
        } else if value < 10.0 {
            format!("{}{:.2}", currency_symbol, value)
        } else {
            format!(
                "{}{}",
                currency_symbol,
                (value.round() as i64).to_formatted_string(&Locale::en)
            )
        }
        // ω <fn Currency::as_money>
    }

    /// Show single value in original currency and its new converted form.
    ///
    ///   * **original_value** - Value before conversion.
    ///   * **original_currency** - Original currency before conversion to `self`.
    ///   * **converted_value** - Converted value.
    ///   * _return_ - String representation of the original and converted value.
    #[inline]
    pub fn converted_value(
        &self,
        original_value: f64,
        original_currency: Currency,
        converted_value: f64,
    ) -> String {
        // α <fn Currency::converted_value>
        if original_value != converted_value {
            format!(
                "{}->{}",
                original_currency.as_money(original_value),
                self.as_money(converted_value)
            )
        } else {
            original_currency.as_money(original_value)
        }
        // ω <fn Currency::converted_value>
    }

    /// Assuming a value has been converted from `self` to `converted_currency`
    /// adds a consistent suffix to provided string. For example:
    ///
    ///  ```
    ///  let from = plus_modeled::Currency::Usd;
    ///  let s = from.add_conversion_suffix("purchased", plus_modeled::Currency::Jpy);
    ///  assert_eq!("purchased ($->¥)", s)
    ///  ```
    ///
    ///
    ///   * **text** - Text to suffix
    ///   * **converted_currency** - Currency converted to
    ///   * _return_ - The text with the consistent conversion suffix
    #[inline]
    pub fn add_conversion_suffix(&self, text: &str, converted_currency: Currency) -> String {
        // α <fn Currency::add_conversion_suffix>

        if *self != converted_currency {
            format!(
                "{} ({}->{})",
                text,
                self.to_currency_symbol(),
                converted_currency.to_currency_symbol()
            )
        } else {
            text.to_string()
        }

        // ω <fn Currency::add_conversion_suffix>
    }

    /// Convert currency to its symbol
    ///
    ///   * _return_ - The currency symbol
    #[inline]
    pub fn to_currency_symbol(&self) -> &'static str {
        match self {
            Currency::Usd => "$",
            Currency::Eur => "€",
            Currency::Jpy => "¥",
            Currency::Gbp => "£",
            Currency::Aud => "A$",
            Currency::Cad => "C$",
            Currency::Chf => "CHF",
            Currency::Cny => "CN¥",
            Currency::Hkd => "HK$",
            Currency::Nzd => "NZ$",
            Currency::Crc => "₡",
            Currency::Rub => "₽",
            Currency::Krw => "₩",
            Currency::Sek => "kr",
        }
    }

    /// Convert from currency symbol to [Currency]
    ///
    ///   * **currency_symbol** - The currency symbol
    ///   * _return_ - The [Currency] matching the symbol
    #[inline]
    pub fn from_currency_symbol(currency_symbol: &str) -> Option<Currency> {
        match currency_symbol {
            "$" => Some(Currency::Usd),
            "€" => Some(Currency::Eur),
            "¥" => Some(Currency::Jpy),
            "£" => Some(Currency::Gbp),
            "A$" => Some(Currency::Aud),
            "C$" => Some(Currency::Cad),
            "CHF" => Some(Currency::Chf),
            "CN¥" => Some(Currency::Cny),
            "HK$" => Some(Currency::Hkd),
            "NZ$" => Some(Currency::Nzd),
            "₡" => Some(Currency::Crc),
            "₽" => Some(Currency::Rub),
            "₩" => Some(Currency::Krw),
            "kr" => Some(Currency::Sek),
            _ => None,
        }
    }
}

/// Unit tests for `currency_impl`
#[cfg(test)]
pub mod unit_tests {

    /// Test type Currency
    mod test_currency {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn as_money() {
            // α <fn test Currency::as_money>
            assert_eq!("CHF100", Currency::Chf.as_money(100.0));
            // ω <fn test Currency::as_money>
        }

        #[test]
        fn converted_value() {
            // α <fn test Currency::converted_value>
            assert_eq!(
                "¥14,235->CHF100",
                Currency::Chf.converted_value(14235.0, Currency::Jpy, 100.0)
            );
            // ω <fn test Currency::converted_value>
        }

        #[test]
        fn to_currency_symbol() {
            // α <fn test Currency::to_currency_symbol>
            assert_eq!(Currency::Cad.to_currency_symbol(), "C$");
            // ω <fn test Currency::to_currency_symbol>
        }

        #[test]
        fn from_currency_symbol() {
            // α <fn test Currency::from_currency_symbol>
            assert_eq!("C$", Currency::Cad.to_currency_symbol());
            // ω <fn test Currency::from_currency_symbol>
        }

        // α <mod-def test_currency>
        use super::*;
        // ω <mod-def test_currency>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def currency_impl>
// ω <mod-def currency_impl>
