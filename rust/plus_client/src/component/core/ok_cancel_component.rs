//! Module for ok_cancel_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::AppContext;
use leptos::component;
use leptos::expect_context;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::SignalGet;
use std::rc::Rc;

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
    use plus_lookup::i18n::ok_cancel_component::*;
    pub const SELF_CLASS: &str = "plus-occ";
    let lang_selector = expect_context::<Rc<AppContext>>().lang_selector;
    let i18n_ok = move || i18n_ok(lang_selector.get());
    let i18n_cancel = move || i18n_cancel(lang_selector.get());
    let component_id = crate::component_id!("`OkCancelComponent`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn ok_cancel_component>

    use crate::ClientCssClasses;
    use leptos::store_value;
    let on_ok_cancel_stored_value = store_value(on_ok_cancel);

    // ω <fn ok_cancel_component>
    view! {
        <div class=SELF_CLASS>
            // α <plus-occ-view>
            <button
                class=ClientCssClasses::OccOkBtn.as_str()
                on:click=move |_| { on_ok_cancel_stored_value.update_value(|f| f(OkCancel::Ok)) }
            >
                {i18n_ok}
            </button>
            <button
                class=ClientCssClasses::OccCancelBtn.as_str()
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
