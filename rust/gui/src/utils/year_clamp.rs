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

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl YearClamp {
    /// Will return the year clamped to a min/max.
    ///
    ///   * **value** - The value to clamp.
    ///   * _return_ - The clamped value.
    pub fn clamp(&self, value: &str) -> Year {
        // α <fn YearClamp::clamp>

        leptos_dom::console_log(&format!("Attempting to clamp({value})"));

        // We know all characters in the string must be digits
        debug_assert!(value.bytes().all(|b| (b as char).is_ascii_digit()));

        let num_digits = value.len();

        if let Some(year_range) = self.ranges_by_digit.get(num_digits - 1) {
            // Index into the proper ranges_by_digit, do the clamp and return the number
            // TODO:
        }

        // For now, just return the input
        value.parse::<u32>().expect("Should be valid year")

        // ω <fn YearClamp::clamp>
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

        let ranges_by_digit = Vec::new();
        //////////////////////////
        let num_digits_start = (year_range.start as f64).log10() as usize - 1;
        let num_digits_end = (year_range.end as f64).log10() as usize - 1;
        let vec_size = num_digits_start.max(num_digits_end);
        
        // iterate from 0 to vec_size and for each number of digits create a new
        // range of that size and push into vec.

        // TODO: Push the ranges here

        YearClamp {
            year_range,
            ranges_by_digit,
        }
        // ω <fn YearClamp::new>
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
                end: 2300,
            });

            println!("Clamping 2023 -> {}", year_clamp.clamp("2023"));

            assert_eq!(1, year_clamp.clamp("1"));
            assert_eq!(2, year_clamp.clamp("2"));
            assert_eq!(2, year_clamp.clamp("3"));
            assert_eq!(0, year_clamp.clamp("0"));
            assert_eq!(18, year_clamp.clamp("19"));

            // ω <fn test YearClamp::clamp>
        }

        // α <mod-def test_year_clamp>
        use super::*;
        // ω <mod-def test_year_clamp>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def year_clamp>
// ω <mod-def year_clamp>
