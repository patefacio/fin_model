//! Defines struct to bind a string representation of a number with the number.

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// A number as both string and parsed u32 value
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParsedNum {
    /// The number as u32
    pub as_u32: u32,
    /// The number as string
    pub as_string: String,
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl ParsedNum {
    /// Create from str by parsing - panics with invalid number.
    ///
    ///   * **number** - The number as String
    ///   * _return_ - The parsed number
    #[inline]
    pub fn from_string(number: String) -> ParsedNum {
        // α <fn ParsedNum::from_string>
        ParsedNum {
            as_u32: number.parse::<u32>().expect("valid number"),
            as_string: number,
        }
        // ω <fn ParsedNum::from_string>
    }

    /// Create from str by parsing - panics with invalid number.
    ///
    ///   * **number** - The number as str
    ///   * _return_ - The parsed number
    #[inline]
    pub fn from_str(number: &str) -> ParsedNum {
        // α <fn ParsedNum::from_str>
        ParsedNum::from_string(number.to_string())
        // ω <fn ParsedNum::from_str>
    }

    /// Create new instance of ParsedNum
    ///
    ///   * **as_u32** - The number as u32
    ///   * _return_ - The new instance
    #[inline]
    pub fn new(as_u32: u32) -> ParsedNum {
        // α <fn ParsedNum::new>
        ParsedNum {
            as_u32,
            as_string: as_u32.to_string(),
        }
        // ω <fn ParsedNum::new>
    }
}

/// Unit tests for `parsed_num`
#[cfg(test)]
pub mod unit_tests {

    /// Test type ParsedNum
    mod test_parsed_num {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn from_string() {
            // α <fn test ParsedNum::from_string>
            let num = "432".to_string();
            assert_eq!(
                ParsedNum {
                    as_u32: 432,
                    as_string: num.clone()
                },
                ParsedNum::from_string(num)
            )
            // ω <fn test ParsedNum::from_string>
        }

        #[test]
        fn from_str() {
            // α <fn test ParsedNum::from_str>
            let num = "432";
            assert_eq!(
                ParsedNum {
                    as_u32: 432,
                    as_string: num.to_string()
                },
                ParsedNum::from_str(num)
            );
            // ω <fn test ParsedNum::from_str>
        }

        // α <mod-def test_parsed_num>
        use super::*;
        // ω <mod-def test_parsed_num>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def parsed_num>
// ω <mod-def parsed_num>
