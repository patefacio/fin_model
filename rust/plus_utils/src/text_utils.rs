//! Contains general text utilities.

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
        .to_string()

    // ω <fn with_max_precision>
}

// α <mod-def text_utils>
// ω <mod-def text_utils>
