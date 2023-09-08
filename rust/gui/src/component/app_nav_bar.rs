//! Module for app_nav_bar leptos function/component

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
/// Top navigation bar
///
///   * **cx** - Context
///   * _return_ - View for app_nav_bar
#[component]
pub fn AppNavBar(
    /// Context
    cx: Scope,
) -> impl IntoView {
    // α <fn app_nav_bar>

    use plus_lookup::I18nEnums;
    use plus_modeled::LangSelector;
    use leptos::use_context;
    use leptos::SignalGetUntracked;
    use leptos::SignalSet;
    use crate::AppContext;
    use crate::Updatable;
    use crate::EnumSelect;
    let lang_selector = use_context::<AppContext>(cx).unwrap().lang_selector;

    let lang_selector_updatable =
    Updatable::new(lang_selector.get_untracked(), move |new_lang_selector| {
        lang_selector.set(new_lang_selector.clone())
    });

    view! {
        cx,
        <div class="app-nav-bar">
        <div class="app-lang-select">
        <EnumSelect
            updatable=lang_selector_updatable
            column_count=1
            label=Some(
                Box::new(move |e| {
                    I18nEnums::LangSelector(lang_selector.get_untracked(), e).to_string()
                }),
            )
        />
        </div>
        </div>
    }

    // ω <fn app_nav_bar>
}

// α <mod-def app_nav_bar>
// ω <mod-def app_nav_bar>
