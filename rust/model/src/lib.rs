//! Top library module for crate plus_modeled
//!
//! Modeled data.

////////////////////////////////////////////////////////////////////////////////////
// --- macro-use imports ---
////////////////////////////////////////////////////////////////////////////////////
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate strum_macros;

////////////////////////////////////////////////////////////////////////////////////
// --- pub module uses ---
////////////////////////////////////////////////////////////////////////////////////
pub use crate::account::Account;
pub use crate::account::AccountTreatment;
pub use crate::account::BondSpec;
pub use crate::account::Holding;
pub use crate::account::RequiredMinimumDistribution;
pub use crate::age_assumptions::AgeAssumptions;
pub use crate::balance_sheet::BalanceSheet;
pub use crate::core::CurrencyValue;
pub use crate::core::Date;
pub use crate::core::DbId;
pub use crate::core::DossierCorrelationEntry;
pub use crate::core::DossierCorrelationMatrix;
pub use crate::core::DossierHoldingIndex;
pub use crate::core::DossierItemIndex;
pub use crate::core::NormalSpec;
pub use crate::core::PeriodBalance;
pub use crate::core::RateCurve;
pub use crate::core::YearCurrencyValue;
pub use crate::core::YearRange;
pub use crate::core::YearValue;
pub use crate::core::YearValueSeries;
pub use crate::core_enums::AccountType;
pub use crate::core_enums::BasicAllocationType;
pub use crate::core_enums::Country;
pub use crate::core_enums::Currency;
pub use crate::core_enums::DistributionInstrument;
pub use crate::core_enums::DistributionInstrumentType;
pub use crate::core_enums::DossierItemType;
pub use crate::core_enums::FlowDirection;
pub use crate::core_enums::FlowType;
pub use crate::core_enums::ForecastSortCriteria;
pub use crate::core_enums::ForecastTaxTreatment;
pub use crate::core_enums::ForecastYearMarkerType;
pub use crate::core_enums::HoldingType;
pub use crate::core_enums::NamedRateCurve;
pub use crate::core_enums::PersonType;
pub use crate::core_enums::StateOfResidence;
pub use crate::core_enums::TaxTreatment;
pub use crate::core_enums::TaxUsCategory;
pub use crate::core_enums::TaxUsFilingStatus;
pub use crate::core_enums::WorthType;
pub use crate::distributions::DistributionSpec;
pub use crate::flow_specs::FlowSpec;
pub use crate::flow_specs::GrowingFlowSpec;
pub use crate::flow_specs::HoldingLinks;
pub use crate::flow_specs::ValueFlowSpec;
pub use crate::growth::GrowthAssumption;
pub use crate::growth::GrowthItemMappings;
pub use crate::growth::ItemGrowth;
pub use crate::growth::MarketAssumptions;
pub use crate::growth::OutlookMarketAssumptions;
pub use crate::growth::SystemCorrelationEntry;
pub use crate::growth::SystemCorrelationMatrix;
pub use crate::growth::SystemGrowthId;
pub use crate::person::Person;
pub use crate::worth::Worth;
pub use scale_by::scale_by;
pub use system_unicodes::SystemUnicodes;

////////////////////////////////////////////////////////////////////////////////////
// --- mod decls ---
////////////////////////////////////////////////////////////////////////////////////
pub mod account;
pub mod account_display;
pub mod age_assumptions;
pub mod age_assumptions_display;
pub mod balance_sheet;
pub mod balance_sheet_display;
pub mod core;
pub mod core_display;
pub mod core_enums;
pub mod distributions;
pub mod distributions_display;
pub mod flow_specs;
pub mod flow_specs_display;
pub mod growth;
pub mod growth_display;
pub mod person;
pub mod person_display;
pub mod scale_by;
pub mod system_unicodes;
pub mod worth;
pub mod worth_display;

// α <mod-def lib>
// ω <mod-def lib>
