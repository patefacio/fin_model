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
pub use self::constants::DEFAULT_DEATH_AGE;
pub use self::constants::DEFAULT_RETIREMENT_AGE;
pub use crate::core::dossier_item_index::ItemIndex;
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
pub use crate::core_enums::LangSelector;
pub use crate::core_enums::NamedRateCurve;
pub use crate::core_enums::PersonType;
pub use crate::core_enums::ReturnStatsType;
pub use crate::core_enums::StateOfResidence;
pub use crate::core_enums::TaxTreatment;
pub use crate::core_enums::TaxUsCategory;
pub use crate::core_enums::TaxUsFilingStatus;
pub use crate::core_enums::WorthType;
pub use crate::core_enums::YearEndpoint;
pub use plus_utils::SystemUnicodes;

////////////////////////////////////////////////////////////////////////////////////
// --- mod decls ---
////////////////////////////////////////////////////////////////////////////////////
pub mod constants;
pub mod core;
pub mod core_display;
pub mod core_enums;
pub mod currency_impl;

// α <mod-def lib>

pub struct Dossier{}

// ω <mod-def lib>
