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
        <h3>"Numbers"</h3>
        <div style="display: grid; grid-template-columns: 1fr 1fr 1fr 1fr;">
            <div style="padding: 1em;">
                <h4>"Numeric Input Range(-5.0,5.0)"</h4>
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
                <li>Specify Range</li>
                <li>max_len: maps to html attribute <em>maxlength</em></li>
                <li>size: maps to html attribute <em>size</em></li>
                </ul>
                "></p>
                <NumericInput
                    updatable=Updatable::new(Some(32.3), move |n| { show_update(format!("Number updated -> {n:?}")) })
                    range=Some(-5.0..=5.0)
                    placeholder=Some("temp".to_string())
                    size=13
                />
            </div>
            <div style="padding: 1em;">
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
                    <h4>"Numeric Input Prefix & Suffix RangeInclusive(0 to 5,000)"</h4>
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
                        range=Some(0.0..=5_000.0)
                        max_len=14
                        size=15
                    />
                </div>
            </div>
            <div style="padding: 1em;">
                <h4>"Integer Input on Range 0..5000"</h4>
                <p inner_html="Models a single integer with similar features to <em>Numeric Input</em> without decimals.
                <ul>
                <li>Special characters ('k', 'm', 'b')</li>
                <li>Optional commify</li>
                </ul>
                "></p>
                <IntegerInput
                    updatable=Updatable::new(None, move |n| { show_update(format!("Input updated -> {n:?}")) })
                    placeholder=Some("Integer".to_string())
                    range=Some(0..=5000)
                    live_clamp=true
                />
            </div>
            <div style="padding: 1em;">
                <h4 inner_html="Percent Input (i.e. suffix `%`) <strong>max_len=8</strong> RangeInclusive(0% to 40%)"></h4>
                <p inner_html="
                Provides a <em>NumericInput<em> with a percent suffix modification.
                "></p>
                <PercentInput
                    updatable=Updatable::new(
                        Some(0.0315),
                        move |n| { show_update(format!("Percent updated -> {n:?}")) },
                    )
                    placeholder=Some("pct complete".to_string())
                    max_len=8
                    range=Some(0.0..=0.4)
                />
            </div>
            <hr/>
        </div>
        <hr/>
        <h3>"Years and Dates"</h3>
        <div style="display: grid; grid-template-columns: 2fr 1.5fr">
            <div style="padding: 1em;">
                <h4>"Year Input"</h4>
                <p inner_html="Year Input - Supports range and provides a <em>live</em> clamp type
                functionality. With <em>live clamp</em> true, if the user enters a year with the proper number
                of digits it will be within range. This is achieved by modifying user input on the fly to
                stay within range. As this may be disorienting it is optional.
                "></p>
                <div>
                    <h5>"With Clamp, RangeInclusive(1900 to 2300)"</h5>
                    <YearInput
                        updatable=Updatable::new(None, move |y| { show_update(format!("Year updated -> {y:?}")) })
                        placeholder=Some("year".to_string())
                        live_clamp=true
                        year_range=YearRange {
                            start: 1900,
                            end: 2300,
                        }
                    />
                </div>
                <div>
                    <h5>"Without Clamp, RangeInclusive(1900 to 2300)"</h5>
                    <YearInput
                        updatable=Updatable::new(None, move |y| { show_update(format!("Year updated -> {y:?}")) })
                        placeholder=Some("year".to_string())
                        year_range=YearRange {
                            start: 1900,
                            end: 2300,
                        }
                    />
                </div>
                <div>
                    <h4>"Without Clamp, RangeInclusive(2020 to 2030) With Initial Valid Year"</h4>
                    <YearInput
                        updatable=Updatable::new(Some(2030), move |y| { show_update(format!("Year updated -> {y:?}")) })
                        placeholder=Some("year".to_string())
                        year_range=YearRange {
                            start: 2020,
                            end: 2030,
                        }
                    />
                </div>
            </div>
            <div style="padding: 1em;">
                <h4>"Date Input (Range (1990 -> 2070))"</h4>
                <p inner_html="
                <p>
                A date input component with following features:
                </p>
                <ul>
                <li>Any non numeric character (e.g. space or '/' advances from month or day field)</li>
                <li>Tab from month to day to year and shift-tab for reverse</li>
                <li>Year range <strong>with clamp</strong> is supported</li>
                <li>Signals on complete/valid input</li>
                <li>Class `invalid` if input is not valid</li>
                </ul>
                "></p>
                <DateInput
                    updatable=Updatable::new(
                        None,
                        move |n| {
                            show_update(format!("Date updated -> {n:?}"));
                        },
                    )
                    placeholder=Some("MM/DD/YYYY".to_string())
                    year_range=Some(YearRange {
                        start: 1990,
                        end: 2070,
                    })
                />
            </div>
        </div>
        <hr/>
        <h3>"Select Lists"</h3>
        <div style="display: grid; grid-template-columns: 1fr 1fr 1fr">
            <div style="padding: 1em;">
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
                    column_count=5
                />
            </div>
            <div style="padding: 1em;">
                <h4>"Mutli-Column Select (Left To Right)"</h4>
                <EnumSelect
                    updatable=Updatable::new(
                        StateOfResidence::Fl,
                        move |state| { show_update(format!("State updated -> {state:?}")) },
                    )
                    direction=SelectDirection::LeftToRight
                    column_count=5
                />
            </div>
            <div style="padding: 1em;">
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
                        std_dev: 0.002,
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
        }>{move || format!("Inc({})", read_count.get())}</button>
    }

    // ω <fn component_display_component>
}

// α <mod-def component_display_component>
// ω <mod-def component_display_component>
