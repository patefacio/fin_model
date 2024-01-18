//! Module for app_nav_bar leptos function/component

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
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Top navigation bar
///
///   * _return_ - View for app_nav_bar
#[component]
pub fn AppNavBar() -> impl IntoView {
    use plus_lookup::i18n::app_nav_bar::*;
    pub const SELF_CLASS: &str = "plus-anb";
    let lang_selector = expect_context::<Rc<AppContext>>().lang_selector;
    let i18n_dossiers = move || i18n_dossiers(lang_selector.get());
    let i18n_display_currency = move || i18n_display_currency(lang_selector.get());
    let i18n_my_dossier = move || i18n_my_dossier(lang_selector.get());
    let component_id = crate::component_id!("`AppNavBar`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn app_nav_bar>

    use crate::ClientCssClasses;
    use crate::CurrencySelect;
    use crate::EnumSelect;
    use crate::Updatable;
    use leptos::Signal;
    use leptos::SignalGetUntracked;
    use leptos::SignalSet;
    use leptos::SignalWithUntracked;
    use plus_lookup::I18nEnums;
    use plus_lookup::SystemUnicodes;

    let app_context = expect_context::<Rc<AppContext>>();

    let lang_selector = app_context.lang_selector;
    let display_currency = app_context.display_currency;
    let grid_edit_active_count = app_context.grid_edit_active_count;
    let lang_select_disabled = move || grid_edit_active_count.get() > 0;

    let lang_selector_updatable =
        Updatable::new(lang_selector.get_untracked(), move |new_lang_selector| {
            lang_selector.set(new_lang_selector.clone());
        });

    let display_currency_updatable =
        Updatable::new(display_currency.get_untracked(), move |new_currency| {
            display_currency.set(*new_currency)
        });

    // ω <fn app_nav_bar>
    view! {
        <div class=SELF_CLASS>
            // α <plus-anb-view>

            <div class=ClientCssClasses::AppLangSelect.as_str()>
                <div class=ClientCssClasses::AppCurrencySelect.as_str()>
                    <label>
                        <CurrencySelect updatable=display_currency_updatable/>
                        {i18n_display_currency}
                    </label>
                </div>
                <div class=ClientCssClasses::AppLangSelectInner.as_str()>
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
