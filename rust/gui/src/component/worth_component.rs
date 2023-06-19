//! Module for worth_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::component::numeric_input::{Modification, NumericInput};
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::core::NormalSpec;

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

    use leptos::IntoAttribute;

    view! {
        cx,
        <fieldset class="worth">
        <legend>"Worth"</legend>

        <input
            type="text"
            placeholder="Worth-".to_string()
        />
        </fieldset>
    }

    // ω <fn worth_component>
}

// α <mod-def worth_component>
// ω <mod-def worth_component>
