//! Module for ok_cancel_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// Models exist status of component
#[derive(Debug, Clone)]
pub enum OkCancel {
    /// Indicates component accepted data edit
    Ok,
    /// Indicates component canceled data edit
    Cancel,
}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models an ok/cancel button pair.
///
///   * **on_ok_cancel** - Function to call when edit complete
///   * _return_ - View for ok_cancel_component
#[component]
pub fn OkCancelComponent<F>(
    /// Function to call when edit complete
    on_ok_cancel: F,
) -> impl IntoView
where
    F: FnMut(OkCancel) + 'static,
{
    // α <fn ok_cancel_component>

    use leptos::store_value;

    let on_ok_cancel = store_value(on_ok_cancel);

    view! {
        <button
            class="ok-button"
            on:click=move |_| { on_ok_cancel.update_value(|f| f(OkCancel::Ok)) }
        >
            "Ok"
        </button>
        <button
            class="cancel-button"
            on:click=move |_| { on_ok_cancel.update_value(|f| f(OkCancel::Cancel)) }
        >
            "Cancel"
        </button>
    }

    // ω <fn ok_cancel_component>
}

// α <mod-def ok_cancel_component>
// ω <mod-def ok_cancel_component>
