
//////////////////////////////////////////////////////////////////////////////
/// This function will take in a string containing numbers
/// and two integers, an upper bound and a lower bound
/// and will clamp the digits of the string to be inside the bounds
/// returns a tuple with the clamped value as an integer and as a string
///
/// The number of digits in the boundaries must be the same,
/// but there is no restriction on the number of characters on the input
///
pub fn clamp(year_input: &str, min_year: u32, max_year: u32) -> (u32, String) {
    let mut inside_lower_bound = false; //range checks
    let mut inside_upper_bound = false;

    let num_digits_in_range = (max_year as f64).log10() as usize + 1; //size of bounds must be the same
    debug_assert!(
        num_digits_in_range == (min_year as f64).log10() as usize + 1,
        "min and max must be same number of digits"
    );

    // String initialization
    //let mut return_year = String::with_capacity(num_digits_in_range);
    let size = num_digits_in_range.min(year_input.len());

    for (i, c) in year_input[0..size]
        .chars()
        .enumerate()
    {
        debug_assert!(c.to_digit(10).is_some(), "all characters must be digits");

        let current_power = u32::pow(10, (num_digits_in_range - 1 - i) as u32);
        let input_char_digit = c.to_digit(10).unwrap() as u32;

        let digit_from_min = min_year / current_power % 10;
        let digit_from_max = max_year / current_power % 10;

        //#[cfg(debug_assertions)]
        // checks if the number is within the upper bound, skips if so
        // clamps to the upper bound value if not
        
        if !inside_lower_bound {
            //Same as above but for the lower bound
            if input_char_digit > digit_from_min {
                inside_lower_bound = true;
            } else if input_char_digit < digit_from_min {
                return (min_year/(u32::pow(10, (num_digits_in_range-size) as u32)), (min_year/(u32::pow(10, (num_digits_in_range-size) as u32))).to_string());
                //c = char::from_digit(digit_from_min, 10).unwrap();
            }
        }

        if !inside_upper_bound {
            if input_char_digit < digit_from_max {
                inside_upper_bound = true;
            } else if input_char_digit > digit_from_max {
                return (max_year/(u32::pow(10, (num_digits_in_range-size) as u32)), (max_year/u32::pow(10, (num_digits_in_range-size) as u32)).to_string());
                //c = char::from_digit(digit_from_max, 10).unwrap();
            }
        }

        
        //return_year.push(c);
    }
    return (year_input.parse().unwrap(), year_input.to_string());
    //return_year=return_year+year_input;
    //return (return_year.parse().unwrap(), return_year);
}
#[cfg(test)]
pub mod clamp_test {
    use super::*;

    fn parsed_num(n: u32) -> (u32, String) {
        (n, n.to_string())
    }

    #[test]
    fn clamp_test() {
        let min = 1980;
        let max = 2200;
        
        assert_eq!(parsed_num(1984), clamp("1984", min, max));
        assert_eq!(parsed_num(2025), clamp("2025", min, max));
        assert_eq!(parsed_num(2200), clamp("2200", min, max));
        assert_eq!(parsed_num(1980), clamp("1980", min, max));
        assert_eq!(parsed_num(1980), clamp("1979", min, max));
        assert_eq!(parsed_num(1999), clamp("1999", min, max));
        assert_eq!(parsed_num(2200), clamp("2540", min, max));
        assert_eq!(parsed_num(2200), clamp("99999", min, max));
        assert_eq!(parsed_num(198), clamp("193", min, max));
        assert_eq!(parsed_num(209), clamp("209", min, max));
        
        println!("{:?} , {:?}", parsed_num(1984), clamp("1984", min, max));
        println!("{:?} , {:?}", parsed_num(2025), clamp("2025", min, max));
        println!("{:?} , {:?}", parsed_num(2200), clamp("2200", min, max));
        println!("{:?} , {:?}", parsed_num(1980), clamp("1980", min, max));
        println!("{:?} , {:?}", parsed_num(1980), clamp("1979", min, max));
        println!("{:?} , {:?}", parsed_num(1999), clamp("1999", min, max));
        println!("{:?} , {:?}", parsed_num(2200), clamp("2540", min, max));
        println!("{:?} , {:?}", parsed_num(2200), clamp("99999", min, max));
        println!("{:?} , {:?}", parsed_num(198), clamp("193", min, max));
        println!("{:?} , {:?}", parsed_num(209), clamp("209", min, max));
        
    }
}

//  294.95 296.12 294.55

//  297.28 297.12 298.64
