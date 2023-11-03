//! Module for ok_cancel_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::AppContext;
use leptos::use_context;
use leptos::IntoAttribute;
use leptos::SignalGet;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;
use plus_lookup::I18nOkCancelComponent;

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
    pub const SELF_CLASS: &str = "plus-occ";
    let lang_selector = use_context::<AppContext>().unwrap().lang_selector;
    let i18n_ok = move || I18nOkCancelComponent::Ok(lang_selector.get()).to_string();
    let i18n_cancel = move || I18nOkCancelComponent::Cancel(lang_selector.get()).to_string();
    crate::log_component!("`OkCancelComponent`");
    // α <fn ok_cancel_component>

    use crate::CssClasses;
    use leptos::store_value;
    let on_ok_cancel_stored_value = store_value(on_ok_cancel);

    // ω <fn ok_cancel_component>
    view! {
        <div class=SELF_CLASS>
            // α <plus-occ-view>
            <button
                class=CssClasses::OccOkBtn.to_string()
                on:click=move |_| { on_ok_cancel_stored_value.update_value(|f| f(OkCancel::Ok)) }
            >
                {i18n_ok}
            </button>
            <button
                class=CssClasses::OccCancelBtn.to_string()
                on:click=move |_| {
                    on_ok_cancel_stored_value.update_value(|f| f(OkCancel::Cancel))
                }
            >

                {i18n_cancel}
            </button>
        // ω <plus-occ-view>
        </div>
    }
}

// α <mod-def ok_cancel_component>
// ω <mod-def ok_cancel_component>
