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
    use crate::ItemGrowthComponent;
    use crate::NormalSpecComponent;
    use crate::NumericInput;
    use crate::OkCancelComponent;
    use crate::PercentInput;
    use crate::RateCurveComponent;
    use crate::SymbolInput;
    use crate::WorthComponent;
    use crate::YearCurrencyValueInput;
    use crate::YearInput;
    use crate::{InitialValue, MultiColumnSelect, SelectOption};
    use leptos_dom::console_log;

    use crate::utils::updatable::Updatable;
    use plus_modeled::{
        BalanceSheet, Currency, DossierHoldingIndex, DossierItemType, GrowthItemMappings, Holding,
        ItemGrowth, NormalSpec, RateCurve, YearCurrencyValue,
    };

    let symbol_updatable = Updatable::new("foobar".to_string(), move |s| {
        console_log(&format!("Got symbol update -> {s:?}"));
    });

    let options: Vec<_> = (0..50)
        .map(|i| SelectOption::Label(format!("Selection {i}")))
        .collect();
    let on_select = move |sel| {
        console_log(&format!("Selection -> {sel}"));
    };

    let on_number_update = Updatable::new(Some(32.3), move |n| {
        console_log(&format!("Number updated -> {n:?}"));
    });

    let on_percent_update = Updatable::new(Some(43.23), move |n| {
        console_log(&format!("Percent updated -> {n:?}"));
    });

    let year_updateable = Updatable::new(Some(1999), |y| {
        console_log(&format!("Year updated -> {y:?}"));
    });


    let holding_updatable = Updatable::new(
        Holding {
            ..Default::default()
        },
        |holding: &Holding| {
            console_log(&format!("Normal Spec -> {holding:?}"));
        },
    );

    let currency_updatable = Updatable::new(Currency::Eur, |currency: &Currency| {
        console_log(&format!("Currency set to {currency:?}"));
    });

    // let balance_sheet = BalanceSheet::default();
    // let instrument_growth_mappings_updatable = Updatable::new(
    //     InstrumentGrowthSync::new(&balance_sheet, DossierHoldingIndex::default()),
    //     |mappings: &InstrumentGrowthSync| {
    //         leptos_dom::console_log(&format!("Mappings -> {mappings:?}"));
    //     },
    // );

    let (read_count, write_count) = create_signal(cx, 0);
    leptos_dom::console_log(&format!("App cx({cx:?}"));

    let growth_item_mappings = &GrowthItemMappings::default();

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
                Some(YearCurrencyValue{ year: 1998, currency: Currency::Jpy as i32, value: 25.55}),
                |ycv| leptos_dom::console_log(&format!("YearCurrencyValue set to {ycv:?}"))
            )

        />


        <div>"YearCurrencyValueInput with None"</div>
        <YearCurrencyValueInput
            updatable = Updatable::new(
                None,
                |ycv| leptos_dom::console_log(&format!("YearCurrencyValue set to {ycv:?}"))
            )

        />

        <div>"Normal Spec With Values"</div>
        <NormalSpecComponent
            updatable = Updatable::new(
                Some(NormalSpec {
                    mean: 10.0,
                    std_dev: 20.0,
                }),
                |ns: &Option<NormalSpec>| {
                    console_log(&format!("Normal Spec -> {ns:?}"));
                },
            )
        />

        <NormalSpecComponent
            updatable = Updatable::new(None, |ns| { 
                console_log(&format!("NS Updated to -> {ns:?}"));
            } )
        />

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
            updatable=Updatable::new(BalanceSheet::default(), |bs| console_log(&format!("BS -> {bs:?}")))
        />
        <br/>

        <div>"Ok/Cancel"</div>
        <OkCancelComponent
            on_ok=|| {
                console_log("Ok pressed")
            }
            on_cancel=|| {
                console_log("Cancel pressed")
            }
        />

        <div>"Rate Curve Component"</div>
        <RateCurveComponent
            updatable=Updatable::new(RateCurve::default(), |rc| {
                console_log(&format!("RateCurve updated -> {rc:?}"));
            })
        />

        <div>"Holding Growth Component"</div>
        <ItemGrowthComponent
            updatable=Updatable::new(
                ItemGrowth::default(),
                |ig| {
                    console_log(&format!("ItemGrowth updated -> {ig:?}"));
                }
            )
            dossier_item_type=DossierItemType::Holding
            growth_item_mappings=growth_item_mappings
        />

        <div>"Worth Growth Component"</div>
        <ItemGrowthComponent
            updatable=Updatable::new(
                ItemGrowth::default(),
                |ig| {
                    console_log(&format!("ItemGrowth updated -> {ig:?}"));
                }
            )
            dossier_item_type=DossierItemType::Worth
            growth_item_mappings=growth_item_mappings
        />

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
