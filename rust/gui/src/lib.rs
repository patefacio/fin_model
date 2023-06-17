//! The root module
#![feature(variant_count)]

////////////////////////////////////////////////////////////////////////////////////
// --- pub module uses ---
////////////////////////////////////////////////////////////////////////////////////
pub use component::balance_sheet_component::BalanceSheetComponent;
pub use component::currency_select::CurrencySelect;
pub use component::currency_value_input::CurrencyValueInput;
pub use component::dispose_test::DisposeTest;
pub use component::enum_select::EnumSelect;
pub use component::growth_assumption_component::GrowthAssumptionComponent;
pub use component::holding_component::HoldingComponent;
pub use component::item_growth_component::ItemGrowthComponent;
pub use component::multi_column_select::InitialValue;
pub use component::multi_column_select::MultiColumnSelect;
pub use component::multi_column_select::SelectDirection;
pub use component::multi_column_select::SelectOption;
pub use component::normal_spec_component::NormalSpecComponent;
pub use component::numeric_input::Modification;
pub use component::numeric_input::NumericInput;
pub use component::ok_cancel_component::OkCancelComponent;
pub use component::percent_input::PercentInput;
pub use component::rate_curve_component::RateCurveComponent;
pub use component::symbol_input::SymbolInput;
pub use component::worth_component::WorthComponent;
pub use component::year_currency_value_input::YearCurrencyValueInput;
pub use component::year_input::YearInput;
pub use utils::updatable::Updatable;
pub use utils::year_clamp::ParsedNum;
pub use utils::year_clamp::YearClamp;

////////////////////////////////////////////////////////////////////////////////////
// --- mod decls ---
////////////////////////////////////////////////////////////////////////////////////
pub mod app;
pub mod component;
pub mod utils;

////////////////////////////////////////////////////////////////////////////////////
// --- type aliases ---
////////////////////////////////////////////////////////////////////////////////////
type Year = u32;

// α <mod-def lib>

use cfg_if::cfg_if;
use num_format::ToFormattedStr;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;
      use leptos::*;

      // initializes logging using the `log` crate
      _ = console_log::init_with_level(log::Level::Debug);
      console_error_panic_hook::set_once();

      leptos::mount_to_body(move |cx| {
          view! { cx, <App/> }
      });
    }
}
}

// ω <mod-def lib>
