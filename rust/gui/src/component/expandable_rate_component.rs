//! Module for expandable_rate_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use crate::Year;
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::RateCurve;
use plus_modeled::YearRange;
use plus_modeled::YearValue;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Starts as a single rate entry. User has option to expand entry to
/// a full rate curve. If as single rate, assumes the year is MIN_DATE.
/// MIN_DATE is a system constant that represents the earliest date we deal with.
/// If expanded user enters list of YearValue entries. The expand button
/// turns into a collapse button when expanded. If user tries to collapse without
/// any entries it simply collapses. If they collapse with 1 entry it collapses
/// with that one entry and keeps the date they've entered. If they collapse with
/// more than one entry the component throws up a popup dialog warning that
/// all entries except the first will be discarded.
///
///   * **cx** - Context
///   * **updatable** - The [RateCurve] being edited
///   * **year_range** - Range of valid years.
///   * _return_ - View for expandable_rate_component
#[component]
pub fn ExpandableRateComponent(
    /// Context
    cx: Scope,
    /// The [RateCurve] being edited
    updatable: Updatable<RateCurve>,
    /// Range of valid years.
    #[prop(default=YearRange{ start: 1900, end: 2400 })]
    year_range: YearRange,
) -> impl IntoView {
    // α <fn expandable_rate_component>
    todo!("Implement `expandable_rate_component`")
    // ω <fn expandable_rate_component>
}

// α <mod-def expandable_rate_component>
// ω <mod-def expandable_rate_component>
