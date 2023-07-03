////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::ParsedNum;
use std::ops::RangeInclusive;

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// The struct is configured with a `RangeInclusive<u32>` and supports clamping input integer
/// values to the range. What is special about this clamp is that the input
/// is a string representing the value as it is being typed and therefore may
/// not be complete. So if the range is: `{ start: 1900, end: 2300 }` the following
/// outputs will be provided by the clamp function:
///
/// - "1" -> "1" : "1" is a fine start to entering a valid number
/// - "2" -> "2" : "2" is a fine start to entering a valid number
/// - "3" -> "2" : "3" is a poor start if the number is to be in (1900,2300). It is "clamped"
///                to "2"
/// - "0" -> "1" : "0" is a poor start (too low). It is "clamped" to "1"
/// - "18" -> "19" : "18" is too small, the smallest number will begin with "19"
/// - "24" -> "23" : "24" is too large, the largest number will begin with "23"
/// - "198" -> "198" : "198" is valid
#[derive(Debug, Clone)]
pub struct IntegerClamp {
    /// The supplied range (inclusive) of valid integers.
    pub integer_range: RangeInclusive<u32>,
    /// A vector of `RangeInclusive<u32>` instances created on construction, one for each number of digits up to the
    /// number of digits in `integer_range.end`.
    ///
    /// So for the range `{ start: 1900, end: 2300 }` the resulting vector would look like:
    ///
    /// vec!{
    ///    Range{ start: 1, end: 2 },
    ///    Range{ start: 19, end: 23 },
    ///    Range{ start: 190, end: 230 }
    /// }
    ///
    /// Now to clamp a value, the function can find the length of the string and index
    /// into the appropriate range in the vector to make the check.
    pub ranges_by_digit: Vec<RangeInclusive<u32>>,
    /// Number of digits in the maximum
    pub max_len: usize,
}

/// Integer clamp where the ranges are stored as strings
pub struct IntegerClampStrings {
    /// The supplied range (inclusive) of valid integers.
    pub integer_range: RangeInclusive<u32>,
    /// A vector of integer range instances created on construction, one for each number of digits up to the
    /// number of digits in `integer_range.end`.
    ///
    /// So for the range `{ start: 1900, end: 2300 }` the resulting vector would look like:
    ///
    /// vec!{
    ///    IntegerRange{ start: 1, end: 2 },
    ///    IntegerRange{ start: 19, end: 23 },
    ///    IntegerRange{ start: 190, end: 230 }
    /// }
    ///
    /// Now to clamp a value, the function can find the length of the string and index
    /// into the appropriate range in the vector to make the check.
    pub ranges_by_digit: Vec<(ParsedNum, ParsedNum)>,
    /// Number of digits in the maximum
    pub max_len: usize,
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl IntegerClamp {
    /// Will return the integer clamped to a min/max.
    ///
    ///   * **value** - The value to clamp.
    ///   * _return_ - The clamped value.
    #[inline]
    pub fn clamp(&self, value: &str) -> ParsedNum {
        // α <fn IntegerClamp::clamp>

        // We know all characters in the string must be digits
        debug_assert!(value.bytes().all(|b| (b as char).is_ascii_digit()));

        let num_digits = value.len();
        let value_as_u32 = value.parse::<u32>().expect("valid number");

        if num_digits > self.max_len {
            // If number of digits is greater than max_len allowed, recurse
            self.clamp(&value[0..self.max_len])
        } else if let Some(year_range) = self.ranges_by_digit.get(num_digits - 1) {
            // Index into the proper ranges_by_digit, do the clamp and return the number
            let clamped = value_as_u32.clamp(*year_range.start(), *year_range.end());
            ParsedNum::new(clamped)
        } else {
            ParsedNum::new(*self.ranges_by_digit.last().expect("not empty").end())
        }

        // ω <fn IntegerClamp::clamp>
    }
}

impl IntegerClampStrings {
    /// Will return the integer clamped to a min/max.
    ///
    ///   * **value** - The value to clamp.
    ///   * _return_ - The clamped value.
    #[inline]
    pub fn clamp(&self, value: &str) -> ParsedNum {
        // α <fn IntegerClampStrings::clamp>

        // We know all characters in the string must be digits
        debug_assert!(value.bytes().all(|b| (b as char).is_ascii_digit()));
        let num_digits = value.len();

        if num_digits > self.max_len {
            // If number of digits is greater than max_len allowed, recurse
            self.clamp(&value[0..self.max_len])
        } else if let Some((start_parsed_num, end_parsed_num)) =
            self.ranges_by_digit.get(num_digits - 1)
        {
            // Index into the proper ranges_by_digit, do the clamp and return the number
            let clamped = value.clamp(&start_parsed_num.as_string, &end_parsed_num.as_string);
            ParsedNum::from_str(clamped)
        } else {
            ParsedNum::from_str(value)
        }

        // ω <fn IntegerClampStrings::clamp>
    }
}

impl IntegerClamp {
    /// Create new instance of IntegerClamp
    ///
    ///   * **integer_range** - The supplied range (inclusive) of valid integers.
    ///   * _return_ - The new instance
    pub fn new(integer_range: RangeInclusive<u32>) -> IntegerClamp {
        // α <fn IntegerClamp::new>

        debug_assert!(integer_range.start() <= integer_range.end(), "Start <= end");

        //////////////////////////
        let num_digits_start = (*integer_range.start() as f64).log10().round() as usize + 1;
        let num_digits_end = (*integer_range.end() as f64).log10().round() as usize + 1;
        let vec_size = num_digits_start.max(num_digits_end);
        let mut ranges_by_digit: Vec<RangeInclusive<u32>> = Vec::with_capacity(vec_size);

        // Convert the numbers in range to strings so iteration over digits is simpler
        let start_year = integer_range.start().to_string();
        let end_year = integer_range.end().to_string();

        let mut start_partial_value = 0;
        let mut end_partial_value = 0;

        // iterate from 0 to vec_size and for each number of digits create a new
        // range of that size and push into vec.
        for i in 0..vec_size {
            start_partial_value *= 10;
            end_partial_value *= 10;

            if let Some(digit_as_u32) = start_year
                .as_bytes()
                .get(i)
                .and_then(|digit| (*digit as char).to_digit(10))
            {
                start_partial_value += digit_as_u32;
            }

            if let Some(digit_as_u32) = end_year
                .as_bytes()
                .get(i)
                .and_then(|digit| (*digit as char).to_digit(10))
            {
                end_partial_value += digit_as_u32;
            }

            ranges_by_digit.push(RangeInclusive::<u32>::new(
                start_partial_value,
                end_partial_value,
            ));
        }

        IntegerClamp {
            integer_range,
            ranges_by_digit,
            max_len: num_digits_end,
        }
        // ω <fn IntegerClamp::new>
    }
}

impl IntegerClampStrings {
    /// Create new instance of IntegerClampStrings
    ///
    ///   * **integer_range** - The supplied range (inclusive) of valid integers.
    ///   * _return_ - The new instance
    pub fn new(integer_range: RangeInclusive<u32>) -> IntegerClampStrings {
        // α <fn IntegerClampStrings::new>

        debug_assert!(integer_range.start() <= integer_range.end(), "Start <= end");

        //////////////////////////
        let num_digits_start = (*integer_range.start() as f64).log10().round() as usize + 1;
        let num_digits_end = (*integer_range.end() as f64).log10().round() as usize + 1;
        let vec_size = num_digits_start.max(num_digits_end);
        let mut ranges_by_digit: Vec<(ParsedNum, ParsedNum)> = Vec::with_capacity(vec_size);

        // Convert the numbers in range to strings so iteration over digits is simpler
        let start_year = integer_range.start().to_string();
        let end_year = integer_range.end().to_string();

        let mut start_partial_value = 0;
        let mut end_partial_value = 0;

        // iterate from 0 to vec_size and for each number of digits create a new
        // range of that size and push into vec.
        for i in 0..vec_size {
            start_partial_value *= 10;
            end_partial_value *= 10;

            if let Some(digit_as_u32) = start_year
                .as_bytes()
                .get(i)
                .and_then(|digit| (*digit as char).to_digit(10))
            {
                start_partial_value += digit_as_u32;
            }

            if let Some(digit_as_u32) = end_year
                .as_bytes()
                .get(i)
                .and_then(|digit| (*digit as char).to_digit(10))
            {
                end_partial_value += digit_as_u32;
            }

            ranges_by_digit.push((
                ParsedNum::new(start_partial_value),
                ParsedNum::new(end_partial_value),
            ));
        }

        IntegerClampStrings {
            integer_range,
            ranges_by_digit,
            max_len: num_digits_end,
        }
        // ω <fn IntegerClampStrings::new>
    }
}

/// Unit tests for `integer_clamp`
#[cfg(test)]
pub mod unit_tests {

    /// Test type IntegerClamp
    mod test_integer_clamp {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn clamp() {
            // α <fn test IntegerClamp::clamp>

            let mut integer_clamp = IntegerClamp::new(3_500_000..3_800_000);

            assert_eq!(ParsedNum::new(3507001), integer_clamp.clamp("3507001"));

            integer_clamp = IntegerClamp::new(1900..2300);
            assert_eq!(ParsedNum::new(2200), integer_clamp.clamp("2200"));
            assert_eq!(ParsedNum::new(2025), integer_clamp.clamp("2025"));
            assert_eq!(ParsedNum::new(1980), integer_clamp.clamp("1980"));
            assert_eq!(ParsedNum::new(1979), integer_clamp.clamp("1979"));
            assert_eq!(ParsedNum::new(1999), integer_clamp.clamp("1999"));
            assert_eq!(ParsedNum::new(1999), integer_clamp.clamp("19999"));
            assert_eq!(ParsedNum::new(2300), integer_clamp.clamp("23092"));
            assert_eq!(ParsedNum::new(2), integer_clamp.clamp("2"));
            assert_eq!(ParsedNum::new(23), integer_clamp.clamp("25"));
            assert_eq!(ParsedNum::new(205), integer_clamp.clamp("205"));
            assert_eq!(ParsedNum::new(2300), integer_clamp.clamp("2500"));
            assert_eq!(ParsedNum::new(2300), integer_clamp.clamp("99999"));
            assert_eq!(ParsedNum::new(193), integer_clamp.clamp("193"));
            assert_eq!(ParsedNum::new(209), integer_clamp.clamp("209"));
            assert_eq!(ParsedNum::new(1900), integer_clamp.clamp("1900"));
            assert_eq!(ParsedNum::new(2300), integer_clamp.clamp("2300"));

            // ω <fn test IntegerClamp::clamp>
        }

        // α <mod-def test_integer_clamp>
        use super::*;
        // ω <mod-def test_integer_clamp>
    }

    /// Test type IntegerClampStrings
    mod test_integer_clamp_strings {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn clamp() {
            // α <fn test IntegerClampStrings::clamp>
            let mut integer_clamp = IntegerClampStrings::new(Range::<u32> {
                start: 3500000,
                end: 3800000,
            });

            assert_eq!(ParsedNum::new(3507001), integer_clamp.clamp("3507001"));

            integer_clamp = IntegerClampStrings::new(Range::<u32> {
                start: 1900,
                end: 2300,
            });
            assert_eq!(ParsedNum::new(2200), integer_clamp.clamp("2200"));
            assert_eq!(ParsedNum::new(2025), integer_clamp.clamp("2025"));
            assert_eq!(ParsedNum::new(1980), integer_clamp.clamp("1980"));
            assert_eq!(ParsedNum::new(1979), integer_clamp.clamp("1979"));
            assert_eq!(ParsedNum::new(1999), integer_clamp.clamp("1999"));
            assert_eq!(ParsedNum::new(1999), integer_clamp.clamp("19999"));
            assert_eq!(ParsedNum::new(2300), integer_clamp.clamp("23092"));

            assert_eq!(ParsedNum::new(2), integer_clamp.clamp("2"));
            assert_eq!(ParsedNum::new(23), integer_clamp.clamp("25"));
            assert_eq!(ParsedNum::new(205), integer_clamp.clamp("205"));
            assert_eq!(ParsedNum::new(2300), integer_clamp.clamp("2500"));
            assert_eq!(ParsedNum::new(2300), integer_clamp.clamp("99999"));
            assert_eq!(ParsedNum::new(193), integer_clamp.clamp("193"));
            assert_eq!(ParsedNum::new(209), integer_clamp.clamp("209"));
            // ω <fn test IntegerClampStrings::clamp>
        }

        // α <mod-def test_integer_clamp_strings>
        use super::*;
        // ω <mod-def test_integer_clamp_strings>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def integer_clamp>
// ω <mod-def integer_clamp>
