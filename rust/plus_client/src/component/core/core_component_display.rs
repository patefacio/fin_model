//! Module for core_component_display leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::component;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;

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
    use crate::CurrencySelect;
    use crate::DateInput;
    use crate::EnumSelect;
    use crate::IntegerInput;
    use crate::Modification;
    use crate::MultiButtonData;
    use crate::MultiButtonSelect;
    use crate::NormalSpecComponent;
    use crate::NumericInput;
    use crate::OkCancelComponent;
    use crate::PercentInput;
    use crate::SelectDirection;
    use crate::SliderWithNumericInput;
    use crate::ToggleState;
    use crate::Updatable;
    use crate::ViewSide;
    use crate::YearCurrencyValueInput;
    use crate::YearInput;
    use crate::YearRangeInput;
    use crate::YearValueInput;
    use crate::YearValueSeriesComponent;
    use crate::YearValueSeriesType;
    use plus_lookup::LangSelector;

    use leptos::create_signal;
    use leptos::use_context;
    use leptos::MaybeSignal;
    use leptos::SignalGet;
    use leptos::SignalGetUntracked;
    use leptos::SignalUpdate;

    use plus_modeled::Currency;
    use plus_modeled::NormalSpec;
    use plus_modeled::StateOfResidence;
    use plus_modeled::YearCurrencyValue;
    use plus_modeled::YearRange;
    use plus_modeled::YearValue;

    let (last_update_read, last_update_write) = create_signal(String::default());
    let lang_selector = use_context::<AppContext>().unwrap().lang_selector;
    let display_currency = use_context::<AppContext>().unwrap().display_currency;
    const german_flag: &str = "🇩🇪";
    const french_flag: &str = "🇫🇷";

    let prefix_lang = move |s| {
        let flag = match lang_selector.get() {
            LangSelector::French => french_flag,
            LangSelector::German => german_flag,
            LangSelector::UsEnglish => "",
        };
        format!("{flag} {s}")
    };

    let (currency_read, _currency_write) = create_signal(
        display_currency
            .get_untracked()
            .to_currency_symbol()
            .to_string(),
    );

    let show_update = move |s: String| {
        last_update_write.update(|original| {
            tracing::debug!("{}", &s);
            *original = s;
        });
    };

    #[allow(unused)]
    let multi_button_example = move |side: ViewSide| {
        let people_lbl = prefix_lang("People");
        let valuables_lbl = prefix_lang("Valuables");
        let accounts_lbl = prefix_lang("Accounts");
        let button_data = vec![
            MultiButtonData::new(
                ButtonSelection::new(
                    "persons_button.png".into(),
                    people_lbl.clone(),
                    ToggleState::Selected,
                ),
                view! {
                        <div>
                        <h1>{people_lbl}</h1>
                        </div>
                }
                .into_view(),
            ),
            MultiButtonData::new(
                ButtonSelection::new(
                    "worths_button.png".into(),
                    valuables_lbl.clone(),
                    ToggleState::Deselected,
                ),
                view! {
                        <div>
                        <h1>{valuables_lbl}</h1>
                        </div>
                }
                .into_view(),
            ),
            MultiButtonData::new(
                ButtonSelection::new(
                    "accounts_button.png".into(),
                    accounts_lbl.clone(),
                    ToggleState::Selected,
                ),
                view! {
                    <div>
                    <h1>{accounts_lbl}</h1>
                    </div>
                }
                .into_view(),
            ),
        ];

        tracing::warn!("Instantiating MultiButtonSelect with {button_data:?}");

        view! {
            <div class="title">{move || format!("Multi-Button Select ({side:?})")}</div>
            <MultiButtonSelect button_data=button_data.clone() button_bar_side=side/>
        }
        .into_view()
    };

    let one_of_example = move || {
        use crate::OneOfComponent;
        use crate::SelectDirection;

        #[derive(Debug, Clone, Copy, PartialEq, EnumVariantNames, EnumIter)]
        enum LostInSpace {
            Will,
            Penny,
            Judy,
            Don,
            John,
            Maureen,
            DrSmith,
            Robot,
        }

        let selection = LostInSpace::Judy;

        let content_maker = move |selection: LostInSpace| {
            format!(
                "My favorite is {} {selection:?}",
                match lang_selector.get() {
                    LangSelector::French => french_flag,
                    LangSelector::German => german_flag,
                    LangSelector::UsEnglish => "",
                }
            )
            .into_view()
        };

        let label_maker = move |selection: LostInSpace| {
            format!(
                "{} {selection:?}",
                match lang_selector.get() {
                    LangSelector::French => french_flag,
                    LangSelector::German => german_flag,
                    LangSelector::UsEnglish => "",
                }
            )
            .into_view()
        };

        view! {
            <div class="title">"One Of Select"</div>
            <div class="ccd-one-of-ctnr">
                <OneOfComponent
                    updatable=Updatable::new(
                        selection,
                        move |new_selection| {
                            show_update(format!("Waltons selection updated -> {new_selection:?}"))
                        },
                    )

                    name="robinsons-ltr".to_string()
                    content_maker
                    label_maker
                />
                <OneOfComponent
                    updatable=Updatable::new(
                        selection,
                        move |new_selection| {
                            show_update(format!("Waltons selection updated -> {new_selection:?}"))
                        },
                    )

                    name="robinsons-ttb".to_string()
                    direction=SelectDirection::TopToBottom
                    content_maker
                    label_maker
                />
            </div>
        }
    };

    let number_examples = move || {
        view! {
            <div class="title">"Numbers"</div>
            <div class="ccd-numbers">
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
                                Modification::Prefix(MaybeSignal::Static("€ ".to_string())),
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
                                move |n| { show_update(format!("Percent updated -> {n:?}")) },
                            )

                            placeholder="pct complete"
                            max_len=8
                            range=Some(0.0..=0.4)
                        />
                    </div>
                </div>
            </div>
        }
    };

    let years_and_date_examples = move || {
        view! {
            <div class="title">"Years and Dates"</div>
            <div class="ccd-time">

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
        }
    };

    let select_lists_example = move || {
        view! {
            <div class="title">"Select Lists"</div>
            <div class="ccd-selects">
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
                            move |state| { show_update(format!("State updated -> {state:?}")) },
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
        }
    };

    let normal_spec_example = move || {
        view! {
            <div>
                <div class="title">"Normal Spec With Values"</div>
                <div class="ccd-normal-spec">
                    <NormalSpecComponent updatable=Updatable::new(
                        Some(NormalSpec {
                            mean: 0.1,
                            std_dev: 0.2,
                        }),
                        move |ns: &Option<NormalSpec>| show_update(format!("Normal Spec -> {ns:?}")),
                    )/>
                </div>
            </div>
        }
    };

    let ok_cancel_example = move || {
        view! {
            <div>
                <div class="title">"Ok/Cancel"</div>
                <div class="ccd-ok-cancel">
                    <OkCancelComponent on_ok_cancel=move |ok_cancel| {
                        show_update(format!("Ok/Cancel -> {ok_cancel:?}"))
                    }/>
                </div>
            </div>
        }
    };

    let sliders_example = move || {
        view! {
            <div>
                <div class="title">"Sliders"</div>
                <div class="ccd-sliders">
                    <SliderWithNumericInput
                        updatable=Updatable::new(
                            50.0,
                            move |new_value| {
                                show_update(format!("Slider Value -> {new_value:?}"))
                            },
                        )

                        label=MaybeSignal::Static("Value".into())
                        slider_id="value-sldr".into()
                        range=-100.0..=100.0
                        step=1.0
                        size=6
                    />
                    <SliderWithNumericInput
                        updatable=Updatable::new(
                            50.0,
                            move |new_value| {
                                show_update(format!("Slider Value -> {new_value:?}"))
                            },
                        )

                        label=MaybeSignal::Static("μ *".into())
                        slider_id="mean-sldr".into()
                        range=0.0..=100.0
                        step=1.0
                        size=6
                    />
                    <SliderWithNumericInput
                        updatable=Updatable::new(
                            0.0,
                            move |new_value| {
                                show_update(format!("Slider Value -> {new_value:?}"))
                            },
                        )

                        label=MaybeSignal::Static("σ *".into())
                        slider_id="std-dev-fact-sldr".into()
                        range=-6.0..=6.0
                        step=0.25
                        size=6
                    />
                </div>
            </div>
        }
    };

    let rate_curve_examples = move || {
        view! {
            <div>
                <div class="title">"Rate Curves"</div>
                <div class="ccd-rate-curve">
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
                        <h4>"Rate Curve With Data"</h4>
                        <YearValueSeriesComponent updatable=Updatable::new(
                            vec![
                                YearValue { year : 2002, value : 0.045 }, YearValue { year : 2000,
                                value : 0.06 }, YearValue { year : 1980, value : 0.025, }, YearValue
                                { year : 2000, value : - 0.0334 }
                            ],
                            move |rc| {
                                show_update(format!("Rate Curve -> {rc:?}"));
                            },
                        )/>
                    </div>

                    <div>
                        <h4>"Year Value Series With Data"</h4>
                        <YearValueSeriesComponent
                            updatable=Updatable::new(
                                vec![
                                    YearValue { year : 2002, value : 45.0 }, YearValue { year :
                                    2000, value : 6.0 }, YearValue { year : 1980, value : 25.0 },
                                    YearValue { year : 2000, value : - 334.0 }
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
        }
    };

    // ω <fn core_component_display>
    view! {
        <div class=SELF_CLASS>
            // α <plus-ccd-view>

            <div class="ccd-top-notify">
                <h4>"Last Update"</h4>
                <p>{last_update_read}</p>
            </div>

            <div class="ccd-ctnr">

                <div class="ccd-section">{move || multi_button_example(ViewSide::Left)}</div>

                <div class="ccd-section">{move || multi_button_example(ViewSide::Top)}</div>

                <div class="ccd-section-2col">{one_of_example}</div>

                <div class="ccd-section">{years_and_date_examples}</div>

                <div class="ccd-section">{number_examples}</div>

                <div class="ccd-section">{select_lists_example}</div>

                <div class="ccd-section">
                    {normal_spec_example}
                    {ok_cancel_example}
                    {sliders_example}
                    {rate_curve_examples}
                </div>
            </div>

        // ω <plus-ccd-view>
        </div>
    }
}

// α <mod-def core_component_display>
// ω <mod-def core_component_display>
