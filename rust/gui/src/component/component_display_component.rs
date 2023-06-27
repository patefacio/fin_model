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
    use crate::CurrencySelect;
    use crate::DossierCorrelationMatrixComponent;
    use crate::NormalSpecComponent;
    use crate::NumericInput;
    use crate::OkCancelComponent;
    use crate::PercentInput;
    use crate::RateCurveComponent;
    use crate::YearCurrencyValueInput;
    use crate::YearInput;
    use crate::YearRangeInput;
    use crate::YearValueInput;
    use crate::{InitialValue, MultiColumnSelect, SelectOption};
    use leptos::*;
    use leptos_dom::console_log;

    use crate::utils::updatable::Updatable;
    use plus_modeled::core::dossier_item_index::ItemIndex;
    use plus_modeled::Currency;
    use plus_modeled::DossierCorrelationEntry;
    use plus_modeled::DossierItemIndex;
    use plus_modeled::DossierHoldingIndex;
    use plus_modeled::DossierCorrelationMatrix;
    use plus_modeled::GrowthItemMappings;
    use plus_modeled::ItemGrowth;
    use plus_modeled::NormalSpec;
    use plus_modeled::RateCurve;
    use plus_modeled::YearCurrencyValue;
    use plus_modeled::YearRange;
    use plus_modeled::YearValue;
    
    use crate::DateInput;
    use plus_modeled::Date;

    use crate::component::dossier_correlation_matrix_component::set_matrix_correlation;
    use crate::component::dossier_correlation_matrix_component::DisplayEntireMatrix;

    let (read_count, write_count) = create_signal(cx, 0);
    leptos_dom::console_log(&format!("App cx({cx:?}"));
    let (last_update, set_last_update) = create_signal(cx, String::default());

    let show_update = move |s: String| {
        set_last_update.update(|original| {
            console_log(&s);
            *original = s;
        });
    };

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

    let mut sample_dossier_matrix = DossierCorrelationMatrix {
        mappings: vec![
            make_cor_entry(0, 1, -0.31),
            make_cor_entry(1, 1, 0.71),
            make_cor_entry(1, 2, 0.342),
            make_cor_entry(2, 3, 2.3),
        ],
    };
    set_matrix_correlation(&mut sample_dossier_matrix, (1, 1), 0.25);
    view! { cx,
        <div style="
        position: -webkit-sticky; 
        position: sticky; 
        top:0; 
        right: 0; 
        background-color: #bbb; 
        padding: 6px;
        border-style: inset solid;
        border-color: green;
        border-width: 6px;
        z-index: 2;
        ">
            <h4>"Last Update"</h4>
            <p>{last_update}</p>
        </div>
        <h4>"Dossier Correlation Matrix"</h4>
        <DossierCorrelationMatrixComponent
            updatable=Updatable::new(
                sample_dossier_matrix.clone(),
                |m| {
                    console_log(&format!("Matrix updated to -> {m:?}"))
                }
            )
        />

        

        <DisplayEntireMatrix
            updatable=Updatable::new(
                sample_dossier_matrix.clone(),
                |m| {
                    console_log(&format!("Matrix displayed -> {m:?}"))
                }
            )
        />

        <h4>"Date Input"</h4>
        <DateInput
            updatable=Updatable::new(None, move |n| {
                console_log(&format!("Date updated -> {n:?}"));
            })
            placeholder = Some("00/00/00".to_string())
        />
        <hr/>

        <h4>"Numeric Input"</h4>
        <NumericInput updatable=Updatable::new(Some(32.3), move |n| { show_update(format!("Number updated -> {n:?}")) })/>
        <hr/>
        <h4>"Currency Select"</h4>
        <CurrencySelect updatable=Updatable::new(
            Currency::Eur,
            move |currency: &Currency| {
                show_update(format!("Currency set to {currency:?}"));
            },
        )/>
        <hr/>
        <h4>"Percent Input"</h4>
        <PercentInput
            updatable=Updatable::new(
                Some(0.4324),
                move |n| { show_update(format!("Percent updated -> {n:?}")) },
            )
            placeholder=Some("pct complete".to_string())
        />
        <hr/>
        <h4>"Year Input (min=1900, max=2300)"</h4>
        <YearInput
            updatable=Updatable::new(Some(1999), move |y| { show_update(format!("Year updated -> {y:?}")) })
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
            move |ycv| { show_update(format!("YearCurrencyValue set to {ycv:?}")) },
        )/>
        <h4>"Year Currency Value Input Without Values"</h4>
        <YearCurrencyValueInput updatable=Updatable::new(None, move |ycv| show_update(format!("YearCurrencyValue set to {ycv:?}")))/>
        <hr/>
        <h4>"Normal Spec With Values"</h4>
        <NormalSpecComponent updatable=Updatable::new(
            Some(NormalSpec {
                mean: 0.1,
                std_dev: 0.2,
            }),
            move |ns: &Option<NormalSpec>| show_update(format!("Normal Spec -> {ns:?}")),
        )/>
        <h4>"Normal Spec Without Values"</h4>
        <NormalSpecComponent updatable=Updatable::new(
            None,
            move |ns| {
                show_update(format!("NS Updated to -> {ns:?}"));
            },
        )/>
        <hr/>
        <h4>"Ok/Cancel"</h4>
        <OkCancelComponent
            on_ok=move || { show_update("Ok pressed".into()) }
            on_cancel=move || { show_update("Cancel pressed".into()) }
        />
        <hr/>
        <h4>"Year Range Input"</h4>
        <YearRangeInput updatable=Updatable::new(
            None,
            move |yr| {
                show_update(format!("Year Range updated -> {yr:?}"));
            },
        )/>
        <h4>"Year Value Input"</h4>
        <YearValueInput updatable=Updatable::new(
            None,
            move |yv| {
                show_update(format!("Year Value updated -> {yv:?}"));
            },
        )/>
        <h4>"Rate Curve"</h4>
        <RateCurveComponent updatable=Updatable::new(
            RateCurve::default(),
            move |rc| {
                show_update(format!("RateCurve updated -> {rc:?}"));
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
