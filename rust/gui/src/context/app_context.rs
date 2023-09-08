//! Context shared throughout the application editor

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::RwSignal;
use plus_modeled::LangSelector;

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Contains context required by many components like:
///  - Language To use for display text
///  - GridEditActiveCount - To support disabling `ok/cancel` components when
///    they nest
#[derive(Debug, Clone, Copy)]
pub struct AppContext {
    /// Selector of language for display
    pub lang_selector: RwSignal<LangSelector>,
    /// Number of active nested grid component edits.
    /// When more than 1 level only the inner-most `ok/cancel` is displayed.
    pub grid_edit_active_count: RwSignal<u32>,
}

// α <mod-def app_context>
// ω <mod-def app_context>
