//! Module for core_component_display leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::IntoAttribute;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Component that displays various core components.
/// Useful for seeing how they work and testing.
///
///   * _return_ - View for core_component_display
#[component]
pub fn CoreComponentDisplay() -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-ccd";
    crate::log_component!("`CoreComponentDisplay`");
    // α <fn core_component_display>

    use crate::AppContext;
    use crate::ButtonSelection;
    use crate::CssClasses;
    use crate::CurrencySelect;
    use crate::DateInput;
    use crate::EnumSelect;
    use crate::IntegerInput;
    use crate::Modification;
    use crate::MultiButtonSelect;
    use crate::MultiButtonData;
    use crate::NormalSpecComponent;
    use crate::NumericInput;
    use crate::OkCancelComponent;
    use crate::PercentInput;
    use crate::SelectDirection;
    use crate::ToggleState;
    use crate::Updatable;
    use crate::ViewSide;
    use crate::YearCurrencyValueInput;
    use crate::YearInput;
    use crate::YearRangeInput;
    use crate::YearValueInput;
    use crate::YearValueSeriesComponent;
    use crate::YearValueSeriesType;
    use leptos::*;

    use plus_modeled::Currency;
    use plus_modeled::NormalSpec;
    use plus_modeled::StateOfResidence;
    use plus_modeled::YearCurrencyValue;
    use plus_modeled::YearRange;
    use plus_modeled::YearValue;

    let (last_update_read, last_update_write) = create_signal(String::default());
    let display_currency = use_context::<AppContext>().unwrap().display_currency;
    let (currency_read, currency_write) = create_signal(
        display_currency
            .get_untracked()
            .to_currency_symbol()
            .to_string(),
    );

    let show_update = move |s: String| {
        last_update_write.update(|original| {
            log!("{}", &s);
            *original = s;
        });
    };

    let multi_button_example = move || {
        let button_data = vec![
            MultiButtonData::new(
                ButtonSelection::new(
                    "persons_button.png".into(),
                    "People".into(),
                    ToggleState::Selected,
                ),
                view! {
                        <div>
                        <h1>"People"</h1>
                        </div>
                }
                .into_view(),
            ),
            MultiButtonData::new(
                ButtonSelection::new(
                    "worths_button.png".into(),
                    "Valuables".into(),
                    ToggleState::Selected,
                ),
                view! {
                        <div>
                        <h1>"Valuables"</h1>
                        </div>
                }
                .into_view(),
            ),
            MultiButtonData::new(
                ButtonSelection::new(
                    "accounts_button.png".into(),
                    "Accounts".into(),
                    ToggleState::Deselected,
                ),
                view! {
                    <div>
                    <h1>"Accounts"</h1>
                    </div>
                }
                .into_view(),
            ),
        ];

        tracing::warn!("Instantiating MultiButtonSelect with {button_data:?}");

        view! { <MultiButtonSelect button_data=button_data.clone() button_bar_side=ViewSide::Left/> }
        .into_view()
    };

    // ω <fn core_component_display>
    view! {
        <div class=SELF_CLASS>
            // α <plus-ccd-view>

            <div class=CssClasses::CcdTopNotify.to_string()>
                <h4>"Last Update"</h4>
                <p>{last_update_read}</p>
            </div>

            <div class=CssClasses::CcdCtnr.to_string()>
              //  {multi_button_example}

                <div>
                    <div class=CssClasses::Title.to_string()>"Numbers"</div>
                    <div class=CssClasses::CcdNumbers.to_string()>
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
                            <li>Up/Down Arrow Increment/Decrement digit to left</li>
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
                                placeholder="temp"
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

                                    placeholder="dollars"
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
                                        Modification::Prefix(
                                            MaybeSignal::Static("€ ".to_string()),
                                        ),
                                    )

                                    placeholder="euros"
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

                                    placeholder="expense/yr"
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

                                    placeholder="Integer"
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

                                    placeholder="Integer"
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
                                        move |n| {
                                            show_update(format!("Percent updated -> {n:?}"))
                                        },
                                    )

                                    placeholder="pct complete"
                                    max_len=8
                                    range=Some(0.0..=0.4)
                                />
                            </div>
                        </div>
                    </div>
                </div>

                <div>
                    <div class=CssClasses::Title.to_string()>"Years and Dates"</div>
                    <div class=CssClasses::CcdTime.to_string()>

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

                                    placeholder="year"
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

                                    placeholder="year"
                                    year_range=YearRange {
                                        start: 1900,
                                        end: 2300,
                                    }
                                />

                            </div>
                            <div>
                                <h4>
                                    "Without Clamp, RangeInclusive(2020 to 2030) With Initial Valid Year"
                                </h4>
                                <YearInput
                                    updatable=Updatable::new(
                                        Some(2030),
                                        move |y| { show_update(format!("Year updated -> {y:?}")) },
                                    )

                                    placeholder="year"
                                    year_range=YearRange {
                                        start: 2020,
                                        end: 2030,
                                    }
                                />

                            </div>

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

                            <div>
                                <h4>"Year Currency Value Input Without Values"</h4>
                                <YearCurrencyValueInput updatable=Updatable::new(
                                    None,
                                    move |ycv| show_update(
                                        format!("YearCurrencyValue set to {ycv:?}"),
                                    ),
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
                                    move |ycv| {
                                        show_update(format!("YearCurrencyValue set to {ycv:?}"))
                                    },
                                )/>
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

                                year_range=Some(YearRange {
                                    start: 1990,
                                    end: 2070,
                                })
                            />

                        </div>
                    </div>
                </div>

                <div>
                    <div class=CssClasses::Title.to_string()>"Select Lists"</div>
                    <div class=CssClasses::CcdSelects.to_string()>
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
                            <li>EnumSelect supports filtering (e.g. this has filtered out <strong>Il</strong> and <strong>Ca</strong>)</li>
                            </ul>
                            "></p>
                            <EnumSelect
                                updatable=Updatable::new(
                                    StateOfResidence::Fl,
                                    move |state| {
                                        show_update(format!("State updated -> {state:?}"))
                                    },
                                )

                                direction=SelectDirection::TopToBottom
                                column_count=5
                                filter=Some(
                                    std::boxed::Box::new(|&e| {
                                        e != StateOfResidence::Il && e != StateOfResidence::Ca
                                    }),
                                )
                            />

                        </div>
                        <div style="padding: 1em;">
                            <h4>"Mutli-Column Select (Left To Right)"</h4>
                            <EnumSelect
                                updatable=Updatable::new(
                                    StateOfResidence::Fl,
                                    move |state| {
                                        show_update(format!("State updated -> {state:?}"))
                                    },
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
                </div>

                <div>
                    <div>
                        <div class=CssClasses::Title.to_string()>"Normal Spec With Values"</div>
                        <div class=CssClasses::CcdNormalSpec.to_string()>
                            <NormalSpecComponent updatable=Updatable::new(
                                Some(NormalSpec {
                                    mean: 0.1,
                                    std_dev: 0.2,
                                }),
                                move |ns: &Option<NormalSpec>| show_update(
                                    format!("Normal Spec -> {ns:?}"),
                                ),
                            )/>
                        </div>
                    </div>

                    <div>
                        <div class=CssClasses::Title.to_string()>"Ok/Cancel"</div>
                        <div class=CssClasses::CcdOkCancel.to_string()>
                            <OkCancelComponent on_ok_cancel=move |ok_cancel| {
                                show_update(format!("Ok/Cancel -> {ok_cancel:?}"))
                            }/>
                        </div>
                    </div>

                    <div>
                        <div class=CssClasses::Title.to_string()>"Rate Curves"</div>
                        <div class=CssClasses::CcdRateCurve.to_string()>
                            <div>
                                <h4>"Empty Rate Curve"</h4>
                                <YearValueSeriesComponent updatable=Updatable::new(
                                    Vec::new(),
                                    move |rc| {
                                        show_update(format!("RateCurve updated -> {rc:?}"));
                                    },
                                )/>
                            </div>
                            <div>
                                <h4>"Rate Curve with Sample Data"</h4>
                                <YearValueSeriesComponent updatable=Updatable::new(
                                    vec![
                                        YearValue { year : 2002, value : 0.045 }, YearValue { year :
                                        2000, value : 0.06 }, YearValue { year : 1980, value :
                                        0.025, }, YearValue { year : 2000, value : - 0.0334 }
                                    ],
                                    move |rc| {
                                        show_update(format!("Rate Curve -> {rc:?}"));
                                    },
                                )/>
                            </div>

                            <div>
                                <h4>"Year Value Series with Sample Data"</h4>
                                <YearValueSeriesComponent
                                    updatable=Updatable::new(
                                        vec![
                                            YearValue { year : 2002, value : 45.0 }, YearValue { year :
                                            2000, value : 6.0 }, YearValue { year : 1980, value : 25.0
                                            }, YearValue { year : 2000, value : - 334.0 }
                                        ],
                                        move |rc| {
                                            show_update(format!("Rate Curve -> {rc:?}"));
                                        },
                                    )

                                    series_type=YearValueSeriesType::MarketValue {
                                        currency_read: currency_read,
                                    }
                                />

                            </div>
                        </div>
                    </div>
                </div>
            </div>

        // ω <plus-ccd-view>
        </div>
    }
}

// α <mod-def core_component_display>
// ω <mod-def core_component_display>
