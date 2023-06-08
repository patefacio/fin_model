//! Module for percent_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::component::numeric_input::{Modification, NumericInput, NumericInputProps};
use crate::utils::updatable::Updatable;
use leptos::{component, view, IntoView, Scope};

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Provides a [NumericInput] with a percent suffix modification.
///
///   * **cx** - Context
///   * **updatable** - Called when input is updated.
///   * _return_ - View for percent_input
#[component]
pub fn PercentInput(
    /// Context
    cx: Scope,
    /// Called when input is updated.
    updatable: Updatable<Option<f64>>,
) -> impl IntoView {
    // α <fn percent_input>

    view! { cx,
        <NumericInput
                    updatable=updatable
                    modification=Some(Modification::Suffix("%".into()))
                    non_negative=true
                />

    }

    // ω <fn percent_input>
}

// α <mod-def percent_input>
// ω <mod-def percent_input>
