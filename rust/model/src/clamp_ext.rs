#[inline]
fn parsed_num(n: u32) -> (u32, String) {
    (n, n.to_string())
}

#[inline]
fn truncate_digits(mut n: u32, moribund_digits: usize) -> u32 {
    for _ in 0..moribund_digits {
        n /= 10;
    }
    n
}

#[inline]
pub fn clamp_ext(
    year_input: &str,
    input_digit_count: usize,
    min_year: u32,
    max_year: u32,
    range_digit_count: usize,
) -> (u32, String) {
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
        Ordering::Equal => parsed_num(
            year_input
                .parse::<u32>()
                .expect("valid number")
                .clamp(min_year, max_year),
        ),
    }
}

#[inline]
pub fn clamp(year_input: &str, min_year: u32, max_year: u32) -> (u32, String) {
    let range_digit_count = (max_year as f64).log10() as usize + 1;
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
}

#[cfg(test)]
pub mod clamp_test {

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

        min = 919;
        max = 2000;
        [ 
            (909, "222"), 
        ].iter().for_each(|(expected, input)| {
        //    println!("({expected}, {input}) on range({min}, {max}) -> {:?}", clamp(input, min, max));

        }
        )

    }
}
