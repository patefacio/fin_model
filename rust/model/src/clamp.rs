pub fn clamp(year_input: &str, max_year: u32, min_year: u32) -> (u32, String) {
    /* 
        This function will take in a string containing numbers
        and two integers, an upper bound and a lower bound
        and will clamp the digits of the string to be inside the bounds
        returns a tuple with the clamped value as an integer and as a string

        The number of digits in the boundaries must be the same,
        but there is no restriction on the number of characters on the input
     */

    let mut inside_upper_bound = false; //range checks
    let mut inside_lower_bound = false;

    let num_digits_in_range = (max_year as f64).log10() as usize + 1;   //size of bounds must be the same
    debug_assert!(
        num_digits_in_range == (min_year as f64).log10() as usize + 1,
        "min and max must be same number of digits"
    );

    let mut return_year = String::with_capacity(num_digits_in_range);   // String initializatoin

    for (i, mut c) in year_input[0..num_digits_in_range.min(year_input.len())]
        .chars()
        .enumerate()
    {
        debug_assert!(c.to_digit(10).is_some(), "all characters must be digits");

        let current_power = u32::pow(10, (num_digits_in_range - 1 - i) as u32);
        let input_char_digit = c.to_digit(10).unwrap() as u32;

        let digit_from_max = max_year / current_power % 10;
        let digit_from_min = min_year / current_power % 10;

        #[cfg(debug_assertions)]    //checks if the number is within the upper bound, skips if so
        if !inside_upper_bound {    // clamps to the upper bound value if not
            if input_char_digit < digit_from_max {
                inside_upper_bound = true;
            } else if input_char_digit > digit_from_max {
                c = char::from_digit(digit_from_max, 10).unwrap();
            }
        }

        if !inside_lower_bound {    //Same as above but for the lower bound
            if input_char_digit > digit_from_min {
                inside_lower_bound = true;
            } else if input_char_digit < digit_from_min {
                c = char::from_digit(digit_from_min, 10).unwrap();
            }
        }
        return_year.push(c);
    }

    return (return_year.parse().unwrap(), return_year);
}

<<<<<<< HEAD

=======
>>>>>>> 81e98e99479558af042d8a7e4564ee708dbc6a99
#[cfg(test)]
pub mod clamp_test {
    use crate::clamp::clamp;

    fn parsed_num(n: u32) -> (u32, String) {
        (n, n.to_string())
    }

    #[test]
    fn clamp_test() {
        let max = 2200;
        let min = 1980;
<<<<<<< HEAD
        assert_eq!((1984, String::from("1984")), clamp("1984", max, min));
        assert_eq!((2025, String::from("2025")), clamp("2025", max, min));
        assert_eq!((2200, String::from("2200")), clamp("2200", max, min));
        assert_eq!((1980, String::from("1980")), clamp("1980", max, min));
        assert_eq!((1989, String::from("1989")), clamp("1979", max, min));
        assert_eq!((1999, String::from("1999")), clamp("1999", max, min));
        assert_eq!((2200, String::from("2200")), clamp("2540", max, min));
        assert_eq!((2200, String::from("2200")), clamp("99999", max, min));
        assert_eq!((198, String::from("198")), clamp("193", max, min));
        assert_eq!((209, String::from("209")), clamp("209", max, min));
    }
}

=======
        assert_eq!(parsed_num(1984), clamp("1984", max, min));
        assert_eq!(parsed_num(2025), clamp("2025", max, min));
        assert_eq!(parsed_num(2200), clamp("2200", max, min));
        assert_eq!(parsed_num(1980), clamp("1980", max, min));
        assert_eq!(parsed_num(1980), clamp("1979", max, min));
        assert_eq!(parsed_num(1999), clamp("1999", max, min));
        assert_eq!(parsed_num(2200), clamp("2540", max, min));
        assert_eq!(parsed_num(2200), clamp("99999", max, min));
        assert_eq!(parsed_num(198), clamp("193", max, min));
        assert_eq!(parsed_num(209), clamp("209", max, min));
    }
}
>>>>>>> 81e98e99479558af042d8a7e4564ee708dbc6a99

//  294.95 296.12 294.55

//  297.28 297.12 298.64
