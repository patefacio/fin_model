//! Top library module for crate fin_model
//!
//! Modeled data.

////////////////////////////////////////////////////////////////////////////////////
// --- macro-use imports ---
////////////////////////////////////////////////////////////////////////////////////
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate strum_macros;

////////////////////////////////////////////////////////////////////////////////////
// --- mod decls ---
////////////////////////////////////////////////////////////////////////////////////
pub mod account;
pub mod age_assumptions;
pub mod balance_sheet;
pub mod core;
pub mod core_enums;
pub mod distributions;
pub mod growth;
pub mod worth;

// α <mod-def lib>
// ω <mod-def lib>