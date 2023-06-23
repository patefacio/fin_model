//! Extended version of clamp

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::ParsedNum;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Truncate digits of `n` to by removing last `moribund_digits` number of digits
///
///   * **n** - Number to truncate
///   * **moribund_digits** - Digits to take off the right end
///   * _return_ - Number with digits truncated
#[inline]
pub fn truncate_digits(mut n: u32, moribund_digits: usize) -> u32 {
    // α <fn truncate_digits>
    for _ in 0..moribund_digits {
        n /= 10;
    }
    n
    // ω <fn truncate_digits>
}

/// Clamp function on year input text
///
///   * **year_input** - The input text
///   * **input_digit_count** - Number of digits in input
///   * **min_year** - Minimum year to clamp on
///   * **max_year** - Maximum year to clamp on
///   * **range_digit_count** - Number of digits in range
///   * _return_ - The number and its string representation
#[inline]
pub fn clamp_ext(
    year_input: &str,
    input_digit_count: usize,
    min_year: u32,
    max_year: u32,
    range_digit_count: usize,
) -> ParsedNum {
    // α <fn clamp_ext>

    use std::cmp::Ordering;

    tracing::info!("y({year_input}), dig({input_digit_count}, min({min_year}), max({max_year}), range_dig({range_digit_count})");

    match input_digit_count.cmp(&range_digit_count) {
        Ordering::Greater => clamp_ext(
            &year_input[0..range_digit_count],
            range_digit_count,
            min_year,
            max_year,
            range_digit_count,
        ),
        Ordering::Less => {
            let digits_to_truncate = range_digit_count - input_digit_count;
            clamp_ext(
                year_input,
                input_digit_count,
                truncate_digits(min_year, digits_to_truncate),
                truncate_digits(max_year, digits_to_truncate),
                input_digit_count,
            )
        }
        Ordering::Equal => ParsedNum::new(
            year_input
                .parse::<u32>()
                .expect("valid number")
                .clamp(min_year, max_year),
        ),
    }
    // ω <fn clamp_ext>
}

/// Clamp function on year input text.
///
///   * **year_input** - The input text
///   * **min_year** - Minimum year to clamp on
///   * **max_year** - Maximum year to clamp on
///   * _return_ - The number and its string representation
#[inline]
pub fn clamp(year_input: &str, min_year: u32, max_year: u32) -> ParsedNum {
    // α <fn clamp>

    let range_digit_count = (max_year as f64).log10() as usize + 1;
    debug_assert!(min_year <= max_year);
    debug_assert!(
        range_digit_count == (min_year as f64).log10() as usize + 1,
        "min and max must be same number of digits"
    );
    let input_digit_count = year_input.len();
    clamp_ext(
        year_input,
        input_digit_count,
        min_year,
        max_year,
        range_digit_count,
    )

    // ω <fn clamp>
}

/// Unit tests for `clamp_ext`
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
        assert_eq!(ParsedNum::new(2), clamp("2", min, max));
        assert_eq!(ParsedNum::new(23), clamp("25", min, max));
        assert_eq!(ParsedNum::new(205), clamp("205", min, max));
        assert_eq!(ParsedNum::new(2300), clamp("2500", min, max));
        assert_eq!(ParsedNum::new(2300), clamp("99999", min, max));
        //assert_eq!(ParsedNum::new(), clamp("-1", min, max));
        //assert_eq!(false, clamp("hi", min, max));
        assert_eq!(ParsedNum::new(193), clamp("193", min, max));
        assert_eq!(ParsedNum::new(209), clamp("209", min, max));

        // ω <fn test_clamp>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def clamp_ext>
// ω <mod-def clamp_ext>
