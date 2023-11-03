//! Support for currency combined with value

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use ::core::fmt::Display;
use ::core::fmt::Formatter;
use plus_modeled::Currency;
use serde::Serialize;
use std::ops::Mul;
use std::ops::Neg;

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Pairs a value with the currency.
#[derive(Debug, Default, PartialEq, PartialOrd, Copy, Clone, Serialize)]
pub struct CurrencyValue {
    /// Currency of the value.
    pub currency: Currency,
    /// Value in denominated currency.
    pub value: f64,
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl CurrencyValue {
    /// Convenience new.
    ///
    ///   * **currency** - Currency of the value.
    ///   * **value** - Value in denominated currency.
    ///   * _return_ - The new [CurrencyValue]
    #[inline]
    pub fn new(currency: Currency, value: f64) -> CurrencyValue {
        // α <fn CurrencyValue::new>
        CurrencyValue {
            currency: currency,
            value,
        }
        // ω <fn CurrencyValue::new>
    }

    /// Get string representation of [CurrencyValue].
    ///
    ///   * _return_ - String representation of the value with currency symbol.
    #[inline]
    pub fn as_money(&self) -> String {
        // α <fn CurrencyValue::as_money>
        self.currency.as_money(self.value)
        // ω <fn CurrencyValue::as_money>
    }

    /// Show single string representation of two currency values representing a conversion from `self` to `to`.
    ///
    ///   * **to** - Converted value.
    ///   * _return_ - String representation of the original and converted value.
    #[inline]
    pub fn converted_to(&self, to: CurrencyValue) -> String {
        // α <fn CurrencyValue::converted_to>
        if self.currency != to.currency {
            format!("{}->{}", self.as_money(), to.as_money())
        } else {
            self.as_money()
        }
        // ω <fn CurrencyValue::converted_to>
    }
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl Mul<f64> for CurrencyValue {
    type Output = Self;
    /// Return new `self` multiplied by `Rhs`.
    ///
    ///   * **rhs** - Right hand side
    ///   * _return_ - The new scaled `Self`
    fn mul(self, rhs: f64) -> Self::Output {
        // α <fn Mul::mul for CurrencyValue>

        CurrencyValue::new(self.currency, self.value * rhs)

        // ω <fn Mul::mul for CurrencyValue>
    }
}

impl Neg for CurrencyValue {
    type Output = Self;
    /// Return negated `self`.
    ///
    ///   * _return_ - The new negated `Self`
    fn neg(self) -> Self::Output {
        // α <fn Neg::neg for CurrencyValue>

        CurrencyValue::new(self.currency, -self.value)

        // ω <fn Neg::neg for CurrencyValue>
    }
}

impl Display for CurrencyValue {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        // α <fn Display::fmt for CurrencyValue>
        write!(f, "{}", self.as_money())
        // ω <fn Display::fmt for CurrencyValue>
    }
}

/// Unit tests for `currency_value`
#[cfg(test)]
pub mod unit_tests {

    /// Test type CurrencyValue
    mod test_currency_value {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn new() {
            // α <fn test CurrencyValue::new>
            assert_eq!(
                CurrencyValue {
                    currency: Currency::Chf,
                    value: 100.0
                },
                CurrencyValue::new(Currency::Chf, 100.0)
            )
            // ω <fn test CurrencyValue::new>
        }

        #[test]
        fn as_money() {
            // α <fn test CurrencyValue::as_money>
            assert_eq!(
                Currency::Chf.as_money(100.0),
                CurrencyValue::new(Currency::Chf, 100.0).as_money()
            )
            // ω <fn test CurrencyValue::as_money>
        }

        #[test]
        fn converted_to() {
            // α <fn test CurrencyValue::converted_to>
            assert_eq!(
                "CHF100->¥14,235",
                CurrencyValue::new(Currency::Chf, 100.0)
                    .converted_to(CurrencyValue::new(Currency::Jpy, 14235.0))
            )
            // ω <fn test CurrencyValue::converted_to>
        }

        // α <mod-def test_currency_value>
        use super::*;
        // ω <mod-def test_currency_value>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def currency_value>
// ω <mod-def currency_value>
