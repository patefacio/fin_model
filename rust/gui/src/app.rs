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

    use crate::component::multi_column_select::{InitialValue, MultiColumnSelect, SelectOption};
    use crate::component::normal_spec_component::NormalSpecComponent;
    use crate::component::numeric_input::NumericInput;
    use crate::component::percent_input::PercentInput;
    use crate::component::year_input::YearInput;
    use crate::component::holding_component::HoldingComponent;
    use crate::component::worth_component::WorthComponent;
    use crate::component::balance_sheet_component::BalanceSheetComponent;
    use crate::component::InstrumentGrowthMappings;

    use crate::utils::updatable::Updatable;
    use fin_model::{ account::Holding, core::NormalSpec, balance_sheet::BalanceSheet} ;

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

    let holding_updatable = Updatable::new(
        Holding {
            ..Default::default()
        },
        |holding: &Holding| {
            leptos_dom::console_log(&format!("Normal Spec -> {holding:?}"));
        }
    );

    let balance_sheet = BalanceSheet::default();
    let instrument_growth_mappings_updatable = Updatable::new(
        balance_sheet.instrument_growth_mappings.clone(),
        |mappings: &InstrumentGrowthMappings| {
            leptos_dom::console_log(&format!("Mappings -> {mappings:?}"));
        }
    );

    view! { cx,
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
        <NormalSpecComponent
            updatable = normal_spec_updatable
        />

        <div>"Holding"</div>
        <HoldingComponent
            updatable = holding_updatable
            updatable_mappings = instrument_growth_mappings_updatable
        />

        <div>"Worth"</div>
        <WorthComponent
        />

        <div>"Balance Sheet"</div>
        <BalanceSheetComponent
        />

    }
}
