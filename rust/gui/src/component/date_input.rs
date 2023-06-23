//! Module for date_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::Date;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a date of format _MM/DD/YYYY_.
///
///   * **cx** - Context
///   * **updatable** - The [Date] being edited
///   * _return_ - View for date_input
#[component]
pub fn DateInput(
    /// Context
    cx: Scope,
    /// The [Date] being edited
    updatable: Updatable<Option<Date>>,
) -> impl IntoView {
    // α <fn date_input>
    todo!("Implement `date_input`")
    // ω <fn date_input>
}

// α <mod-def date_input>
// ω <mod-def date_input>
