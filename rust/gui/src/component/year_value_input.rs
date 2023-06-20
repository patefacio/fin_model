//! Module for year_value_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::YearRange;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a _year_ combined with a _value_.
///
///   * **cx** - Context
///   * **updatable** - The [YearValue] being edited
///   * _return_ - View for year_value_input
#[component]
pub fn YearValueInput(
    /// Context
    cx: Scope,
    /// The [YearValue] being edited
    updatable: Updatable<Option<YearRange>>,
) -> impl IntoView {
    // α <fn year_value_input>
    view! {
        cx,
        <h3>"TODO YearValue"</h3>
    }
    // ω <fn year_value_input>
}

// α <mod-def year_value_input>
// ω <mod-def year_value_input>
