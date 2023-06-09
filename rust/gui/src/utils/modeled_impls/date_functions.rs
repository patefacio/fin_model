//! Implementation of trait `DateFunctions` modeled type `Date`

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use plus_modeled::Date;

////////////////////////////////////////////////////////////////////////////////////
// --- traits ---
////////////////////////////////////////////////////////////////////////////////////
/// Functions applied to [Date](plus_modeled::Date)
pub trait DateFunctions {}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl DateFunctions for Date {}

/// Unit tests for `date_functions`
#[cfg(test)]
pub mod unit_tests {
    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def date_functions>
// ω <mod-def date_functions>
