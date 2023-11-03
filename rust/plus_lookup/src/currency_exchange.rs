////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::CurrencyValue;
use once_cell::sync::Lazy;
use plus_modeled::Currency;
use std::collections::HashMap;
use std::sync::Arc;

////////////////////////////////////////////////////////////////////////////////////
// --- lazy inits ---
////////////////////////////////////////////////////////////////////////////////////
/// The set of supported currencies
pub static SUPPORTED_CURRENCIES: Lazy<Vec<Currency>> = Lazy::new(|| {
    vec![
        Currency::Usd,
        Currency::Eur,
        Currency::Jpy,
        Currency::Gbp,
        Currency::Aud,
        Currency::Cad,
        Currency::Chf,
        Currency::Cny,
        Currency::Hkd,
        Currency::Nzd,
        Currency::Crc,
        Currency::Rub,
        Currency::Krw,
        Currency::Sek,
    ]
});

/// Currencies downloaded from _Bloomberg_ on/around 2022, Dec 23
pub static WEB_CURRENCY_EXCHANGE: Lazy<Arc<CurrencyExchange>> = Lazy::new(|| {
    Arc::new(CurrencyExchange::from_subset(HashMap::from([
        (Currency::Eur, 1.0618),
        (Currency::Gbp, 1.2047),
        (Currency::Chf, 1.0716965126395577),
        (Currency::Sek, 0.09503444964359453),
        (Currency::Rub, 0.014356944763464538),
        (Currency::Jpy, 0.007527853263834854),
        (Currency::Hkd, 0.128118069747016),
        (Currency::Aud, 0.672),
        (Currency::Nzd, 0.6289),
        (Currency::Cny, 0.14306152113650694),
        (Currency::Cad, 0.7357268977624227),
        (Currency::Crc, 0.0017200750097163661),
    ])))
});

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Provides conversions between all currencies and USD.
#[derive(Debug, Clone)]
pub struct CurrencyExchange {
    /// An exchange rate to convert each into USD.
    exchange_rates: Vec<(f64, f64)>,
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl CurrencyExchange {
    /// Create a new [CurrencyExchange] from conversion rates of each currency to USD.
    ///
    ///   * **to_usd_rates** - Conversion rates, one for each currency ordered as defined in the [Currency] enum.
    ///   * _return_ - [CurrencyExchange] initialized with provided rates
    #[inline]
    pub fn new(to_usd_rates: &[f64]) -> CurrencyExchange {
        // α <fn CurrencyExchange::new>

        debug_assert!(
            to_usd_rates.len() == std::mem::variant_count::<Currency>(),
            "Must pass value for each currency!"
        );

        debug_assert!(
            to_usd_rates.iter().all(|r| r > &0.0),
            "Rates must all be positive."
        );

        CurrencyExchange {
            exchange_rates: to_usd_rates
                .iter()
                .map(|to_usd| (*to_usd, 1.0 / to_usd))
                .collect(),
        }

        // ω <fn CurrencyExchange::new>
    }

    /// Creates a new [CurrencyExchange] from conversion rates provided - all others set to 1.0
    ///
    ///   * **to_usd_rates** - Subset of conversion (to `USD`) rates.
    ///   * _return_ - [CurrencyExchange] initialized with provided rates
    #[inline]
    pub fn from_subset(to_usd_rates: HashMap<Currency, f64>) -> CurrencyExchange {
        // α <fn CurrencyExchange::from_subset>

        CurrencyExchange::new(
            &SUPPORTED_CURRENCIES
                .iter()
                .map(|currency| to_usd_rates.get(currency).map(|rate| *rate).unwrap_or(1.0))
                .collect::<Vec<f64>>(),
        )

        // ω <fn CurrencyExchange::from_subset>
    }

    /// Get conversion rate from `currency` to `USD`.
    ///
    ///   * **currency** - Currency to convert to `USD`.
    ///   * _return_ - The conversion rate.
    #[inline]
    pub fn to_usd_rate(&self, currency: Currency) -> f64 {
        // α <fn CurrencyExchange::to_usd_rate>

        match currency {
            Currency::Usd => 1.0,
            _ => self.exchange_rates[currency as usize].0,
        }

        // ω <fn CurrencyExchange::to_usd_rate>
    }

    /// Convert `value` from `currency` to `USD`.
    ///
    ///   * **currency** - Currency to convert to `USD`.
    ///   * **value** - Value to convert to `USD`.
    ///   * _return_ - The converted value.
    #[inline]
    pub fn value_to_usd(&self, currency: Currency, value: f64) -> f64 {
        // α <fn CurrencyExchange::value_to_usd>

        value * self.to_usd_rate(currency)

        // ω <fn CurrencyExchange::value_to_usd>
    }

    /// Get conversion rate from `USD` to `currency`.
    ///
    ///   * **currency** - Currency to convert from `USD`.
    ///   * _return_ - The conversion rate.
    #[inline]
    pub fn from_usd_rate(&self, currency: Currency) -> f64 {
        // α <fn CurrencyExchange::from_usd_rate>

        match currency {
            Currency::Usd => 1.0,
            _ => self.exchange_rates[currency as usize].1,
        }

        // ω <fn CurrencyExchange::from_usd_rate>
    }

    /// Convert `value` from `USD` to `currency`.
    ///
    ///   * **currency** - Currency to convert from `USD`.
    ///   * **value** - Value to convert from `USD`.
    ///   * _return_ - The converted value.
    #[inline]
    pub fn value_from_usd(&self, currency: Currency, value: f64) -> f64 {
        // α <fn CurrencyExchange::value_from_usd>

        value * self.from_usd_rate(currency)

        // ω <fn CurrencyExchange::value_from_usd>
    }

    /// Get rate to convert `value` from `from_currency` to `to_currency`.
    ///
    ///   * **from_currency** - Currency to convert from.
    ///   * **to_currency** - Currency to convert to.
    ///   * _return_ - The conversion exchange rate.
    #[inline]
    pub fn exchange_currency_rate(&self, from_currency: Currency, to_currency: Currency) -> f64 {
        // α <fn CurrencyExchange::exchange_currency_rate>

        if from_currency == to_currency {
            1.0
        } else {
            self.to_usd_rate(from_currency) * self.from_usd_rate(to_currency)
        }

        // ω <fn CurrencyExchange::exchange_currency_rate>
    }

    /// Convert `value` of `from_currency_value` to `to_currency`.
    ///
    ///   * **from_currency_value** - Currency to convert from.
    ///   * **to_currency** - Currency to convert to.
    ///   * _return_ - The converted value.
    #[inline]
    pub fn exchange_currency_value(
        &self,
        from_currency_value: CurrencyValue,
        to_currency: Currency,
    ) -> CurrencyValue {
        // α <fn CurrencyExchange::exchange_currency_value>

        CurrencyValue::new(
            to_currency,
            from_currency_value.value
                * self.exchange_currency_rate(from_currency_value.currency, to_currency),
        )

        // ω <fn CurrencyExchange::exchange_currency_value>
    }
}

/// Accessors for [CurrencyExchange] fields
impl CurrencyExchange {
    #[inline]
    pub fn get_exchange_rates(&self) -> &Vec<(f64, f64)> {
        &self.exchange_rates
    }
}

/// Unit tests for `currency_exchange`
#[cfg(test)]
pub mod unit_tests {

    /// Test type CurrencyExchange
    mod test_currency_exchange {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn exchange_rates() {
            // α <fn test CurrencyExchange::exchange_rates>

            let currency_exchange = CurrencyExchange::from_subset(HashMap::from([
                // EUR/USD is 1.06227 (i.e. 1 euro exchanges for 1.06227 USD)
                (Currency::Eur, 1.06227),
                (Currency::Gbp, 1.217),
                (Currency::Cad, 0.733609),
            ]));

            assert_eq!(1.06227, currency_exchange.to_usd_rate(Currency::Eur));
            assert_eq!(
                1.0 / 1.06227,
                currency_exchange.from_usd_rate(Currency::Eur)
            );

            assert_eq!(1.217, currency_exchange.to_usd_rate(Currency::Gbp));
            assert_eq!(1.0 / 1.217, currency_exchange.from_usd_rate(Currency::Gbp));

            assert_eq!(0.733609, currency_exchange.to_usd_rate(Currency::Cad));
            assert_eq!(
                1.0 / 0.733609,
                currency_exchange.from_usd_rate(Currency::Cad)
            );

            assert_eq!(
                2.0 * 1.06227,
                currency_exchange.value_to_usd(Currency::Eur, 2.0)
            );
            assert_eq!(
                2.0 / 1.06227,
                currency_exchange.value_from_usd(Currency::Eur, 2.0)
            );

            assert_eq!(
                2.0 * 1.217 / 0.733609,
                currency_exchange
                    .exchange_currency_value(CurrencyValue::new(Currency::Gbp, 2.0), Currency::Cad)
                    .value
            )

            // ω <fn test CurrencyExchange::exchange_rates>
        }

        // α <mod-def test_currency_exchange>
        use super::*;

        // ω <mod-def test_currency_exchange>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def currency_exchange>
// ω <mod-def currency_exchange>
