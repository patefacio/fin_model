//! Display implementations

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::CurrencyValue;
use crate::Date;
use crate::DbId;
use crate::DossierCorrelationEntry;
use crate::DossierCorrelationMatrix;
use crate::DossierHoldingIndex;
use crate::DossierItemIndex;
use crate::NormalSpec;
use crate::PeriodBalance;
use crate::RateCurve;
use crate::YearCurrencyValue;
use crate::YearRange;
use crate::YearValue;
use crate::YearValueSeries;
use core::fmt::Display;
use core::fmt::Formatter;

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl Display for Date {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for Date>
        todo!("Implement `fmt`")
        // ω <fn Display::fmt for Date>
    }
}

impl Display for YearValue {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for YearValue>
        use num_format::{Locale, ToFormattedString};

        write!(
            f,
            "({}-＄{})",
            self.year,
            (self.value.round() as i64).to_formatted_string(&Locale::en)
        )

        // ω <fn Display::fmt for YearValue>
    }
}

impl Display for CurrencyValue {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for CurrencyValue>

        use num_format::{Locale, ToFormattedString};

        let unsigned = format!(
            "{}{}",
            if let Some(currency) = Currency::from_i32(self.currency) {
                currency.to_currency_symbol()
            } else {
                "?"
            },
            match self.value {
                value if value < -10.0 => (-value.round() as i64).to_formatted_string(&Locale::en),
                value if value < 10.0 => format!("{:.2}", value.abs()),
                value => (value.round() as i64).to_formatted_string(&Locale::en),
            }
        );

        if self.value < 0.0 {
            write!(f, "({})", unsigned)
        } else {
            write!(f, "{}", unsigned)
        }

        // ω <fn Display::fmt for CurrencyValue>
    }
}

impl Display for YearCurrencyValue {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for YearCurrencyValue>

        write!(
            f,
            "({}:{})",
            self.year,
            CurrencyValue {
                currency: self.currency,
                value: self.value
            }
        )

        // ω <fn Display::fmt for YearCurrencyValue>
    }
}

impl Display for YearRange {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for YearRange>
        write!(f, "({}->{})", self.start, self.end)
        // ω <fn Display::fmt for YearRange>
    }
}

impl Display for NormalSpec {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for NormalSpec>
        use crate::SystemUnicodes;
        use plus_utils::scale_by;
        use plus_utils::with_max_precision;

        let precision = 2usize;

        write!(
            f,
            "𝑁({}={}%,{}={}%)",
            SystemUnicodes::MathMu.as_unicode(),
            with_max_precision(scale_by(self.mean, 2), precision),
            SystemUnicodes::MathSigma.as_unicode(),
            with_max_precision(scale_by(self.std_dev, 2), precision)
        )
        // ω <fn Display::fmt for NormalSpec>
    }
}

impl Display for RateCurve {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for RateCurve>
        write!(
            f,
            "RateCurve({} entries)",
            self.curve.len()
        )
        // ω <fn Display::fmt for RateCurve>
    }
}

impl Display for YearValueSeries {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for YearValueSeries>
        todo!("Implement `fmt`")
        // ω <fn Display::fmt for YearValueSeries>
    }
}

impl Display for DbId {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for DbId>
        todo!("Implement `fmt`")
        // ω <fn Display::fmt for DbId>
    }
}

impl Display for DossierHoldingIndex {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for DossierHoldingIndex>
        todo!("Implement `fmt`")
        // ω <fn Display::fmt for DossierHoldingIndex>
    }
}

impl Display for DossierItemIndex {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for DossierItemIndex>
        todo!("Implement `fmt`")
        // ω <fn Display::fmt for DossierItemIndex>
    }
}

impl Display for DossierCorrelationEntry {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for DossierCorrelationEntry>
        todo!("Implement `fmt`")
        // ω <fn Display::fmt for DossierCorrelationEntry>
    }
}

impl Display for DossierCorrelationMatrix {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for DossierCorrelationMatrix>
        todo!("Implement `fmt`")
        // ω <fn Display::fmt for DossierCorrelationMatrix>
    }
}

impl Display for PeriodBalance {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for PeriodBalance>
        todo!("Implement `fmt`")
        // ω <fn Display::fmt for PeriodBalance>
    }
}

// α <mod-def core_display>

use crate::Currency;

impl Currency {
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
            Currency::Cny => "¥",
            Currency::Hkd => "HK$",
            Currency::Nzd => "NZ$",
            Currency::Crc => "₡",
            Currency::Rub => "₽",
            Currency::Krw => "₩",
            Currency::Sek => "kr",
        }
    }

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
            "¥" => Some(Currency::Cny),
            "HK$" => Some(Currency::Hkd),
            "NZ$" => Some(Currency::Nzd),
            "₡" => Some(Currency::Crc),
            "₽" => Some(Currency::Rub),
            "₩" => Some(Currency::Krw),
            "kr" => Some(Currency::Sek),
            _ => None,
        }
    }

    #[inline]
    fn as_money(&self, value: f64) -> String {
        // α <fn CurrencyFunctions::as_money for Currency>
        use num_format::{Locale, ToFormattedString};

        if value < -10.0 {
            format!(
                "{}({})",
                self.to_currency_symbol(),
                (-(value.round()) as i64).to_formatted_string(&Locale::en)
            )
        } else if value < 0.0 {
            format!("{}({:.2})", self.to_currency_symbol(), -value)
        } else if value < 10.0 {
            format!("{}{:.2}", self.to_currency_symbol(), value)
        } else {
            format!(
                "{}{}",
                self.to_currency_symbol(),
                (value.round() as i64).to_formatted_string(&Locale::en)
            )
        }
        // ω <fn CurrencyFunctions::as_money for Currency>
    }
}

// ω <mod-def core_display>
