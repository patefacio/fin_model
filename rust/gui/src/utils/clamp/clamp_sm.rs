//! The clamp function implemented as a state machine.

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::ParsedNum;

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// Enumerates the states for a process of performing a clamp function
#[derive(Debug, Copy, Clone)]
pub enum TrackingState {
    /// Indicates all digits processed so far are equal to _minimum_.
    /// Further - they *do not* also equal the _maximum_.
    TrackingBottom,
    /// Indicates all digits processed so far are equal to _maximum_.
    /// Further - they *do not* also equal the _minimum_.
    TrackingTop,
    /// Indicates all digits processed so far are equal to _minimum_ *and* _maximum.
    TrackingTopAndBottom,
    /// Indicates the number _would be_ smaller than the _minimum_.
    BreachedBottom,
    /// Indicates the number _would be_ greater than the _maximum_.
    BreachedTop,
    /// Indicates the number is strictly within _minimum_ and _maximum_.
    UsingInput,
}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Add `digit` to end of `n`
///
///   * **n** - Number to extend
///   * **digit** - Digit to add to `n`
///   * _return_ - Result of pushing `digit` to end of `n`
#[inline]
pub fn push_digit_to_result(n: u32, digit: u32) -> u32 {
    // α <fn push_digit_to_result>
    n * 10 + digit
    // ω <fn push_digit_to_result>
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

    use std::cmp::Ordering;

    let mut state = TrackingState::TrackingTopAndBottom;
    let mut result_as_u32: u32 = 0u32;
    let (mut current_bottom_digit, mut current_top_digit);
    let min_year_digit_count = (min_year as f64).log10() as usize + 1;
    //if year_input only has 3 chars, you only want to compare the first three of min & max
    let digits_considered = year_input.len().min(min_year_digit_count);
    let mut digit_divisor = u32::pow(10, min_year_digit_count as u32 - 1);

    log::info!("Starting year_input:`{year_input}`, min_year: `{min_year}`, max_year: `{max_year}` ");
    log::info!("Min_year_digit_count: {min_year_digit_count}, digits_considered: {digits_considered}, digit_divisor: {digit_divisor}");

    for c in year_input.chars() {
        let c_as_u32 = c.to_digit(10).expect("Input chars converted to digits");
        //current_bottom_digit & current_top_digit are the min/max digits for the current iteration
        //divides by 1000, 100, etc, and gets rid of the extra digits after the "decimal point"
        (current_bottom_digit, current_top_digit) = (
            (min_year / digit_divisor) % 10,
            (max_year / digit_divisor) % 10,
        );

        //c_as_u32 is the user inputted digit for the current iteration
        let compared_to_bottom = c_as_u32.cmp(&current_bottom_digit);
        let compared_to_top = c_as_u32.cmp(&current_top_digit);
        //tracing::info!...
        log::info!("The current state is: {state:?}, c is {c:?}, {c_as_u32}, current_top_digit is: {current_top_digit}, 
        current_bottom_digit: {current_bottom_digit}");

        state = match state {
            TrackingState::TrackingTopAndBottom => {
                if compared_to_bottom == Ordering::Equal && compared_to_top == Ordering::Equal {
                    result_as_u32 = push_digit_to_result(result_as_u32, c_as_u32);
                    TrackingState::TrackingTopAndBottom
                } else if compared_to_bottom == Ordering::Greater
                    && compared_to_top == Ordering::Less
                {
                    result_as_u32 = push_digit_to_result(result_as_u32, c_as_u32);
                    TrackingState::UsingInput
                } else if compared_to_bottom == Ordering::Less {
                    result_as_u32 = push_digit_to_result(result_as_u32, current_bottom_digit);
                    TrackingState::BreachedBottom
                } else if compared_to_top == Ordering::Greater {
                    result_as_u32 = push_digit_to_result(result_as_u32, current_top_digit);
                    TrackingState::BreachedTop
                } else if compared_to_bottom == Ordering::Greater {
                    result_as_u32 = push_digit_to_result(result_as_u32, c_as_u32);
                    TrackingState::TrackingTop
                } else if compared_to_top == Ordering::Less {
                    result_as_u32 = push_digit_to_result(result_as_u32, c_as_u32);
                    TrackingState::TrackingBottom
                } else {
                    panic!("Missed something!")
                }
            }

            TrackingState::TrackingBottom => match compared_to_bottom {
                Ordering::Equal => {
                    result_as_u32 = push_digit_to_result(result_as_u32, c_as_u32);
                    TrackingState::TrackingBottom
                }
                Ordering::Less => {
                    result_as_u32 = push_digit_to_result(result_as_u32, current_bottom_digit);
                    TrackingState::BreachedBottom
                }
                Ordering::Greater => {
                    result_as_u32 = push_digit_to_result(result_as_u32, c_as_u32);
                    TrackingState::UsingInput
                }
            }

            TrackingState::TrackingTop => match compared_to_top {
                Ordering::Equal => {
                    result_as_u32 = push_digit_to_result(result_as_u32, c_as_u32);
                    TrackingState::TrackingTop
                }
                Ordering::Less => {
                    result_as_u32 = push_digit_to_result(result_as_u32, c_as_u32);
                    TrackingState::UsingInput
                }
                Ordering::Greater => {
                    result_as_u32 = push_digit_to_result(result_as_u32, current_top_digit);
                    TrackingState::BreachedTop
                }
            }

            TrackingState::BreachedBottom => {
                result_as_u32 = push_digit_to_result(result_as_u32, current_bottom_digit);
                TrackingState::BreachedBottom 
                
            }

            TrackingState::BreachedTop => {
                result_as_u32 = push_digit_to_result(result_as_u32, current_top_digit);
                TrackingState::BreachedTop
            }

            TrackingState::UsingInput => {
                result_as_u32 = push_digit_to_result(result_as_u32, c_as_u32);
                TrackingState::UsingInput
            }
        };

        //eliminate a 0 for the next iteration, so divide year input, min, & max by 1000, then 100... 
        digit_divisor /= 10;
        if digit_divisor == 0 {
            break;
        }
    }

    ParsedNum::new(result_as_u32)
    // ω <fn clamp>
}

/// Unit tests for `clamp_sm`
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

// α <mod-def clamp_sm>
// ω <mod-def clamp_sm>
