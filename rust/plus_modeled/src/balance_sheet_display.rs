//! Display implementations

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::BalanceSheet;
use core::fmt::Display;
use core::fmt::Formatter;

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl Display for BalanceSheet {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // α <fn Display::fmt for BalanceSheet>
        write!(
            f,
            "BalanceSheet({} Accounts)",
            self.accounts.len()
        )
        // ω <fn Display::fmt for BalanceSheet>
    }
}

// α <mod-def balance_sheet_display>
// ω <mod-def balance_sheet_display>
