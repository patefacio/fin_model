use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    use crate::AppContext;
    use plus_modeled::LangSelector;
    use crate::ComponentDisplayComponent;

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let grid_edit_active_count = create_rw_signal(cx, 0);
    let lang_selector = create_rw_signal(cx, LangSelector::French);
    let app_context = AppContext {
        lang_selector,
        grid_edit_active_count,
    };

    provide_context(cx, app_context);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/fin_model_gui.css"/>
        <Title text="FinModel Components"/>
        <Router>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=|cx| {
                            view! { cx, <ComponentDisplayComponent/> }
                        }
                    />

                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button

    view! { cx, <div>"TODO"</div> }
}
