//! Module for app_nav_bar leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::IntoAttribute;
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
    pub const SELF_CLASS: &str = "plus-anb";
    crate::log_component!("`AppNavBar`");
    // α <fn app_nav_bar>

    use crate::AppContext;
    use crate::CssClasses;
    use crate::CurrencySelect;
    use crate::EnumSelect;
    use crate::Updatable;
    use leptos::use_context;
    use leptos::Signal;
    use leptos::SignalGet;
    use leptos::SignalGetUntracked;
    use leptos::SignalSet;
    use plus_lookup::I18nEnums;
    let app_context = use_context::<AppContext>().unwrap();
    let lang_selector = app_context.lang_selector;
    let display_currency = app_context.display_currency;
    let grid_edit_active_count = app_context.grid_edit_active_count;
    let lang_select_disabled = move || grid_edit_active_count.get() > 0;

    let lang_selector_updatable =
        Updatable::new(lang_selector.get_untracked(), move |new_lang_selector| {
            lang_selector.set(new_lang_selector.clone())
        });

    let display_currency_updatable =
        Updatable::new(display_currency.get_untracked(), move |new_currency| {
            display_currency.set(*new_currency)
        });

    // ω <fn app_nav_bar>
    view! {
        <div class=SELF_CLASS>
            // α <plus-anb-view>

            <div class=CssClasses::AppLangSelect.to_string()>
                <div class=CssClasses::AppCurrencySelect.to_string()>
                    <label>
                        <CurrencySelect updatable=display_currency_updatable/>
                        "Display Currency"
                    </label>
                </div>
                <div class=CssClasses::AppLangSelectInner.to_string()>
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

        // ω <plus-anb-view>
        </div>
    }
}

// α <mod-def app_nav_bar>
// ω <mod-def app_nav_bar>
