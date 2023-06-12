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

    use crate::component::dispose_test::DisposeTest;
    use crate::component::holding_component::InstrumentGrowthMappings;
    use crate::component::holding_component::{HoldingComponent, InstrumentGrowthSync};
    use crate::BalanceSheetComponent;
    use crate::CurrencySelect;
    use crate::NormalSpecComponent;
    use crate::NumericInput;
    use crate::PercentInput;
    use crate::SymbolInput;
    use crate::WorthComponent;
    use crate::YearCurrencyValueInput;
    use crate::YearInput;
    use crate::{InitialValue, MultiColumnSelect, SelectOption};
    use leptos_dom::console_log;

    use crate::utils::updatable::Updatable;
    use fin_model::{
        account::Holding,
        balance_sheet::BalanceSheet,
        core::{DossierHoldingIndex, NormalSpec, YearCurrencyValue},
        Currency,
    };

    let symbol_updatable = Updatable::new(
        "foobar".to_string(),
        move |s| {
            console_log(&format!("Got symbol update -> {s:?}"));
        },
    );

    let options: Vec<_> = (0..50)
        .map(|i| SelectOption::Label(format!("Selection {i}")))
        .collect();
    let on_select = move |sel| {
        leptos_dom::console_log(&format!("Selection -> {sel}"));
    };

    let on_number_update = Updatable::new(
        Some(32.3),
        move |n| {
            leptos_dom::console_log(&format!("Number updated -> {n:?}"));
        },
    );

    let on_percent_update = Updatable::new(
        Some(43.23),
        move |n| {
            leptos_dom::console_log(&format!("Percent updated -> {n:?}"));
        },
    );

    let year_updateable = Updatable::new(
        Some(1999),
        |y| {
            leptos_dom::console_log(&format!("Year updated -> {y:?}"));
        },
    );

    let normal_spec_updatable = Updatable::new(
        NormalSpec {
            mean: 10.0,
            std_dev: 20.0,
        },
        |ns: &NormalSpec| {
            leptos_dom::console_log(&format!("Normal Spec -> {ns:?}"));
        },
    );

    let holding_updatable = Updatable::new(
        Holding {
            ..Default::default()
        },
        |holding: &Holding| {
            leptos_dom::console_log(&format!("Normal Spec -> {holding:?}"));
        },
    );

    let currency_updatable = Updatable::new(
        Currency::Eur,
        |currency: &Currency| {
            leptos_dom::console_log(&format!("Currency set to {currency:?}"));
        },
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

            <div>"Sample MultiColumnSelect (50 Options)"</div>
            <MultiColumnSelect
                options = options
                on_select = on_select
                initial_value = Some(InitialValue::SelectionIndex(5))
            />
            <br/>

            <div>"SymbolInput"</div>
            <SymbolInput symbol_updatable=symbol_updatable />
            <br/>

            <div>"Number"</div>
            <NumericInput updatable=on_number_update />
            <br/>

            <div>"CurrencySelect"</div>
            <CurrencySelect updatable=currency_updatable />
            <br/>

            <div>"PercentInput"</div>
            <PercentInput updatable=on_percent_update />
            <br/>

            <div>"YearInput"</div>
            <YearInput
                updatable = year_updateable
            />
            <br/>

            <div>"YearCurrencyValueInput"</div>
            <YearCurrencyValueInput
                updatable = Updatable::new(
                    YearCurrencyValue{ year: 1998, currency: Currency::Eur as i32, value: 25.55},
                    |ycv| leptos_dom::console_log(&format!("YearCurrencyValue set to {ycv:?}"))
                )

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
            <br/>

            <div>"Worth"</div>
            <WorthComponent
            />
            <br/>

            <div>"Balance Sheet"</div>
            <BalanceSheetComponent
            />
            <br/>

            <div>"Dispose Test"</div>
                <Show when=move || (read_count() % 2) == 0
                    fallback=|_| "Nothing"
                >
                    <DisposeTest/>
                </Show>
            <p>
            <br/>
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
