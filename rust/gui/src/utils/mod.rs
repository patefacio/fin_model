//! TODO: Document Module(utils)

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use num_format::ToFormattedStr;

////////////////////////////////////////////////////////////////////////////////////
// --- mod decls ---
////////////////////////////////////////////////////////////////////////////////////
pub mod block_time;
pub mod clamp;
pub mod constants;
pub mod distribution_pdf;
pub mod distribution_cdf;
pub mod element_sugar;
pub mod html_tag;
pub mod integer_clamp;
pub mod live_parsed_date;
pub mod log_dispose;
pub mod modeled_impls;
pub mod numeric_text;
pub mod parsed_num;
pub mod plot_data;
pub mod scale_by;
pub mod updatable;

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
