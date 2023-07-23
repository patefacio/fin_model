//! The root module
#![feature(variant_count)]

////////////////////////////////////////////////////////////////////////////////////
// --- pub module uses ---
////////////////////////////////////////////////////////////////////////////////////
pub use component::collection_grid_component::CollectionGrid;
pub use component::collection_grid_component::CollectionGridComponent;
pub use component::component_display_component::ComponentDisplayComponent;
pub use component::currency_select::currency_from_symbol;
pub use component::currency_select::to_currency_symbol;
pub use component::currency_select::CurrencySelect;
pub use component::currency_value_input::CurrencyValueInput;
pub use component::date_input::DateInput;
pub use component::dispose_test::DisposeTest;
pub use component::distribution_cdf_component::DistributionCdfComponent;
pub use component::distribution_pdf_component::DistributionPdfComponent;
pub use component::dossier_correlation_entry_component::DossierCorrelationEntryComponent;
pub use component::dossier_correlation_matrix_component::DossierCorrelationMatrixComponent;
pub use component::dossier_holding_index_input::DossierHoldingIndexInput;
pub use component::dossier_item_index_component::DossierItemIndexComponent;
pub use component::enum_select::EnumSelect;
pub use component::historic_risk_return_component::HistoricRiskReturnComponent;
pub use component::holding_component::HoldingComponent;
pub use component::integer_input::IntegerInput;
pub use component::multi_column_select::InitialValue;
pub use component::multi_column_select::MultiColumnSelect;
pub use component::multi_column_select::SelectDirection;
pub use component::multi_column_select::SelectOption;
pub use component::normal_spec_component::NormalSpecComponent;
pub use component::numeric_input::Modification;
pub use component::numeric_input::NumericInput;
pub use component::ok_cancel_component::OkCancel;
pub use component::ok_cancel_component::OkCancelComponent;
pub use component::percent_input::PercentInput;
pub use component::rate_curve_component::RateCurveComponent;
pub use component::symbol_input::SymbolInput;
pub use component::value_flow_spec_component::ValueFlowSpecComponent;
pub use component::year_currency_value_input::YearCurrencyValueInput;
pub use component::year_input::YearInput;
pub use component::year_range_input::YearRangeInput;
pub use component::year_value_input::YearValueInput;
pub use utils::distribution_cdf::DistributionCdf;
pub use utils::distribution_pdf::DistributionPdf;
pub use utils::historic_risk_return::HistoricRiskReturn;
pub use utils::historic_risk_return::HistoricRiskReturnPlot;
pub use utils::html_tag::HtmlTag;
pub use utils::integer_clamp::IntegerClamp;
pub use utils::leptos_helpers::make_shared_data;
pub use utils::leptos_helpers::SharedData;
pub use utils::live_parsed_date::LiveParsedDate;
pub use utils::parsed_num::ParsedNum;
pub use utils::scale_by::scale_by;
pub use utils::updatable::Updatable;

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

pub use component::table_component::Table;
pub use component::table_component::TableComponent;

use cfg_if::cfg_if;
use num_format::ToFormattedStr;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;
      use leptos::*;
      use tracing::field::debug;
      use tracing_subscriber::util::SubscriberInitExt;

      leptos::log!("HYDRATION HAPPENING");
      console_error_panic_hook::set_once();

      tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .without_time()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_writer(tracing_subscriber_wasm::MakeConsoleWriter::default())
        .with_ansi(false)
        .pretty()
        .finish()
        .init();

      tracing::debug!("What is going on?");

      leptos::mount_to_body(move |cx| {
          view! { cx, <App/> }
      });
    }
}
}

// ω <mod-def lib>
