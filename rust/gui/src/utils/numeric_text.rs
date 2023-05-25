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
  leptos_dom::console_log(&format!("digit_position(`{s}`, {numeric_count})"));

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

  leptos_dom::console_log(&format!("digit_position(...)->{}", pos + 1));

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
  todo!("Implement `format_number_lenient`")
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
    todo!("Add test digit_position")
    // ω <fn test_digit_position>
    }

    #[test]
    fn test_format_number_lenient() {
        // α <fn test_format_number_lenient>
    todo!("Add test format_number_lenient")
    // ω <fn test_format_number_lenient>
    }

    // α <mod-def unit_tests>
  // ω <mod-def unit_tests>
}

// α <mod-def numeric_text>
// ω <mod-def numeric_text>
