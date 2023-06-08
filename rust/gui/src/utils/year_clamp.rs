////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Year;
use fin_model::core::YearRange;

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// The struct is configured with a [YearRange] and supports clamping input year
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
pub struct YearClamp {
    /// The supplied range (inclusive) of valid years.
    pub year_range: YearRange,
    /// A vector of [YearRange] instances created on construction, one for each number of digits up to the
    /// number of digits in `year_range.end`.
    ///
    /// So for the range `{ start: 1900, end: 2300 }` the resulting vector would look like:
    ///
    /// vec!{
    ///    YearRange{ start: 1, end: 2 },
    ///    YearRange{ start: 19, end: 23 },
    ///    YearRange{ start: 190, end: 230 }
    /// }
    ///
    /// Now to clamp a value, the function can find the length of the string and index
    /// into the appropriate range in the vector to make the check.
    pub ranges_by_digit: Vec<YearRange>,
}

/// A number as both string and parsed u32 value
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParsedNum {
    /// The number as u32
    pub as_u32: u32,
    /// The number as string
    pub as_string: String,
}

/// Year clamp where the ranges are stored as strings
pub struct YearClampStrings {
    /// The supplied range (inclusive) of valid years.
    pub year_range: YearRange,
    /// A vector of [YearRange] instances created on construction, one for each number of digits up to the
    /// number of digits in `year_range.end`.
    ///
    /// So for the range `{ start: 1900, end: 2300 }` the resulting vector would look like:
    ///
    /// vec!{
    ///    YearRange{ start: 1, end: 2 },
    ///    YearRange{ start: 19, end: 23 },
    ///    YearRange{ start: 190, end: 230 }
    /// }
    ///
    /// Now to clamp a value, the function can find the length of the string and index
    /// into the appropriate range in the vector to make the check.
    pub ranges_by_digit: Vec<(ParsedNum, ParsedNum)>,
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl YearClamp {
    /// Will return the year clamped to a min/max.
    ///
    ///   * **value** - The value to clamp.
    ///   * _return_ - The clamped value.
    #[inline]
    pub fn clamp(&self, value: &str) -> ParsedNum {
        // α <fn YearClamp::clamp>

        use std::cmp::Ordering;

        // We know all characters in the string must be digits
        debug_assert!(value
            .bytes()
            .all(|b| (b as char).is_ascii_digit()));

        let num_digits = value.len();
        let value_as_u32 = value.parse::<u32>().expect("valid number");

        if let Some(year_range) = self.ranges_by_digit.get(num_digits - 1) {
            // Index into the proper ranges_by_digit, do the clamp and return the number
            let clamped = value_as_u32.clamp(year_range.start, year_range.end);
            ParsedNum::new(clamped)
        } else {
            ParsedNum::new(self.ranges_by_digit.last().expect("not empty").end)
        }

        // ω <fn YearClamp::clamp>
    }
}

impl YearClampStrings {
    /// Will return the year clamped to a min/max.
    ///
    ///   * **value** - The value to clamp.
    ///   * _return_ - The clamped value.
    #[inline]
    pub fn clamp(&self, value: &str) -> ParsedNum {
        // α <fn YearClampStrings::clamp>

        // We know all characters in the string must be digits
        debug_assert!(value.bytes().all(|b| (b as char).is_ascii_digit()));
        let num_digits = value.len();

        if let Some((start_parsed_num, end_parsed_num)) = self.ranges_by_digit.get(num_digits - 1) {
            // Index into the proper ranges_by_digit, do the clamp and return the number
            let clamped = value.clamp(&start_parsed_num.as_string, &end_parsed_num.as_string);
            ParsedNum::from_str(clamped)
        } else {
            ParsedNum::from_str(value)
        }

        // ω <fn YearClampStrings::clamp>
    }
}

impl YearClamp {
    /// Create new instance of YearClamp
    ///
    ///   * **year_range** - The supplied range (inclusive) of valid years.
    ///   * _return_ - The new instance
    pub fn new(year_range: YearRange) -> YearClamp {
        // α <fn YearClamp::new>

        debug_assert!(year_range.start <= year_range.end, "Start <= end");

        //////////////////////////
        let num_digits_start = (year_range.start as f64).log10().round() as usize + 1;
        let num_digits_end = (year_range.end as f64).log10().round() as usize + 1;
        let vec_size = num_digits_start.max(num_digits_end);
        let mut ranges_by_digit: Vec<YearRange> = Vec::with_capacity(vec_size);

        // Convert the numbers in range to strings so iteration over digits is simpler
        let start_year = year_range.start.to_string();
        let end_year = year_range.end.to_string();

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

            ranges_by_digit.push(YearRange {
                start: start_partial_value,
                end: end_partial_value,
            });
        }

        println!("ranges_by_digit -> {ranges_by_digit:?}");

        YearClamp {
            year_range,
            ranges_by_digit,
        }
        // ω <fn YearClamp::new>
    }
}

impl ParsedNum {
    /// Create from str by parsing - panics with invalid number.
    ///
    ///   * **number** - The number as String
    ///   * _return_ - The parsed number
    #[inline]
    pub fn from_string(number: String) -> ParsedNum {
        // α <fn ParsedNum::from_string>

        ParsedNum {
            as_u32: number.parse::<u32>().expect("valid number"),
            as_string: number,
        }
        // ω <fn ParsedNum::from_string>
    }

    /// Create from str by parsing - panics with invalid number.
    ///
    ///   * **number** - The number as str
    ///   * _return_ - The parsed number
    #[inline]
    pub fn from_str(number: &str) -> ParsedNum {
        // α <fn ParsedNum::from_str>
       
       ParsedNum::from_string(number.to_string())
       
        // ω <fn ParsedNum::from_str>
    }

    /// Create new instance of ParsedNum
    ///
    ///   * **as_u32** - The number as u32
    ///   * _return_ - The new instance
    #[inline]
    pub fn new(as_u32: u32) -> ParsedNum {
        // α <fn ParsedNum::new>
        ParsedNum {
            as_u32,
            as_string: as_u32.to_string(),
        }
        // ω <fn ParsedNum::new>
    }
}

impl YearClampStrings {
    /// Create new instance of YearClampStrings
    ///
    ///   * **year_range** - The supplied range (inclusive) of valid years.
    ///   * _return_ - The new instance
    pub fn new(year_range: YearRange) -> YearClampStrings {
        // α <fn YearClampStrings::new>

        debug_assert!(year_range.start <= year_range.end, "Start <= end");

        //////////////////////////
        let num_digits_start = (year_range.start as f64).log10().round() as usize + 1;
        let num_digits_end = (year_range.end as f64).log10().round() as usize + 1;
        let vec_size = num_digits_start.max(num_digits_end);
        let mut ranges_by_digit: Vec<(ParsedNum, ParsedNum)> = Vec::with_capacity(vec_size);

        // Convert the numbers in range to strings so iteration over digits is simpler
        let start_year = year_range.start.to_string();
        let end_year = year_range.end.to_string();

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

        println!("ranges_by_digit -> {ranges_by_digit:?}");

        YearClampStrings {
            year_range,
            ranges_by_digit,
        }

        // ω <fn YearClampStrings::new>
    }
}

/// Unit tests for `year_clamp`
#[cfg(test)]
pub mod unit_tests {

    /// Test type YearClamp
    mod test_year_clamp {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn clamp() {
            // α <fn test YearClamp::clamp>

            let year_clamp = YearClamp::new(YearRange {
                start: 1990,
                end: 23000,
            });

            println!("Clamping 2023 -> {:?}", year_clamp.clamp("2023"));

            assert_eq!(ParsedNum::new(1), year_clamp.clamp("1"));
            assert_eq!(ParsedNum::new(2), year_clamp.clamp("2"));
            assert_eq!(ParsedNum::new(2), year_clamp.clamp("3"));
            assert_eq!(ParsedNum::new(1), year_clamp.clamp("0"));
            assert_eq!(ParsedNum::new(19), year_clamp.clamp("18"));

            // ω <fn test YearClamp::clamp>
        }

        // α <mod-def test_year_clamp>
        use super::*;
        // ω <mod-def test_year_clamp>
    }

    /// Test type YearClampStrings
    mod test_year_clamp_strings {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn clamp() {
            // α <fn test YearClampStrings::clamp>

            let year_clamp = YearClampStrings::new(YearRange {
                start: 1990,
                end: 23000,
            });

            println!("Clamping 2023 -> {:?}", year_clamp.clamp("2023"));

            assert_eq!(ParsedNum::new(1), year_clamp.clamp("1"));
            assert_eq!(ParsedNum::new(2), year_clamp.clamp("2"));
            assert_eq!(ParsedNum::new(2), year_clamp.clamp("3"));
            assert_eq!(ParsedNum::new(1), year_clamp.clamp("0"));
            assert_eq!(ParsedNum::new(19), year_clamp.clamp("18"));

            // ω <fn test YearClampStrings::clamp>
        }

        // α <mod-def test_year_clamp_strings>
        use super::*;
        // ω <mod-def test_year_clamp_strings>
    }

    /// Test type ParsedNum
    mod test_parsed_num {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn from_string() {
            // α <fn test ParsedNum::from_string>
            todo!("Test from_string")
            // ω <fn test ParsedNum::from_string>
        }

        #[test]
        fn from_str() {
            // α <fn test ParsedNum::from_str>
            todo!("Test from_str")
            // ω <fn test ParsedNum::from_str>
        }

        // α <mod-def test_parsed_num>
        // ω <mod-def test_parsed_num>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def year_clamp>
// ω <mod-def year_clamp>
