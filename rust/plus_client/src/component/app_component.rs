//! Module for app_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
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
    use plus_modeled::LangSelector;

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let grid_edit_active_count = create_rw_signal(0);
    let lang_selector = create_rw_signal(LangSelector::UsEnglish);
    let app_context = AppContext {
        lang_selector,
        grid_edit_active_count,
    };

    provide_context(app_context);

    view! {
        <Stylesheet id="leptos" href="/pkg/plus_client.css"/>
        <Title text="Auric Components"/>
        <Router>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=|| {
                            view! {
                                <div class="app-component">
                                    <AppNavBar/>
                                    <AppSideBar/>
                                    <AppCenterComponent/>
                                </div>
                            }
                        }
                    />

                </Routes>
            </main>
        </Router>
    }

    // ω <fn app_component>
}

// α <mod-def app_component>
// ω <mod-def app_component>
