//! Display implementations

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::AgeAssumptions;
use core::fmt::Display;
use core::fmt::Formatter;

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl Display for AgeAssumptions {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for AgeAssumptions>
        write!(
            f,
            "Ages(Retirement({}), Death({}))",
            self.retirement_age,
            self.death_age
        )
        // ω <fn Display::fmt for AgeAssumptions>
    }
}

// α <mod-def age_assumptions_display>
// ω <mod-def age_assumptions_display>
