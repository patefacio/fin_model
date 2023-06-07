//#[inline(never)]
pub fn get_digit_power(num_digits_in_range: usize, i: usize) -> u32 {
    u32::pow(10, (num_digits_in_range - 1 - i) as u32)
}
