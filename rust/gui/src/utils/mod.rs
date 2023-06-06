//! TODO: Document Module(utils)

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use num_format::ToFormattedStr;

////////////////////////////////////////////////////////////////////////////////////
// --- mod decls ---
////////////////////////////////////////////////////////////////////////////////////
pub mod block_time;
pub mod constants;
pub mod element_sugar;
pub mod html_tag;
pub mod log_dispose;
pub mod numeric_text;
pub mod updatable;
pub mod year_clamp;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Add thousands separators to number
///
///   * **number** - The number to commify.
///   * _return_ - The commified number.
#[inline]
pub fn commify_number<T>(number: T) -> String
where
    T: ToFormattedStr,
{
    // α <fn commify_number>
    use num_format::{Locale, ToFormattedString};
    number.to_formatted_string(&Locale::en)
    // ω <fn commify_number>
}

// α <mod-def utils>
// ω <mod-def utils>
