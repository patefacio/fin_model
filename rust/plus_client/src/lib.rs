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
pub use self::component::app::app_center_component::AppCenterComponent;
pub use self::component::app::app_component::AppComponent;
pub use self::component::app::app_nav_bar::AppNavBar;
pub use self::component::app::app_side_bar::AppSideBar;
pub use self::component::app::error_display_component::ErrorDisplayComponent;
pub use self::component::core::collapsible_component::CollapsibleComponent;
pub use self::component::core::collection_grid_component::CollectionGrid;
pub use self::component::core::collection_grid_component::CollectionGridComponent;
pub use self::component::core::collection_grid_component::CollectionGridEditType;
pub use self::component::core::collection_grid_component::CollectionGridState;
pub use self::component::core::core_component_display::CoreComponentDisplay;
pub use self::component::core::core_display::ccd_collection_grid::CcdCollectionGrid;
pub use self::component::core::core_display::ccd_misc::CcdMisc;
pub use self::component::core::core_display::ccd_multi_button::CcdMultiButton;
pub use self::component::core::core_display::ccd_numbers::CcdNumbers;
pub use self::component::core::core_display::ccd_one_of::CcdOneOf;
pub use self::component::core::core_display::ccd_select_lists::CcdSelectLists;
pub use self::component::core::core_display::ccd_years_and_date::CcdYearsAndDate;
pub use self::component::core::core_display::nested_widget_grid::NestedWidgetGrid;
pub use self::component::core::core_display::sample_widget_grid::SampleWidgetGrid;
pub use self::component::core::css_show::CssShow;
pub use self::component::core::currency_select::currency_from_symbol;
pub use self::component::core::currency_select::to_currency_symbol;
pub use self::component::core::currency_select::CurrencySelect;
pub use self::component::core::date_input::DateInput;
pub use self::component::core::dispose_test::DisposeTest;
pub use self::component::core::distribution_cdf_component::DistributionCdfComponent;
pub use self::component::core::distribution_pdf_component::DistributionPdfComponent;
pub use self::component::core::enum_select::EnumSelect;
pub use self::component::core::historic_risk_return_component::HistoricRiskReturnComponent;
pub use self::component::core::integer_input::IntegerInput;
pub use self::component::core::multi_button_select::ConstrainedToggleStates;
pub use self::component::core::multi_button_select::MbsGroupingConstraint;
pub use self::component::core::multi_button_select::MultiButtonSelect;
pub use self::component::core::multi_column_select::InitialValue;
pub use self::component::core::multi_column_select::MultiColumnSelect;
pub use self::component::core::multi_column_select::SelectOption;
pub use self::component::core::normal_loss_component::NormalLossComponent;
pub use self::component::core::normal_spec_component::NormalSpecComponent;
pub use self::component::core::numeric_input::Modification;
pub use self::component::core::numeric_input::NumericInput;
pub use self::component::core::ok_cancel_component::OkCancel;
pub use self::component::core::ok_cancel_component::OkCancelComponent;
pub use self::component::core::one_of_component::OneOfComponent;
pub use self::component::core::percent_input::PercentInput;
pub use self::component::core::slider_with_numeric_input::SliderWithNumericInput;
pub use self::component::core::toggle_image_button::ButtonData;
pub use self::component::core::toggle_image_button::ToggleImageButton;
pub use self::component::core::toggle_image_button::ToggleState;
pub use self::component::core::year_currency_value_input::YearCurrencyValueInput;
pub use self::component::core::year_input::YearInput;
pub use self::component::core::year_range_input::YearRangeInput;
pub use self::component::core::year_value_input::YearValueInput;
pub use self::component::core::year_value_series_component::YearValueSeriesComponent;
pub use self::component::core::year_value_series_component::YearValueSeriesType;
pub use self::component::CssClasses;
pub use self::context::app_context::AppContext;
pub use self::enums::SelectDirection;
pub use self::enums::ViewSide;
pub use self::utils::distribution_cdf::DistributionCdf;
pub use self::utils::distribution_pdf::DistributionPdf;
pub use self::utils::historic_risk_return::HistoricRiskReturn;
pub use self::utils::historic_risk_return::HistoricRiskReturnPlot;
pub use self::utils::html_tag::HtmlTag;
pub use self::utils::integer_clamp::IntegerClamp;
pub use self::utils::live_parsed_date::LiveParsedDate;
pub use self::utils::numeric_text::LenientFormatted;
pub use self::utils::parsed_num::ParsedNum;
pub use self::utils::plot_data::RateCurveData;
pub use self::utils::updatable::Updatable;
pub use plus_utils::commify_int;
pub use plus_utils::scale_by;

////////////////////////////////////////////////////////////////////////////////////
// --- mod decls ---
////////////////////////////////////////////////////////////////////////////////////
pub mod component;
pub mod context;
pub mod enums;
#[cfg(feature = "ssr")]
pub mod file_serve;
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
      let mut tracing_config =
      tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::new()
        .set_max_level(tracing::Level::INFO)
        .build()
      );

      leptos::mount_to_body(move || {
          view! { <AppComponent/> }
      });
    }
}
}

// ω <mod-def lib>
