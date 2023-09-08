//! Utilities for dealing with money in components.

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// Enumerates constraints on a number
pub enum FormatConstraint {
    /// No sign, no decimal
    PositiveInteger,
    /// No sign, decimal optional
    PositiveDecimal,
    /// Optional signed integer
    Integer,
    /// No constraint
    None,
}

/// The result of [format_number_lenient].
/// In the general success case includes the value as f64, the value as string
/// with formatting and the new position in the formatted value. In cases where
/// there is no or invalid data the result is None. In the special case of
/// all, zero result from any input (eg `0`, `0.000`, `,000,000`) the result
/// is the formatted string value, as complete as possible and the new position.
#[derive(Debug, Clone, PartialEq)]
pub enum LenientFormatted {
    /// The complete value, formatted string, and new position
    NonZeroValue(f64, String, u32),
    /// Indicates 0 or partially complete fractional value being entered.
    Zero(String, u32),
    /// Indicates the number was incomplete/invalid (eg no valid numeric data in the input)
    Incomplete(String, u32),
}

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Helper class for [format_number_lenient] function that tracks the state
/// as characters are iterated in a single pass
#[derive(Debug)]
struct CharState {
    /// Index of current character
    pub i: usize,
    /// Current character being processed
    pub current: char,
    /// Position of the caret for the number field
    pub initial_caret: u32,
    /// Number of digits encountered after decimal point
    pub digits_after_decimal: usize,
    /// The formatted string, built up character by character
    pub current_text: String,
    /// True if `current` character is numeric
    pub is_current_numeric: bool,
    /// Number of digits to encountered
    pub numeric_char_count: u32,
    /// True if current char, based on position, precedes caret.
    /// Used to track number of digits seen before the caret. The general strategy is to
    /// count the digits before the caret - removing any non-numeric text - and then use the
    /// `numeric_to_caret` to find the appropriate new position after.
    pub precedes_caret: bool,
    /// True if ascii digit has been seen.
    /// Once an ascii digit has been seen there should be no negative signs after.
    /// Once an ascii digit has been seen and a decimal has been seen any decimals
    /// after are invalid.
    pub ascii_digit_seen: bool,
    /// Number of digits encountered up to the initial caret
    pub numeric_to_caret: u32,
    /// The position of the decimal point
    pub decimal_position: Option<usize>,
    /// True if negative sign encountered.
    /// Used to prevent multiple negative signs from being added.
    pub negative_seen: bool,
    /// True if non-zero digit encountered.
    /// Special processing is required for the set of strings that map to the value 0:
    /// `0`, `-0`, `0.0` `-0.0`, ... The user needs to be able to enter those values
    /// without the entry field forcing them back to the value to that point (i.e. `0.0`)
    pub non_zero_seen: bool,
}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Given an input string _s_ which may have negative sign '-', decimal,
/// commas ',' and possibly a prefix or suffix - goes over all characters
/// until it reaches _current_caret_ of valid number characters have been encountered
/// returning that position.
/// **Note** Resulting position may be beyond the length of the string, so this
/// is fine for `set_selection_range` but not for direct access.
///
///   * **s** - The input string
///   * **numeric_count** - Number of numeric characters to count.
///   * _return_ - Position **after** _numeric_count_ characters encountered.
pub fn digit_position(s: &str, mut numeric_count: u32) -> u32 {
    // α <fn digit_position>

    if numeric_count == 0 {
        return 0;
    }

    debug_assert!(
        s.chars().count() >= numeric_count as usize,
        "digit_position input `{s}` has length smaller than numeric_count {numeric_count}"
    );

    // debug_assert!(
    //     s.chars()
    //         .all(|c| c.is_ascii_digit() || c == '.' || c == '-'),
    //     "Not all characters numeric! `{s}`"
    // );

    let mut pos = 0;
    for (i, c) in s.chars().enumerate() {
        if numeric_count == 0 {
            break;
        }

        if match c {
            '-' | '.' => true,
            c if c.is_ascii_digit() => true,
            _ => false,
        } {
            numeric_count -= 1;
        }

        pos = i;
    }

    pos as u32 + 1
    // ω <fn digit_position>
}

/// Given an input number _n_ as str formats the number (i.e. adds commas for large numbers).
/// All non-numeric characters in _n_ are ignored except the following characters which
/// have special meaning if they are the last character in _n_:
///
/// - 'k' Treat number entered as thousands
/// - 'm' Treat number entered as millions
/// - 'b' Treat number entered as billions
///
/// Tracks the number of numeric characters (ascii digits, '-', and '.' appropriately positioned)
/// appearing before _current_caret_. This can be used to set the new caret position.
///
///   * **n** - The input number
///   * **current_caret** - Current position of the caret.
///   * _return_ - The formatted number without any non-numeric characters and the new caret position as tuple
pub fn format_number_lenient(n: &str, current_caret: u32) -> LenientFormatted {
    // α <fn format_number_lenient>

    debug_assert!(
        n.chars().count() >= current_caret as usize,
        "format_number_lenient input `{n}` has length smaller than caret position {current_caret}"
    );

    let mut char_state = CharState::new(n, current_caret);

    for (i, c) in n.chars().enumerate() {
        char_state.next_char(i, c);
    }

    char_state.process_digit_shift();

    if let Some(result) = char_state.check_for_zero() {
        return result;
    }

    char_state.finalize_number()

    // ω <fn format_number_lenient>
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl CharState {
    /// Initialize the state
    ///
    ///   * **initial_text** - Text to format
    ///   * **initial_caret** - Initial position of the caret
    ///   * _return_ - Initialized instance
    pub fn new(initial_text: &str, initial_caret: u32) -> CharState {
        // α <fn CharState::new>
        let initial_len = initial_text.len();
        CharState {
            i: 0,
            current: '.',
            initial_caret,
            digits_after_decimal: 0,
            current_text: String::with_capacity(initial_len),
            is_current_numeric: false,
            precedes_caret: false,
            ascii_digit_seen: false,
            numeric_char_count: 0,
            numeric_to_caret: 0,
            decimal_position: None,
            negative_seen: false,
            non_zero_seen: false,
        }
        // ω <fn CharState::new>
    }

    /// Process the next char from the input
    ///
    ///   * **i** - Index of char
    ///   * **c** - Current char
    pub fn next_char(&mut self, i: usize, c: char) {
        // α <fn CharState::next_char>
        self.i = i;

        // If k,m,b suffix encountered, stop updating - that must be the last char
        match self.current {
            'k' | 'b' | 'm' => return,
            _ => (),
        }
        self.current = c;
        self.precedes_caret = (i as u32) < self.initial_caret;
        self.is_current_numeric = true;

        match c {
            c if c.is_ascii_digit() => {
                self.ascii_digit_seen = true;
                self.current_text.push(c);
                if self.decimal_position.is_some() {
                    self.digits_after_decimal += 1;
                }
                if c != '0' {
                    self.non_zero_seen = true;
                }
            }
            '-' => {
                if !self.negative_seen && !self.ascii_digit_seen && self.decimal_position.is_none()
                {
                    self.negative_seen = true;
                    self.current_text.push(c)
                } else {
                    self.is_current_numeric = false;
                }
            }
            '.' => {
                if self.decimal_position.is_none() {
                    self.decimal_position = Some(self.numeric_char_count as usize);
                    // Special code to auto-normalize ".35" -> "0.35"
                    if !self.ascii_digit_seen {
                        if self.precedes_caret {
                            self.numeric_to_caret += 1;
                        }
                        self.current_text.push('0');
                    }
                    self.current_text.push(c)
                }
            }
            _ => self.is_current_numeric = false,
        }

        self.advance_position();
        // ω <fn CharState::next_char>
    }

    /// Advance the position of the character just processed
    #[inline]
    pub fn advance_position(&mut self) {
        // α <fn CharState::advance_position>
        if self.is_current_numeric {
            self.numeric_char_count += 1;
            if self.precedes_caret {
                self.numeric_to_caret += 1;
            }
        }
        // ω <fn CharState::advance_position>
    }

    /// Special logic to convert `k`, `m`, `b` characters a the end of an
    /// input string to thousands, millions, billions, respectively.
    pub fn process_digit_shift(&mut self) {
        // α <fn CharState::process_digit_shift>
        let digit_shift: usize = match self.current {
            'k' => 3,
            'm' => 6,
            'b' => 9,
            _ => 0,
        };

        if digit_shift > 0 {
            if let Some(decimal_pos) = self.decimal_position {
                if self.digits_after_decimal > digit_shift {
                    let start_of_move = decimal_pos + 1;
                    let mut portion_to_move =
                        self.current_text[start_of_move..start_of_move + digit_shift].to_string();
                    portion_to_move.push('.');
                    self.current_text.replace_range(
                        (decimal_pos)..(decimal_pos + digit_shift + 1),
                        &portion_to_move,
                    );
                    self.decimal_position = Some(decimal_pos + digit_shift);
                } else {
                    let removed_decimal = self.current_text.remove(decimal_pos);
                    self.numeric_to_caret -= 1;
                    debug_assert!(removed_decimal == '.');
                    self.decimal_position = None;

                    if self.digits_after_decimal < digit_shift {
                        for _i in 0..(digit_shift - self.digits_after_decimal) {
                            self.numeric_to_caret += 1;
                            self.current_text.push('0');
                        }
                    }
                }
            } else {
                for _i in 0..digit_shift {
                    self.numeric_to_caret += 1;
                    self.current_text.push('0');
                }
            }
        }
        // ω <fn CharState::process_digit_shift>
    }

    /// Special logic to early exit when numeric input is really an in-process edit
    /// of a number that looks like zero.
    ///
    ///   * _return_ - The results wrapped in Some if value is zero, else None.
    pub fn check_for_zero(&mut self) -> Option<LenientFormatted> {
        // α <fn CharState::check_for_zero>
        if !self.non_zero_seen {
            let current = &mut self.current_text;
            if current.starts_with("0.") || current.starts_with("-0.") {
                let pos = current.len() as u32;
                return Some(LenientFormatted::Zero(std::mem::take(current), pos));
            }

            let neg_pos = current.find('-');
            let zero_text = if let Some(neg_pos) = neg_pos {
                &current[neg_pos + 1..]
            } else {
                current.as_str()
            };

            return if "" == zero_text.trim_matches('0') {
                let num_zero = zero_text.len();
                let quotient = num_zero / 3;
                let remainder = num_zero % 3;
                let mut result = "0".repeat(remainder);
                result.push_str(&",000".repeat(quotient));
                let trimmed = result.trim_start_matches(',');
                if trimmed != &result[..] {
                    result = trimmed.to_string()
                }
                if neg_pos.is_some() {
                    result.insert(0, '-');
                    Some(LenientFormatted::Zero(result, self.numeric_to_caret))
                } else {
                    Some(LenientFormatted::Zero(result, 0))
                }
            } else {
                None
            };
        } else {
            None
        }
        // ω <fn CharState::check_for_zero>
    }

    /// All characters have been processed - this parses the number into an f64,
    /// commifies the number if required
    ///
    ///
    ///   * _return_ - The formatted number and caret position in it.
    pub fn finalize_number(self) -> LenientFormatted {
        // α <fn CharState::finalize_number>
        use crate::commify_int;

        let (parsed_value, text) = if let Ok(parsed_number) = self.current_text.parse::<f64>() {
            if let Some(decimal_position) = self.decimal_position {
                let integer_part = parsed_number as i64;
                if integer_part <= -1000 || integer_part >= 1000 {
                    let mut final_number = commify_int(integer_part);
                    final_number.push_str(&self.current_text[decimal_position..]);
                    (parsed_number, final_number)
                } else {
                    (parsed_number, self.current_text)
                }
            } else {
                (parsed_number, commify_int(parsed_number as i64))
            }
        } else {
            return LenientFormatted::Incomplete(self.current_text, self.numeric_to_caret);
        };

        LenientFormatted::NonZeroValue(parsed_value, text, self.numeric_to_caret)

        // ω <fn CharState::finalize_number>
    }
}

/// Unit tests for `numeric_text`
#[cfg(test)]
pub mod unit_tests {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn test_digit_position() {
        // α <fn test_digit_position>
        let should_panic = || digit_position("", 1);
        assert!(std::panic::catch_unwind(should_panic).is_err());

        for ele in [
            ("foo234,343.00", 3, 6),
            ("1", 1, 1),
            ("1", 0, 0),
            ("123", 0, 0),
            ("EUR 1,275,400", 5, 11),
            ("000", 0, 0),
            ("234", 0, 0),
            //0123456     This is the position values
            //...123      These are the numeric characters
        ] {
            let (s, numeric_count, expected) = ele;
            assert_eq!(expected, digit_position(s, numeric_count));
        }
        // ω <fn test_digit_position>
    }

    #[test]
    fn test_format_number_lenient() {
        // α <fn test_format_number_lenient>
        for ele in [
            (
                ".3",
                1,
                LenientFormatted::NonZeroValue(0.3, String::from("0.3"), 2),
            ),
            (
                "$1.3456",
                2,
                LenientFormatted::NonZeroValue(1.3456, String::from("1.3456"), 1),
            ),
            (
                "$$$1.3456",
                0,
                LenientFormatted::NonZeroValue(1.3456, String::from("1.3456"), 0),
            ),
            (
                "$$$1.3456",
                8,
                LenientFormatted::NonZeroValue(1.3456, String::from("1.3456"), 5),
            ),
            (
                "$1.2k",
                4,
                LenientFormatted::NonZeroValue(1200.0, String::from("1,200"), 4),
            ),
            (
                "$1.2m",
                4,
                LenientFormatted::NonZeroValue(1200000.0, String::from("1,200,000"), 7),
            ),
            (
                "$1.2b",
                4,
                LenientFormatted::NonZeroValue(1200000000.0, String::from("1,200,000,000"), 10),
            ),
            (
                ".23%",
                1,
                LenientFormatted::NonZeroValue(0.23, String::from("0.23"), 2),
            ),
            (
                "$1.3456k",
                7,
                LenientFormatted::NonZeroValue(1345.6, String::from("1,345.6"), 6),
            ),
            (
                "€3.5k/yr",
                4,
                LenientFormatted::NonZeroValue(3500.0, String::from("3,500"), 4),
            ),
            ("-", 1, LenientFormatted::Zero(String::from("-"), 1)),
            ("-0", 2, LenientFormatted::Zero(String::from("-0"), 2)),
            ("-0.", 3, LenientFormatted::Zero(String::from("-0."), 3)),
            ("$,000", 0, LenientFormatted::Zero(String::from("000"), 0)),
            (
                "$,000,000",
                0,
                LenientFormatted::Zero(String::from("000,000"), 0),
            ),
            (
                "$,000,000,000",
                0,
                LenientFormatted::Zero(String::from("000,000,000"), 0),
            ),
            (",000", 0, LenientFormatted::Zero(String::from("000"), 0)),
            (
                ",000,000",
                0,
                LenientFormatted::Zero(String::from("000,000"), 0),
            ),
            (
                ",000,000,000",
                0,
                LenientFormatted::Zero(String::from("000,000,000"), 0),
            ),
            (
                ",122,345",
                1,
                LenientFormatted::NonZeroValue(122345.0, String::from("122,345"), 0),
            ),
            ("$ --", 3, LenientFormatted::Zero(String::from("-"), 1)),
            ("--", 2, LenientFormatted::Zero(String::from("-"), 1)),
        ] {
            let (n, current_caret, expected) = ele;
            tracing::info!("format_number_lenient on `{n}` with caret -> {current_caret}");
            assert_eq!(expected, format_number_lenient(n, current_caret));
        }
        // ω <fn test_format_number_lenient>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def numeric_text>

// ω <mod-def numeric_text>
