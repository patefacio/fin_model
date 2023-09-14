//! Module for app_nav_bar leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Top navigation bar
///
///   * _return_ - View for app_nav_bar
#[component]
pub fn AppNavBar() -> impl IntoView {
    // α <fn app_nav_bar>

    use crate::AppContext;
    use crate::EnumSelect;
    use crate::Updatable;
    use leptos::use_context;
    use leptos::Signal;
    use leptos::SignalGet;
    use leptos::SignalGetUntracked;
    use leptos::SignalSet;
    use plus_lookup::I18nEnums;
    use plus_modeled::LangSelector;
    let lang_selector = use_context::<AppContext>().unwrap().lang_selector;
    let grid_edit_active_count = use_context::<AppContext>().unwrap().grid_edit_active_count;
    let lang_select_disabled = move || grid_edit_active_count.get() > 0;

    let lang_selector_updatable =
        Updatable::new(lang_selector.get_untracked(), move |new_lang_selector| {
            lang_selector.set(new_lang_selector.clone())
        });

    view! {
        <div class="app-nav-bar">
            <div class="app-lang-select">
                <div class="app-lang-select-inner">
                    <EnumSelect
                        updatable=lang_selector_updatable
                        column_count=1
                        label=Some(
                            Box::new(move |e| {
                                I18nEnums::LangSelector(lang_selector.get_untracked(), e)
                                    .to_string()
                            }),
                        )

                        disabled=Signal::derive(lang_select_disabled)
                    />
                </div>
            </div>
        </div>
    }

    // ω <fn app_nav_bar>
}

// α <mod-def app_nav_bar>
// ω <mod-def app_nav_bar>
