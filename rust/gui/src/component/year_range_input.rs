//! Module for year_range_input leptos function/component

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
/// Models a _start_ and _end_ year.
///
///   * **cx** - Context
///   * **updatable** - The [YearRange] being edited
///   * _return_ - View for year_range_input
#[component]
pub fn YearRangeInput(
    /// Context
    cx: Scope,
    /// The [YearRange] being edited
    updatable: Updatable<Option<YearRange>>,
) -> impl IntoView {
    // α <fn year_range_input>
    view! {
        cx,
        <h3>"TODO Year Range"</h3>
    }
    // ω <fn year_range_input>
}

// α <mod-def year_range_input>
// ω <mod-def year_range_input>
