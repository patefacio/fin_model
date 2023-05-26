//! TODO: Document Module(percent_input)

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::component::numeric_input::{Modification, NumericInput, NumericInputProps};
use leptos::{component, tracing, view, IntoView, Scope};

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Provides a [NumericInput] with a percent suffix modification.
///
///   * **cx** - Context
///   * **on_update** - Called when input is updated.
///   * _return_ - View for percent_input
#[component]
pub fn PercentInput<F>(
    /// Context
    cx: Scope,
    /// Called when input is updated.
    on_update: F,
) -> impl IntoView
where
    F: Fn(f64) + 'static,
{
    // α <fn percent_input>
    
    view! { cx,
        <NumericInput
                    on_update=on_update
                    modification=Some(Modification::Suffix("%".into()))
                    non_negative=true
                />

    }

    // ω <fn percent_input>
}

// α <mod-def percent_input>
// ω <mod-def percent_input>
