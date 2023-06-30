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
    use crate::EnumSelect;
    use crate::NormalSpecComponent;
    use crate::NumericInput;
    use crate::Modification;
    use crate::OkCancelComponent;
    use crate::PercentInput;
    use crate::RateCurveComponent;
    use crate::SelectDirection;
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
    use plus_modeled::StateOfResidence;
    use plus_modeled::NormalSpec;
    use plus_modeled::RateCurve;
    use plus_modeled::YearCurrencyValue;
    use plus_modeled::YearRange;
    use plus_modeled::YearValue;

    use crate::DateInput;
    use crate::IntegerInput;

    let (read_count, write_count) = create_signal(cx, 0);
    leptos_dom::console_log(&format!("App cx({cx:?}"));
    let (last_update, set_last_update) = create_signal(cx, String::default());

    let show_update = move |s: String| {
        set_last_update.update(|original| {
            console_log(&s);
            *original = s;
        });
    };

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
        <h4>"Date Input"</h4>
        <DateInput
            updatable=Updatable::new(
                None,
                move |n| {
                    console_log(&format!("Date updated -> {n:?}"));
                },
            )
            placeholder=Some("MM/DD/YYYY".to_string())
        />
        <hr/>
        <div style="display: grid; grid-template-columns: 1fr 1fr 1fr 1fr">
            <div>
                <h4>"Numeric Input"</h4>
                <p>"Models a single floating point number."</p>
                <h5>"Special Features"</h5>
                <p inner_html="
                <ul>
                <li>Auto commify</li>
                <li>Special Characters (e.g. type in `2.5k`)</li>
                <ul>
                <li>'k' -> Convert to thousands</li>
                <li>'m' -> Convert to millions</li>
                <li>'b' -> Convert to billions</li>
                </ul>
                <li>Support for <em>Prefix</em> and/or <em>Suffix</em> (See <em>Percent</em> 
                for suffix example and <em>NormalSpec</em> for prefix and suffix)
                </li>
                </ul>
                "></p>
                <NumericInput updatable=Updatable::new(Some(32.3), move |n| { show_update(format!("Number updated -> {n:?}")) })/>
            </div>
            <div>
                <div>
                    <h4>"Numeric Input With Prefix"</h4>
                    <p inner_html="
                    Provides a <em>NumericInput<em> with <em>prefix</em>.
                    "></p>
                    <NumericInput
                        updatable=Updatable::new(None, move |n| { show_update(format!("Input updated -> {n:?}")) })
                        modification=Some(Modification::Prefix("$ ".to_string()))
                        placeholder=Some("dollars".to_string())
                        size=15
                    />
                </div>
                <div>
                    <h4>"Numeric Input With Prefix Unicode"</h4>
                    <p inner_html="
                    Provides a <em>NumericInput<em> with <em>prefix</em>.
                    "></p>
                    <NumericInput
                        updatable=Updatable::new(None, move |n| { show_update(format!("Input updated -> {n:?}")) })
                        modification=Some(Modification::Prefix("€ ".to_string()))
                        placeholder=Some("euros".to_string())
                        size=15
                    />
                </div>
                <div>
                    <h4>"Numeric Input With Prefix & Suffix"</h4>
                    <p inner_html="
                    Provides a <em>NumericInput<em> with <em>prefix</em> and <em>suffix</em>.
                    "></p>
                    <NumericInput
                        updatable=Updatable::new(None, move |n| { show_update(format!("Input updated -> {n:?}")) })
                        modification=Some(Modification::PrefixAndSuffix {
                            prefix: "€ ".into(),
                            suffix: "/yr".into(),
                        })
                        placeholder=Some("expense/yr".to_string())
                        size=15
                    />
                </div>
            </div>
            <div>
                <h4>"Integer Input"</h4>
                <p inner_html="Models a single integer with similar features to <em>Numeric Input</em> without decimals"></p>
                <IntegerInput
                    updatable=Updatable::new(
                        None,
                        move |n| {
                            console_log(&format!("Integer updated -> {n:?}"));
                        },
                    )
                    placeholder=Some("Integer".to_string())
                />
            </div>
            <div>
                <h4>"Percent Input (i.e. suffix `%`)"</h4>
                <p inner_html="
                Provides a <em>NumericInput<em> with a percent suffix modification.
                "></p>
                <PercentInput
                    updatable=Updatable::new(
                        Some(0.4324),
                        move |n| { show_update(format!("Percent updated -> {n:?}")) },
                    )
                    placeholder=Some("pct complete".to_string())
                />
            </div>
            <hr/>
        </div>
        <hr/>
        <div style="display: grid; grid-template-columns: 1fr 1fr 1fr">
            <div>
                <h4>"Mutli-Column Select (Top To Bottom)"</h4>
                <p inner_html="
                <p>
                A component that supports a drop down selection which can span multiple columns.
                The purpose is to be able to better style large selection lists that otherwise would
                be very long vertical lists. The features include:
                </p>
                <ul>
                <li>Navigating the selections with left/right and up/down arrow keys</li>
                <li> Tab support for each entry</li>
                <li>Focus <strong>auto-selects</strong> the item so tabbing and navigating to selection makes it the
                current selection</li>
                <li>Accept the selection <strong>and signals</strong> on Enter, Space</li>
                <li>No-Op on Escape</li>
                <li>Display Selections Left-To-Right or Top-To-Bottom</li>
                </ul>
                "></p>
                <EnumSelect
                    updatable=Updatable::new(
                        StateOfResidence::Fl,
                        move |state| { show_update(format!("State updated -> {state:?}")) },
                    )
                    direction=SelectDirection::TopToBottom
                />
            </div>
            <div>
                <h4>"Mutli-Column Select (Left To Right)"</h4>
                <EnumSelect
                    updatable=Updatable::new(
                        StateOfResidence::Fl,
                        move |state| { show_update(format!("State updated -> {state:?}")) },
                    )
                    direction=SelectDirection::LeftToRight
                />
            </div>
            <div>
                <h4>"Currency Select"</h4>
                <p inner_html="
                <em>Two Field, Three Column Example of <strong>MultiColumnSelect</strong></em>.
                "></p>
                <CurrencySelect updatable=Updatable::new(
                    Currency::Eur,
                    move |currency: &Currency| {
                        show_update(format!("Currency set to {currency:?}"));
                    },
                )/>
            </div>
        </div>
        <hr/>
        <div style="display: grid; grid-template-columns: 1fr 1fr">
            <div>
                <h4>"Year Input (min=1900, max=2300) With No Data"</h4>
                <p inner_html="Year Input - Supports range and provides a <em>live</em> clamp type
                functionality. Currently invalid/incomplete is is indicated by css <strong>redish</strong> text field."></p>
                <YearInput
                    updatable=Updatable::new(None, move |y| { show_update(format!("Year updated -> {y:?}")) })
                    placeholder=Some("year".to_string())
                />
            </div>
            <div>
                <h4>"Year Input (min=1900, max=2300) With Data"</h4>
                <YearInput
                    updatable=Updatable::new(Some(1999), move |y| { show_update(format!("Year updated -> {y:?}")) })
                    placeholder=Some("year".to_string())
                />
            </div>
        </div>
        <hr/>
        <div style="display: grid; grid-template-columns: 1fr 1fr">
            <div>
                <h4>"Year Currency Value Input Without Values"</h4>
                <YearCurrencyValueInput updatable=Updatable::new(None, move |ycv| show_update(format!("YearCurrencyValue set to {ycv:?}")))/>
            </div>
            <div>
                <h4>"Year Currency Value Input With Values"</h4>
                <YearCurrencyValueInput updatable=Updatable::new(
                    Some(YearCurrencyValue {
                        year: 1998,
                        currency: Currency::Jpy as i32,
                        value: 25.55,
                    }),
                    move |ycv| { show_update(format!("YearCurrencyValue set to {ycv:?}")) },
                )/>
            </div>
        </div>
        <hr/>
        <hr/>
        <div style="display: grid; grid-template-columns: 1fr 1fr">
            <div>
                <h4>"Normal Spec Without Values"</h4>
                <NormalSpecComponent updatable=Updatable::new(
                    None,
                    move |ns| {
                        show_update(format!("NS Updated to -> {ns:?}"));
                    },
                )/>
            </div>
            <div>
                <h4>"Normal Spec With Values"</h4>
                <NormalSpecComponent updatable=Updatable::new(
                    Some(NormalSpec {
                        mean: 0.1,
                        std_dev: 0.2,
                    }),
                    move |ns: &Option<NormalSpec>| show_update(format!("Normal Spec -> {ns:?}")),
                )/>
            </div>
        </div>
        <hr/>
        <h4>"Ok/Cancel"</h4>
        <OkCancelComponent
            on_ok=move || { show_update("Ok pressed".into()) }
            on_cancel=move || { show_update("Cancel pressed".into()) }
        />
        <hr/>
        <div style="display: grid; grid-template-columns: 1fr 1fr">
            <div>
                <h4>"Year Range Input"</h4>
                <YearRangeInput updatable=Updatable::new(
                    None,
                    move |yr| {
                        show_update(format!("Year Range updated -> {yr:?}"));
                    },
                )/>
            </div>
            <div>
                <h4>"Year Value Input"</h4>
                <YearValueInput updatable=Updatable::new(
                    None,
                    move |yv| {
                        show_update(format!("Year Value updated -> {yv:?}"));
                    },
                )/>
            </div>
        </div>
        <hr/>
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
                    YearValue { year : 2002, value : 0.045 }, YearValue { year : 2000, value :
                    0.06 }, YearValue { year : 1980, value : 0.025, }, YearValue { year : 2000,
                    value : - 0.0334 }
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
