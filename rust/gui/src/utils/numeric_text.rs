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
pub fn format_number_lenient(n: &str, current_caret: u32) -> (Option<f64>, String, u32) {
    // α <fn format_number_lenient>
    debug_assert!(n.chars().count() >= current_caret as usize);

    use crate::utils::commify_number;
    let mut numeric = String::with_capacity(n.len());
    let mut ascii_digit_seen = false;
    let mut decimal_position = None;
    let mut negative_seen = false;
    let mut numeric_to_caret = 0u32;
    let mut last_char = '?';
    let mut digits_after_decimal = 0usize;
    let mut numeric_char_count = 0u32;
    let mut non_zero_seen = false;

    for (i, c) in n.chars().enumerate() {
        let precedes_caret = (i as u32) < current_caret;
        // Assume current char is numeric - if not match catch-all will set false
        let mut is_current_numeric = true;

        match c {
            c if c.is_ascii_digit() => {
                ascii_digit_seen = true;
                numeric.push(c);
                if decimal_position.is_some() {
                    digits_after_decimal += 1;
                }
                if c != '0' {
                    non_zero_seen = true;
                }
            }
            '-' => {
                if !negative_seen && !ascii_digit_seen && decimal_position.is_none() {
                    negative_seen = true;
                    numeric.push(c)
                }
            }
            '.' => {
                if decimal_position.is_none() {
                    decimal_position = Some(numeric_char_count as usize);
                    // Special code to auto-normalize ".35" -> "0.35"
                    if !ascii_digit_seen {
                        if precedes_caret {
                            numeric_to_caret += 1;
                        }
                        numeric.push('0');
                    }
                    numeric.push(c)
                }
            }
            _ => is_current_numeric = false,
        }

        if is_current_numeric {
            numeric_char_count += 1;
            if precedes_caret {
                numeric_to_caret += 1;
            }
        }

        if ascii_digit_seen {
            last_char = c;
            match c {
                'k' | 'm' | 'b' => break,
                _ => (),
            }
        }
    }

    let digit_shift: usize = match last_char {
        'k' => 3,
        'm' => 6,
        'b' => 9,
        _ => 0,
    };

    if digit_shift > 0 {
        if let Some(decimal_pos) = decimal_position {
            if digits_after_decimal > digit_shift {
                let start_of_move = decimal_pos + 1;
                let mut portion_to_move =
                    numeric[start_of_move..start_of_move + digit_shift].to_string();
                portion_to_move.push('.');
                numeric.replace_range(
                    (decimal_pos)..(decimal_pos + digit_shift + 1),
                    &portion_to_move,
                );
                decimal_position = Some(decimal_pos + digit_shift);
            } else if digits_after_decimal == digit_shift {
                numeric.remove(decimal_pos);
                decimal_position = None;
            } else {
                let _x = numeric.remove(decimal_pos);
                debug_assert!(_x == '.');
                for _i in 0..(digit_shift - digits_after_decimal) {
                    numeric.push('0');
                }
                decimal_position = None;
            }
        } else {
            for _i in 0..digit_shift {
                numeric.push('0');
            }
        }
    }

    // Leave early with any _effective_ 0, but keep the digits
    if !non_zero_seen {
        match numeric.as_str() {
            "0.0" => {return (None, "0.0".into(), 3);}
            "0.00" => {return (None, "0.00".into(), 4);}
            "0.000" => {return (None, "0.000".into(), 5);}
            "0.0000" => {return (None, "0.0000".into(), 6);}
            "0.00000" => {return (None, "0.00000".into(), 7);}
            _ => ()
        }
    }

    match numeric.as_str() {
        "-" => {
            return (None, "-".into(), 1);
        }
        "-0" => {
            return (None, "-0".into(), 2);
        }
        "-0." => {
            return (None, "-0.".into(), 3);
        }
        _ => (),
    }

    let (parsed_value, text) = if let Ok(parsed_number) = numeric.parse::<f64>() {
        if let Some(decimal_position) = decimal_position {
            let integer_part = parsed_number as i64;
            if integer_part <= -1000 || integer_part >= 1000 {
                let mut final_number = commify_number(integer_part);
                final_number.push_str(&numeric[decimal_position..]);
                (Some(parsed_number), final_number)
            } else {
                (Some(parsed_number), numeric)
            }
        } else {
            (Some(parsed_number), commify_number(parsed_number as i64))
        }
    } else {
        (None, numeric)
    };

    if digit_shift > 0 {
        let last_pos = text
            .chars()
            .filter(|&c| c.is_ascii_digit() || c == '.' || c == '-')
            .count() as u32;
        (parsed_value, text, last_pos)
    } else {
        (parsed_value, text, numeric_to_caret)
    }

    // ω <fn format_number_lenient>
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
        for ele in [
            //
            ("", 1, 1),
            ("foo234,343.00", 3, 6),
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
            (".3", 1, (Some(0.3), String::from("0.3"), 2)),
            ("$1.3456", 2, (Some(1.3456), String::from("1.3456"), 1)),
            ("$$$1.3456", 0, (Some(1.3456), String::from("1.3456"), 0)),
            ("$$$1.3456", 8, (Some(1.3456), String::from("1.3456"), 5)),
            ("$1.2k", 4, (Some(1200.0), String::from("1,200"), 4)),
            ("$1.2m", 4, (Some(1200000.0), String::from("1,200,000"), 7)),
            (
                "$1.2b",
                4,
                (Some(1200000000.0), String::from("1,200,000,000"), 10),
            ),
            (".23%", 1, (Some(0.23), String::from("0.23"), 2)),
            ("$1.3456k", 7, (Some(1345.6), String::from("1,345.6"), 6)),
            ("€3.5k/yr", 4, (Some(3500.0), String::from("3,500"), 4)),
            ("-", 1, (None, String::from("-"), 1)),
            ("-0", 2, (None, String::from("-0"), 2)),
            ("-0.", 3, (None, String::from("-0."), 3)),
        ] {
            let (n, current_caret, expected) = ele;
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
