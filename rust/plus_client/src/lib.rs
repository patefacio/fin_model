//! The root module
#![feature(variant_count)]

////////////////////////////////////////////////////////////////////////////////////
// --- macro-use imports ---
////////////////////////////////////////////////////////////////////////////////////
#[macro_use]
extern crate strum_macros;

////////////////////////////////////////////////////////////////////////////////////
// --- pub module uses ---
////////////////////////////////////////////////////////////////////////////////////
pub use component::core::collapsible_component::CollapsibleComponent;
pub use component::core::collection_grid_component::CollectionGrid;
pub use component::core::collection_grid_component::CollectionGridComponent;
pub use component::core::collection_grid_component::CollectionGridEditType;
pub use component::core::collection_grid_component::CollectionGridState;
pub use component::core::currency_select::currency_from_symbol;
pub use component::core::currency_select::to_currency_symbol;
pub use component::core::currency_select::CurrencySelect;
pub use component::core::currency_value_input::CurrencyValueInput;
pub use component::core::date_input::DateInput;
pub use component::core::enum_select::EnumSelect;
pub use component::core::integer_input::IntegerInput;
pub use component::core::multi_column_select::InitialValue;
pub use component::core::multi_column_select::MultiColumnSelect;
pub use component::core::multi_column_select::SelectOption;
pub use component::core::normal_loss_component::NormalLossComponent;
pub use component::core::normal_spec_component::NormalSpecComponent;
pub use component::core::numeric_input::Modification;
pub use component::core::numeric_input::NumericInput;
pub use component::core::ok_cancel_component::OkCancel;
pub use component::core::ok_cancel_component::OkCancelComponent;
pub use component::core::one_of_component::OneOfComponent;
pub use component::core::percent_input::PercentInput;
pub use component::core::year_currency_value_input::YearCurrencyValueInput;
pub use component::core::year_input::YearInput;
pub use component::core::year_range_input::YearRangeInput;
pub use component::core::year_value_input::YearValueInput;
pub use component::core::year_value_series_component::YearValueSeriesComponent;
pub use component::core::year_value_series_component::YearValueSeriesType;
pub use component::dossier::account_component::AccountComponent;
pub use component::dossier::accounts_grid::AccountSharedContext;
pub use component::dossier::accounts_grid::AccountsGrid;
pub use component::dossier::age_assumptions_component::AgeAssumptionsComponent;
pub use component::dossier::app_center_component::AppCenterComponent;
pub use component::dossier::app_component::AppComponent;
pub use component::dossier::app_nav_bar::AppNavBar;
pub use component::dossier::app_side_bar::AppSideBar;
pub use component::dossier::bond_spec_component::BondSpecComponent;
pub use component::dossier::component_display_component::ComponentDisplayComponent;
pub use component::dossier::dispose_test::DisposeTest;
pub use component::dossier::distribution_cdf_component::DistributionCdfComponent;
pub use component::dossier::distribution_pdf_component::DistributionPdfComponent;
pub use component::dossier::distribution_policy_component::DistributionPolicyComponent;
pub use component::dossier::distribution_spec_component::DistributionSpecComponent;
pub use component::dossier::dossier_component::DossierComponent;
pub use component::dossier::expandable_rate_component::ExpandableRateComponent;
pub use component::dossier::growth_assumption_component::GrowthAssumptionComponent;
pub use component::dossier::historic_risk_return_component::HistoricRiskReturnComponent;
pub use component::dossier::holding_component::HoldingComponent;
pub use component::dossier::holdings_grid::HoldingSharedContext;
pub use component::dossier::holdings_grid::HoldingsGrid;
pub use component::dossier::item_growth_component::ItemGrowthComponent;
pub use component::dossier::item_growth_select::ItemGrowthSelect;
pub use component::dossier::person_component::PersonComponent;
pub use component::dossier::person_component::PersonSharedContext;
pub use component::dossier::persons_grid::PersonsGrid;
pub use component::dossier::symbol_input::SymbolInput;
pub use component::SymbolGrowthMap;
pub use context::app_context::AppContext;
pub use enums::SelectDirection;
pub use plus_modeled::ItemGrowth;
pub use plus_utils::commify_int;
pub use plus_utils::scale_by;
pub use utils::distribution_cdf::DistributionCdf;
pub use utils::distribution_pdf::DistributionPdf;
pub use utils::historic_risk_return::HistoricRiskReturn;
pub use utils::historic_risk_return::HistoricRiskReturnPlot;
pub use utils::html_tag::HtmlTag;
pub use utils::integer_clamp::IntegerClamp;
pub use utils::live_parsed_date::LiveParsedDate;
pub use utils::numeric_text::LenientFormatted;
pub use utils::parsed_num::ParsedNum;
pub use utils::plot_data::RateCurveData;
pub use utils::updatable::Updatable;

////////////////////////////////////////////////////////////////////////////////////
// --- mod decls ---
////////////////////////////////////////////////////////////////////////////////////
pub mod component;
pub mod context;
pub mod enums;
pub mod utils;

////////////////////////////////////////////////////////////////////////////////////
// --- type aliases ---
////////////////////////////////////////////////////////////////////////////////////
type Year = u32;

// α <mod-def lib>

use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {

      use crate::AppCenterComponent;
      use leptos::*;
      use tracing::field::debug;
      use tracing_subscriber::util::SubscriberInitExt;

      console_error_panic_hook::set_once();
      tracing_wasm::set_as_global_default();

      tracing::debug!("Tracing debug message - test!");
      tracing::trace!("Tracing tracing message - test!");
      tracing::warn!("Tracing warning message - test!");



      leptos::mount_to_body(move || {
          view! { <AppComponent/> }
      });
    }
}
}

// ω <mod-def lib>
