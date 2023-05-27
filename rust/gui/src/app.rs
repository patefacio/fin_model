use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/fin_model_gui.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>


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

    use crate::component::multi_column_select::{InitialValue, MultiColumnSelect, SelectOption};
    use crate::component::normal_spec_growth::NormalSpecGrowth;
    use crate::component::numeric_input::NumericInput;
    use crate::component::percent_input::PercentInput;
    use crate::component::year_input::YearInput;
    use crate::utils::updatable::Updatable;
    use fin_model::core::NormalSpec;

    let options: Vec<_> = (0..50)
        .map(|i| SelectOption::Label(format!("Selection {i}")))
        .collect();
    let on_select = move |sel| {
        leptos_dom::console_log(&format!("Selection -> {sel}"));
    };

    let on_number_update = move |n| {
        leptos_dom::console_log(&format!("Number updated -> {n}"));
    };

    let on_percent_update = move |n| {
        leptos_dom::console_log(&format!("Percent updated -> {n}"));
    };

    let year_updateable = Updatable::new(1999, |y| {
        leptos_dom::console_log(&format!("Year updated -> {y}"));
    });

    let normal_spec_updatable = Updatable::new(
        NormalSpec {
            mean: 10.0,
            std_dev: 20.0,
        },
        |ns:&NormalSpec| {
            leptos_dom::console_log(&format!("Normal Spec -> {ns:?}"));
        },
    );

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <MultiColumnSelect
            options = options
            on_select = on_select
            initial_value = Some(InitialValue::SelectionIndex(5))
        />
        <br/>

        <div>"Number"</div>
        <NumericInput on_update=on_number_update />

        <br/>

        <div>"Percent"</div>
        <PercentInput on_update=on_percent_update />

        <br/>

        <div>"Year"</div>
        <YearInput
            updatable = year_updateable
        />

        <div>"Normal Spec"</div>
        <NormalSpecGrowth
            updatable = normal_spec_updatable
        />

    }
}
