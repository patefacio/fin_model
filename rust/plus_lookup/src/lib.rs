//! Top module

////////////////////////////////////////////////////////////////////////////////////
// --- pub module uses ---
////////////////////////////////////////////////////////////////////////////////////
pub use i18n_component_display::I18nAccountComponent;
pub use i18n_component_display::I18nAccountsGrid;
pub use i18n_component_display::I18nAgeAssumptionsComponent;
pub use i18n_component_display::I18nBondSpecComponent;
pub use i18n_component_display::I18nDistributionPolicyComponent;
pub use i18n_component_display::I18nDistributionSpecComponent;
pub use i18n_component_display::I18nDossierComponent;
pub use i18n_component_display::I18nFlowSpecComponent;
pub use i18n_component_display::I18nHoldingComponent;
pub use i18n_component_display::I18nHoldingsGrid;
pub use i18n_component_display::I18nOkCancelComponent;
pub use i18n_component_display::I18nPersonComponent;
pub use i18n_component_display::I18nPersonsGrid;
pub use i18n_component_display::I18nWorthComponent;
pub use i18n_component_display::I18nWorthsGrid;
pub use i18n_enum_display::I18nEnums;
pub use plus_modeled::LangSelector;
pub use plus_utils::SystemUnicodes;
pub use system_lookup::SystemLookup;

////////////////////////////////////////////////////////////////////////////////////
// --- mod decls ---
////////////////////////////////////////////////////////////////////////////////////
pub mod generated_system_lookup;
pub mod i18n_component_display;
pub mod i18n_enum_display;
pub mod system_lookup;

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
pub use generated_system_lookup::GENERATED_SYSTEM_LOOKUP;
pub use plus_modeled::FLOW_DIRECTIONS_MAP;
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
