//! Top module

////////////////////////////////////////////////////////////////////////////////////
// --- pub module uses ---
////////////////////////////////////////////////////////////////////////////////////
pub use self::date_utils::next_year;
pub use self::date_utils::this_year;
pub use self::scale_by::scale_by;
pub use self::svg::SvgArea;
pub use self::svg::SvgDim;
pub use self::svg::SvgPoint;
pub use self::system_unicodes::SystemUnicodes;
pub use self::text_utils::commify_float;
pub use self::text_utils::commify_int;
pub use self::text_utils::with_max_precision;

////////////////////////////////////////////////////////////////////////////////////
// --- mod decls ---
////////////////////////////////////////////////////////////////////////////////////
pub mod date_utils;
pub mod plus_constants;
pub mod scale_by;
pub mod svg;
pub mod system_unicodes;
pub mod text_utils;

// α <mod-def lib>
// ω <mod-def lib>
