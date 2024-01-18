//! Support for internationalization of enum labels.

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::lang_selector_to_language_id;
use crate::LOCALES;
use ::core::fmt::Display;
use ::core::fmt::Formatter;
use fluent_templates::Loader;
use plus_modeled::AccountType;
use plus_modeled::BasicAllocationType;
use plus_modeled::Country;
use plus_modeled::DistributionInstrument;
use plus_modeled::DistributionInstrumentType;
use plus_modeled::DossierItemType;
use plus_modeled::FlowDirection;
use plus_modeled::FlowType;
use plus_modeled::ForecastSortCriteria;
use plus_modeled::ForecastTaxTreatment;
use plus_modeled::ForecastYearMarkerType;
use plus_modeled::HoldingType;
use plus_modeled::LangSelector;
use plus_modeled::NamedRateCurve;
use plus_modeled::PersonType;
use plus_modeled::ReturnStatsType;
use plus_modeled::TaxTreatment;
use plus_modeled::TaxUsCategory;
use plus_modeled::TaxUsFilingStatus;
use plus_modeled::WorthType;
use plus_modeled::YearEndpoint;

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// Common strings referenced by various components, tables, etc
#[derive(Debug, Copy, Clone)]
pub enum CommonStrings {
    /// i18n value for `account`
    Account,
    /// i18n value for `accounts`
    Accounts,
    /// i18n value for `adjusted_gross_income`
    AdjustedGrossIncome,
    /// i18n value for `annual_pct_placeholder`
    AnnualPctPlaceholder,
    /// i18n value for `as_of`
    AsOf,
    /// i18n value for `assets`
    Assets,
    /// i18n value for `assumptions`
    Assumptions,
    /// i18n value for `balance_sheet`
    BalanceSheet,
    /// i18n value for `borrowed_funds`
    BorrowedFunds,
    /// i18n value for `capital_gain`
    CapitalGain,
    /// i18n value for `capital_gain_distributions`
    CapitalGainDistributions,
    /// i18n value for `cash_flow`
    CashFlow,
    /// i18n value for `cash_flows`
    CashFlows,
    /// i18n value for `category`
    Category,
    /// i18n value for `cost`
    Cost,
    /// i18n value for `cost_basis`
    CostBasis,
    /// i18n value for `current`
    Current,
    /// i18n value for `current_price`
    CurrentPrice,
    /// i18n value for `current_value`
    CurrentValue,
    /// i18n value for `death_age`
    DeathAge,
    /// i18n value for `deterministic`
    Deterministic,
    /// i18n value for `display_currency`
    DisplayCurrency,
    /// i18n value for `distributions`
    Distributions,
    /// i18n value for `distributions_reinvested`
    DistributionsReinvested,
    /// i18n value for `distributions_spendable`
    DistributionsSpendable,
    /// i18n value for `dossier`
    Dossier,
    /// i18n value for `dossier_name`
    DossierName,
    /// i18n value for `earned_income`
    EarnedIncome,
    /// i18n value for `end_placeholder`
    EndPlaceholder,
    /// i18n value for `end_value`
    EndValue,
    /// i18n value for `expense`
    Expense,
    /// i18n value for `expenses`
    Expenses,
    /// i18n value for `forecast`
    Forecast,
    /// i18n value for `forecast_id`
    ForecastId,
    /// i18n value for `from`
    From,
    /// i18n value for `geometric_mean_forecast`
    GeometricMeanForecast,
    /// i18n value for `growth`
    Growth,
    /// i18n value for `holding`
    Holding,
    /// i18n value for `holding_type`
    HoldingType,
    /// i18n value for `holdings`
    Holdings,
    /// i18n value for `in_flow`
    InFlow,
    /// i18n value for `income`
    Income,
    /// i18n value for `incomes`
    Incomes,
    /// i18n value for `interest`
    Interest,
    /// i18n value for `investments`
    Investments,
    /// i18n value for `liability`
    Liability,
    /// i18n value for `linked`
    Linked,
    /// i18n value for `linked_income`
    LinkedIncome,
    /// i18n value for `linked_investments`
    LinkedInvestments,
    /// i18n value for `linked_sales`
    LinkedSales,
    /// i18n value for `linked_expense`
    LinkedExpense,
    /// i18n value for `long_term_capital_gain`
    LongTermCapitalGain,
    /// i18n value for `mean_placeholder`
    MeanPlaceholder,
    /// i18n value for `mean`
    Mean,
    /// i18n value for `medicare`
    Medicare,
    /// i18n value for `mv`
    Mv,
    /// i18n value for `name`
    Name,
    /// i18n value for `net_flows`
    NetFlows,
    /// i18n value for `obligation_sale`
    ObligationSale,
    /// i18n value for `other_ordinary_income`
    OtherOrdinaryIncome,
    /// i18n value for `out_flow`
    OutFlow,
    /// i18n value for `outlook`
    Outlook,
    /// i18n value for `passive_income`
    PassiveIncome,
    /// i18n value for `people`
    People,
    /// i18n value for `person`
    Person,
    /// i18n value for `price`
    Price,
    /// i18n value for `price_placeholder`
    PricePlaceholder,
    /// i18n value for `primary_owner`
    PrimaryOwner,
    /// i18n value for `qty_placeholder`
    QtyPlaceholder,
    /// i18n value for `qualified_div`
    QualifiedDiv,
    /// i18n value for `quantity`
    Quantity,
    /// i18n value for `rate`
    Rate,
    /// i18n value for `rate_placeholder`
    RatePlaceholder,
    /// i18n value for `random`
    Random,
    /// i18n value for `real_asset`
    RealAsset,
    /// i18n value for `repaid_borrowed_funds`
    RepaidBorrowedFunds,
    /// i18n value for `retirement_age`
    RetirementAge,
    /// i18n value for `rmd_sales`
    RmdSales,
    /// i18n value for `role`
    Role,
    /// i18n value for `start_placeholder`
    StartPlaceholder,
    /// i18n value for `start_value`
    StartValue,
    /// i18n value for `social_security`
    SocialSecurity,
    /// i18n value for `sold`
    Sold,
    /// i18n value for `standard_deduction`
    StandardDeduction,
    /// i18n value for `std_dev_placeholder`
    StdDevPlaceholder,
    /// i18n value for `std_dev`
    StdDev,
    /// i18n value for `symbol`
    Symbol,
    /// i18n value for `target`
    Target,
    /// i18n value for `tax`
    Tax,
    /// i18n value for `tax_basis`
    TaxBasis,
    /// i18n value for `tax_bill`
    TaxBill,
    /// i18n value for `taxable_distributions`
    TaxableDistributions,
    /// i18n value for `taxes`
    Taxes,
    /// i18n value for `total`
    Total,
    /// i18n value for `totals`
    Totals,
    /// i18n value for `toward`
    Toward,
    /// i18n value for `type`
    Type,
    /// i18n value for `ugl`
    Ugl,
    /// i18n value for `unqualified_div`
    UnqualifiedDiv,
    /// i18n value for `us_taxes`
    UsTaxes,
    /// i18n value for `user`
    User,
    /// i18n value for `value`
    Value,
    /// i18n value for `value_placeholder`
    ValuePlaceholder,
    /// i18n value for `worth`
    Worth,
    /// i18n value for `worths`
    Worths,
    /// i18n value for `year`
    Year,
    /// i18n value for `year_placeholder`
    YearPlaceholder,
    /// i18n value for `year_range`
    YearRange,
    /// i18n value for `zoom`
    Zoom,
}

/// Enumeration of all protobuf enums
#[derive(Debug, Copy, Clone)]
pub enum I18nEnums<'a> {
    /// Enumerates supported enums and implements display to dispatch on language and value
    AccountType(LangSelector, &'a AccountType),
    /// Enumerates supported enums and implements display to dispatch on language and value
    BasicAllocationType(LangSelector, &'a BasicAllocationType),
    /// Enumerates supported enums and implements display to dispatch on language and value
    Country(LangSelector, &'a Country),
    /// Enumerates supported enums and implements display to dispatch on language and value
    DistributionInstrument(LangSelector, &'a DistributionInstrument),
    /// Enumerates supported enums and implements display to dispatch on language and value
    DistributionInstrumentType(LangSelector, &'a DistributionInstrumentType),
    /// Enumerates supported enums and implements display to dispatch on language and value
    DossierItemType(LangSelector, &'a DossierItemType),
    /// Enumerates supported enums and implements display to dispatch on language and value
    FlowDirection(LangSelector, &'a FlowDirection),
    /// Enumerates supported enums and implements display to dispatch on language and value
    FlowType(LangSelector, &'a FlowType),
    /// Enumerates supported enums and implements display to dispatch on language and value
    ForecastSortCriteria(LangSelector, &'a ForecastSortCriteria),
    /// Enumerates supported enums and implements display to dispatch on language and value
    ForecastTaxTreatment(LangSelector, &'a ForecastTaxTreatment),
    /// Enumerates supported enums and implements display to dispatch on language and value
    ForecastYearMarkerType(LangSelector, &'a ForecastYearMarkerType),
    /// Enumerates supported enums and implements display to dispatch on language and value
    HoldingType(LangSelector, &'a HoldingType),
    /// Enumerates supported enums and implements display to dispatch on language and value
    LangSelector(LangSelector, &'a LangSelector),
    /// Enumerates supported enums and implements display to dispatch on language and value
    NamedRateCurve(LangSelector, &'a NamedRateCurve),
    /// Enumerates supported enums and implements display to dispatch on language and value
    PersonType(LangSelector, &'a PersonType),
    /// Enumerates supported enums and implements display to dispatch on language and value
    ReturnStatsType(LangSelector, &'a ReturnStatsType),
    /// Enumerates supported enums and implements display to dispatch on language and value
    TaxTreatment(LangSelector, &'a TaxTreatment),
    /// Enumerates supported enums and implements display to dispatch on language and value
    TaxUsCategory(LangSelector, &'a TaxUsCategory),
    /// Enumerates supported enums and implements display to dispatch on language and value
    TaxUsFilingStatus(LangSelector, &'a TaxUsFilingStatus),
    /// Enumerates supported enums and implements display to dispatch on language and value
    WorthType(LangSelector, &'a WorthType),
    /// Enumerates supported enums and implements display to dispatch on language and value
    YearEndpoint(LangSelector, &'a YearEndpoint),
    /// Enumerates supported enums and implements display to dispatch on language and value
    CommonStrings(LangSelector, &'a CommonStrings),
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl<'a> Display for I18nEnums<'a> {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, #[allow(unused)] f: &mut Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                I18nEnums::AccountType(lang_selector, e) => match e {
                    AccountType::Taxable => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "account_type.taxable"
                        )
                        .unwrap_or_default(),
                    AccountType::RothIrs401K => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "account_type.roth_irs_401_k"
                        )
                        .unwrap_or_default(),
                    AccountType::TraditionalIrs401K => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "account_type.traditional_irs_401_k"
                        )
                        .unwrap_or_default(),
                    AccountType::CollegeIrs529 => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "account_type.college_irs_529"
                        )
                        .unwrap_or_default(),
                    AccountType::TraditionalIra => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "account_type.traditional_ira"
                        )
                        .unwrap_or_default(),
                    AccountType::Demand => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "account_type.demand"
                        )
                        .unwrap_or_default(),
                    AccountType::HealthSavingsAccount => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "account_type.health_savings_account"
                        )
                        .unwrap_or_default(),
                    AccountType::OtherAccountType => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "account_type.other_account_type"
                        )
                        .unwrap_or_default(),
                },
                I18nEnums::BasicAllocationType(lang_selector, e) => match e {
                    BasicAllocationType::OtherBasicAllocationType => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "basic_allocation_type.other_basic_allocation_type"
                        )
                        .unwrap_or_default(),
                    BasicAllocationType::Stock => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "basic_allocation_type.stock"
                        )
                        .unwrap_or_default(),
                    BasicAllocationType::Bond => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "basic_allocation_type.bond"
                        )
                        .unwrap_or_default(),
                    BasicAllocationType::Cash => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "basic_allocation_type.cash"
                        )
                        .unwrap_or_default(),
                },
                I18nEnums::Country(lang_selector, e) => match e {
                    Country::UnitedStates => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "country.united_states"
                        )
                        .unwrap_or_default(),
                    Country::France => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "country.france"
                        )
                        .unwrap_or_default(),
                    Country::UnitedKingdom => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "country.united_kingdom"
                        )
                        .unwrap_or_default(),
                },
                I18nEnums::DistributionInstrument(lang_selector, e) => match e {
                    DistributionInstrument::Equity => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "distribution_instrument.equity"
                        )
                        .unwrap_or_default(),
                    DistributionInstrument::InterestBearing => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "distribution_instrument.interest_bearing"
                        )
                        .unwrap_or_default(),
                    DistributionInstrument::MutualFund => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "distribution_instrument.mutual_fund"
                        )
                        .unwrap_or_default(),
                },
                I18nEnums::DistributionInstrumentType(lang_selector, e) => match e {
                    DistributionInstrumentType::DistUnknown => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "distribution_instrument_type.dist_unknown"
                        )
                        .unwrap_or_default(),
                    DistributionInstrumentType::DistEquity => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "distribution_instrument_type.dist_equity"
                        )
                        .unwrap_or_default(),
                    DistributionInstrumentType::DistBond => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "distribution_instrument_type.dist_bond"
                        )
                        .unwrap_or_default(),
                    DistributionInstrumentType::DistMutualFund => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "distribution_instrument_type.dist_mutual_fund"
                        )
                        .unwrap_or_default(),
                },
                I18nEnums::DossierItemType(lang_selector, e) => match e {
                    DossierItemType::Worth => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "dossier_item_type.worth"
                        )
                        .unwrap_or_default(),
                    DossierItemType::Holding => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "dossier_item_type.holding"
                        )
                        .unwrap_or_default(),
                    DossierItemType::Flow => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "dossier_item_type.flow"
                        )
                        .unwrap_or_default(),
                },
                I18nEnums::FlowDirection(lang_selector, e) => match e {
                    FlowDirection::UnspecifiedFlow => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_direction.unspecified_flow"
                        )
                        .unwrap_or_default(),
                    FlowDirection::InFlow => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_direction.in_flow"
                        )
                        .unwrap_or_default(),
                    FlowDirection::OutFlow => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_direction.out_flow"
                        )
                        .unwrap_or_default(),
                    FlowDirection::InternalFlow => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_direction.internal_flow"
                        )
                        .unwrap_or_default(),
                },
                I18nEnums::FlowType(lang_selector, e) => match e {
                    FlowType::EarnedIncome => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_type.earned_income"
                        )
                        .unwrap_or_default(),
                    FlowType::PensionIncome => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_type.pension_income"
                        )
                        .unwrap_or_default(),
                    FlowType::SocialSecurityIncome => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_type.social_security_income"
                        )
                        .unwrap_or_default(),
                    FlowType::RentalIncome => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_type.rental_income"
                        )
                        .unwrap_or_default(),
                    FlowType::RoyaltyIncome => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_type.royalty_income"
                        )
                        .unwrap_or_default(),
                    FlowType::InternetAdvertisingIncome => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_type.internet_advertising_income"
                        )
                        .unwrap_or_default(),
                    FlowType::PassiveBusinessIncome => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_type.passive_business_income"
                        )
                        .unwrap_or_default(),
                    FlowType::OrdinaryIncome => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_type.ordinary_income"
                        )
                        .unwrap_or_default(),
                    FlowType::LivingExpense => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_type.living_expense"
                        )
                        .unwrap_or_default(),
                    FlowType::HealthCareExpense => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_type.health_care_expense"
                        )
                        .unwrap_or_default(),
                    FlowType::CollegeExpense => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_type.college_expense"
                        )
                        .unwrap_or_default(),
                    FlowType::PropertyTaxes => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_type.property_taxes"
                        )
                        .unwrap_or_default(),
                    FlowType::MortgageInterest => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_type.mortgage_interest"
                        )
                        .unwrap_or_default(),
                    FlowType::StateTaxesPaid => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_type.state_taxes_paid"
                        )
                        .unwrap_or_default(),
                    FlowType::CharitableDonations => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_type.charitable_donations"
                        )
                        .unwrap_or_default(),
                    FlowType::MedicalExpenses => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_type.medical_expenses"
                        )
                        .unwrap_or_default(),
                    FlowType::RetirementCredits => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_type.retirement_credits"
                        )
                        .unwrap_or_default(),
                    FlowType::IraContributions => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_type.ira_contributions"
                        )
                        .unwrap_or_default(),
                    FlowType::OtherInFlow => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_type.other_in_flow"
                        )
                        .unwrap_or_default(),
                    FlowType::OtherOutFlow => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_type.other_out_flow"
                        )
                        .unwrap_or_default(),
                    FlowType::CustomFlow => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "flow_type.custom_flow"
                        )
                        .unwrap_or_default(),
                },
                I18nEnums::ForecastSortCriteria(lang_selector, e) => match e {
                    ForecastSortCriteria::ByFinalEndBalance => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "forecast_sort_criteria.by_final_end_balance"
                        )
                        .unwrap_or_default(),
                },
                I18nEnums::ForecastTaxTreatment(lang_selector, e) => match e {
                    ForecastTaxTreatment::AsModeled => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "forecast_tax_treatment.as_modeled"
                        )
                        .unwrap_or_default(),
                    ForecastTaxTreatment::AsTaxable => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "forecast_tax_treatment.as_taxable"
                        )
                        .unwrap_or_default(),
                    ForecastTaxTreatment::AsTaxDeferred => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "forecast_tax_treatment.as_tax_deferred"
                        )
                        .unwrap_or_default(),
                    ForecastTaxTreatment::AsTaxExempt => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "forecast_tax_treatment.as_tax_exempt"
                        )
                        .unwrap_or_default(),
                },
                I18nEnums::ForecastYearMarkerType(lang_selector, e) => match e {
                    ForecastYearMarkerType::FirstInsolvency => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "forecast_year_marker_type.first_insolvency"
                        )
                        .unwrap_or_default(),
                    ForecastYearMarkerType::RetirementStart => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "forecast_year_marker_type.retirement_start"
                        )
                        .unwrap_or_default(),
                    ForecastYearMarkerType::Death => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "forecast_year_marker_type.death"
                        )
                        .unwrap_or_default(),
                },
                I18nEnums::HoldingType(lang_selector, e) => match e {
                    HoldingType::UsEquitySample => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.us_equity_sample"
                        )
                        .unwrap_or_default(),
                    HoldingType::UsEquityMarket => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.us_equity_market"
                        )
                        .unwrap_or_default(),
                    HoldingType::UsLargeEquityMarket => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.us_large_equity_market"
                        )
                        .unwrap_or_default(),
                    HoldingType::UsLargeEquitySample => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.us_large_equity_sample"
                        )
                        .unwrap_or_default(),
                    HoldingType::UsSmallEquityMarket => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.us_small_equity_market"
                        )
                        .unwrap_or_default(),
                    HoldingType::UsSmallEquitySample => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.us_small_equity_sample"
                        )
                        .unwrap_or_default(),
                    HoldingType::UsMidEquityMarket => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.us_mid_equity_market"
                        )
                        .unwrap_or_default(),
                    HoldingType::UsMidEquitySample => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.us_mid_equity_sample"
                        )
                        .unwrap_or_default(),
                    HoldingType::EmergingEquityMarket => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.emerging_equity_market"
                        )
                        .unwrap_or_default(),
                    HoldingType::EmergingEquityMarketSample => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.emerging_equity_market_sample"
                        )
                        .unwrap_or_default(),
                    HoldingType::Gold => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.gold"
                        )
                        .unwrap_or_default(),
                    HoldingType::Tips => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.tips"
                        )
                        .unwrap_or_default(),
                    HoldingType::RealEstate => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.real_estate"
                        )
                        .unwrap_or_default(),
                    HoldingType::DevelopedMarkets => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.developed_markets"
                        )
                        .unwrap_or_default(),
                    HoldingType::DevelopedMarketsSample => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.developed_markets_sample"
                        )
                        .unwrap_or_default(),
                    HoldingType::UsHighYieldBonds => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.us_high_yield_bonds"
                        )
                        .unwrap_or_default(),
                    HoldingType::Commodities => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.commodities"
                        )
                        .unwrap_or_default(),
                    HoldingType::UsLongTermTreasury => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.us_long_term_treasury"
                        )
                        .unwrap_or_default(),
                    HoldingType::UsIntermediateTermTreasury => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.us_intermediate_term_treasury"
                        )
                        .unwrap_or_default(),
                    HoldingType::UsShortTermTreasury => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.us_short_term_treasury"
                        )
                        .unwrap_or_default(),
                    HoldingType::UsShortTermCorporateBond => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.us_short_term_corporate_bond"
                        )
                        .unwrap_or_default(),
                    HoldingType::UsLongTermCorporateBond => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.us_long_term_corporate_bond"
                        )
                        .unwrap_or_default(),
                    HoldingType::UsMoneyMarket => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.us_money_market"
                        )
                        .unwrap_or_default(),
                    HoldingType::InternationalBonds => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.international_bonds"
                        )
                        .unwrap_or_default(),
                    HoldingType::CertificateOfDeposit => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.certificate_of_deposit"
                        )
                        .unwrap_or_default(),
                    HoldingType::OtherInstrument => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "holding_type.other_instrument"
                        )
                        .unwrap_or_default(),
                },
                I18nEnums::LangSelector(lang_selector, e) => match e {
                    LangSelector::UsEnglish => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "lang_selector.us_english"
                        )
                        .unwrap_or_default(),
                    LangSelector::French => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "lang_selector.french"
                        )
                        .unwrap_or_default(),
                    LangSelector::German => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "lang_selector.german"
                        )
                        .unwrap_or_default(),
                },
                I18nEnums::NamedRateCurve(lang_selector, e) => match e {
                    NamedRateCurve::NoGrowthCurve => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "named_rate_curve.no_growth_curve"
                        )
                        .unwrap_or_default(),
                    NamedRateCurve::ReportInflation => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "named_rate_curve.report_inflation"
                        )
                        .unwrap_or_default(),
                    NamedRateCurve::CostOfCapital => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "named_rate_curve.cost_of_capital"
                        )
                        .unwrap_or_default(),
                },
                I18nEnums::PersonType(lang_selector, e) => match e {
                    PersonType::PrimaryOwner => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "person_type.primary_owner"
                        )
                        .unwrap_or_default(),
                    PersonType::SecondaryOwner => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "person_type.secondary_owner"
                        )
                        .unwrap_or_default(),
                    PersonType::Dependent => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "person_type.dependent"
                        )
                        .unwrap_or_default(),
                },
                I18nEnums::ReturnStatsType(lang_selector, e) => match e {
                    ReturnStatsType::NoTracking => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "return_stats_type.no_tracking"
                        )
                        .unwrap_or_default(),
                    ReturnStatsType::CorrelationsAndReturn => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "return_stats_type.correlations_and_return"
                        )
                        .unwrap_or_default(),
                    ReturnStatsType::ReturnOnly => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "return_stats_type.return_only"
                        )
                        .unwrap_or_default(),
                },
                I18nEnums::TaxTreatment(lang_selector, e) => match e {
                    TaxTreatment::TaxableAccount => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "tax_treatment.taxable_account"
                        )
                        .unwrap_or_default(),
                    TaxTreatment::TaxDeferredAccount => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "tax_treatment.tax_deferred_account"
                        )
                        .unwrap_or_default(),
                    TaxTreatment::TaxExemptAccount => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "tax_treatment.tax_exempt_account"
                        )
                        .unwrap_or_default(),
                },
                I18nEnums::TaxUsCategory(lang_selector, e) => match e {
                    TaxUsCategory::UsSocialSecurity => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "tax_us_category.us_social_security"
                        )
                        .unwrap_or_default(),
                    TaxUsCategory::UsMedicare => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "tax_us_category.us_medicare"
                        )
                        .unwrap_or_default(),
                    TaxUsCategory::UsQualifiedDividend => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "tax_us_category.us_qualified_dividend"
                        )
                        .unwrap_or_default(),
                    TaxUsCategory::UsLongTermCapitalGain => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "tax_us_category.us_long_term_capital_gain"
                        )
                        .unwrap_or_default(),
                    TaxUsCategory::UsPassiveIncome => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "tax_us_category.us_passive_income"
                        )
                        .unwrap_or_default(),
                    TaxUsCategory::UsEarnedIncome => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "tax_us_category.us_earned_income"
                        )
                        .unwrap_or_default(),
                    TaxUsCategory::UsOrdinaryIncome => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "tax_us_category.us_ordinary_income"
                        )
                        .unwrap_or_default(),
                },
                I18nEnums::TaxUsFilingStatus(lang_selector, e) => match e {
                    TaxUsFilingStatus::MarriedJoint => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "tax_us_filing_status.married_joint"
                        )
                        .unwrap_or_default(),
                    TaxUsFilingStatus::MarriedSeparate => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "tax_us_filing_status.married_separate"
                        )
                        .unwrap_or_default(),
                    TaxUsFilingStatus::Single => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "tax_us_filing_status.single"
                        )
                        .unwrap_or_default(),
                    TaxUsFilingStatus::HeadOfHousehold => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "tax_us_filing_status.head_of_household"
                        )
                        .unwrap_or_default(),
                },
                I18nEnums::WorthType(lang_selector, e) => match e {
                    WorthType::ResidentialRealEstate => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "worth_type.residential_real_estate"
                        )
                        .unwrap_or_default(),
                    WorthType::CommercialRealEstate => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "worth_type.commercial_real_estate"
                        )
                        .unwrap_or_default(),
                    WorthType::FamilyFarm => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "worth_type.family_farm"
                        )
                        .unwrap_or_default(),
                    WorthType::Automobile => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "worth_type.automobile"
                        )
                        .unwrap_or_default(),
                    WorthType::ClassicCar => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "worth_type.classic_car"
                        )
                        .unwrap_or_default(),
                    WorthType::AntiqueCar => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "worth_type.antique_car"
                        )
                        .unwrap_or_default(),
                    WorthType::VintageCar => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "worth_type.vintage_car"
                        )
                        .unwrap_or_default(),
                    WorthType::Boat => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "worth_type.boat"
                        )
                        .unwrap_or_default(),
                    WorthType::Toys => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "worth_type.toys"
                        )
                        .unwrap_or_default(),
                    WorthType::OtherWorth => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "worth_type.other_worth"
                        )
                        .unwrap_or_default(),
                },
                I18nEnums::YearEndpoint(lang_selector, e) => match e {
                    YearEndpoint::StartYear => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "year_endpoint.start_year"
                        )
                        .unwrap_or_default(),
                    YearEndpoint::EndYear => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "year_endpoint.end_year"
                        )
                        .unwrap_or_default(),
                },
                I18nEnums::CommonStrings(lang_selector, e) => match e {
                    CommonStrings::Account => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.account"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Accounts => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.accounts"
                        )
                        .unwrap_or_default(),
                    CommonStrings::AdjustedGrossIncome => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.adjusted_gross_income"
                        )
                        .unwrap_or_default(),
                    CommonStrings::AnnualPctPlaceholder => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.annual_pct_placeholder"
                        )
                        .unwrap_or_default(),
                    CommonStrings::AsOf => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.as_of"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Assets => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.assets"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Assumptions => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.assumptions"
                        )
                        .unwrap_or_default(),
                    CommonStrings::BalanceSheet => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.balance_sheet"
                        )
                        .unwrap_or_default(),
                    CommonStrings::BorrowedFunds => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.borrowed_funds"
                        )
                        .unwrap_or_default(),
                    CommonStrings::CapitalGain => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.capital_gain"
                        )
                        .unwrap_or_default(),
                    CommonStrings::CapitalGainDistributions => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.capital_gain_distributions"
                        )
                        .unwrap_or_default(),
                    CommonStrings::CashFlow => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.cash_flow"
                        )
                        .unwrap_or_default(),
                    CommonStrings::CashFlows => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.cash_flows"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Category => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.category"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Cost => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.cost"
                        )
                        .unwrap_or_default(),
                    CommonStrings::CostBasis => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.cost_basis"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Current => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.current"
                        )
                        .unwrap_or_default(),
                    CommonStrings::CurrentPrice => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.current_price"
                        )
                        .unwrap_or_default(),
                    CommonStrings::CurrentValue => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.current_value"
                        )
                        .unwrap_or_default(),
                    CommonStrings::DeathAge => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.death_age"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Deterministic => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.deterministic"
                        )
                        .unwrap_or_default(),
                    CommonStrings::DisplayCurrency => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.display_currency"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Distributions => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.distributions"
                        )
                        .unwrap_or_default(),
                    CommonStrings::DistributionsReinvested => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.distributions_reinvested"
                        )
                        .unwrap_or_default(),
                    CommonStrings::DistributionsSpendable => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.distributions_spendable"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Dossier => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.dossier"
                        )
                        .unwrap_or_default(),
                    CommonStrings::DossierName => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.dossier_name"
                        )
                        .unwrap_or_default(),
                    CommonStrings::EarnedIncome => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.earned_income"
                        )
                        .unwrap_or_default(),
                    CommonStrings::EndPlaceholder => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.end_placeholder"
                        )
                        .unwrap_or_default(),
                    CommonStrings::EndValue => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.end_value"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Expense => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.expense"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Expenses => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.expenses"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Forecast => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.forecast"
                        )
                        .unwrap_or_default(),
                    CommonStrings::ForecastId => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.forecast_id"
                        )
                        .unwrap_or_default(),
                    CommonStrings::From => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.from"
                        )
                        .unwrap_or_default(),
                    CommonStrings::GeometricMeanForecast => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.geometric_mean_forecast"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Growth => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.growth"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Holding => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.holding"
                        )
                        .unwrap_or_default(),
                    CommonStrings::HoldingType => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.holding_type"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Holdings => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.holdings"
                        )
                        .unwrap_or_default(),
                    CommonStrings::InFlow => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.in_flow"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Income => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.income"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Incomes => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.incomes"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Interest => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.interest"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Investments => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.investments"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Liability => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.liability"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Linked => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.linked"
                        )
                        .unwrap_or_default(),
                    CommonStrings::LinkedIncome => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.linked_income"
                        )
                        .unwrap_or_default(),
                    CommonStrings::LinkedInvestments => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.linked_investments"
                        )
                        .unwrap_or_default(),
                    CommonStrings::LinkedSales => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.linked_sales"
                        )
                        .unwrap_or_default(),
                    CommonStrings::LinkedExpense => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.linked_expense"
                        )
                        .unwrap_or_default(),
                    CommonStrings::LongTermCapitalGain => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.long_term_capital_gain"
                        )
                        .unwrap_or_default(),
                    CommonStrings::MeanPlaceholder => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.mean_placeholder"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Mean => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.mean"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Medicare => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.medicare"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Mv => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.mv"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Name => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.name"
                        )
                        .unwrap_or_default(),
                    CommonStrings::NetFlows => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.net_flows"
                        )
                        .unwrap_or_default(),
                    CommonStrings::ObligationSale => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.obligation_sale"
                        )
                        .unwrap_or_default(),
                    CommonStrings::OtherOrdinaryIncome => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.other_ordinary_income"
                        )
                        .unwrap_or_default(),
                    CommonStrings::OutFlow => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.out_flow"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Outlook => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.outlook"
                        )
                        .unwrap_or_default(),
                    CommonStrings::PassiveIncome => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.passive_income"
                        )
                        .unwrap_or_default(),
                    CommonStrings::People => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.people"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Person => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.person"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Price => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.price"
                        )
                        .unwrap_or_default(),
                    CommonStrings::PricePlaceholder => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.price_placeholder"
                        )
                        .unwrap_or_default(),
                    CommonStrings::PrimaryOwner => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.primary_owner"
                        )
                        .unwrap_or_default(),
                    CommonStrings::QtyPlaceholder => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.qty_placeholder"
                        )
                        .unwrap_or_default(),
                    CommonStrings::QualifiedDiv => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.qualified_div"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Quantity => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.quantity"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Rate => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.rate"
                        )
                        .unwrap_or_default(),
                    CommonStrings::RatePlaceholder => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.rate_placeholder"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Random => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.random"
                        )
                        .unwrap_or_default(),
                    CommonStrings::RealAsset => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.real_asset"
                        )
                        .unwrap_or_default(),
                    CommonStrings::RepaidBorrowedFunds => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.repaid_borrowed_funds"
                        )
                        .unwrap_or_default(),
                    CommonStrings::RetirementAge => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.retirement_age"
                        )
                        .unwrap_or_default(),
                    CommonStrings::RmdSales => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.rmd_sales"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Role => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.role"
                        )
                        .unwrap_or_default(),
                    CommonStrings::StartPlaceholder => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.start_placeholder"
                        )
                        .unwrap_or_default(),
                    CommonStrings::StartValue => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.start_value"
                        )
                        .unwrap_or_default(),
                    CommonStrings::SocialSecurity => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.social_security"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Sold => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.sold"
                        )
                        .unwrap_or_default(),
                    CommonStrings::StandardDeduction => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.standard_deduction"
                        )
                        .unwrap_or_default(),
                    CommonStrings::StdDevPlaceholder => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.std_dev_placeholder"
                        )
                        .unwrap_or_default(),
                    CommonStrings::StdDev => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.std_dev"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Symbol => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.symbol"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Target => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.target"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Tax => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.tax"
                        )
                        .unwrap_or_default(),
                    CommonStrings::TaxBasis => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.tax_basis"
                        )
                        .unwrap_or_default(),
                    CommonStrings::TaxBill => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.tax_bill"
                        )
                        .unwrap_or_default(),
                    CommonStrings::TaxableDistributions => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.taxable_distributions"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Taxes => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.taxes"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Total => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.total"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Totals => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.totals"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Toward => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.toward"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Type => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.type"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Ugl => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.ugl"
                        )
                        .unwrap_or_default(),
                    CommonStrings::UnqualifiedDiv => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.unqualified_div"
                        )
                        .unwrap_or_default(),
                    CommonStrings::UsTaxes => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.us_taxes"
                        )
                        .unwrap_or_default(),
                    CommonStrings::User => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.user"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Value => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.value"
                        )
                        .unwrap_or_default(),
                    CommonStrings::ValuePlaceholder => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.value_placeholder"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Worth => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.worth"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Worths => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.worths"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Year => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.year"
                        )
                        .unwrap_or_default(),
                    CommonStrings::YearPlaceholder => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.year_placeholder"
                        )
                        .unwrap_or_default(),
                    CommonStrings::YearRange => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.year_range"
                        )
                        .unwrap_or_default(),
                    CommonStrings::Zoom => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "common_strings.zoom"
                        )
                        .unwrap_or_default(),
                },
            }
        )
    }
}

// α <mod-def i18n_enum_display>
// ω <mod-def i18n_enum_display>
