//! Support for internationalization of enum labels.

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::{FRENCH, GERMAN, LOCALES, US_ENGLISH};
use core::fmt::Display;
use core::fmt::Formatter;
use fluent_templates::Loader;
use plus_modeled::AccountType;
use plus_modeled::BasicAllocationType;
use plus_modeled::Country;
use plus_modeled::Currency;
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
use plus_modeled::StateOfResidence;
use plus_modeled::TaxTreatment;
use plus_modeled::TaxUsCategory;
use plus_modeled::TaxUsFilingStatus;
use plus_modeled::WorthType;
use unic_langid::lang;
use unic_langid::LanguageIdentifier;

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// Enumeration of all protobuf enums
#[derive(Debug, Copy, Clone)]
pub enum I18nEnums<'a> {
    /// Enumerates supported enums and implements display to dispatch on language and value
    LangSelector(LangSelector, &'a LangSelector),
    /// Enumerates supported enums and implements display to dispatch on language and value
    Currency(LangSelector, &'a Currency),
    /// Enumerates supported enums and implements display to dispatch on language and value
    AccountType(LangSelector, &'a AccountType),
    /// Enumerates supported enums and implements display to dispatch on language and value
    BasicAllocationType(LangSelector, &'a BasicAllocationType),
    /// Enumerates supported enums and implements display to dispatch on language and value
    DistributionInstrument(LangSelector, &'a DistributionInstrument),
    /// Enumerates supported enums and implements display to dispatch on language and value
    DistributionInstrumentType(LangSelector, &'a DistributionInstrumentType),
    /// Enumerates supported enums and implements display to dispatch on language and value
    ForecastTaxTreatment(LangSelector, &'a ForecastTaxTreatment),
    /// Enumerates supported enums and implements display to dispatch on language and value
    FlowDirection(LangSelector, &'a FlowDirection),
    /// Enumerates supported enums and implements display to dispatch on language and value
    PersonType(LangSelector, &'a PersonType),
    /// Enumerates supported enums and implements display to dispatch on language and value
    DossierItemType(LangSelector, &'a DossierItemType),
    /// Enumerates supported enums and implements display to dispatch on language and value
    ForecastYearMarkerType(LangSelector, &'a ForecastYearMarkerType),
    /// Enumerates supported enums and implements display to dispatch on language and value
    TaxUsFilingStatus(LangSelector, &'a TaxUsFilingStatus),
    /// Enumerates supported enums and implements display to dispatch on language and value
    TaxUsCategory(LangSelector, &'a TaxUsCategory),
    /// Enumerates supported enums and implements display to dispatch on language and value
    TaxTreatment(LangSelector, &'a TaxTreatment),
    /// Enumerates supported enums and implements display to dispatch on language and value
    ForecastSortCriteria(LangSelector, &'a ForecastSortCriteria),
    /// Enumerates supported enums and implements display to dispatch on language and value
    Country(LangSelector, &'a Country),
    /// Enumerates supported enums and implements display to dispatch on language and value
    StateOfResidence(LangSelector, &'a StateOfResidence),
    /// Enumerates supported enums and implements display to dispatch on language and value
    NamedRateCurve(LangSelector, &'a NamedRateCurve),
    /// Enumerates supported enums and implements display to dispatch on language and value
    WorthType(LangSelector, &'a WorthType),
    /// Enumerates supported enums and implements display to dispatch on language and value
    HoldingType(LangSelector, &'a HoldingType),
    /// Enumerates supported enums and implements display to dispatch on language and value
    FlowType(LangSelector, &'a FlowType),
}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Get the language identifier from selector.
///
///   * **lang_selector** - The language selector
///   * _return_ - Reference to the language identifier
#[inline]
pub fn lang_selector_to_language_id<'a>(lang_selector: &LangSelector) -> &'a LanguageIdentifier {
    // α <fn lang_selector_to_language_id>
    match lang_selector {
        LangSelector::UsEnglish => &US_ENGLISH,
        LangSelector::French => &FRENCH,
        LangSelector::German => &GERMAN,
    }
    // ω <fn lang_selector_to_language_id>
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl<'a> Display for I18nEnums<'a> {
    /// Format the instance.
    ///
    ///   * **f** - Formatter to push formatted item to.
    ///   * _return_ - Formatted instance
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
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
                I18nEnums::Currency(lang_selector, e) => match e {
                    Currency::Usd => LOCALES
                        .lookup(lang_selector_to_language_id(lang_selector), "currency.usd")
                        .unwrap_or_default(),
                    Currency::Eur => LOCALES
                        .lookup(lang_selector_to_language_id(lang_selector), "currency.eur")
                        .unwrap_or_default(),
                    Currency::Jpy => LOCALES
                        .lookup(lang_selector_to_language_id(lang_selector), "currency.jpy")
                        .unwrap_or_default(),
                    Currency::Gbp => LOCALES
                        .lookup(lang_selector_to_language_id(lang_selector), "currency.gbp")
                        .unwrap_or_default(),
                    Currency::Aud => LOCALES
                        .lookup(lang_selector_to_language_id(lang_selector), "currency.aud")
                        .unwrap_or_default(),
                    Currency::Cad => LOCALES
                        .lookup(lang_selector_to_language_id(lang_selector), "currency.cad")
                        .unwrap_or_default(),
                    Currency::Chf => LOCALES
                        .lookup(lang_selector_to_language_id(lang_selector), "currency.chf")
                        .unwrap_or_default(),
                    Currency::Cny => LOCALES
                        .lookup(lang_selector_to_language_id(lang_selector), "currency.cny")
                        .unwrap_or_default(),
                    Currency::Hkd => LOCALES
                        .lookup(lang_selector_to_language_id(lang_selector), "currency.hkd")
                        .unwrap_or_default(),
                    Currency::Nzd => LOCALES
                        .lookup(lang_selector_to_language_id(lang_selector), "currency.nzd")
                        .unwrap_or_default(),
                    Currency::Crc => LOCALES
                        .lookup(lang_selector_to_language_id(lang_selector), "currency.crc")
                        .unwrap_or_default(),
                    Currency::Rub => LOCALES
                        .lookup(lang_selector_to_language_id(lang_selector), "currency.rub")
                        .unwrap_or_default(),
                    Currency::Krw => LOCALES
                        .lookup(lang_selector_to_language_id(lang_selector), "currency.krw")
                        .unwrap_or_default(),
                    Currency::Sek => LOCALES
                        .lookup(lang_selector_to_language_id(lang_selector), "currency.sek")
                        .unwrap_or_default(),
                },
                I18nEnums::AccountType(lang_selector, e) => match e {
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
                    AccountType::Taxable => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "account_type.taxable"
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
                    DossierItemType::Instrument => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "dossier_item_type.instrument"
                        )
                        .unwrap_or_default(),
                    DossierItemType::Flow => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "dossier_item_type.flow"
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
                I18nEnums::ForecastSortCriteria(lang_selector, e) => match e {
                    ForecastSortCriteria::ByFinalEndBalance => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "forecast_sort_criteria.by_final_end_balance"
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
                I18nEnums::StateOfResidence(lang_selector, e) => match e {
                    StateOfResidence::None => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.none"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Al => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.al"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Ak => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.ak"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Az => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.az"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Ar => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.ar"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Ca => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.ca"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Co => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.co"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Ct => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.ct"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::De => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.de"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Fl => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.fl"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Ga => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.ga"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Hi => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.hi"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Id => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.id"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Il => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.il"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::In => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.in"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Ia => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.ia"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Ks => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.ks"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Ky => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.ky"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::La => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.la"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Me => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.me"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Md => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.md"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Ma => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.ma"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Mi => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.mi"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Mn => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.mn"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Ms => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.ms"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Mo => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.mo"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Mt => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.mt"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Ne => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.ne"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Nv => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.nv"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Nh => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.nh"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Nj => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.nj"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Nm => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.nm"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Ny => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.ny"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Nc => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.nc"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Nd => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.nd"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Oh => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.oh"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Ok => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.ok"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Or => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.or"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Pa => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.pa"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Ri => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.ri"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Sc => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.sc"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Sd => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.sd"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Tn => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.tn"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Tx => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.tx"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Ut => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.ut"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Vt => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.vt"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Va => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.va"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Wa => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.wa"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Wv => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.wv"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Wi => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.wi"
                        )
                        .unwrap_or_default(),
                    StateOfResidence::Wy => LOCALES
                        .lookup(
                            lang_selector_to_language_id(lang_selector),
                            "state_of_residence.wy"
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
                },
            }
        )
    }
}

// α <mod-def i18n_enum_display>
// ω <mod-def i18n_enum_display>
