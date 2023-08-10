//! Module for component_display_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
#[allow(unused_imports)]
use leptos::log;
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
    use crate::CollectionGridComponent;
    use crate::CurrencySelect;
    use crate::DisposeTest;
    use crate::DistributionCdfComponent;
    use crate::DistributionPdfComponent;
    use crate::DistributionSpecComponent;
    use crate::EnumSelect;
    use crate::ExpandableRateComponent;
    use crate::HoldingSharedContext;
    use crate::ItemGrowth;
    use crate::ItemGrowthComponent;
    use crate::Modification;
    use crate::NormalLossComponent;
    use crate::NormalSpecComponent;
    use crate::NumericInput;
    use crate::OkCancelComponent;
    use crate::PercentInput;
    use crate::RateCurveComponent;
    use crate::SelectDirection;
    use crate::Updatable;
    use crate::UpdatablePair;
    use crate::YearCurrencyValueInput;
    use crate::YearInput;
    use crate::YearRangeInput;
    use crate::YearValueInput;
    use leptos::*;
    use leptos_dom::console_log;
    use std::collections::HashMap;
    use std::collections::HashSet;

    use plus_modeled::growth::system_growth_id::SystemId;
    use plus_modeled::Currency;
    use plus_modeled::DistributionSpec;
    use plus_modeled::DossierItemType;
    use plus_modeled::GrowthAssumption;
    use plus_modeled::GrowthItemMappings;
    use plus_modeled::Holding;
    use plus_modeled::HoldingType;
    use plus_modeled::NormalSpec;
    use plus_modeled::RateCurve;
    use plus_modeled::StateOfResidence;
    use plus_modeled::SystemGrowthId;
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

    let holdings = vec![
        Holding {
            instrument_name: "SPY".to_string(),
            quantity: 755.3,
            unit_valuation: Some(YearCurrencyValue {
                year: 2020,
                currency: 0,
                value: 440.1,
            }),
            cost_basis: 320_000.0,
            ..Default::default()
        },
        Holding {
            instrument_name: "IWM".to_string(),
            quantity: 1000.0,
            unit_valuation: Some(YearCurrencyValue {
                year: 2020,
                currency: 0,
                value: 180.1,
            }),
            cost_basis: 150_000.0,
            ..Default::default()
        },
        Holding {
            instrument_name: "NVDA".to_string(),
            quantity: 500.3,
            unit_valuation: Some(YearCurrencyValue {
                year: 2020,
                currency: 0,
                value: 420.1,
            }),
            cost_basis: 140_000.0,
            ..Default::default()
        },
    ];

    let holdings = (0..3)
        .map(|i| {
            holdings.iter().map(move |h| Holding {
                instrument_name: format!("{} -> {i}", h.instrument_name),
                ..h.clone()
            })
        })
        .flatten()
        .collect::<Vec<_>>();

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
        <div style="display: grid; grid-template-columns: 1fr 1fr 1fr;">
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
                    updatable=Updatable::new(
                        Some(32.3),
                        move |n| { show_update(format!("Number updated -> {n:?}")) },
                    )

                    range=Some(-5.0..=5.0)
                    placeholder=Some("temp".to_string())
                    size=13
                />
            </div>
            <div style="padding: 1em;">
                <div>
                    <h4>"Numeric Input With Prefix"</h4>
                    <p inner_html="
                    Provides a <em>NumericInput</em> with <em>prefix</em>.
                    "></p>
                    <NumericInput
                        updatable=Updatable::new(
                            None,
                            move |n| { show_update(format!("Input updated -> {n:?}")) },
                        )

                        modification=Some(
                            Modification::Prefix(MaybeSignal::Static("$ ".to_string())),
                        )

                        placeholder=Some("dollars".to_string())
                        size=12
                    />
                </div>
                <div>
                    <h4>"Numeric Input With Prefix Unicode"</h4>
                    <p inner_html="
                    Provides a <em>NumericInput</em> with <em>prefix</em>.
                    "></p>
                    <NumericInput
                        updatable=Updatable::new(
                            None,
                            move |n| { show_update(format!("Input updated -> {n:?}")) },
                        )

                        modification=Some(
                            Modification::Prefix(MaybeSignal::Static("€ ".to_string())),
                        )

                        placeholder=Some("euros".to_string())
                        size=12
                    />
                </div>
                <div>
                    <h4>"Numeric Input Prefix & Suffix RangeInclusive(0 to 5,000)"</h4>
                    <p inner_html="
                    Provides a <em>NumericInput</em> with <em>prefix</em> and <em>suffix</em>.
                    "></p>
                    <NumericInput
                        updatable=Updatable::new(
                            None,
                            move |n| { show_update(format!("Input updated -> {n:?}")) },
                        )

                        modification=Some(Modification::PrefixAndSuffix {
                            prefix: "€ ".into(),
                            suffix: "/yr".into(),
                        })

                        placeholder=Some("expense/yr".to_string())
                        range=Some(0.0..=5_000.0)
                        max_len=14
                        size=12
                    />
                </div>
                <div>
                    <h4>"Numeric Input (Valid when 42.0)"</h4>
                    <p inner_html="
                    Provides a <em>NumericInput</em> which is only valid via custom validator requiring value to be 42.
                    "></p>
                    <NumericInput
                        updatable=Updatable::new(
                            None,
                            move |n| { show_update(format!("Input updated -> {n:?}")) },
                        )

                        validator=Some(Box::new(|v| v == 42.0))
                        max_len=14
                        size=12
                    />
                </div>
            </div>
            <div style="padding: 1em;">
                <div>
                    <h4>"Integer Input"</h4>
                    <p inner_html="Models a single integer with similar features to <em>Numeric Input</em> without decimals.
                    <ul>
                    <li>Special characters ('k', 'm', 'b')</li>
                    <li>Optional commify</li>
                    </ul>
                    "></p>
                    <IntegerInput
                        updatable=Updatable::new(
                            None,
                            move |n| { show_update(format!("Input updated -> {n:?}")) },
                        )

                        placeholder=Some("Integer".to_string())
                        range=Some(0..=5000)
                    />
                </div>
                <div>
                    <h4>"Integer Input With Validator"</h4>
                    <p inner_html="Integer input with validator requiring value to be even"></p>
                    <IntegerInput
                        updatable=Updatable::new(
                            None,
                            move |n| { show_update(format!("Input updated -> {n:?}")) },
                        )

                        placeholder=Some("Integer".to_string())
                        validator=Some(Box::new(|v| v % 2 == 0))
                    />
                </div>
                <div>
                    <h4 inner_html="Percent Input (i.e. suffix `%`) <strong>max_len=8</strong> RangeInclusive(0% to 40%)"></h4>
                    <p inner_html="
                    Provides a <em>NumericInput</em> with a percent suffix modification.
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
                        updatable=Updatable::new(
                            None,
                            move |y| { show_update(format!("Year updated -> {y:?}")) },
                        )

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
                        updatable=Updatable::new(
                            None,
                            move |y| { show_update(format!("Year updated -> {y:?}")) },
                        )

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
                        updatable=Updatable::new(
                            Some(2030),
                            move |y| { show_update(format!("Year updated -> {y:?}")) },
                        )

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
                <YearCurrencyValueInput updatable=Updatable::new(
                    None,
                    move |ycv| show_update(format!("YearCurrencyValue set to {ycv:?}")),
                )/>
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
        <div style="display: grid; grid-template-columns: 1fr">
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
        <OkCancelComponent on_ok_cancel=move |ok_cancel| {
            show_update("Ok/Cancel -> {ok_cancel:?}".into())
        }/>
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
                    YearValue { year : 2002, value : 0.045 }, YearValue { year : 2000, value : 0.06
                    }, YearValue { year : 1980, value : 0.025, }, YearValue { year : 2000, value : -
                    0.0334 }
                ],
            },
            move |rc| {
                show_update(format!("Rate Curve -> {rc:?}"));
            },
        )/>
        <h4>"Rate Curve with Expandable View"</h4>
        <ExpandableRateComponent updatable=Updatable::new(
            RateCurve::default(),
            move |rc| {
                show_update(format!("RateCurve updated -> {rc:?}"));
            },
        )/>
        <hr/>
        <h4>"Collection Grid Component<Holding>"</h4>
        <CollectionGridComponent
            updatable=UpdatablePair::new(
                holdings,
                HoldingSharedContext {
                    symbol_growth_map: HashMap::from([
                        ("IBM".into(), ItemGrowth::default()),
                        ("MSFT".into(), ItemGrowth::default()),
                    ]),
                    symbol_names: HashSet::from(["IBM".into(), "MSFT".into()]),
                },
                move |holding_list| {
                    show_update(format!("Holding list updated -> {holding_list:?}"));
                },
            )

            read_only=false
        />
        <hr/>
        <div style="margin: 2rem;">"Dispose Test"</div>
        <Show when=move || (read_count.get() % 2) == 0 fallback=|_| "Nothing">
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
