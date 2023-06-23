//! Module for age_assumptions_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::AgeAssumptions;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models age of various stages, eg retirement, death...
///
///   * **cx** - Context
///   * **updatable** - The [AgeAssumptions] being edited.
///   * _return_ - View for age_assumptions_component
#[component]
pub fn AgeAssumptionsComponent(
    /// Context
    cx: Scope,
    /// The [AgeAssumptions] being edited.
    updatable: Updatable<Option<AgeAssumptions>>,
) -> impl IntoView {
    // α <fn age_assumptions_component>

    use crate::NumericInput;
    use crate::Updatable;

    let retirement_age = updatable
        .value
        .as_ref()
        .map(|age_assumption| age_assumption.retirement_age);

    let death_age = updatable
        .value
        .as_ref()
        .map(|age_assumption| age_assumption.death_age);

    view! {
        cx,

        <fieldset>

        <div>"Retirement Age"</div>
        <NumericInput
            updatable=Updatable::new(retirement_age.map(|ra| ra as f64), |retirement_age| {
                console_log(&format!("Retirement age updated to {retirement_age:?}"));
            })
        />

        <div>"Death Age"</div>
        <NumericInput
            updatable=Updatable::new(death_age.map(|da| da as f64), |death_age| {
                console_log(&format!("Death age updated to {death_age:?}"));
            })
        />

        </fieldset>
    }

    // ω <fn age_assumptions_component>
}

// α <mod-def age_assumptions_component>
// ω <mod-def age_assumptions_component>
