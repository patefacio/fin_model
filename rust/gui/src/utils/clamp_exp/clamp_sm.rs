use std::cmp::Ordering;

#[inline]
//allow number to be returned as a u32 and string
fn parsed_num(n: u32) -> (u32, String) {
    (n, n.to_string())
}

#[derive(Debug)]
//Possible states to be in
enum TrackingState {
    TrackingBottom,
    TrackingTop,
    TrackingTopAndBottom,
    BreachedBottom,
    BreachedTop,
    UsingInput,
}

#[inline]
//depending on the state, this function will add the appropriate digit to the final result to be returned
fn push_digit_to_result(n: u32, digit: u32) -> u32 {
    n * 10 + digit
    
}
////////////////////////////////////////////////////////////////////////////////////
/// TODO: Comment this function.
/// 
pub fn clamp(year_input: &str, min_year: u32, max_year: u32) -> (u32, String) {
    //initial state is tracking the top and bottom values, as if 00001980 & 00002300
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

            TrackingState::TrackingBottom => {
                if compared_to_bottom == Ordering::Equal {
                    result_as_u32 = push_digit_to_result(result_as_u32, c_as_u32);
                    TrackingState::TrackingBottom
                } else if compared_to_bottom == Ordering::Less {
                    result_as_u32 = push_digit_to_result(result_as_u32, current_bottom_digit);
                    TrackingState::BreachedBottom
                } else if compared_to_bottom == Ordering::Greater {
                    result_as_u32 = push_digit_to_result(result_as_u32, c_as_u32);
                    TrackingState::UsingInput
                } else {
                    panic!("Missed something!")
                }
            }

            TrackingState::TrackingTop => {
                if compared_to_top == Ordering::Equal {
                    result_as_u32 = push_digit_to_result(result_as_u32, c_as_u32);
                    TrackingState::TrackingTop
                } else if compared_to_top == Ordering::Less {
                    result_as_u32 = push_digit_to_result(result_as_u32, c_as_u32);
                    TrackingState::UsingInput
                } else if compared_to_top == Ordering::Greater {
                    result_as_u32 = push_digit_to_result(result_as_u32, current_top_digit);
                    TrackingState::BreachedTop
                } else {
                    panic!("Missed something!")
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

    (result_as_u32, result_as_u32.to_string())
}

fn main() {
   // println!("Hello, world!");
}


#[cfg(test)]
pub mod clamp_sm_test {

    use super::*;

    #[test]
    fn clamp_test() {
        let mut min = 3500000;
        let mut max = 3800000;
        assert_eq!(parsed_num(3507001), clamp("3507001", min, max));

        let mut min = 1900;
        let mut max = 2300;
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
        assert_eq!(parsed_num(193), clamp("193", min, max));
        assert_eq!(parsed_num(209), clamp("209", min, max));
    }
}
