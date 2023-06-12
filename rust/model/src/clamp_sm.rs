use std::cmp::Ordering;

#[inline]
fn parsed_num(n: u32) -> (u32, String) {
    (n, n.to_string())
}

#[derive(Debug)]
enum TrackingState {
    TrackingTop,
    TrackingBottom,
    TrackingTopAndBottom,
    BreachedTop,
    BreachedBottom,
    UsingInput,
}

///
/// Picture three rows of digits corresponding to the digits of the input,
/// the min_year, and the max_year as three rows of data:
///
/// top row pulled from max_year    =>  t(0)  t(1) ...(t(digits_to_consider - 1))
///
/// input row                       =>  c(0)  c(1) ...(c(digits_to_consider - 1))
///
/// bottom row pulled from min_year =>  b(0)  b(1) ...(b(digits_to_consider - 1))
///
/// Define the min year as the "Bottom" and max year as the "Top".
/// Consider the ith digit of the three number in question as follows:
///
/// year_input => c(i) for _current_ of input. So c(2) means 3rd digit of input
/// min_year => b(i) for min_year (aka Bottom). So b(2) means 3rd digit of bottom(min_year)
/// max_year => t(i) for max_year (aka Top). So t(2) means 3rd digit of top(max_year)
///
/// The idea is progress in finding a result by processing each digit of all three rows
/// and for each digit i looking only at the **current** state and the three values
/// t(i), c(i), and b(i)
///
pub fn clamp(year_input: &str, min_year: u32, max_year: u32) -> (u32, String) {
    tracing::info!("Processing `{year_input}` on ({min_year}, {max_year})");
    let input_digit_count = year_input.len();
    let range_digit_count = (max_year as f64).log10() as usize + 1;
    debug_assert!(
        range_digit_count == (min_year as f64).log10() as usize + 1,
        "min and max must be same number of digits"
    );

    // In all cases, consider only the minimum digits between input and the acceptable year range
    let digits_to_consider = input_digit_count.min(range_digit_count);
    // Assume a start state of tracking top and bottom (i.e. think of some number of leading 0's)
    let mut state = TrackingState::TrackingTopAndBottom;
    // After each character is processed one digit is _appended_ to this result
    let mut result_as_u32 = 0u32;
    // Initialize digit_divisor to 10^(digits_to_consider-1). If considering 4 digits, this
    // is then 1,000. If you take 3 zeros off a 4 digit number you have the first digit
    let mut digit_divisor = u32::pow(10, range_digit_count as u32 - 1);
    // Track the bottom and top row digit
    let (mut current_bottom_digit, mut current_top_digit);

    // closure that appends a digit to result
    let mut append_digit = |digit| result_as_u32 = result_as_u32 * 10 + digit;

    for c in year_input.chars() {
        let c_as_u32 = c.to_digit(10).expect("Input chars filtered to digits");
        (current_bottom_digit, current_top_digit) = (
            (min_year / digit_divisor) % 10,
            (max_year / digit_divisor) % 10,
        );
        let compared_to_bottom = c_as_u32.cmp(&current_bottom_digit);
        let compared_to_top = c_as_u32.cmp(&current_top_digit);

        tracing::info!("{state:?} -> ({current_bottom_digit}, {c_as_u32}, {current_top_digit}) vs_top({compared_to_top:?}), vs_bottom({compared_to_bottom:?})");

        state = match state {
            // If tracking both that means all prior positions of
            TrackingState::TrackingTopAndBottom => {
                if compared_to_bottom == Ordering::Equal && compared_to_top == Ordering::Equal {
                    append_digit(c_as_u32);
                    TrackingState::TrackingTopAndBottom
                } else if compared_to_bottom == Ordering::Greater
                    && compared_to_top == Ordering::Less
                {
                    append_digit(c_as_u32);
                    TrackingState::UsingInput
                } else if compared_to_top == Ordering::Greater {
                    append_digit(current_top_digit);
                    TrackingState::BreachedTop
                } else if compared_to_bottom == Ordering::Less {
                    append_digit(current_bottom_digit);
                    TrackingState::BreachedBottom
                } else if compared_to_bottom == Ordering::Greater {
                    append_digit(c_as_u32);
                    TrackingState::TrackingTop
                } else if compared_to_top == Ordering::Less {
                    append_digit(c_as_u32);
                    TrackingState::TrackingBottom
                } else {
                    panic!("Missed something")
                }
            }
            TrackingState::TrackingTop => match compared_to_top {
                std::cmp::Ordering::Greater => {
                    append_digit(current_top_digit);
                    TrackingState::BreachedTop
                }
                std::cmp::Ordering::Less => {
                    append_digit(c_as_u32);
                    TrackingState::UsingInput
                }
                std::cmp::Ordering::Equal => {
                    append_digit(c_as_u32);
                    TrackingState::TrackingTop
                }
            },
            TrackingState::TrackingBottom => match compared_to_bottom {
                std::cmp::Ordering::Less => {
                    append_digit(current_bottom_digit);
                    TrackingState::BreachedBottom
                }
                std::cmp::Ordering::Greater => {
                    append_digit(c_as_u32);
                    TrackingState::UsingInput
                }
                std::cmp::Ordering::Equal => {
                    append_digit(c_as_u32);
                    TrackingState::TrackingBottom
                }
            },
            TrackingState::BreachedTop => {
                append_digit(current_top_digit);
                TrackingState::BreachedTop
            }
            TrackingState::BreachedBottom => {
                append_digit(current_bottom_digit);
                TrackingState::BreachedBottom
            }
            TrackingState::UsingInput => {
                append_digit(c_as_u32);
                TrackingState::UsingInput
            }
        };

        // Take a 0 off digit divisor so next iteration is looking at subsequent digit of min/max
        digit_divisor /= 10;

        if digit_divisor == 0 {
            break;
        }

        tracing::info!("To {state:?}");
    }

    (result_as_u32, result_as_u32.to_string())
}

#[cfg(test)]
pub mod clamp_sm_test {

    use super::*;

    #[test]
    fn clamp_test() {
        let mut min = 3500000;
        let mut max = 3800000;
        assert_eq!(parsed_num(3507001), clamp("3507001", min, max));

        min = 1900;
        max = 2300;
        assert_eq!(parsed_num(2200), clamp("2200", min, max));
        assert_eq!(parsed_num(2025), clamp("2025", min, max));
        assert_eq!(parsed_num(1980), clamp("1980", min, max));
        assert_eq!(parsed_num(1979), clamp("1979", min, max));
        assert_eq!(parsed_num(1999), clamp("1999", min, max));
        assert_eq!(parsed_num(2), clamp("2", min, max));
        assert_eq!(parsed_num(23), clamp("25", min, max));
        assert_eq!(parsed_num(205), clamp("205", min, max));
        assert_eq!(parsed_num(2300), clamp("2500", min, max));
        assert_eq!(parsed_num(2300), clamp("99999", min, max));
        //assert_eq!(parsed_num(), clamp("-1", min, max));
        //assert_eq!(false, clamp("hi", min, max));
        assert_eq!(parsed_num(193), clamp("193", min, max));
        assert_eq!(parsed_num(209), clamp("209", min, max));
    }
}
