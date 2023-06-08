

pub fn clamp(year_input: &str, max_year: u32, min_year: u32) -> bool {
    let mut filter_max = false; //range checks
    let mut filter_min = false;

    let num_digits_in_range = (max_year as f64).log10() as usize + 1;
    debug_assert!(
        num_digits_in_range == (min_year as f64).log10() as usize + 1,
        "min and max must be same number of digits"
    );
    //assert_eq!(42, num_digits_in_range);

    if year_input.len() > num_digits_in_range {
        //check size
        return false;
    }

    for (i, c) in year_input.chars().enumerate() {
        
        //assert!(c.to_digit(10).is_some());

        debug_assert!(c.to_digit(10).is_some(), "all characters must be digits");

        //let current_power = u32::pow(10, (num_digits_in_range - 1 - i) as u32); //some of these variables are leftovers and probably could be moved inside
        let current_power = super::get_digit_power::get_digit_power(num_digits_in_range, i); //some of these variables are leftovers and probably could be moved inside
        let current_digit = c.to_digit(10).unwrap() as u32;

        let max_so_far = max_year / current_power % 10;
        let min_so_far = min_year / current_power % 10;

        #[cfg(debug_assertions)]
        println!("Examining character ({c}) against ({min_so_far},{max_so_far})");
        
        if !filter_max && current_digit < max_so_far {
            //check within upper range
            filter_max = true;
        } else if !filter_max && current_digit > max_so_far {
            //check outside outer range
            return false;
        }
        if !filter_min && current_digit > min_so_far {
            //check inside lower range
            filter_min = true;
        } else if !filter_min && current_digit < min_so_far {
            //check outside lower range
            return false;
        }
    }

    return true;
}

#[cfg(test)]
pub mod clamp_test {
    use crate::clamp::clamp;

    #[test]
    fn clamp_test() {
        let mut max = 3800000;
        let mut min = 3500000;
        assert_eq!(true, clamp("3507001", max, min));

        min = 1900;
        max = 2300;
        assert_eq!(true, clamp("2200", max, min));
        assert_eq!(true, clamp("2025", max, min));
        assert_eq!(true, clamp("1980", max, min));
        assert_eq!(true, clamp("1979", max, min));
        assert_eq!(true, clamp("1999", max, min));
        assert_eq!(false, clamp("2500", max, min));
        assert_eq!(false, clamp("99999", max, min));
        // assert_eq!(false, clamp("-1", max, min));
        // assert_eq!(false, clamp("hi", max, min));
        assert_eq!(true, clamp("193", max, min));
        assert_eq!(true, clamp("209", max, min));
    }
}
