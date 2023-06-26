use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {

    use crate::ComponentDisplayComponent;

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/fin_model_gui.css"/>
        <Title text="Welcome to Leptos"/>
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

    view!{
        cx,
        <div>"TODO"</div>
    }
}
