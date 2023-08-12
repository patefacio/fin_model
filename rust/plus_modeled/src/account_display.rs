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
        write!(
            f,
            "Bond(Mat:{})",
            self.maturity
                .as_ref()
                .map(|maturity| maturity.to_string())
                .unwrap_or_default()
        )
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
        write!(f, "Symbol({})", self.instrument_name)
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
        use crate::AccountType;
        write!(
            f,
            "Acc({}:{})",
            self.name,
            AccountType::from_i32(self.account_type)
                .unwrap()
                .as_str_name()
        )
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

        write!(f, "Penalty({})", self.penalty)
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

        use crate::account::account_treatment::WithdrawalTreatment;

        write!(
            f,
            "AccountType({})/Withdrawal({})",
            self.account_type().as_str_name(),
            self.withdrawal_treatment.as_ref().map(|wt| format!(
                "{}",
                match wt {
                    WithdrawalTreatment::EarlyWithdrawalPenalty(p) => format!("Early Withdrawal({p})"),
                    WithdrawalTreatment::CollegeIrs529Penalty(p) => format!("College Fund Penalty({p})"),
                    WithdrawalTreatment::HealthSavingsPenalty(p) => format!("Health Saving Penalty({p})"),
                    WithdrawalTreatment::RequiredMinimumDistribution(p) => format!("RMD Penalty({p})"),
                }
            )).unwrap_or_default()
        )

        // ω <fn Display::fmt for AccountTreatment>
    }
}

// α <mod-def account_display>
// ω <mod-def account_display>
