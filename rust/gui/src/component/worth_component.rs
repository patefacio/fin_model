//! Module for worth_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::component::numeric_input::{Modification, NumericInput};
use crate::utils::updatable::Updatable;
use fin_model::core::NormalSpec;
use leptos::{component, tracing, view, IntoView, Scope};

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a single worth, or store of value, like home, land, etc.
///
///   * **cx** - Context
///   * _return_ - View for worth_component
#[component]
pub fn WorthComponent(
    /// Context
    cx: Scope,
) -> impl IntoView {
    // α <fn worth_component>

    view! {
        cx,
        <fieldset class="worth">
        <legend>"Worth"</legend>

        </fieldset>
    }

    // ω <fn worth_component>
}

// α <mod-def worth_component>
// ω <mod-def worth_component>
