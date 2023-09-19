//! Contains general text utilities.

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use num::Float;
use num_format::ToFormattedStr;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Converts the float to number with provided precision but removes
/// trailing zeros. This can be used to save space in situations
/// where you may need to show say 2 decimals but if all are ending
/// in .00 better to just elide the additional.
///
///   * **value** - Number to format
///   * **precision** - Precision to format decimal to.
///   * _return_ - Formatted number
pub fn with_max_precision(value: f64, precision: usize) -> String {
    // α <fn with_max_precision>

    format!("{:.prec$}", value, prec = precision)
        .trim_matches('0')
        .trim_matches('.')
        .to_string()

    // ω <fn with_max_precision>
}

/// Add thousands separators to number
///
///   * **number** - The number to commify.
///   * _return_ - The commified number.
#[inline]
pub fn commify_int<T>(number: T) -> String
where
    T: ToFormattedStr,
{
    // α <fn commify_int>
    use num_format::{Locale, ToFormattedString};
    number.to_formatted_string(&Locale::en)
    // ω <fn commify_int>
}

/// Add thousands separators to number
///
///   * **number** - The number to commify.
///   * _return_ - The commified number.
#[inline]
pub fn commify_float<F>(number: F) -> String
where
    F: Float + std::fmt::Display,
{
    // α <fn commify_float>
    let s = number.to_string();
    if let Some(decimal_pos) = s.find('.') {
        let int_part = &s[0..decimal_pos];
        let as_int = int_part.parse::<i64>().unwrap();
        let mut result = commify_int(as_int);
        result.push_str(&s[decimal_pos..]);
        result
    } else {
        s
    }
    // ω <fn commify_float>
}

/// Unit tests for `text_utils`
#[cfg(test)]
pub mod unit_tests {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn test_commify_int() {
        // α <fn test_commify_int>

        assert_eq!(String::from("12"), commify_int(12));
        assert_eq!(String::from("1,234"), commify_int(1234));
        assert_eq!(String::from("1,234,567"), commify_int(1234567));

        // ω <fn test_commify_int>
    }

    #[test]
    fn test_commify_float() {
        // α <fn test_commify_float>

        assert_eq!(String::from("12"), commify_float(12.0));
        assert_eq!(String::from("1,234.789"), commify_float(1234.789));
        assert_eq!(String::from("1,234,567.2"), commify_float(1234567.2));

        // ω <fn test_commify_float>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def text_utils>

/// Return string representation of the type of the passed argument
pub fn print_type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
// ω <mod-def text_utils>
