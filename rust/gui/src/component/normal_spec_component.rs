//! Module for normal_spec_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::component::numeric_input::{Modification, NumericInput};
use crate::utils::{updatable, historic_risk_return};
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
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

    use crate::utils::distribution_cdf::DistributionCdf;
    use crate::utils::distribution_pdf::DistributionPdf;
    use crate::utils::historic_risk_return::HistoricRiskReturn;
    use crate::utils::historic_risk_return::HistoricRiskReturnPlot;
    use crate::utils::historic_risk_return::HISTORIC_RISK_RETURN_SAMPLES;
    use leptos::create_signal;
    use leptos::For;
    use leptos::IntoAttribute;
    use leptos::SignalGet;
    use leptos::SignalUpdate;
    use leptos::SignalWith;

    struct NormalBits {
        mean: Option<f64>,
        std_dev: Option<f64>,
        updatable: Updatable<Option<NormalSpec>>,
        pdf_drawing_svg: String,
        cdf_drawing_svg: String,
        historic_drawing_svg: String,
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
        .map(|normal_spec| cdf(0.0, normal_spec.std_dev, normal_spec.mean));

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

    let historic_drawing_svg = updatable
        .value
        .as_ref()
        .map(|ns| ns.get_historic_plot(&*HISTORIC_RISK_RETURN_SAMPLES))
        .unwrap_or_default();

    let (normal_bits, set_normal_bits) = create_signal(
        cx,
        NormalBits {
            mean: initial_mean,
            std_dev: initial_std_dev,
            updatable,
            historic_drawing_svg,
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
            cdf_output: initial_output,
        },
    );

    fn make_updates(normal_bits: &mut NormalBits, mut new_normal: NormalSpec, new_input: Option<f64>) {
        new_normal.mean /= 100.0;
        new_normal.std_dev /= 100.0;
        normal_bits.pdf_drawing_svg = new_normal.get_pdf_chart(400);
        normal_bits.cdf_drawing_svg = new_normal.get_cdf_chart(400);
        normal_bits.historic_drawing_svg = new_normal.get_historic_plot((&*HISTORIC_RISK_RETURN_SAMPLES));
        // Before signalling undo the 100x
        normal_bits
            .updatable
            .update_and_then_signal(move |normal_spec| *normal_spec = Some(new_normal));
        match new_input {
            Some(_) => {
                normal_bits.cdf_output =
                    Some(cdf(new_input.unwrap(), new_normal.std_dev, new_normal.mean))
            }
            None => (),
        }
    }

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
            match (loss, normal_bits.std_dev, normal_bits.mean) {
                (Some(_), Some(_), Some(_)) => {
                    normal_bits.cdf_output = Some(cdf(
                        loss.unwrap(),
                        normal_bits.std_dev.unwrap(),
                        normal_bits.mean.unwrap(),
                    ));
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

    view! { cx,
        <fieldset class="nsg">
            <div style="display: grid; grid-template-columns: 1fr 4fr;">
                <div class="header">"Chance to gain(%)"</div>
                <div class="header">"Amount(%)"</div>
                <For
                    each = move || loss_indices.get()
                    key = |item| { format!("{item:?}") }
                    view = move |cx, cdf_input| {
                        view! { cx,
                            <div inner_html=move || { format!("{:.2}%",cdf_input) } ></div>
                            <div inner_html=move || { normal_bits.with(|normal_bits| match (normal_bits.std_dev, normal_bits.mean){
                                    (Some(_), Some(_)) => format!("{:.2}%", cdf(cdf_input , normal_bits.std_dev.unwrap(), normal_bits.mean.unwrap())*100.0 ),
                                    _ => format!("_"),
                                })
                            } ></div>
                        }
                    }
                />
                <NumericInput
                    placeholder=Some("CDF Sample".to_string())
                    modification=Some(Modification::PrefixAndSuffix {
                        prefix: "gain= ".into(),
                        suffix: "%".into(),
                    })
                    non_negative=non_negative_mean
                    updatable=loss_updatable
                    size=9
                    max_len=14
                />


            </div>

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
            <div inner_html=move || { normal_bits.with(|normal_bits| normal_bits.pdf_drawing_svg.clone()) }></div>
            <div inner_html=move || { normal_bits.with(|normal_bits| normal_bits.cdf_drawing_svg.clone()) }></div>
            <div inner_html = move || {normal_bits.with(|normal_bits| normal_bits.historic_drawing_svg.clone())}></div>
            <output></output>
        </fieldset>
    }

    // ω <fn normal_spec_component>
}

pub fn cdf(x: f64, sigma: f64, mu: f64) -> f64 {
    // α <fn DistributionCdf::sigmoid_approx for NormalSpec>
    if sigma == 0.0 {
        return 0.0;
    }
    let correction = 1.70175;
    let temp = (correction * (x - mu) / sigma).exp();
    return temp / (1.0 + temp);
    // ω <fn DistributionCdf::sigmoid_approx for NormalSpec>
}

// α <mod-def normal_spec_component>
// ω <mod-def normal_spec_component>
