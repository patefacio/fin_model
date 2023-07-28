//! Module for normal_spec_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Modification;
use crate::NumericInput;
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::Show;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::core::NormalSpec;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a normal specification -> N(mu, sigma).
///
///   * **cx** - Context
///   * **updatable** - The normal spec being edited
///   * **non_negative_mean** - If set, negative values are disallowed for the mean.
///   * _return_ - View for normal_spec_component
#[component]
pub fn NormalSpecComponent(
    /// Context
    cx: Scope,
    /// The normal spec being edited
    updatable: Updatable<Option<NormalSpec>>,
    /// If set, negative values are disallowed for the mean.
    #[prop(default = false)]
    non_negative_mean: bool,
) -> impl IntoView {
    // α <fn normal_spec_component>

    use crate::utils::historic_risk_return::HISTORIC_RISK_RETURN_SAMPLES;
    use crate::DistributionCdf;
    use crate::DistributionCdfComponent;
    use crate::DistributionPdf;
    use crate::DistributionPdfComponent;
    use crate::HistoricRiskReturn;
    use crate::HistoricRiskReturnComponent;
    use crate::HistoricRiskReturnPlot;
    use leptos::create_signal;
    use leptos::For;
    use leptos::IntoAttribute;
    use leptos::MaybeSignal;
    use leptos::create_rw_signal;
    use leptos::SignalGet;
    use leptos::SignalUpdate;
    use leptos::SignalWith;
    use leptos::WriteSignal;

    enum GraphDisplay {
        Pdf,
        Cdf,
        Historic,
    }

    struct NormalBits {
        mean: Option<f64>,
        std_dev: Option<f64>,
        updatable: Updatable<Option<NormalSpec>>,
        pdf_drawing_svg: String,
        cdf_drawing_svg: String,
        cdf_output: Option<f64>,
    }

    let initial_mean = updatable
        .value
        .as_ref()
        .map(|option_of_normal_spec| option_of_normal_spec.mean * 100.0);

    let initial_std_dev = updatable
        .value
        .as_ref()
        .map(|normal_spec| normal_spec.std_dev * 100.0);
    let initial_loss = Some(0.0);
    let initial_output = updatable
        .value
        .as_ref()
        .map(|normal_spec| normal_spec.cdf_sigmoid_approx(0.0));

    let pdf_drawing_svg = updatable
        .value
        .as_ref()
        .map(|ns| ns.get_pdf_chart(200))
        .unwrap_or_default();

    let cdf_drawing_svg = updatable
        .value
        .as_ref()
        .map(|ns| ns.get_cdf_chart(200))
        .unwrap_or_default();

    let (spec_signal, set_spec_signal) = create_signal(
        cx,
        updatable.value.as_ref().map(|ns| *ns).unwrap_or_default(),
    );

    let (normal_spec, set_normal_spec) = create_signal(cx, updatable.value.unwrap_or_default());

    let (normal_bits, set_normal_bits) = create_signal(
        cx,
        NormalBits {
            mean: initial_mean,
            std_dev: initial_std_dev,
            updatable,
            pdf_drawing_svg: initial_mean
                .and_then(|mean| {
                    initial_std_dev.map(|std_dev| NormalSpec { mean, std_dev }.get_pdf_chart(200))
                })
                .unwrap_or_default(),
            cdf_drawing_svg: initial_mean
                .and_then(|mean| {
                    initial_std_dev.map(|std_dev| NormalSpec { mean, std_dev }.get_cdf_chart(200))
                })
                .unwrap_or_default(),
            cdf_output: initial_output.unwrap_or(None),
        },
    );

    let make_updates =
        move |normal_bits: &mut NormalBits, mut new_normal: NormalSpec, new_input: Option<f64>| {
            new_normal.mean /= 100.0;
            new_normal.std_dev /= 100.0;
            normal_bits.pdf_drawing_svg = new_normal.get_pdf_chart(400);
            normal_bits.cdf_drawing_svg = new_normal.get_cdf_chart(400);
            set_spec_signal.update(|ns| {
                *ns = NormalSpec {
                    mean: new_normal.mean,
                    std_dev: new_normal.std_dev,
                };
                log!("New Specs to Graph")
            });
            new_normal.mean *= 100.0;
            new_normal.std_dev *= 100.0;
            // Before signalling undo the 100x
            normal_bits
                .updatable
                .update_and_then_signal(move |normal_spec| *normal_spec = Some(new_normal));
            match (new_input, normal_bits.updatable.value) {
                (Some(_), Some(_)) => {
                    normal_bits.cdf_output = normal_bits
                        .updatable
                        .value
                        .unwrap()
                        .cdf_sigmoid_approx(new_input.unwrap())
                }
                _ => (),
            }
        };

    let mean_updatable = Updatable::new(initial_mean, move |mean| {
        set_normal_bits.update(|normal_bits| {
            normal_bits.mean = *mean;
            if let (Some(mean), Some(std_dev)) = (normal_bits.mean, normal_bits.std_dev) {
                make_updates(normal_bits, NormalSpec { mean, std_dev }, None);
            }
        });
    });

    let std_dev_updatable = Updatable::new(initial_std_dev, move |std_dev| {
        set_normal_bits.update(|normal_bits| {
            normal_bits.std_dev = *std_dev;
            if normal_bits.std_dev.unwrap_or_default() == 0.0 {
                normal_bits.std_dev = None
            };
            if let (Some(mean), Some(std_dev)) = (normal_bits.mean, normal_bits.std_dev) {
                make_updates(normal_bits, NormalSpec { mean, std_dev }, None);
            }
        });
    });

    let initial_loss_vec = vec![10.0, 5.0, 1.0, 0.0, -1.0, -5.0, -10.0];
    let (loss_indices, set_indices) = create_signal(cx, initial_loss_vec);

    let loss_updatable = Updatable::new(initial_loss, move |loss| {
        set_normal_bits.update(|normal_bits| {
            match (
                loss,
                normal_bits.std_dev,
                normal_bits.mean,
                normal_bits.updatable.value,
            ) {
                (Some(_), Some(_), Some(_), Some(_)) => {
                    normal_bits.cdf_output = normal_bits
                        .updatable
                        .value
                        .unwrap()
                        .cdf_sigmoid_approx(loss.unwrap());
                    make_updates(
                        normal_bits,
                        NormalSpec {
                            mean: normal_bits.mean.unwrap(),
                            std_dev: normal_bits.std_dev.unwrap(),
                        },
                        None,
                    );
                }
                _ => (),
            };
        });
        set_indices.update(|vec| {
            *vec = vec![10.0, 5.0, 1.0, 0.0, -1.0, -5.0, -10.0];
            vec.push(loss.unwrap_or_default());
        })
    });

    let (enable_graphs, set_disable_graphs) = create_signal(cx, GraphDisplay::Pdf);
    let show_pdf = move |_| {
        set_disable_graphs.update(|v| {
            *v = GraphDisplay::Pdf;
            log!("PDF Shown");
        })
    };
    let show_cdf = move |_| {
        set_disable_graphs.update(|v| {
            *v = GraphDisplay::Cdf;
            log!("CDF Shown");
        })
    };
    let show_hist = move |_| {
        set_disable_graphs.update(|v| {
            *v = GraphDisplay::Historic;
            log!("Historic Shown");
        })
    };

    view! { cx,
        <fieldset class="nsg">
            <div class="form">
                <div style="display: grid">
                    "N("
                    <NumericInput
                        placeholder=Some("mean".to_string())
                        modification=Some(Modification::PrefixAndSuffix {
                            prefix: "μ=".into(),
                            suffix: "%".into(),
                        })
                        non_negative=non_negative_mean
                        updatable=mean_updatable
                        size=9
                        max_len=9
                    /> ", "
                    <NumericInput
                        placeholder=Some("std. dev".to_string())
                        modification=Some(Modification::PrefixAndSuffix {
                            prefix: "σ=".into(),
                            suffix: "%".into(),
                        })
                        non_negative=true
                        updatable=std_dev_updatable
                        size=9
                        max_len=9
                    /> ")" <button style="margin-left: 0.2rem;">"…"</button>
                </div>
            </div>
            <hr/>
            <div style="display: grid; grid-template-columns: 1fr 1fr;">
                <div style="text-align: right;" class="header">
                    "Gain Less Than(%)"
                </div>
                <div style="text-align: right;" class="header">
                    "Probability(%)"
                </div>
                <For
                    each=move || loss_indices.get()
                    key=|item| { format!("{item:?}") }
                    view=move |cx, cdf_input| {
                        view! { cx,
                            <div style="text-align: right;" inner_html=move || { format!("{:.2}%", cdf_input) }></div>
                            <div
                                style="text-align: right;"
                                inner_html=move || {
                                    normal_bits
                                        .with(|normal_bits| match (
                                            normal_bits.std_dev,
                                            normal_bits.mean,
                                            normal_bits.updatable.value,
                                        ) {
                                            (Some(_), Some(_), Some(_)) => {
                                                format!(
                                                    "{:.2}%", normal_bits.updatable.value.unwrap()
                                                    .cdf_sigmoid_approx(cdf_input).unwrap() * 100.0
                                                )
                                            }
                                            _ => format!("_"),
                                        })
                                }
                            ></div>
                        }
                    }
                />
                <div>
                    <hr/>
                </div>
                <div>
                    <hr/>
                </div>
                <div style="text-align: right">
                    <NumericInput
                        placeholder=Some("CDF Sample".to_string())
                        modification=Some(Modification::PrefixAndSuffix {
                            prefix: "gain= ".into(),
                            suffix: "%".into(),
                        })
                        non_negative=non_negative_mean
                        updatable=loss_updatable
                        size=12
                        max_len=14
                    />
                </div>
                <div style="text-align: right">"Loss for sample => "</div>
            </div>
            <hr/>
            <div style="display: grid; grid-template-columns: 1fr 1fr 1fr;">
                <div>
                    <input on:click=show_pdf type="radio" id="pdf" name="chart-select" value="PDF"/>
                    <label for="pdf">"PDF"</label>
                </div>
                <div>
                    <input on:click=show_cdf type="radio" id="cdf" name="chart-select" value="CDF"/>
                    <label for="cdf">"CDF"</label>
                </div>
                <div>
                    <input
                        on:click=show_hist
                        type="radio"
                        id="historic"
                        name="chart-select"
                        value="Historic"
                    />
                    <label for="historic">"Historic"</label>
                </div>
            </div>
            <div style="margin-top: 10px">
                <Show
                    when=move || enable_graphs.with(|v| matches!(v, GraphDisplay::Pdf))
                    fallback=|_| ()
                >
                    <DistributionPdfComponent normal_spec=MaybeSignal::Dynamic(spec_signal.into())/>
                </Show>
                <Show
                    when=move || enable_graphs.with(|v| matches!(v, GraphDisplay::Cdf))
                    fallback=|_| ()
                >
                    <DistributionCdfComponent normal_spec=MaybeSignal::Dynamic(spec_signal.into())/>
                </Show>
                <Show
                    when=move || enable_graphs.with(|v| matches!(v, GraphDisplay::Historic))
                    fallback=|_| ()
                >
                    <HistoricRiskReturnComponent normal_spec=MaybeSignal::Dynamic(spec_signal.into())/>
                </Show>
            </div>
            <output></output>
        </fieldset>
    }

    // ω <fn normal_spec_component>
}

// α <mod-def normal_spec_component>
// ω <mod-def normal_spec_component>
