////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Scale the number by factor (10^scale_factor) keeping precision.
/// Consider 0.033 to be scaled up by 100 to show in an percent entry.
/// Because these are double precision and there are only discrete number
/// of values in the set of floats, this multiplication (0.333 * 100.0)
/// gives 33.300000000000004. This function adjusts for that and returns
/// 33.3 which is a valid f64. The max supported by this function is a
/// shift of 10 decimal places.
///
///   * **n** - Number to scale up
///   * **scale_factor** - Factor to scale the number
///   * _return_ - The number scaled by factor
#[inline]
pub fn scale_by(n: f64, scale_factor: i32) -> f64 {
    // α <fn scale_by>

    // TODO: Don't do it this way. Either use some math trickery or
    // convert input to string up front, iterate through adjusting the
    // number

    let mut s = n.to_string();
    let temp = s.find('.');
    let point: usize;
    match temp {
        Some(_) => point = temp.unwrap(),
        None => {
            s.push_str(".0");
            point = s.len() - 2;
        }
    };

    let mut lhs = (&s[0..point]).to_string();
    let mut rhs = (&s[point + 1..s.len()]).to_string();

    if scale_factor > 0 {
        for _i in 0..scale_factor {
            if rhs.len() != 0 {
                lhs.push(rhs.remove(0));
            } else {
                lhs.push('0');
            }
        }
    } else if scale_factor < 0 {
        for _i in scale_factor..0 {
            if lhs.len() != 0 {
                rhs.insert(0, lhs.pop().unwrap());
            } else {
                rhs.insert(0, '0');
            }
        }
    }

    let s = lhs + "." + &rhs;
    return s.parse::<f64>().unwrap();

    // ω <fn scale_by>
}

/// Unit tests for `scale_by`
#[cfg(test)]
pub mod unit_tests {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn test_scale_by() {
        // α <fn test_scale_by>

        assert_eq!(30.0, scale_by(3.0, 1));
        assert_eq!(3.33, scale_by(0.0333, 2));
        assert_eq!(33333.3333, scale_by(3.33333333, 4));
        assert_eq!(55.5, scale_by(555.0, -1));
        assert_eq!(55.5, scale_by(55.5, 0));

        // ω <fn test_scale_by>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def scale_by>
// ω <mod-def scale_by>
