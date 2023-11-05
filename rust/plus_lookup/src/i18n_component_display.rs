//! Support for internationalization of component text

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::lang_selector_to_language_id;
use crate::LOCALES;
use ::core::fmt::Display;
use ::core::fmt::Formatter;
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

/// Strings for `asset_liability_timeline`
#[derive(Debug, Copy, Clone)]
pub enum I18nAssetLiabilityTimeline {
    /// Strings for component AssetLiabilityTimeline
    AssetLiabilityTimeline(LangSelector),
}

/// Strings for `balance_sheet_statement`
#[derive(Debug, Copy, Clone)]
pub enum I18nBalanceSheetStatement {
    /// Strings for component BalanceSheet
    BalanceSheet(LangSelector),
    /// Strings for component BalanceSheetStatement
    BalanceSheetStatement(LangSelector),
    /// Strings for component StartValue
    StartValue(LangSelector),
    /// Strings for component EndValue
    EndValue(LangSelector),
    /// Strings for component AppreciationAbbrev
    AppreciationAbbrev(LangSelector),
    /// Strings for component AppreciationAbbrevPct
    AppreciationAbbrevPct(LangSelector),
    /// Strings for component SpendableDistributions
    SpendableDistributions(LangSelector),
    /// Strings for component BuySell
    BuySell(LangSelector),
}

/// Strings for `bond_spec_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nBondSpecComponent {
    /// Strings for component AnnualCoupon
    AnnualCoupon(LangSelector),
    /// Strings for component MaturityYear
    MaturityYear(LangSelector),
}

/// Strings for `cash_flow_statement`
#[derive(Debug, Copy, Clone)]
pub enum I18nCashFlowStatement {
    /// Strings for component CashFlowStatement
    CashFlowStatement(LangSelector),
    /// Strings for component CashFlows
    CashFlows(LangSelector),
    /// Strings for component Value
    Value(LangSelector),
}

/// Strings for `cash_flow_timeline`
#[derive(Debug, Copy, Clone)]
pub enum I18nCashFlowTimeline {
    /// Strings for component CashFlowTimeline
    CashFlowTimeline(LangSelector),
    /// Strings for component Zoom
    Zoom(LangSelector),
    /// Strings for component Balanced
    Balanced(LangSelector),
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
    /// Strings for component AnnualPctPlaceholder
    AnnualPctPlaceholder(LangSelector),
    /// Strings for component QualifiedDiv
    QualifiedDiv(LangSelector),
    /// Strings for component UnqualifiedDiv
    UnqualifiedDiv(LangSelector),
    /// Strings for component CapitalGain
    CapitalGain(LangSelector),
    /// Strings for component Interest
    Interest(LangSelector),
}

/// Strings for `dossier_editor`
#[derive(Debug, Copy, Clone)]
pub enum I18nDossierEditor {
    /// Strings for component DossierName
    DossierName(LangSelector),
    /// Strings for component DossierEditor
    DossierEditor(LangSelector),
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

/// Strings for `dossier_resolved_view`
#[derive(Debug, Copy, Clone)]
pub enum I18nDossierResolvedView {
    /// Strings for component DimIndex
    DimIndex(LangSelector),
    /// Strings for component MatrixIndex
    MatrixIndex(LangSelector),
    /// Strings for component Name
    Name(LangSelector),
    /// Strings for component User
    User(LangSelector),
    /// Strings for component Outlook
    Outlook(LangSelector),
    /// Strings for component Category
    Category(LangSelector),
    /// Strings for component Growth
    Growth(LangSelector),
    /// Strings for component ResolvedDossier
    ResolvedDossier(LangSelector),
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
    /// Strings for component CustomFlows
    CustomFlows(LangSelector),
    /// Strings for component GrowingFlows
    GrowingFlows(LangSelector),
}

/// Strings for `flow_specs_grid`
#[derive(Debug, Copy, Clone)]
pub enum I18nFlowSpecsGrid {
    /// Strings for component CashFlows
    CashFlows(LangSelector),
    /// Strings for component Name
    Name(LangSelector),
    /// Strings for component InOut
    InOut(LangSelector),
    /// Strings for component YearRange
    YearRange(LangSelector),
    /// Strings for component NewCashFlow
    NewCashFlow(LangSelector),
}

/// Strings for `forecast_config_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nForecastConfigComponent {
    /// Strings for component ForecastConfig
    ForecastConfig(LangSelector),
    /// Strings for component ScopedYearRange
    ScopedYearRange(LangSelector),
    /// Strings for component YearRangeOverride
    YearRangeOverride(LangSelector),
    /// Strings for component DisplayCurrency
    DisplayCurrency(LangSelector),
}

/// Strings for `forecast_id_selector`
#[derive(Debug, Copy, Clone)]
pub enum I18nForecastIdSelector {
    /// Strings for component GeometricMeanForecast
    GeometricMeanForecast(LangSelector),
    /// Strings for component RandomForecastId
    RandomForecastId(LangSelector),
}

/// Strings for `forecast_summary_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nForecastSummaryComponent {
    /// Strings for component ForecastSummary
    ForecastSummary(LangSelector),
}

/// Strings for `forecaster_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nForecasterComponent {
    /// Strings for component Forecaster
    Forecaster(LangSelector),
}

/// Strings for `growing_flow_spec_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nGrowingFlowSpecComponent {
    /// Strings for component Initial
    Initial(LangSelector),
    /// Strings for component YearRange
    YearRange(LangSelector),
    /// Strings for component YearPlaceholder
    YearPlaceholder(LangSelector),
    /// Strings for component ValuePlaceholder
    ValuePlaceholder(LangSelector),
}

/// Strings for `growth_assumption_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nGrowthAssumptionComponent {
    /// Strings for component NormalSpec
    NormalSpec(LangSelector),
    /// Strings for component FixedRateCurve
    FixedRateCurve(LangSelector),
}

/// Strings for `historic_risk_return_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nHistoricRiskReturnComponent {
    /// Strings for component HoldingType
    HoldingType(LangSelector),
}

/// Strings for `holding_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nHoldingComponent {
    /// Strings for component Cost
    Cost(LangSelector),
    /// Strings for component CurrentPrice
    CurrentPrice(LangSelector),
    /// Strings for component Distributions
    Distributions(LangSelector),
    /// Strings for component Growth
    Growth(LangSelector),
    /// Strings for component Holding
    Holding(LangSelector),
    /// Strings for component Mv
    Mv(LangSelector),
    /// Strings for component Price
    Price(LangSelector),
    /// Strings for component PricePlaceholder
    PricePlaceholder(LangSelector),
    /// Strings for component QtyPlaceholder
    QtyPlaceholder(LangSelector),
    /// Strings for component Quantity
    Quantity(LangSelector),
    /// Strings for component Symbol
    Symbol(LangSelector),
    /// Strings for component Ugl
    Ugl(LangSelector),
    /// Strings for component YearPlaceholder
    YearPlaceholder(LangSelector),
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

/// Strings for `investment_plan_view`
#[derive(Debug, Copy, Clone)]
pub enum I18nInvestmentPlanView {
    /// Strings for component Name
    Name(LangSelector),
    /// Strings for component Value
    Value(LangSelector),
    /// Strings for component Current
    Current(LangSelector),
    /// Strings for component Target
    Target(LangSelector),
    /// Strings for component TowardTarget
    TowardTarget(LangSelector),
    /// Strings for component TotalInvestments
    TotalInvestments(LangSelector),
    /// Strings for component InvestmentPlan
    InvestmentPlan(LangSelector),
}

/// Strings for `item_growth_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nItemGrowthComponent {
    /// Strings for component OverrideSystemGrowth
    OverrideSystemGrowth(LangSelector),
    /// Strings for component UseSystemGrowth
    UseSystemGrowth(LangSelector),
}

/// Strings for `matrix_resolved_view`
#[derive(Debug, Copy, Clone)]
pub enum I18nMatrixResolvedView {
    /// Strings for component ResolvedMatrix
    ResolvedMatrix(LangSelector),
}

/// Strings for `monte_config_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nMonteConfigComponent {
    /// Strings for component MonteConfig
    MonteConfig(LangSelector),
}

/// Strings for `monte_simulation_summary_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nMonteSimulationSummaryComponent {
    /// Strings for component SimulationSummary
    SimulationSummary(LangSelector),
}

/// Strings for `monte_simulator_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nMonteSimulatorComponent {
    /// Strings for component MonteSimulator
    MonteSimulator(LangSelector),
    /// Strings for component RunSimulation
    RunSimulation(LangSelector),
    /// Strings for component ForecastCount
    ForecastCount(LangSelector),
}

/// Strings for `normal_loss_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nNormalLossComponent {
    /// Strings for component GainPct
    GainPct(LangSelector),
    /// Strings for component ProbPct
    ProbPct(LangSelector),
    /// Strings for component ProbAbbrev
    ProbAbbrev(LangSelector),
    /// Strings for component CdfSample
    CdfSample(LangSelector),
    /// Strings for component GainPrefix
    GainPrefix(LangSelector),
    /// Strings for component LossTable
    LossTable(LangSelector),
}

/// Strings for `normal_spec_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nNormalSpecComponent {
    /// Strings for component MeanPlaceholder
    MeanPlaceholder(LangSelector),
    /// Strings for component StdDevPlaceholder
    StdDevPlaceholder(LangSelector),
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
    /// Strings for component Person
    Person(LangSelector),
    /// Strings for component BirthYear
    BirthYear(LangSelector),
    /// Strings for component Role
    Role(LangSelector),
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

/// Strings for `sales_plan_view`
#[derive(Debug, Copy, Clone)]
pub enum I18nSalesPlanView {
    /// Strings for component Name
    Name(LangSelector),
    /// Strings for component Value
    Value(LangSelector),
    /// Strings for component Current
    Current(LangSelector),
    /// Strings for component Target
    Target(LangSelector),
    /// Strings for component TowardTarget
    TowardTarget(LangSelector),
    /// Strings for component FromTarget
    FromTarget(LangSelector),
    /// Strings for component TotalSales
    TotalSales(LangSelector),
    /// Strings for component SalesPlan
    SalesPlan(LangSelector),
}

/// Strings for `tax_determinants_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nTaxDeterminantsComponent {
    /// Strings for component SimpleEffectiveRate
    SimpleEffectiveRate(LangSelector),
    /// Strings for component ByCountry
    ByCountry(LangSelector),
    /// Strings for component TaxDeterminants
    TaxDeterminants(LangSelector),
}

/// Strings for `timeline_group_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nTimelineGroupComponent {
    /// Strings for component Deterministic
    Deterministic(LangSelector),
    /// Strings for component Random
    Random(LangSelector),
    /// Strings for component TimelineDetail
    TimelineDetail(LangSelector),
    /// Strings for component Year
    Year(LangSelector),
    /// Strings for component Forecast
    Forecast(LangSelector),
}

/// Strings for `us_tax_statement_view`
#[derive(Debug, Copy, Clone)]
pub enum I18nUsTaxStatementView {
    /// Strings for component UsTaxStatement
    UsTaxStatement(LangSelector),
    /// Strings for component UsTaxes
    UsTaxes(LangSelector),
    /// Strings for component TaxBasis
    TaxBasis(LangSelector),
    /// Strings for component Tax
    Tax(LangSelector),
    /// Strings for component EarnedIncome
    EarnedIncome(LangSelector),
    /// Strings for component EstimateFromDossier
    EstimateFromDossier(LangSelector),
    /// Strings for component TaxableDistributions
    TaxableDistributions(LangSelector),
    /// Strings for component QualifiedDiv
    QualifiedDiv(LangSelector),
    /// Strings for component UnqualifiedDiv
    UnqualifiedDiv(LangSelector),
    /// Strings for component AdjustedGrossIncome
    AdjustedGrossIncome(LangSelector),
    /// Strings for component CapitalGainDistributions
    CapitalGainDistributions(LangSelector),
    /// Strings for component Interest
    Interest(LangSelector),
    /// Strings for component Medicare
    Medicare(LangSelector),
    /// Strings for component OtherOrdinaryIncome
    OtherOrdinaryIncome(LangSelector),
    /// Strings for component PassiveIncome
    PassiveIncome(LangSelector),
    /// Strings for component SocialSecurity
    SocialSecurity(LangSelector),
    /// Strings for component StandardDeduction
    StandardDeduction(LangSelector),
    /// Strings for component LongTermCapitalGain
    LongTermCapitalGain(LangSelector),
    /// Strings for component TaxBill
    TaxBill(LangSelector),
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
    /// Strings for component Cost
    Cost(LangSelector),
    /// Strings for component YearPlaceholder
    YearPlaceholder(LangSelector),
    /// Strings for component ValuePlaceholder
    ValuePlaceholder(LangSelector),
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

/// Strings for `year_currency_value_input`
#[derive(Debug, Copy, Clone)]
pub enum I18nYearCurrencyValueInput {
    /// Strings for component AsOf
    AsOf(LangSelector),
}

/// Strings for `year_range_input`
#[derive(Debug, Copy, Clone)]
pub enum I18nYearRangeInput {
    /// Strings for component StartPlaceholder
    StartPlaceholder(LangSelector),
    /// Strings for component EndPlaceholder
    EndPlaceholder(LangSelector),
}

/// Strings for `year_value_series_component`
#[derive(Debug, Copy, Clone)]
pub enum I18nYearValueSeriesComponent {
    /// Strings for component ShowRateCurve
    ShowRateCurve(LangSelector),
    /// Strings for component HideRateCurve
    HideRateCurve(LangSelector),
    /// Strings for component RatePlaceholder
    RatePlaceholder(LangSelector),
    /// Strings for component RatePercent
    RatePercent(LangSelector),
    /// Strings for component Value
    Value(LangSelector),
    /// Strings for component Year
    Year(LangSelector),
    /// Strings for component ValuePlaceholder
    ValuePlaceholder(LangSelector),
    /// Strings for component YearPlaceholder
    YearPlaceholder(LangSelector),
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl Display for I18nAccountComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
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
            }
        )
    }
}

impl Display for I18nAccountsGrid {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
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
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
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

impl Display for I18nAssetLiabilityTimeline {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nAssetLiabilityTimeline::AssetLiabilityTimeline(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "asset_liability_timeline.asset_liability_timeline"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nBalanceSheetStatement {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nBalanceSheetStatement::BalanceSheet(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "balance_sheet_statement.balance_sheet"
                    )
                    .unwrap_or_default(),
                I18nBalanceSheetStatement::BalanceSheetStatement(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "balance_sheet_statement.balance_sheet_statement"
                    )
                    .unwrap_or_default(),
                I18nBalanceSheetStatement::StartValue(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "balance_sheet_statement.start_value"
                    )
                    .unwrap_or_default(),
                I18nBalanceSheetStatement::EndValue(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "balance_sheet_statement.end_value"
                    )
                    .unwrap_or_default(),
                I18nBalanceSheetStatement::AppreciationAbbrev(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "balance_sheet_statement.appreciation_abbrev"
                    )
                    .unwrap_or_default(),
                I18nBalanceSheetStatement::AppreciationAbbrevPct(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "balance_sheet_statement.appreciation_abbrev_pct"
                    )
                    .unwrap_or_default(),
                I18nBalanceSheetStatement::SpendableDistributions(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "balance_sheet_statement.spendable_distributions"
                    )
                    .unwrap_or_default(),
                I18nBalanceSheetStatement::BuySell(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "balance_sheet_statement.buy_sell"
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
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
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

impl Display for I18nCashFlowStatement {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nCashFlowStatement::CashFlowStatement(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "cash_flow_statement.cash_flow_statement"
                    )
                    .unwrap_or_default(),
                I18nCashFlowStatement::CashFlows(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "cash_flow_statement.cash_flows"
                    )
                    .unwrap_or_default(),
                I18nCashFlowStatement::Value(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "cash_flow_statement.value"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nCashFlowTimeline {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nCashFlowTimeline::CashFlowTimeline(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "cash_flow_timeline.cash_flow_timeline"
                    )
                    .unwrap_or_default(),
                I18nCashFlowTimeline::Zoom(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "cash_flow_timeline.zoom"
                    )
                    .unwrap_or_default(),
                I18nCashFlowTimeline::Balanced(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "cash_flow_timeline.balanced"
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
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
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
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nDistributionSpecComponent::AnnualPctPlaceholder(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "distribution_spec_component.annual_pct_placeholder"
                    )
                    .unwrap_or_default(),
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

impl Display for I18nDossierEditor {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nDossierEditor::DossierName(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_editor.dossier_name"
                    )
                    .unwrap_or_default(),
                I18nDossierEditor::DossierEditor(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_editor.dossier_editor"
                    )
                    .unwrap_or_default(),
                I18nDossierEditor::People(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_editor.people"
                    )
                    .unwrap_or_default(),
                I18nDossierEditor::Worths(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_editor.worths"
                    )
                    .unwrap_or_default(),
                I18nDossierEditor::Accounts(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_editor.accounts"
                    )
                    .unwrap_or_default(),
                I18nDossierEditor::CashFlows(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_editor.cash_flows"
                    )
                    .unwrap_or_default(),
                I18nDossierEditor::Taxes(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_editor.taxes"
                    )
                    .unwrap_or_default(),
                I18nDossierEditor::Assumptions(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_editor.assumptions"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nDossierResolvedView {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nDossierResolvedView::DimIndex(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_resolved_view.dim_index"
                    )
                    .unwrap_or_default(),
                I18nDossierResolvedView::MatrixIndex(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_resolved_view.matrix_index"
                    )
                    .unwrap_or_default(),
                I18nDossierResolvedView::Name(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_resolved_view.name"
                    )
                    .unwrap_or_default(),
                I18nDossierResolvedView::User(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_resolved_view.user"
                    )
                    .unwrap_or_default(),
                I18nDossierResolvedView::Outlook(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_resolved_view.outlook"
                    )
                    .unwrap_or_default(),
                I18nDossierResolvedView::Category(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_resolved_view.category"
                    )
                    .unwrap_or_default(),
                I18nDossierResolvedView::Growth(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_resolved_view.growth"
                    )
                    .unwrap_or_default(),
                I18nDossierResolvedView::ResolvedDossier(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "dossier_resolved_view.resolved_dossier"
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
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
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
                I18nFlowSpecComponent::CustomFlows(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "flow_spec_component.custom_flows"
                    )
                    .unwrap_or_default(),
                I18nFlowSpecComponent::GrowingFlows(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "flow_spec_component.growing_flows"
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
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nFlowSpecsGrid::CashFlows(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "flow_specs_grid.cash_flows"
                    )
                    .unwrap_or_default(),
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

impl Display for I18nForecastConfigComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nForecastConfigComponent::ForecastConfig(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "forecast_config_component.forecast_config"
                    )
                    .unwrap_or_default(),
                I18nForecastConfigComponent::ScopedYearRange(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "forecast_config_component.scoped_year_range"
                    )
                    .unwrap_or_default(),
                I18nForecastConfigComponent::YearRangeOverride(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "forecast_config_component.year_range_override"
                    )
                    .unwrap_or_default(),
                I18nForecastConfigComponent::DisplayCurrency(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "forecast_config_component.display_currency"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nForecastIdSelector {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nForecastIdSelector::GeometricMeanForecast(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "forecast_id_selector.geometric_mean_forecast"
                    )
                    .unwrap_or_default(),
                I18nForecastIdSelector::RandomForecastId(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "forecast_id_selector.random_forecast_id"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nForecastSummaryComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nForecastSummaryComponent::ForecastSummary(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "forecast_summary_component.forecast_summary"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nForecasterComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nForecasterComponent::Forecaster(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "forecaster_component.forecaster"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nGrowingFlowSpecComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nGrowingFlowSpecComponent::Initial(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "growing_flow_spec_component.initial"
                    )
                    .unwrap_or_default(),
                I18nGrowingFlowSpecComponent::YearRange(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "growing_flow_spec_component.year_range"
                    )
                    .unwrap_or_default(),
                I18nGrowingFlowSpecComponent::YearPlaceholder(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "growing_flow_spec_component.year_placeholder"
                    )
                    .unwrap_or_default(),
                I18nGrowingFlowSpecComponent::ValuePlaceholder(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "growing_flow_spec_component.value_placeholder"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nGrowthAssumptionComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nGrowthAssumptionComponent::NormalSpec(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "growth_assumption_component.normal_spec"
                    )
                    .unwrap_or_default(),
                I18nGrowthAssumptionComponent::FixedRateCurve(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "growth_assumption_component.fixed_rate_curve"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nHistoricRiskReturnComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nHistoricRiskReturnComponent::HoldingType(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "historic_risk_return_component.holding_type"
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
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nHoldingComponent::Cost(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holding_component.cost"
                    )
                    .unwrap_or_default(),
                I18nHoldingComponent::CurrentPrice(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holding_component.current_price"
                    )
                    .unwrap_or_default(),
                I18nHoldingComponent::Distributions(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holding_component.distributions"
                    )
                    .unwrap_or_default(),
                I18nHoldingComponent::Growth(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holding_component.growth"
                    )
                    .unwrap_or_default(),
                I18nHoldingComponent::Holding(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holding_component.holding"
                    )
                    .unwrap_or_default(),
                I18nHoldingComponent::Mv(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holding_component.mv"
                    )
                    .unwrap_or_default(),
                I18nHoldingComponent::Price(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holding_component.price"
                    )
                    .unwrap_or_default(),
                I18nHoldingComponent::PricePlaceholder(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holding_component.price_placeholder"
                    )
                    .unwrap_or_default(),
                I18nHoldingComponent::QtyPlaceholder(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holding_component.qty_placeholder"
                    )
                    .unwrap_or_default(),
                I18nHoldingComponent::Quantity(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holding_component.quantity"
                    )
                    .unwrap_or_default(),
                I18nHoldingComponent::Symbol(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holding_component.symbol"
                    )
                    .unwrap_or_default(),
                I18nHoldingComponent::Ugl(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holding_component.ugl"
                    )
                    .unwrap_or_default(),
                I18nHoldingComponent::YearPlaceholder(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "holding_component.year_placeholder"
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
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
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

impl Display for I18nInvestmentPlanView {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nInvestmentPlanView::Name(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "investment_plan_view.name"
                    )
                    .unwrap_or_default(),
                I18nInvestmentPlanView::Value(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "investment_plan_view.value"
                    )
                    .unwrap_or_default(),
                I18nInvestmentPlanView::Current(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "investment_plan_view.current"
                    )
                    .unwrap_or_default(),
                I18nInvestmentPlanView::Target(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "investment_plan_view.target"
                    )
                    .unwrap_or_default(),
                I18nInvestmentPlanView::TowardTarget(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "investment_plan_view.toward_target"
                    )
                    .unwrap_or_default(),
                I18nInvestmentPlanView::TotalInvestments(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "investment_plan_view.total_investments"
                    )
                    .unwrap_or_default(),
                I18nInvestmentPlanView::InvestmentPlan(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "investment_plan_view.investment_plan"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nItemGrowthComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nItemGrowthComponent::OverrideSystemGrowth(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "item_growth_component.override_system_growth"
                    )
                    .unwrap_or_default(),
                I18nItemGrowthComponent::UseSystemGrowth(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "item_growth_component.use_system_growth"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nMatrixResolvedView {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nMatrixResolvedView::ResolvedMatrix(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "matrix_resolved_view.resolved_matrix"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nMonteConfigComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nMonteConfigComponent::MonteConfig(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "monte_config_component.monte_config"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nMonteSimulationSummaryComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nMonteSimulationSummaryComponent::SimulationSummary(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "monte_simulation_summary_component.simulation_summary"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nMonteSimulatorComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nMonteSimulatorComponent::MonteSimulator(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "monte_simulator_component.monte_simulator"
                    )
                    .unwrap_or_default(),
                I18nMonteSimulatorComponent::RunSimulation(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "monte_simulator_component.run_simulation"
                    )
                    .unwrap_or_default(),
                I18nMonteSimulatorComponent::ForecastCount(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "monte_simulator_component.forecast_count"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nNormalLossComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nNormalLossComponent::GainPct(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "normal_loss_component.gain_pct"
                    )
                    .unwrap_or_default(),
                I18nNormalLossComponent::ProbPct(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "normal_loss_component.prob_pct"
                    )
                    .unwrap_or_default(),
                I18nNormalLossComponent::ProbAbbrev(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "normal_loss_component.prob_abbrev"
                    )
                    .unwrap_or_default(),
                I18nNormalLossComponent::CdfSample(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "normal_loss_component.cdf_sample"
                    )
                    .unwrap_or_default(),
                I18nNormalLossComponent::GainPrefix(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "normal_loss_component.gain_prefix"
                    )
                    .unwrap_or_default(),
                I18nNormalLossComponent::LossTable(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "normal_loss_component.loss_table"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nNormalSpecComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nNormalSpecComponent::MeanPlaceholder(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "normal_spec_component.mean_placeholder"
                    )
                    .unwrap_or_default(),
                I18nNormalSpecComponent::StdDevPlaceholder(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "normal_spec_component.std_dev_placeholder"
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
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
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
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
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
                I18nPersonComponent::Person(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "person_component.person"
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
            }
        )
    }
}

impl Display for I18nPersonsGrid {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
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

impl Display for I18nSalesPlanView {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nSalesPlanView::Name(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "sales_plan_view.name"
                    )
                    .unwrap_or_default(),
                I18nSalesPlanView::Value(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "sales_plan_view.value"
                    )
                    .unwrap_or_default(),
                I18nSalesPlanView::Current(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "sales_plan_view.current"
                    )
                    .unwrap_or_default(),
                I18nSalesPlanView::Target(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "sales_plan_view.target"
                    )
                    .unwrap_or_default(),
                I18nSalesPlanView::TowardTarget(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "sales_plan_view.toward_target"
                    )
                    .unwrap_or_default(),
                I18nSalesPlanView::FromTarget(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "sales_plan_view.from_target"
                    )
                    .unwrap_or_default(),
                I18nSalesPlanView::TotalSales(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "sales_plan_view.total_sales"
                    )
                    .unwrap_or_default(),
                I18nSalesPlanView::SalesPlan(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "sales_plan_view.sales_plan"
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
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
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
                I18nTaxDeterminantsComponent::TaxDeterminants(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "tax_determinants_component.tax_determinants"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nTimelineGroupComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nTimelineGroupComponent::Deterministic(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "timeline_group_component.deterministic"
                    )
                    .unwrap_or_default(),
                I18nTimelineGroupComponent::Random(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "timeline_group_component.random"
                    )
                    .unwrap_or_default(),
                I18nTimelineGroupComponent::TimelineDetail(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "timeline_group_component.timeline_detail"
                    )
                    .unwrap_or_default(),
                I18nTimelineGroupComponent::Year(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "timeline_group_component.year"
                    )
                    .unwrap_or_default(),
                I18nTimelineGroupComponent::Forecast(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "timeline_group_component.forecast"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nUsTaxStatementView {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nUsTaxStatementView::UsTaxStatement(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "us_tax_statement_view.us_tax_statement"
                    )
                    .unwrap_or_default(),
                I18nUsTaxStatementView::UsTaxes(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "us_tax_statement_view.us_taxes"
                    )
                    .unwrap_or_default(),
                I18nUsTaxStatementView::TaxBasis(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "us_tax_statement_view.tax_basis"
                    )
                    .unwrap_or_default(),
                I18nUsTaxStatementView::Tax(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "us_tax_statement_view.tax"
                    )
                    .unwrap_or_default(),
                I18nUsTaxStatementView::EarnedIncome(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "us_tax_statement_view.earned_income"
                    )
                    .unwrap_or_default(),
                I18nUsTaxStatementView::EstimateFromDossier(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "us_tax_statement_view.estimate_from_dossier"
                    )
                    .unwrap_or_default(),
                I18nUsTaxStatementView::TaxableDistributions(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "us_tax_statement_view.taxable_distributions"
                    )
                    .unwrap_or_default(),
                I18nUsTaxStatementView::QualifiedDiv(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "us_tax_statement_view.qualified_div"
                    )
                    .unwrap_or_default(),
                I18nUsTaxStatementView::UnqualifiedDiv(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "us_tax_statement_view.unqualified_div"
                    )
                    .unwrap_or_default(),
                I18nUsTaxStatementView::AdjustedGrossIncome(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "us_tax_statement_view.adjusted_gross_income"
                    )
                    .unwrap_or_default(),
                I18nUsTaxStatementView::CapitalGainDistributions(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "us_tax_statement_view.capital_gain_distributions"
                    )
                    .unwrap_or_default(),
                I18nUsTaxStatementView::Interest(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "us_tax_statement_view.interest"
                    )
                    .unwrap_or_default(),
                I18nUsTaxStatementView::Medicare(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "us_tax_statement_view.medicare"
                    )
                    .unwrap_or_default(),
                I18nUsTaxStatementView::OtherOrdinaryIncome(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "us_tax_statement_view.other_ordinary_income"
                    )
                    .unwrap_or_default(),
                I18nUsTaxStatementView::PassiveIncome(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "us_tax_statement_view.passive_income"
                    )
                    .unwrap_or_default(),
                I18nUsTaxStatementView::SocialSecurity(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "us_tax_statement_view.social_security"
                    )
                    .unwrap_or_default(),
                I18nUsTaxStatementView::StandardDeduction(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "us_tax_statement_view.standard_deduction"
                    )
                    .unwrap_or_default(),
                I18nUsTaxStatementView::LongTermCapitalGain(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "us_tax_statement_view.long_term_capital_gain"
                    )
                    .unwrap_or_default(),
                I18nUsTaxStatementView::TaxBill(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "us_tax_statement_view.tax_bill"
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
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
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
                I18nWorthComponent::Cost(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "worth_component.cost"
                    )
                    .unwrap_or_default(),
                I18nWorthComponent::YearPlaceholder(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "worth_component.year_placeholder"
                    )
                    .unwrap_or_default(),
                I18nWorthComponent::ValuePlaceholder(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "worth_component.value_placeholder"
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
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
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

impl Display for I18nYearCurrencyValueInput {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nYearCurrencyValueInput::AsOf(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "year_currency_value_input.as_of"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nYearRangeInput {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nYearRangeInput::StartPlaceholder(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "year_range_input.start_placeholder"
                    )
                    .unwrap_or_default(),
                I18nYearRangeInput::EndPlaceholder(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "year_range_input.end_placeholder"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

impl Display for I18nYearValueSeriesComponent {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nYearValueSeriesComponent::ShowRateCurve(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "year_value_series_component.show_rate_curve"
                    )
                    .unwrap_or_default(),
                I18nYearValueSeriesComponent::HideRateCurve(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "year_value_series_component.hide_rate_curve"
                    )
                    .unwrap_or_default(),
                I18nYearValueSeriesComponent::RatePlaceholder(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "year_value_series_component.rate_placeholder"
                    )
                    .unwrap_or_default(),
                I18nYearValueSeriesComponent::RatePercent(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "year_value_series_component.rate_percent"
                    )
                    .unwrap_or_default(),
                I18nYearValueSeriesComponent::Value(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "year_value_series_component.value"
                    )
                    .unwrap_or_default(),
                I18nYearValueSeriesComponent::Year(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "year_value_series_component.year"
                    )
                    .unwrap_or_default(),
                I18nYearValueSeriesComponent::ValuePlaceholder(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "year_value_series_component.value_placeholder"
                    )
                    .unwrap_or_default(),
                I18nYearValueSeriesComponent::YearPlaceholder(lang_selector) => LOCALES
                    .lookup(
                        lang_selector_to_language_id(lang_selector),
                        "year_value_series_component.year_placeholder"
                    )
                    .unwrap_or_default(),
            }
        )
    }
}

// α <mod-def i18n_component_display>
// ω <mod-def i18n_component_display>
