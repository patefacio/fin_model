//! Module for app_component leptos function/component

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
/// Top level component
///
///   * _return_ - View for app_component
#[component]
pub fn AppComponent() -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-app-component";
    crate::log_component!("`AppComponent`");
    // α <fn app_component>

    use crate::AppCenterComponent;
    use crate::AppContext;
    use crate::AppNavBar;
    use crate::AppSideBar;
    use leptos::create_rw_signal;
    use leptos::provide_context;
    use leptos_meta::provide_meta_context;
    use leptos_meta::Stylesheet;
    use leptos_meta::Title;
    use leptos_router::Route;
    use leptos_router::Router;
    use leptos_router::Routes;
    use plus_lookup::WEB_CURRENCY_EXCHANGE;
    use plus_modeled::Currency;
    use plus_modeled::LangSelector;

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let app_context = AppContext::new(
        create_rw_signal(LangSelector::UsEnglish),
        create_rw_signal(Currency::Usd),
        create_rw_signal(0),
        create_rw_signal(WEB_CURRENCY_EXCHANGE.clone()),
    );

    provide_context(app_context);

    // ω <fn app_component>
    view! {
        <div class=SELF_CLASS>
            // α <plus-app-component-view>

            <Stylesheet id="leptos" href="/pkg/plus_client.css"/>
            <Title text="Auric Components"/>
            <Router>
                <main>
                    <Routes>
                        <Route
                            path=""
                            view=|| {
                                view! {
                                    <AppNavBar/>
                                    <AppSideBar/>
                                    <AppCenterComponent/>
                                }
                            }
                        />

                    </Routes>
                </main>
            </Router>
        // ω <plus-app-component-view>
        </div>
    }
}

// α <mod-def app_component>
// ω <mod-def app_component>