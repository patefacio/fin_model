//! Top module
#![feature(variant_count)]

////////////////////////////////////////////////////////////////////////////////////
// --- pub module uses ---
////////////////////////////////////////////////////////////////////////////////////
pub use self::currency_exchange::CurrencyExchange;
pub use self::currency_exchange::WEB_CURRENCY_EXCHANGE;
pub use self::currency_value::CurrencyValue;
pub use self::i18n_enum_display::CommonStrings;
pub use self::i18n_enum_display::I18nEnums;
pub use self::system_defaults::SystemDefaults;
pub use self::system_defaults::SYSTEM_DEFAULTS;
pub use plus_modeled::LangSelector;
pub use plus_utils::SystemUnicodes;

////////////////////////////////////////////////////////////////////////////////////
// --- mod decls ---
////////////////////////////////////////////////////////////////////////////////////
pub mod currency_exchange;
pub mod currency_value;
pub mod i18n;
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
