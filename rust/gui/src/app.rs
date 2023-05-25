use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    use crate::component::multi_column_select::{MultiColumnSelect, MultiColumnSelectProps, SelectOption, InitialValue};

    let options: Vec<_> = (0..50).map(|i| SelectOption::Label(format!("Selection {i}"))).collect();
    let on_select = move |sel| {
        leptos_dom::console_log(&format!("Selection -> {sel}"));
    };

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/fin_model_gui.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        <MultiColumnSelect 
            options = options
            on_select = on_select
            initial_value = Some(InitialValue::SelectionIndex(5))
        />


        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
