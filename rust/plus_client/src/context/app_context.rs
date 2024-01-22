//! Context shared throughout the application editor

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::ToggleState;
use leptos::ReadSignal;
use leptos::RwSignal;
use leptos::WriteSignal;
use plus_lookup::CurrencyExchange;
use plus_modeled::Currency;
use plus_modeled::LangSelector;
use std::sync::Arc;

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
    /// Selector of the display currency
    pub display_currency: RwSignal<Currency>,
    /// Number of active nested grid component edits.
    /// When more than 1 level only the inner-most `ok/cancel` is displayed.
    pub grid_edit_active_count: RwSignal<u32>,
    /// Provides ability to convert values between currencies
    pub currency_exchange: RwSignal<Arc<CurrencyExchange>>,
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl AppContext {
    /// Create new instance of AppContext
    ///
    ///   * **lang_selector** - Selector of language for display
    ///   * **display_currency** - Selector of the display currency
    ///   * **grid_edit_active_count** - Number of active nested grid component edits.
    /// When more than 1 level only the inner-most `ok/cancel` is displayed.
    ///   * **currency_exchange** - Provides ability to convert values between currencies
    ///   * _return_ - The new instance
    pub fn new(
        lang_selector: RwSignal<LangSelector>,
        display_currency: RwSignal<Currency>,
        grid_edit_active_count: RwSignal<u32>,
        currency_exchange: RwSignal<Arc<CurrencyExchange>>,
    ) -> AppContext {
        AppContext {
            lang_selector,
            display_currency,
            grid_edit_active_count,
            currency_exchange,
        }
    }
}

// α <mod-def app_context>

// ω <mod-def app_context>
