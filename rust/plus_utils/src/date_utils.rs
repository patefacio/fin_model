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

/// Return next year
///
///   * _return_ - Next year
#[inline]
pub fn next_year() -> u32 {
    // α <fn next_year>
    this_year() + 1
    // ω <fn next_year>
}

// α <mod-def date_utils>
// ω <mod-def date_utils>
