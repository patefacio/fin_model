//! A collection of components for data entry modeling a financial dossier.

////////////////////////////////////////////////////////////////////////////////////
// --- mod decls ---
////////////////////////////////////////////////////////////////////////////////////
pub mod bond_spec_component;
pub mod collection_grid_component;
pub mod collection_grid_impls;
pub mod component_display_component;
pub mod currency_select;
pub mod currency_value_input;
pub mod date_input;
pub mod dispose_test;
pub mod distribution_cdf_component;
pub mod distribution_pdf_component;
pub mod distribution_spec_component;
pub mod dossier_correlation_entry_component;
pub mod dossier_correlation_matrix_component;
pub mod dossier_holding_index_input;
pub mod dossier_item_index_component;
pub mod enum_select;
pub mod expandable_rate_component;
pub mod growth_assumption_component;
pub mod historic_risk_return_component;
pub mod holding_component;
pub mod integer_input;
pub mod item_growth_component;
pub mod item_growth_select;
pub mod multi_column_select;
pub mod normal_loss_component;
pub mod normal_spec_component;
pub mod numeric_input;
pub mod ok_cancel_component;
pub mod percent_input;
pub mod rate_curve_component;
pub mod symbol_input;
pub mod value_flow_spec_component;
pub mod year_currency_value_input;
pub mod year_input;
pub mod year_range_input;
pub mod year_value_input;

////////////////////////////////////////////////////////////////////////////////////
// --- type aliases ---
////////////////////////////////////////////////////////////////////////////////////
pub type SymbolGrowthMap = std::collections::HashMap<String, crate::ItemGrowth>;

// α <mod-def component>
// ω <mod-def component>
