//! Components for displaying core components by feature

////////////////////////////////////////////////////////////////////////////////////
// --- mod decls ---
////////////////////////////////////////////////////////////////////////////////////
pub mod ccd_misc;
pub mod ccd_multi_button;
pub mod ccd_numbers;
pub mod ccd_one_of;
pub mod ccd_select_lists;
pub mod ccd_years_and_date;

// α <mod-def core_display>

use plus_lookup::LangSelector;

fn prefix_lang_flag(lang_selector: LangSelector, s: &str) -> String {
    const GERMAN_FLAG: &str = "🇩🇪";
    const FRENCH_FLAG: &str = "🇫🇷";
    let flag = match lang_selector {
        LangSelector::French => FRENCH_FLAG,
        LangSelector::German => GERMAN_FLAG,
        LangSelector::UsEnglish => "",
    };
    format!("{flag} {s}")
}

// ω <mod-def core_display>
