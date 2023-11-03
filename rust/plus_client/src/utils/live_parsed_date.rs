//! Support for parsing dates as characters are entered (ie live parsing)

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::IntegerClamp;
use plus_modeled::Date;
use plus_modeled::YearRange;

////////////////////////////////////////////////////////////////////////////////////
// --- constants ---
////////////////////////////////////////////////////////////////////////////////////
/// Index of M1 char
const M1: usize = 0;
/// Index of M2 char
const M2: usize = 1;
/// Index of D1 char
const D1: usize = 3;
/// Index of D2 char
const D2: usize = 4;
/// Index of Y1 char
const Y1: usize = 6;
/// Index of Y2 char
const Y2: usize = 7;
/// Index of Y3 char
const Y3: usize = 8;
/// Index of Y4 char
const Y4: usize = 9;

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// The state while parsing a date
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub enum ParsedState {
    /// Expecting first month digit
    M1,
    /// Expecting second month digit
    M2,
    /// Expecting first day digit
    D1,
    /// Expecting second day digit
    D2,
    /// Expecting first year digit
    Y1,
    /// Expecting second year digit
    Y2,
    /// Expecting third year digit
    Y3,
    /// Expecting fourth year digit
    Y4,
    /// Finished parsing to end of a date
    Done,
}

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a date in the process of being written in the format `MM/DD/YYYY`.
#[derive(Debug)]
pub struct LiveParsedDate {
    /// The date as string formatted
    pub formatted: String,
    /// The position of the caret, including last entered character, before normalization
    pub position: u32,
    /// Indicates likely delete requiring special position logic
    pub probable_delete: bool,
    /// The state while parsing a date input
    pub parsed_state: ParsedState,
    /// Optional constraint on year of date
    pub year_clamp: Option<IntegerClamp>,
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl LiveParsedDate {
    /// Create a new instance.
    ///
    ///   * **year_range** - Constraint on year of date
    ///   * _return_ - The initialized instance
    pub fn new(year_range: Option<YearRange>) -> Self {
        // α <fn LiveParsedDate::new>

        LiveParsedDate {
            formatted: "__/__/____".to_string(),
            position: 0,
            probable_delete: false,
            parsed_state: ParsedState::M1,
            year_clamp: year_range
                .map(|year_range| IntegerClamp::new(year_range.start..=year_range.end)),
        }

        // ω <fn LiveParsedDate::new>
    }

    /// Create a new instance from the `date`.
    ///
    ///   * **date** - Initial date
    ///   * **year_range** - Constraint on year of date
    ///   * _return_ - The initialized instance
    pub fn from_date(date: &Date, year_range: Option<YearRange>) -> Self {
        // α <fn LiveParsedDate::from_date>

        LiveParsedDate {
            formatted: format!(
                "{:0>2}/{:0>2}/{:_<4}",
                date.month.min(12),
                date.day.min(31),
                date.year.min(3000)
            ),
            position: 0,
            probable_delete: false,
            parsed_state: ParsedState::M1,
            year_clamp: year_range
                .map(|year_range| IntegerClamp::new(year_range.start..=year_range.end)),
        }
        // ω <fn LiveParsedDate::from_date>
    }

    /// Parses the `input_date`
    ///
    ///   * **input_date** - The date input to be formatted
    ///   * **position** - The position of the caret
    ///   * _return_ - The potentially partial date formatted like `MM/DD/YYYY`, the Date object if valid,
    /// and the caret position.
    pub fn parse_date(&mut self, input_date: &str, position: u32) -> (String, Option<Date>, u32) {
        // α <fn LiveParsedDate::parse_date>
        debug_assert!(position <= input_date.chars().count() as u32);

        self.reset();

        if position <= self.position {
            self.probable_delete = true;
        }
        self.position = position;

        input_date.chars().for_each(|c| self.put_char(c));

        if self.position == 2 {
            // If the position is at the first '/' then move beyond it
            // unless they were going backwards - in which case allow it
            if !self.probable_delete {
                self.position = 3;
            }
        } else if self.position == 5 {
            // Similarly, if the position is at the second '/' then move beyond it
            // unless they were going backwards - in which case allow it
            if !self.probable_delete {
                self.position = 6;
            }
        }

        self.zero_fill_month();
        self.zero_fill_day();

        // Finally, the position is mostly good, but a couple more checks.
        if !self.probable_delete {
            if let Some(i) = self.formatted.find('_') {
                self.position = i as u32;
            }
        }

        if let Some(year_clamp) = self.year_clamp.as_ref() {
            let year_part = self
                .formatted
                .get(6..)
                .expect("Year is there")
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>();

            if !year_part.is_empty() {
                let parsed_num = year_clamp.clamp(&year_part);
                _ = self
                    .formatted
                    .replace_range(6..(6 + year_part.len()), &parsed_num.as_string);
            }
        }

        (self.formatted.clone(), self.to_date(), self.position)
        // ω <fn LiveParsedDate::parse_date>
    }

    /// Clear the date string
    #[inline]
    fn reset(&mut self) {
        // α <fn LiveParsedDate::reset>
        self.formatted.replace_range(0..10, "__/__/____");
        self.parsed_state = ParsedState::M1;
        self.probable_delete = false;
        // ω <fn LiveParsedDate::reset>
    }

    /// Put the input character int its proper place and update the state
    ///
    ///   * **c** - The next character
    fn put_char(&mut self, c: char) {
        // α <fn LiveParsedDate::put_char>

        self.parsed_state = match c {
            c if c.is_ascii_digit() => match self.parsed_state {
                ParsedState::M1 => {
                    self.set_char(c, M1);
                    ParsedState::M2
                }
                ParsedState::M2 => {
                    self.set_char(c, M2);
                    unsafe {
                        let bytes = self.formatted.as_bytes_mut();
                        bytes[0] = bytes[0].min('1' as u8);
                        if bytes[0] == 1 {
                            bytes[1] = bytes[1].min('2' as u8);
                        }
                    }
                    ParsedState::D1
                }
                ParsedState::D1 => {
                    self.set_char(c, D1);
                    ParsedState::D2
                }
                ParsedState::D2 => {
                    self.set_char(c, D2);
                    unsafe {
                        let bytes = self.formatted.as_bytes_mut();
                        bytes[D1] = bytes[D1].min('3' as u8);
                        if bytes[D1] == '3' as u8 {
                            bytes[D2] = bytes[D2].min('1' as u8);
                        }
                    }
                    ParsedState::Y1
                }
                ParsedState::Y1 => {
                    self.set_char(c, Y1);
                    ParsedState::Y2
                }
                ParsedState::Y2 => {
                    self.set_char(c, Y2);
                    ParsedState::Y3
                }
                ParsedState::Y3 => {
                    self.set_char(c, Y3);
                    ParsedState::Y4
                }
                ParsedState::Y4 => {
                    self.set_char(c, Y4);
                    ParsedState::Done
                }
                ParsedState::Done => ParsedState::Done,
            },
            // Ignore all other chars
            _ => self.parsed_state,
        }

        // ω <fn LiveParsedDate::put_char>
    }

    /// Set the char in formatted
    ///
    ///   * **c** - Updated character
    ///   * **offset** - Offset to set
    #[inline]
    fn set_char(&mut self, c: char, offset: usize) {
        // α <fn LiveParsedDate::set_char>

        debug_assert!(offset < 11);

        unsafe {
            self.formatted.as_bytes_mut()[offset] = c as u8;
        }

        // ω <fn LiveParsedDate::set_char>
    }

    /// Given single digit month `d_/`, change to `0d/`
    #[inline]
    fn zero_fill_month(&mut self) {
        // α <fn LiveParsedDate::zero_fill_month>

        if self.position > 2 {
            unsafe {
                let bytes = self.formatted.as_bytes_mut();
                let m1 = bytes[0];
                let m2 = bytes[1];
                if m2 == '_' as u8 {
                    bytes[0] = '0' as u8;
                    bytes[1] = m1;
                }
            }
        }

        // ω <fn LiveParsedDate::zero_fill_month>
    }

    /// Given single digit day `d_/`, change to `0d/`
    #[inline]
    fn zero_fill_day(&mut self) {
        // α <fn LiveParsedDate::zero_fill_day>

        if self.position > 5 {
            unsafe {
                let bytes = self.formatted.as_bytes_mut();
                let d1 = bytes[3];
                let d2 = bytes[4];
                if d2 == '_' as u8 {
                    bytes[3] = '0' as u8;
                    bytes[4] = d1;
                }
            }
        }

        // ω <fn LiveParsedDate::zero_fill_day>
    }

    /// Convert the resulting date from `formatted` into a date if possible
    ///
    ///   * _return_ - The date if complete
    #[inline]
    fn to_date(&self) -> Option<Date> {
        // α <fn LiveParsedDate::to_date>

        if let Ok(month) = self.formatted[0..2].parse::<u32>() {
            if let Ok(day) = self.formatted[3..5].parse::<u32>() {
                if let Ok(year) = self.formatted[6..].parse::<u32>() {
                    if let Some(_) = chrono::NaiveDate::from_ymd_opt(year as i32, month, day) {
                        return Some(Date { month, day, year });
                    }
                }
            }
        }

        None

        // ω <fn LiveParsedDate::to_date>
    }
}

/// Unit tests for `live_parsed_date`
#[cfg(test)]
pub mod unit_tests {

    /// Test type LiveParsedDate
    mod test_live_parsed_date {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn parse_date() {
            // α <fn test LiveParsedDate::parse_date>
            let mut lpd = LiveParsedDate::new(None);

            // If users enters `1/`
            assert_eq!(
                ("01/__/____".into(), None, 3),
                lpd.parse_date("1/_/__/____", 2)
            );

            // Reset instance so backspace logic not triggered
            lpd = LiveParsedDate::new(None);
            // Use of space instead of '/' to _jump_ to next field
            assert_eq!(
                ("01/__/____".into(), None, 3),
                lpd.parse_date("1 _/__/____", 2)
            );

            assert_eq!(
                ("04/__/____".into(), None, 3),
                lpd.parse_date("04_/__/____", 3)
            );
            assert_eq!(("12/__/____".into(), None, 3), lpd.parse_date("12/", 3));

            // Only possible via copy/paste since missing `/`
            assert_eq!(("12/1_/____".into(), None, 3), lpd.parse_date("121", 3));

            assert_eq!(
                (
                    "12/12/2023".into(),
                    Some(Date {
                        year: 2023,
                        month: 12,
                        day: 12
                    }),
                    3
                ),
                lpd.parse_date("12122023", 3)
            );

            assert_eq!(
                ("12/01/____".into(), None, 6),
                lpd.parse_date("12/1 _/____", 5)
            );
            assert_eq!(
                ("12/01/1___".into(), None, 7),
                lpd.parse_date("12/01/1____", 7)
            );
            assert_eq!(
                ("12/01/19__".into(), None, 8),
                lpd.parse_date("12/01/19___", 8)
            );
            assert_eq!(
                ("12/01/199_".into(), None, 9),
                lpd.parse_date("12/01/199__", 9)
            );
            assert_eq!(
                (
                    "12/01/1999".into(),
                    Some(Date {
                        year: 1999,
                        month: 12,
                        day: 1
                    }),
                    10
                ),
                lpd.parse_date("12/01/1999_", 10)
            );

            // ω <fn test LiveParsedDate::parse_date>
        }

        // α <mod-def test_live_parsed_date>
        use super::*;
        // ω <mod-def test_live_parsed_date>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def live_parsed_date>

// ω <mod-def live_parsed_date>
