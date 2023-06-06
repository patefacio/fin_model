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

    use crate::component::balance_sheet_component::BalanceSheetComponent;
    use crate::component::dispose_test::DisposeTest;
    use crate::component::holding_component::InstrumentGrowthMappings;
    use crate::component::holding_component::{HoldingComponent, InstrumentGrowthSync};
    use crate::component::multi_column_select::{InitialValue, MultiColumnSelect, SelectOption};
    use crate::component::normal_spec_component::NormalSpecComponent;
    use crate::component::numeric_input::NumericInput;
    use crate::component::percent_input::PercentInput;
    use crate::component::worth_component::WorthComponent;
    use crate::component::symbol_input::SymbolInput;
    use crate::component::year_input::YearInput;
    use leptos_dom::console_log;

    use crate::utils::updatable::Updatable;
    use fin_model::{
        account::Holding,
        balance_sheet::BalanceSheet,
        core::{DossierHoldingIndex, NormalSpec},
    };

    let symbol_updatable = Updatable::new(
        "foobar".to_string(),
        Box::new(
            move |s| {
                console_log(&format!("Got symbol update -> {s:?}"));
            }
        )
    );

    let options: Vec<_> = (0..50)
        .map(|i| SelectOption::Label(format!("Selection {i}")))
        .collect();
    let on_select = move |sel| {
        leptos_dom::console_log(&format!("Selection -> {sel}"));
    };

    let on_number_update = Updatable::new(
        Some(32.3),
        Box::new(move |n| {
            leptos_dom::console_log(&format!("Number updated -> {n:?}"));
        }),
    );

    let on_percent_update = Updatable::new(
        Some(43.23),
        Box::new(move |n| {
            leptos_dom::console_log(&format!("Percent updated -> {n:?}"));
        }),
    );

    let year_updateable = Updatable::new(
        Some(1999),
        Box::new(|y| {
            leptos_dom::console_log(&format!("Year updated -> {y:?}"));
        }),
    );

    let normal_spec_updatable = Updatable::new(
        NormalSpec {
            mean: 10.0,
            std_dev: 20.0,
        },
        Box::new(|ns: &NormalSpec| {
            leptos_dom::console_log(&format!("Normal Spec -> {ns:?}"));
        }),
    );

    let holding_updatable = Updatable::new(
        Holding {
            ..Default::default()
        },
        Box::new(|holding: &Holding| {
            leptos_dom::console_log(&format!("Normal Spec -> {holding:?}"));
        }),
    );

    // let balance_sheet = BalanceSheet::default();
    // let instrument_growth_mappings_updatable = Updatable::new(
    //     InstrumentGrowthSync::new(&balance_sheet, DossierHoldingIndex::default()),
    //     |mappings: &InstrumentGrowthSync| {
    //         leptos_dom::console_log(&format!("Mappings -> {mappings:?}"));
    //     },
    // );

    let (read_count, write_count) = create_signal(cx, 0);
    leptos_dom::console_log(&format!("App cx({cx:?}"));

    view! { cx,
        <MultiColumnSelect
            options = options
            on_select = on_select
            initial_value = Some(InitialValue::SelectionIndex(5))
        />
        <br/>

        <div>"Symbol"</div>
        <SymbolInput symbol_updatable=symbol_updatable />

        <div>"Number"</div>
        <NumericInput updatable=on_number_update />

        <br/>

        <div>"Percent"</div>
        <PercentInput updatable=on_percent_update />

        <br/>

        <div>"Year"</div>
        <YearInput
            updatable = year_updateable
        />
/* 
        <div>"Normal Spec"</div>
        <NormalSpecComponent
            updatable = normal_spec_updatable
        />
*/
        <div>"Holding"</div>
        <HoldingComponent
            holding_updatable = holding_updatable
        />

        <div>"Worth"</div>
        <WorthComponent
        />

        <div>"Balance Sheet"</div>
        <BalanceSheetComponent
        />

        <div>"Dispose Test"</div>
            <Show when=move || (read_count() % 2) == 0
                fallback=|_| "Nothing"
            >
                <DisposeTest/>
            </Show>
        <p>
        /*
        { move || if (read_count() % 2) == 0 {
            view! { cx, <DisposeTest/> }.into_view(cx)
        } else {
            view! { cx, "Nothing" }.into_view(cx)
        }
        }
        */
        </p>

        <button on:click=move |_| {
            write_count.update(|c| *c = *c + 1);
        }>"INC"</button>
    }
}
