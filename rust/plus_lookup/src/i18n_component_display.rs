//! Support for internationalization of component text

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::lang_selector_to_language_id;
use crate::{FRENCH, GERMAN, LOCALES, US_ENGLISH};
use core::fmt::Display;
use core::fmt::Formatter;
use fluent_templates::Loader;
use plus_modeled::LangSelector;

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// Strings for `account_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nAccountComponent {
    /// Strings for component Account
    Account(LangSelector),
    /// Strings for component Name
    Name(LangSelector),
    /// Strings for component AccountType
    AccountType(LangSelector),
    /// Strings for component Holdings
    Holdings(LangSelector),
}

/// Strings for `accounts_grid`
#[derive(Debug, Copy, Clone)]
pub enum I18nAccountsGrid {
    /// Strings for component Name
    Name(LangSelector),
    /// Strings for component Accounts
    Accounts(LangSelector),
    /// Strings for component Mv
    Mv(LangSelector),
    /// Strings for component Type
    Type(LangSelector),
    /// Strings for component NewAccount
    NewAccount(LangSelector),
    /// Strings for component GridHelp
    GridHelp(LangSelector),
}

/// Strings for `age_assumptions_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nAgeAssumptionsComponent {
    /// Strings for component RetirementAge
    RetirementAge(LangSelector),
    /// Strings for component DeathAge
    DeathAge(LangSelector),
}

/// Strings for `bond_spec_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nBondSpecComponent {
    /// Strings for component AnnualCoupon
    AnnualCoupon(LangSelector),
    /// Strings for component MaturityYear
    MaturityYear(LangSelector),
}

/// Strings for `distribution_policy_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nDistributionPolicyComponent {
    /// Strings for component NoDistributions
    NoDistributions(LangSelector),
    /// Strings for component Distributions
    Distributions(LangSelector),
    /// Strings for component Bond
    Bond(LangSelector),
}

/// Strings for `distribution_spec_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nDistributionSpecComponent {
    /// Strings for component QualifiedDiv
    QualifiedDiv(LangSelector),
    /// Strings for component UnqualifiedDiv
    UnqualifiedDiv(LangSelector),
    /// Strings for component CapitalGain
    CapitalGain(LangSelector),
    /// Strings for component Interest
    Interest(LangSelector),
}

/// Strings for `dossier_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nDossierComponent {
    /// Strings for component People
    People(LangSelector),
    /// Strings for component Worths
    Worths(LangSelector),
    /// Strings for component Accounts
    Accounts(LangSelector),
    /// Strings for component CashFlows
    CashFlows(LangSelector),
    /// Strings for component Taxes
    Taxes(LangSelector),
    /// Strings for component Assumptions
    Assumptions(LangSelector),
}

/// Strings for `flow_spec_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nFlowSpecComponent {
    /// Strings for component Name
    Name(LangSelector),
    /// Strings for component InFlow
    InFlow(LangSelector),
    /// Strings for component OutFlow
    OutFlow(LangSelector),
    /// Strings for component CashFlow
    CashFlow(LangSelector),
}

/// Strings for `flow_specs_grid`
#[derive(Debug, Copy, Clone)]
pub enum I18nFlowSpecsGrid {
    /// Strings for component Name
    Name(LangSelector),
    /// Strings for component InOut
    InOut(LangSelector),
    /// Strings for component YearRange
    YearRange(LangSelector),
    /// Strings for component NewCashFlow
    NewCashFlow(LangSelector),
}

/// Strings for `holding_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nHoldingComponent {
    /// Strings for component Mv
    Mv(LangSelector),
    /// Strings for component Holding
    Holding(LangSelector),
    /// Strings for component Ugl
    Ugl(LangSelector),
    /// Strings for component Symbol
    Symbol(LangSelector),
    /// Strings for component CurrentPrice
    CurrentPrice(LangSelector),
    /// Strings for component Growth
    Growth(LangSelector),
    /// Strings for component Cost
    Cost(LangSelector),
    /// Strings for component Quantity
    Quantity(LangSelector),
}

/// Strings for `holdings_grid`
#[derive(Debug, Copy, Clone)]
pub enum I18nHoldingsGrid {
    /// Strings for component Symbol
    Symbol(LangSelector),
    /// Strings for component Mv
    Mv(LangSelector),
    /// Strings for component Cb
    Cb(LangSelector),
    /// Strings for component Ugl
    Ugl(LangSelector),
    /// Strings for component Holdings
    Holdings(LangSelector),
    /// Strings for component NewHolding
    NewHolding(LangSelector),
}

/// Strings for `ok_cancel_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nOkCancelComponent {
    /// Strings for component Ok
    Ok(LangSelector),
    /// Strings for component Cancel
    Cancel(LangSelector),
}

/// Strings for `person_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nPersonComponent {
    /// Strings for component Name
    Name(LangSelector),
    /// Strings for component BirthYear
    BirthYear(LangSelector),
    /// Strings for component Role
    Role(LangSelector),
    /// Strings for component RetirementAge
    RetirementAge(LangSelector),
}

/// Strings for `persons_grid`
#[derive(Debug, Copy, Clone)]
pub enum I18nPersonsGrid {
    /// Strings for component People
    People(LangSelector),
    /// Strings for component Name
    Name(LangSelector),
    /// Strings for component Role
    Role(LangSelector),
    /// Strings for component RetirementAge
    RetirementAge(LangSelector),
    /// Strings for component NewPerson
    NewPerson(LangSelector),
    /// Strings for component GridHelp
    GridHelp(LangSelector),
}

/// Strings for `tax_determinants_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nTaxDeterminantsComponent {
    /// Strings for component SimpleEffectiveRate
    SimpleEffectiveRate(LangSelector),
    /// Strings for component ByCountry
    ByCountry(LangSelector),
}

/// Strings for `worth_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nWorthComponent {
    /// Strings for component Worth
    Worth(LangSelector),
    /// Strings for component Name
    Name(LangSelector),
    /// Strings for component CurrentValue
    CurrentValue(LangSelector),
}

/// Strings for `worths_grid`
#[derive(Debug, Copy, Clone)]
pub enum I18nWorthsGrid {
    /// Strings for component Worths
    Worths(LangSelector),
    /// Strings for component Name
    Name(LangSelector),
    /// Strings for component Type
    Type(LangSelector),
    /// Strings for component NewWorth
    NewWorth(LangSelector),
    /// Strings for component Value
    Value(LangSelector),
    /// Strings for component GridHelp
    GridHelp(LangSelector),
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl Display for I18nAccountComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nAccountComponent::Account(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "account_component.account"
                    )
                    .unwrap_or_default(),
                I18nAccountComponent::Name(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "account_component.name"
                    )
                    .unwrap_or_default(),
                I18nAccountComponent::AccountType(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "account_component.account_type"
                    )
                    .unwrap_or_default(),
                I18nAccountComponent::Holdings(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "account_component.holdings"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nAccountsGrid {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nAccountsGrid::Name(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "accounts_grid.name"
                    )
                    .unwrap_or_default(),
                I18nAccountsGrid::Accounts(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "accounts_grid.accounts"
                    )
                    .unwrap_or_default(),
                I18nAccountsGrid::Mv(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "accounts_grid.mv"
                    )
                    .unwrap_or_default(),
                I18nAccountsGrid::Type(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "accounts_grid.type"
                    )
                    .unwrap_or_default(),
                I18nAccountsGrid::NewAccount(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "accounts_grid.new_account"
                    )
                    .unwrap_or_default(),
                I18nAccountsGrid::GridHelp(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "accounts_grid.grid_help"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nAgeAssumptionsComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nAgeAssumptionsComponent::RetirementAge(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "age_assumptions_component.retirement_age"
                    )
                    .unwrap_or_default(),
                I18nAgeAssumptionsComponent::DeathAge(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "age_assumptions_component.death_age"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nBondSpecComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nBondSpecComponent::AnnualCoupon(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "bond_spec_component.annual_coupon"
                    )
                    .unwrap_or_default(),
                I18nBondSpecComponent::MaturityYear(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "bond_spec_component.maturity_year"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nDistributionPolicyComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nDistributionPolicyComponent::NoDistributions(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "distribution_policy_component.no_distributions"
                    )
                    .unwrap_or_default(),
                I18nDistributionPolicyComponent::Distributions(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "distribution_policy_component.distributions"
                    )
                    .unwrap_or_default(),
                I18nDistributionPolicyComponent::Bond(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "distribution_policy_component.bond"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nDistributionSpecComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nDistributionSpecComponent::QualifiedDiv(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "distribution_spec_component.qualified_div"
                    )
                    .unwrap_or_default(),
                I18nDistributionSpecComponent::UnqualifiedDiv(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "distribution_spec_component.unqualified_div"
                    )
                    .unwrap_or_default(),
                I18nDistributionSpecComponent::CapitalGain(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "distribution_spec_component.capital_gain"
                    )
                    .unwrap_or_default(),
                I18nDistributionSpecComponent::Interest(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "distribution_spec_component.interest"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nDossierComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nDossierComponent::People(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_component.people"
                    )
                    .unwrap_or_default(),
                I18nDossierComponent::Worths(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_component.worths"
                    )
                    .unwrap_or_default(),
                I18nDossierComponent::Accounts(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_component.accounts"
                    )
                    .unwrap_or_default(),
                I18nDossierComponent::CashFlows(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_component.cash_flows"
                    )
                    .unwrap_or_default(),
                I18nDossierComponent::Taxes(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_component.taxes"
                    )
                    .unwrap_or_default(),
                I18nDossierComponent::Assumptions(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_component.assumptions"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nFlowSpecComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nFlowSpecComponent::Name(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "flow_spec_component.name"
                    )
                    .unwrap_or_default(),
                I18nFlowSpecComponent::InFlow(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "flow_spec_component.in_flow"
                    )
                    .unwrap_or_default(),
                I18nFlowSpecComponent::OutFlow(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "flow_spec_component.out_flow"
                    )
                    .unwrap_or_default(),
                I18nFlowSpecComponent::CashFlow(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "flow_spec_component.cash_flow"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nFlowSpecsGrid {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nFlowSpecsGrid::Name(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "flow_specs_grid.name"
                    )
                    .unwrap_or_default(),
                I18nFlowSpecsGrid::InOut(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "flow_specs_grid.in_out"
                    )
                    .unwrap_or_default(),
                I18nFlowSpecsGrid::YearRange(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "flow_specs_grid.year_range"
                    )
                    .unwrap_or_default(),
                I18nFlowSpecsGrid::NewCashFlow(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "flow_specs_grid.new_cash_flow"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nHoldingComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nHoldingComponent::Mv(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holding_component.mv"
                    )
                    .unwrap_or_default(),
                I18nHoldingComponent::Holding(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holding_component.holding"
                    )
                    .unwrap_or_default(),
                I18nHoldingComponent::Ugl(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holding_component.ugl"
                    )
                    .unwrap_or_default(),
                I18nHoldingComponent::Symbol(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holding_component.symbol"
                    )
                    .unwrap_or_default(),
                I18nHoldingComponent::CurrentPrice(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holding_component.current_price"
                    )
                    .unwrap_or_default(),
                I18nHoldingComponent::Growth(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holding_component.growth"
                    )
                    .unwrap_or_default(),
                I18nHoldingComponent::Cost(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holding_component.cost"
                    )
                    .unwrap_or_default(),
                I18nHoldingComponent::Quantity(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holding_component.quantity"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nHoldingsGrid {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nHoldingsGrid::Symbol(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holdings_grid.symbol"
                    )
                    .unwrap_or_default(),
                I18nHoldingsGrid::Mv(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holdings_grid.mv"
                    )
                    .unwrap_or_default(),
                I18nHoldingsGrid::Cb(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holdings_grid.cb"
                    )
                    .unwrap_or_default(),
                I18nHoldingsGrid::Ugl(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holdings_grid.ugl"
                    )
                    .unwrap_or_default(),
                I18nHoldingsGrid::Holdings(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holdings_grid.holdings"
                    )
                    .unwrap_or_default(),
                I18nHoldingsGrid::NewHolding(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holdings_grid.new_holding"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nOkCancelComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nOkCancelComponent::Ok(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "ok_cancel_component.ok"
                    )
                    .unwrap_or_default(),
                I18nOkCancelComponent::Cancel(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "ok_cancel_component.cancel"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nPersonComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nPersonComponent::Name(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "person_component.name"
                    )
                    .unwrap_or_default(),
                I18nPersonComponent::BirthYear(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "person_component.birth_year"
                    )
                    .unwrap_or_default(),
                I18nPersonComponent::Role(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "person_component.role"
                    )
                    .unwrap_or_default(),
                I18nPersonComponent::RetirementAge(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "person_component.retirement_age"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nPersonsGrid {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nPersonsGrid::People(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "persons_grid.people"
                    )
                    .unwrap_or_default(),
                I18nPersonsGrid::Name(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "persons_grid.name"
                    )
                    .unwrap_or_default(),
                I18nPersonsGrid::Role(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "persons_grid.role"
                    )
                    .unwrap_or_default(),
                I18nPersonsGrid::RetirementAge(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "persons_grid.retirement_age"
                    )
                    .unwrap_or_default(),
                I18nPersonsGrid::NewPerson(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "persons_grid.new_person"
                    )
                    .unwrap_or_default(),
                I18nPersonsGrid::GridHelp(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "persons_grid.grid_help"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nTaxDeterminantsComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nTaxDeterminantsComponent::SimpleEffectiveRate(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "tax_determinants_component.simple_effective_rate"
                    )
                    .unwrap_or_default(),
                I18nTaxDeterminantsComponent::ByCountry(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "tax_determinants_component.by_country"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nWorthComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nWorthComponent::Worth(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "worth_component.worth"
                    )
                    .unwrap_or_default(),
                I18nWorthComponent::Name(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "worth_component.name"
                    )
                    .unwrap_or_default(),
                I18nWorthComponent::CurrentValue(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "worth_component.current_value"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nWorthsGrid {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nWorthsGrid::Worths(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "worths_grid.worths"
                    )
                    .unwrap_or_default(),
                I18nWorthsGrid::Name(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "worths_grid.name"
                    )
                    .unwrap_or_default(),
                I18nWorthsGrid::Type(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "worths_grid.type"
                    )
                    .unwrap_or_default(),
                I18nWorthsGrid::NewWorth(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "worths_grid.new_worth"
                    )
                    .unwrap_or_default(),
                I18nWorthsGrid::Value(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "worths_grid.value"
                    )
                    .unwrap_or_default(),
                I18nWorthsGrid::GridHelp(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "worths_grid.grid_help"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

// α <mod-def i18n_component_display>
// ω <mod-def i18n_component_display>
