////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::ParsedNum;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// For comparison in benchmarks, first converts the _year_input_ to a
/// number and then clamps the resulting number and converts to
/// String.
///
///   * **year_input** - The input text
///   * **min_year** - Minimum year to clamp on
///   * **max_year** - Maximum year to clamp on
///   * _return_ - The number and its string representation
#[inline]
pub fn clamp(year_input: &str, min_year: u32, max_year: u32) -> ParsedNum {
    // α <fn clamp>
    let year_num: u32 = year_input.parse().unwrap();
    let mut return_year = String::new();

    if year_num > max_year {
        return_year = format!("{}", max_year);
    } else if year_num < min_year {
        return_year = format!("{}", min_year);
    } else {
        return_year.push_str(year_input)
    }
    ParsedNum::from_string(return_year)
    // ω <fn clamp>
}

/// Unit tests for `clamp_parse_input`
#[cfg(test)]
pub mod unit_tests {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn test_clamp() {
        // α <fn test_clamp>

        let mut min = 3500000;
        let mut max = 3800000;
        assert_eq!(ParsedNum::new(3507001), clamp("3507001", min, max));

        min = 1900;
        max = 2300;
        assert_eq!(ParsedNum::new(2200), clamp("2200", min, max));
        assert_eq!(ParsedNum::new(2025), clamp("2025", min, max));
        assert_eq!(ParsedNum::new(1980), clamp("1980", min, max));
        assert_eq!(ParsedNum::new(1979), clamp("1979", min, max));
        assert_eq!(ParsedNum::new(1999), clamp("1999", min, max));
        assert_eq!(ParsedNum::new(1999), clamp("19999", min, max));
        assert_eq!(ParsedNum::new(2300), clamp("23092", min, max));
        assert_eq!(ParsedNum::new(2), clamp("2", min, max));
        assert_eq!(ParsedNum::new(23), clamp("25", min, max));
        assert_eq!(ParsedNum::new(205), clamp("205", min, max));
        assert_eq!(ParsedNum::new(2300), clamp("2500", min, max));
        assert_eq!(ParsedNum::new(2300), clamp("99999", min, max));
        assert_eq!(ParsedNum::new(193), clamp("193", min, max));
        assert_eq!(ParsedNum::new(209), clamp("209", min, max));

        // ω <fn test_clamp>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def clamp_parse_input>
// ω <mod-def clamp_parse_input>
