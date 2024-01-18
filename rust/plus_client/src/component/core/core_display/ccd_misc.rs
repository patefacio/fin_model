//! Module for ccd_misc leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::component;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::WriteSignal;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Miscellaneous components [NormalSpecComponent], [OkCancelComponent]
/// [SliderWithNumericInput], [YearValueSeriesComponent]
///
///   * **show_update** - Function to display state updates
///   * _return_ - View for ccd_misc
#[component]
pub fn CcdMisc(
    /// Function to display state updates
    show_update: WriteSignal<String>,
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-cm; ccd-section";
    let component_id = crate::component_id!("`CcdMisc`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn ccd_misc>

    use crate::AppContext;
    use crate::NormalSpecComponent;
    use crate::OkCancelComponent;
    use crate::SliderWithNumericInput;
    use crate::Updatable;
    use crate::YearValueSeriesComponent;
    use crate::YearValueSeriesType;
    use leptos::create_signal;
    use leptos::expect_context;
    use leptos::MaybeSignal;
    use leptos::SignalGetUntracked;
    use leptos::SignalSet;
    use plus_modeled::NormalSpec;
    use plus_modeled::YearValue;
    use std::rc::Rc;

    let display_currency = expect_context::<Rc<AppContext>>().display_currency;

    let (currency_read, _currency_write) = create_signal(
        display_currency
            .get_untracked()
            .to_currency_symbol()
            .to_string(),
    );

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
                        move |ns: &Option<NormalSpec>| {
                            show_update.set(format!("Normal Spec -> {ns:?}"))
                        },
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
                        show_update.set(format!("Ok/Cancel -> {ok_cancel:?}"))
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
                                show_update.set(format!("Slider Value -> {new_value:?}"))
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
                                show_update.set(format!("Slider Value -> {new_value:?}"))
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
                                show_update.set(format!("Slider Value -> {new_value:?}"))
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
                                show_update.set(format!("RateCurve updated -> {rc:?}"));
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
                                show_update.set(format!("Rate Curve -> {rc:?}"));
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
                                    show_update.set(format!("Rate Curve -> {rc:?}"));
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

    // ω <fn ccd_misc>
    view! {
        <div class=SELF_CLASS>
            // α <plus-cm-view>

            {normal_spec_example} {ok_cancel_example} {sliders_example} {rate_curve_examples}

        // ω <plus-cm-view>
        </div>
    }
}

// α <mod-def ccd_misc>
// ω <mod-def ccd_misc>
