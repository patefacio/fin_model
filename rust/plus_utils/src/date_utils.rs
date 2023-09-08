//! Functions dealing with dates

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Return current year
///
///   * _return_ - Current year
#[inline]
pub fn this_year() -> u32 {
    // α <fn this_year>

    use chrono::Datelike;

    chrono::Utc::now().year() as u32

    // ω <fn this_year>
}

// α <mod-def date_utils>
// ω <mod-def date_utils>
