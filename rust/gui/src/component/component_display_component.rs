//! Module for component_display_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Displays several of current components
///
///   * **cx** - Context
///   * _return_ - View for component_display_component
#[component]
pub fn ComponentDisplayComponent(
    /// Context
    cx: Scope,
) -> impl IntoView {
    // α <fn component_display_component>
    use crate::component::dispose_test::DisposeTest;
    use crate::component::holding_component::InstrumentGrowthMappings;
    use crate::component::holding_component::{HoldingComponent, InstrumentGrowthSync};
    use crate::AgeAssumptionsComponent;
    use crate::BalanceSheetComponent;
    use crate::CurrencySelect;
    use crate::DossierCorrelationEntryComponent;
    use crate::DossierCorrelationMatrixComponent;
    use crate::DossierHoldingIndexInput;
    use crate::DossierItemIndexComponent;
    use crate::FlowSpecComponent;
    use crate::GrowingFlowSpecComponent;
    use crate::ItemGrowthComponent;
    use crate::NormalSpecComponent;
    use crate::NumericInput;
    use crate::OkCancelComponent;
    use crate::PercentInput;
    use crate::PersonComponent;
    use crate::RateCurveComponent;
    use crate::StringInput;
    use crate::SymbolInput;
    use crate::ValueFlowSpecComponent;
    use crate::WorthComponent;
    use crate::YearCurrencyValueInput;
    use crate::YearInput;
    use crate::YearRangeInput;
    use crate::YearValueInput;
    use crate::{InitialValue, MultiColumnSelect, SelectOption};
    use leptos::*;
    use leptos_dom::console_log;

    use crate::utils::updatable::Updatable;
    use plus_modeled::core::dossier_item_index::ItemIndex;
    use plus_modeled::AgeAssumptions;
    use plus_modeled::BalanceSheet;
    use plus_modeled::Currency;
    use plus_modeled::DossierCorrelationEntry;
    use plus_modeled::DossierCorrelationMatrix;
    use plus_modeled::DossierHoldingIndex;
    use plus_modeled::DossierItemIndex;
    use plus_modeled::DossierItemType;
    use plus_modeled::FlowSpec;
    use plus_modeled::GrowingFlowSpec;
    use plus_modeled::GrowthItemMappings;
    use plus_modeled::Holding;
    use plus_modeled::ItemGrowth;
    use plus_modeled::NormalSpec;
    use plus_modeled::Person;
    use plus_modeled::PersonType;
    use plus_modeled::RateCurve;
    use plus_modeled::ValueFlowSpec;
    use plus_modeled::YearCurrencyValue;
    use plus_modeled::YearRange;
    use plus_modeled::YearValue;

    let (read_count, write_count) = create_signal(cx, 0);
    leptos_dom::console_log(&format!("App cx({cx:?}"));

    let growth_item_mappings = &GrowthItemMappings::default();

    let make_cor_entry = |row_holding_id, column_holding_id, correlation| DossierCorrelationEntry {
        row_index: Some(DossierItemIndex {
            item_index: Some(ItemIndex::HoldingIndex(DossierHoldingIndex {
                holding_index: Some(row_holding_id),
                ..Default::default()
            })),
        }),
        column_index: Some(DossierItemIndex {
            item_index: Some(ItemIndex::HoldingIndex(DossierHoldingIndex {
                holding_index: Some(column_holding_id),
                ..Default::default()
            })),
        }),
        correlation: correlation,
    };

    let sample_dossier_matrix = DossierCorrelationMatrix {
        mappings: vec![
            make_cor_entry(0, 1, -0.31),
            make_cor_entry(1, 1, 0.71),
            make_cor_entry(1, 2, 0.342),
        ],
    };

    view! { cx,
        <h4>"Dossier Correlation Matrix"</h4>
        <DossierCorrelationMatrixComponent updatable=Updatable::new(
            sample_dossier_matrix,
            |m| { console_log(&format!("Matrix updated to -> {m:?}")) },
        )/>
        <h4>"String Input"</h4>
        <StringInput updatable=Updatable::new(
            "Initial Value".to_string(),
            |s| { console_log(&format!("String Input updated to -> {s}")) },
        )/>
        <hr/>
        <h4>"Symbol Input"</h4>
        <SymbolInput symbol_updatable=Updatable::new(
            "foobar".to_string(),
            move |s| {
                console_log(&format!("Got symbol update -> {s:?}"));
            },
        )/>
        <hr/>
        <h4>"Number"</h4>
        <NumericInput updatable=Updatable::new(
            Some(32.3),
            move |n| {
                console_log(&format!("Number updated -> {n:?}"));
            },
        )/>
        <hr/>
        <h4>"Currency Select"</h4>
        <CurrencySelect updatable=Updatable::new(
            Currency::Eur,
            |currency: &Currency| {
                console_log(&format!("Currency set to {currency:?}"));
            },
        )/>
        <hr/>
        <h4>"Percent Input"</h4>
        <PercentInput
            updatable=Updatable::new(
                Some(0.4324),
                move |n| {
                    console_log(&format!("Percent updated -> {n:?}"));
                },
            )
            placeholder=Some("pct complete".to_string())
        />
        <hr/>
        <h4>"Year Input"</h4>
        <YearInput
            updatable=Updatable::new(
                Some(1999),
                |y| {
                    console_log(&format!("Year updated -> {y:?}"));
                },
            )
            placeholder=Some("year".to_string())
        />
        <hr/>
        <h4>"Year Currency Value Input With Values"</h4>
        <YearCurrencyValueInput updatable=Updatable::new(
            Some(YearCurrencyValue {
                year: 1998,
                currency: Currency::Jpy as i32,
                value: 25.55,
            }),
            |ycv| leptos_dom::console_log(&format!("YearCurrencyValue set to {ycv:?}")),
        )/>
        <h4>"Year Currency Value Input Without Values"</h4>
        <YearCurrencyValueInput updatable=Updatable::new(
            None,
            |ycv| leptos_dom::console_log(&format!("YearCurrencyValue set to {ycv:?}")),
        )/>
        <hr/>
        <h4>"Normal Spec With Values"</h4>
        <NormalSpecComponent updatable=Updatable::new(
            Some(NormalSpec {
                mean: 0.1,
                std_dev: 0.2,
            }),
            |ns: &Option<NormalSpec>| {
                console_log(&format!("Normal Spec -> {ns:?}"));
            },
        )/>
        <h4>"Normal Spec Without Values"</h4>
        <NormalSpecComponent updatable=Updatable::new(
            None,
            |ns| {
                console_log(&format!("NS Updated to -> {ns:?}"));
            },
        )/>
        <hr/>
        <h4>"Person with None"</h4>
        <PersonComponent updatable=Updatable::new(
            None,
            |person| {
                console_log(&format!("Person Updated to -> {person:?}"));
            },
        )/>
        <h4>"Person with Some"</h4>
        <PersonComponent updatable=Updatable::new(
            Some(Person {
                name: "John Doe".to_string(),
                person_type: PersonType::PrimaryOwner as i32,
                birth_year: 1995,
                age_assumptions: Some(AgeAssumptions {
                    retirement_age: 60,
                    death_age: 88,
                }),
            }),
            |person| {
                console_log(&format!("Person Updated to -> {person:?}"));
            },
        )/>
        <HoldingComponent holding_updatable=Updatable::new(
            Holding { ..Default::default() },
            |holding: &Holding| {
                console_log(&format!("Holding updated -> {holding:?}"));
            },
        )/>
        <hr/>
        <h4>"Worth"</h4>
        <WorthComponent/>
        <hr/>
        <h4>"Balance Sheet"</h4>
        <BalanceSheetComponent updatable=Updatable::new(BalanceSheet::default(), |bs| console_log(&format!("BS -> {bs:?}")))/>
        <hr/>
        <h4>"Ok/Cancel"</h4>
        <OkCancelComponent
            on_ok=|| { console_log("Ok pressed") }
            on_cancel=|| { console_log("Cancel pressed") }
        />
        <hr/>
        <h4>"Year Range Input"</h4>
        <YearRangeInput updatable=Updatable::new(
            None,
            |yr| {
                console_log(&format!("Year Range updated -> {yr:?}"));
            },
        )/>
        <h4>"Year Value Input"</h4>
        <YearValueInput updatable=Updatable::new(
            None,
            |yv| {
                console_log(&format!("Year Value updated -> {yv:?}"));
            },
        )/>
        <h4>"Dossier Holding Index"</h4>
        <DossierHoldingIndexInput updatable=Updatable::new(
            None,
            |dhi| {
                console_log(&format!("Dossier Holding Index updated -> {dhi:?}"));
            },
        )/>
        <h4>"Dossier Item Index"</h4>
        <DossierItemIndexComponent updatable=Updatable::new(
            None,
            |dii| {
                console_log(&format!("Dossier Item Index updated -> {dii:?}"));
            },
        )/>
        <h4>"Dossier Correlation Entry"</h4>
        <DossierCorrelationEntryComponent updatable=Updatable::new(
            None,
            |dce| {
                console_log(&format!("Dossier Correlation Entry -> {dce:?}"));
            },
        )/>
        <h4>"Rate Curve"</h4>
        <RateCurveComponent updatable=Updatable::new(
            RateCurve::default(),
            |rc| {
                console_log(&format!("RateCurve updated -> {rc:?}"));
            },
        )/>
        <hr/>
        <h4>"Rate Curve with Sample Data"</h4>
        <RateCurveComponent updatable=Updatable::new(
            RateCurve {
                curve: vec![
                    YearValue { year : 2002, value : 4.5 }, YearValue { year : 2000, value : 0.6
                    }, YearValue { year : 1980, value : 0.025, }
                ],
            },
            |rc| {
                console_log(&format!("RateCurve updated -> {rc:?}"));
            },
        )/>
        <hr/>
        <h4>"Growth Component (Holding)"</h4>
        <ItemGrowthComponent
            updatable=Updatable::new(
                ItemGrowth::default(),
                |ig| {
                    console_log(&format!("ItemGrowth updated -> {ig:?}"));
                },
            )
            dossier_item_type=DossierItemType::Holding
            growth_item_mappings=growth_item_mappings
        />
        <hr/>
        <h4>"Growth Component (Worth)"</h4>
        <ItemGrowthComponent
            updatable=Updatable::new(
                ItemGrowth::default(),
                |ig| {
                    console_log(&format!("ItemGrowth updated -> {ig:?}"));
                },
            )
            dossier_item_type=DossierItemType::Worth
            growth_item_mappings=growth_item_mappings
        />
        <hr/>
        <h4>"Growth Component (Flow)"</h4>
        <ItemGrowthComponent
            updatable=Updatable::new(
                ItemGrowth::default(),
                |ig| {
                    console_log(&format!("ItemGrowth updated -> {ig:?}"));
                },
            )
            dossier_item_type=DossierItemType::Flow
            growth_item_mappings=growth_item_mappings
        />
        <hr/>
        <h4>"Growing Flow Spec"</h4>
        <GrowingFlowSpecComponent updatable=Updatable::new(
            None,
            |gfs| {
                console_log(&format!("GrowingFlowSpec updated -> {gfs:?}"));
            },
        )/>
        <hr/>
        <h4>"Value Flow Spec"</h4>
        <ValueFlowSpecComponent updatable=Updatable::new(
            None,
            |gfs| {
                console_log(&format!("ValueFlowSpec updated -> {gfs:?}"));
            },
        )/>
        <hr/>
        <h4>"Flow Spec"</h4>
        <FlowSpecComponent updatable=Updatable::new(
            None,
            |gfs| {
                console_log(&format!("FlowSpec updated -> {gfs:?}"));
            },
        )/>
        <hr/>
        <div>"Dispose Test"</div>
        <Show when=move || (read_count() % 2) == 0 fallback=|_| "Nothing">
            <DisposeTest/>
        </Show>
        <p>
            <hr/>
            <br/>
        </p>
        <button on:click=move |_| {
            write_count.update(|c| *c = *c + 1);
        }>"INC"</button>
    }

    // ω <fn component_display_component>
}

// α <mod-def component_display_component>
// ω <mod-def component_display_component>
