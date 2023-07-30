//! Display implementations

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Account;
use crate::AccountTreatment;
use crate::BondSpec;
use crate::Holding;
use crate::RequiredMinimumDistribution;
use core::fmt::Display;
use core::fmt::Formatter;

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl Display for BondSpec {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for BondSpec>
        todo!("Implement `fmt`")
        // ω <fn Display::fmt for BondSpec>
    }
}

impl Display for Holding {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for Holding>
        todo!("Implement `fmt`")
        // ω <fn Display::fmt for Holding>
    }
}

impl Display for Account {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for Account>
        todo!("Implement `fmt`")
        // ω <fn Display::fmt for Account>
    }
}

impl Display for RequiredMinimumDistribution {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for RequiredMinimumDistribution>
        todo!("Implement `fmt`")
        // ω <fn Display::fmt for RequiredMinimumDistribution>
    }
}

impl Display for AccountTreatment {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for AccountTreatment>
        todo!("Implement `fmt`")
        // ω <fn Display::fmt for AccountTreatment>
    }
}

// α <mod-def account_display>
// ω <mod-def account_display>
