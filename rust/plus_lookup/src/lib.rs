//! Top module
#![feature(variant_count)]

////////////////////////////////////////////////////////////////////////////////////
// --- pub module uses ---
////////////////////////////////////////////////////////////////////////////////////
pub use self::currency_exchange::CurrencyExchange;
pub use self::currency_exchange::WEB_CURRENCY_EXCHANGE;
pub use self::currency_value::CurrencyValue;
pub use self::i18n_component_display::I18nAccountComponent;
pub use self::i18n_component_display::I18nAccountsGrid;
pub use self::i18n_component_display::I18nAgeAssumptionsComponent;
pub use self::i18n_component_display::I18nAssetLiabilityTimeline;
pub use self::i18n_component_display::I18nBalanceSheetStatement;
pub use self::i18n_component_display::I18nBondSpecComponent;
pub use self::i18n_component_display::I18nCashFlowStatement;
pub use self::i18n_component_display::I18nCashFlowTimeline;
pub use self::i18n_component_display::I18nDistributionPolicyComponent;
pub use self::i18n_component_display::I18nDistributionSpecComponent;
pub use self::i18n_component_display::I18nDossierEditor;
pub use self::i18n_component_display::I18nDossierResolvedView;
pub use self::i18n_component_display::I18nFlowSpecComponent;
pub use self::i18n_component_display::I18nFlowSpecsGrid;
pub use self::i18n_component_display::I18nForecastConfigComponent;
pub use self::i18n_component_display::I18nForecastIdSelector;
pub use self::i18n_component_display::I18nForecastSummaryComponent;
pub use self::i18n_component_display::I18nForecasterComponent;
pub use self::i18n_component_display::I18nGrowingFlowSpecComponent;
pub use self::i18n_component_display::I18nGrowthAssumptionComponent;
pub use self::i18n_component_display::I18nHistoricRiskReturnComponent;
pub use self::i18n_component_display::I18nHoldingComponent;
pub use self::i18n_component_display::I18nHoldingsGrid;
pub use self::i18n_component_display::I18nInvestmentPlanView;
pub use self::i18n_component_display::I18nItemGrowthComponent;
pub use self::i18n_component_display::I18nMatrixResolvedView;
pub use self::i18n_component_display::I18nMonteConfigComponent;
pub use self::i18n_component_display::I18nMonteSimulationSummaryComponent;
pub use self::i18n_component_display::I18nMonteSimulatorComponent;
pub use self::i18n_component_display::I18nNormalLossComponent;
pub use self::i18n_component_display::I18nNormalSpecComponent;
pub use self::i18n_component_display::I18nOkCancelComponent;
pub use self::i18n_component_display::I18nPersonComponent;
pub use self::i18n_component_display::I18nPersonsGrid;
pub use self::i18n_component_display::I18nSalesPlanView;
pub use self::i18n_component_display::I18nSymbolInput;
pub use self::i18n_component_display::I18nTaxDeterminantsComponent;
pub use self::i18n_component_display::I18nTimelineGroupComponent;
pub use self::i18n_component_display::I18nUsTaxStatementView;
pub use self::i18n_component_display::I18nWorthComponent;
pub use self::i18n_component_display::I18nWorthsGrid;
pub use self::i18n_component_display::I18nYearCurrencyValueInput;
pub use self::i18n_component_display::I18nYearRangeInput;
pub use self::i18n_component_display::I18nYearValueSeriesComponent;
pub use self::i18n_enum_display::CommonStrings;
pub use self::i18n_enum_display::I18nEnums;
pub use self::system_defaults::SystemDefaults;
pub use plus_modeled::LangSelector;
pub use plus_utils::SystemUnicodes;

////////////////////////////////////////////////////////////////////////////////////
// --- mod decls ---
////////////////////////////////////////////////////////////////////////////////////
pub mod currency_exchange;
pub mod currency_value;
pub mod i18n_component_display;
pub mod i18n_enum_display;
pub mod system_defaults;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Get the language identifier from selector.
///
///   * **lang_selector** - The language selector
///   * _return_ - Reference to the language identifier
#[inline]
pub(crate) fn lang_selector_to_language_id<'a>(
    lang_selector: &LangSelector,
) -> &'a LanguageIdentifier {
    // α <fn lang_selector_to_language_id>
    match lang_selector {
        LangSelector::UsEnglish => &US_ENGLISH,
        LangSelector::French => &FRENCH,
        LangSelector::German => &GERMAN,
    }
    // ω <fn lang_selector_to_language_id>
}

// α <mod-def lib>
use fluent_templates::static_loader;
use unic_langid::{langid, LanguageIdentifier};

pub(crate) static US_ENGLISH: LanguageIdentifier = langid!("en-US");
pub(crate) static FRENCH: LanguageIdentifier = langid!("fr-FR");
pub(crate) static GERMAN: LanguageIdentifier = langid!("de-DE");

static_loader! {
    pub(crate) static LOCALES = {
        locales: "./i18n",
        fallback_language: "en-US",
        // Removes unicode isolating marks around arguments, you typically
        // should only set to false when testing.
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

// ω <mod-def lib>
