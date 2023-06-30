////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Scale the number by factor (10^scale_factor) keeping precision.
/// Consider 0.033 to be scaled up by 100 to show in an percent entry.
/// Because these are double precision and there are only discrete number
/// of values in the set of floats, this multiplication (0.333 * 100.0)
/// gives 33.300000000000004. This function adjusts for that and returns
/// 33.3 which is a valid f64. The max supported by this function is a
/// shift of 10 decimal places.
///
///   * **n** - Number to scale up
///   * **scale_factor** - Factor to scale the number
///   * _return_ - The number scaled by factor
#[inline]
pub fn scale_by(n: f64, scale_factor: i32) -> f64 {
    // α <fn scale_by>

    // TODO: Don't do it this way. Either use some math trickery or
    // convert input to string up front, iterate through adjusting the
    // number
    n * 10.0f64.powf(scale_factor as f64) as f64

    // ω <fn scale_by>
}

/// Unit tests for `scale_by`
#[cfg(test)]
pub mod unit_tests {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn test_scale_by() {
        // α <fn test_scale_by>

        assert_eq!(3.33, scale_by(0.0333, 2));

        // ω <fn test_scale_by>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def scale_by>
// ω <mod-def scale_by>
