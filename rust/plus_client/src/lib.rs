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
pub use component::account_component::AccountComponent;
pub use component::accounts_grid::AccountSharedContext;
pub use component::accounts_grid::AccountsGrid;
pub use component::app_center_component::AppCenterComponent;
pub use component::app_component::AppComponent;
pub use component::app_nav_bar::AppNavBar;
pub use component::app_side_bar::AppSideBar;
pub use component::bond_spec_component::BondSpecComponent;
pub use component::collapsible_component::CollapsibleComponent;
pub use component::collection_grid_component::CollectionGrid;
pub use component::collection_grid_component::CollectionGridComponent;
pub use component::collection_grid_component::CollectionGridEditType;
pub use component::collection_grid_component::CollectionGridState;
pub use component::component_display_component::ComponentDisplayComponent;
pub use component::currency_select::currency_from_symbol;
pub use component::currency_select::to_currency_symbol;
pub use component::currency_select::CurrencySelect;
pub use component::currency_value_input::CurrencyValueInput;
pub use component::date_input::DateInput;
pub use component::dispose_test::DisposeTest;
pub use component::distribution_cdf_component::DistributionCdfComponent;
pub use component::distribution_pdf_component::DistributionPdfComponent;
pub use component::distribution_policy_component::DistributionPolicyComponent;
pub use component::distribution_spec_component::DistributionSpecComponent;
pub use component::dossier_component::DossierComponent;
pub use component::dossier_correlation_entry_component::DossierCorrelationEntryComponent;
pub use component::dossier_correlation_matrix_component::DossierCorrelationMatrixComponent;
pub use component::dossier_holding_index_input::DossierHoldingIndexInput;
pub use component::dossier_item_index_component::DossierItemIndexComponent;
pub use component::enum_select::EnumSelect;
pub use component::expandable_rate_component::ExpandableRateComponent;
pub use component::growth_assumption_component::GrowthAssumptionComponent;
pub use component::historic_risk_return_component::HistoricRiskReturnComponent;
pub use component::holding_component::HoldingComponent;
pub use component::holdings_grid::HoldingSharedContext;
pub use component::holdings_grid::HoldingsGrid;
pub use component::integer_input::IntegerInput;
pub use component::item_growth_component::ItemGrowthComponent;
pub use component::item_growth_select::ItemGrowthSelect;
pub use component::multi_column_select::InitialValue;
pub use component::multi_column_select::MultiColumnSelect;
pub use component::multi_column_select::SelectDirection;
pub use component::multi_column_select::SelectOption;
pub use component::normal_loss_component::NormalLossComponent;
pub use component::normal_spec_component::NormalSpecComponent;
pub use component::numeric_input::Modification;
pub use component::numeric_input::NumericInput;
pub use component::ok_cancel_component::OkCancel;
pub use component::ok_cancel_component::OkCancelComponent;
pub use component::one_of_component::OneOfComponent;
pub use component::percent_input::PercentInput;
pub use component::symbol_input::SymbolInput;
pub use component::year_currency_value_input::YearCurrencyValueInput;
pub use component::year_input::YearInput;
pub use component::year_range_input::YearRangeInput;
pub use component::year_value_input::YearValueInput;
pub use component::year_value_series_component::YearValueSeriesComponent;
pub use component::year_value_series_component::YearValueSeriesType;
pub use component::SymbolGrowthMap;
pub use context::app_context::AppContext;
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

      leptos_dom::log!("HYDRATION HAPPENING");
      console_error_panic_hook::set_once();

      tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .without_time()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_writer(tracing_subscriber_wasm::MakeConsoleWriter::default())
        .with_ansi(false)
        .compact()
        .pretty()
        .finish()
        .init();

      tracing::debug!("What is going on?");

      leptos::mount_to_body(move || {
          view! { <AppComponent/> }
      });
    }
}
}

// ω <mod-def lib>
