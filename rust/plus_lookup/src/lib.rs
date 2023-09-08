//! Top module

////////////////////////////////////////////////////////////////////////////////////
// --- pub module uses ---
////////////////////////////////////////////////////////////////////////////////////
pub use i18n_enum_display::I18nEnums;
pub use plus_utils::SystemUnicodes;
pub use system_lookup::SystemLookup;

////////////////////////////////////////////////////////////////////////////////////
// --- mod decls ---
////////////////////////////////////////////////////////////////////////////////////
pub mod generated_system_lookup;
pub mod i18n_enum_display;
pub mod system_lookup;

// α <mod-def lib>
use fluent_templates::static_loader;
pub use generated_system_lookup::GENERATED_SYSTEM_LOOKUP;
pub use plus_modeled::FLOW_DIRECTIONS_MAP;
use unic_langid::{langid, LanguageIdentifier};

pub(crate) static US_ENGLISH: LanguageIdentifier = langid!("en-US");
pub(crate) static FRENCH: LanguageIdentifier = langid!("fr-FR");
pub(crate) static GERMAN: LanguageIdentifier = langid!("de");

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
