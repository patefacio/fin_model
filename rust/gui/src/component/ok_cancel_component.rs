//! Module for ok_cancel_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::{component, tracing, view, IntoView, Scope};

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models an ok/cancel button pair.
///
///   * **cx** - Context
///   * **on_ok** - Function to call when ok
///   * **on_cancel** - Function to call when canceled
///   * _return_ - View for ok_cancel_component
#[component]
pub fn OkCancelComponent<F>(
    /// Context
    cx: Scope,
    /// Function to call when ok
    on_ok: F,
    /// Function to call when canceled
    on_cancel: F,
) -> impl IntoView
where
    F: FnMut(&String) + 'static,
{
    // α <fn ok_cancel_component>
    todo!("Implement `ok_cancel_component`")
    // ω <fn ok_cancel_component>
}

// α <mod-def ok_cancel_component>
// ω <mod-def ok_cancel_component>
