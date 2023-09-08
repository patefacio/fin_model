//! Module for app_component leptos function/component

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
/// Top level component
///
///   * **cx** - Context
///   * _return_ - View for app_component
#[component]
pub fn AppComponent(
    /// Context
    cx: Scope,
) -> impl IntoView {
    // α <fn app_component>

    use crate::AppContext;
    use plus_modeled::LangSelector;
    use crate::AppNavBar;
    use crate::AppSideBar;
    use crate::AppCenterComponent;
    use leptos::create_rw_signal;
    use leptos::provide_context;
    use leptos_meta::Stylesheet;
    use leptos_meta::provide_meta_context;
    use leptos_router::Route;
    use leptos_router::Router;
    use leptos_router::Routes;
    use leptos_meta::Title;

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let grid_edit_active_count = create_rw_signal(cx, 0);
    let lang_selector = create_rw_signal(cx, LangSelector::French);
    let app_context = AppContext {
        lang_selector,
        grid_edit_active_count,
    };

    provide_context(cx, app_context);

    view! {
        cx,

        <Stylesheet id="leptos" href="/pkg/plus_client.css"/>
        <Title text="Auric Components"/>
        <Router>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=|cx| {
                            view! { cx,
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
