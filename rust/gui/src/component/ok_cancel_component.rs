//! Module for ok_cancel_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;

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
pub fn OkCancelComponent<O, C>(
    /// Context
    cx: Scope,
    /// Function to call when ok
    on_ok: O,
    /// Function to call when canceled
    on_cancel: C,
) -> impl IntoView
where
    O: FnMut() + 'static,
    C: FnMut() + 'static,
{
    // α <fn ok_cancel_component>

    let mut on_ok = on_ok;
    let mut on_cancel = on_cancel;

    view! { cx,
        <button on:click=move |_| { on_ok() }>"Ok"</button>
        <button on:click=move |_| { on_cancel() }>"Cancel"</button>
    }

    // ω <fn ok_cancel_component>
}

// α <mod-def ok_cancel_component>
// ω <mod-def ok_cancel_component>
