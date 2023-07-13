//! Module for normal_spec_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::component::numeric_input::{Modification, NumericInput};
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

    use crate::utils::distribution_pdf::DistributionPdf;
    use crate::utils::distribution_cdf::DistributionCdf;
    use leptos::create_signal;
    use leptos::IntoAttribute;
    use leptos::SignalUpdate;
    use leptos::SignalWith;

    struct NormalBits {
        mean: Option<f64>,
        std_dev: Option<f64>,
        updatable: Updatable<Option<NormalSpec>>,
        pdf_drawing_svg: String,
        cdf_drawing_svg: String
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


    let (normal_bits, set_normal_bits) = create_signal(
        cx,
        NormalBits {
            mean: initial_mean,
            std_dev: initial_std_dev,
            updatable,
            pdf_drawing_svg: initial_mean
                .and_then(|mean| {
                    initial_std_dev.map(|std_dev| NormalSpec { mean, std_dev }.get_chart(200))
                })
                .unwrap_or_default(),
            cdf_drawing_svg: initial_mean
                .and_then(|mean| {
                    initial_std_dev.map(|std_dev| NormalSpec { mean, std_dev }.get_cdf_chart(200))
                })
                .unwrap_or_default(),
        },
    );
    


    fn make_updates(normal_bits: &mut NormalBits, mut new_normal: NormalSpec) {
        normal_bits.pdf_drawing_svg = new_normal.get_chart(400);
        normal_bits.cdf_drawing_svg = new_normal.get_cdf_chart(400);
        // Before signalling undo the 100x
        new_normal.mean /= 100.0;
        new_normal.std_dev /= 100.0;
        normal_bits
            .updatable
            .update_and_then_signal(move |normal_spec| *normal_spec = Some(new_normal));
    }

    let mean_updatable = Updatable::new(initial_mean, move |mean| {
        set_normal_bits.update(|normal_bits| {
            normal_bits.mean = *mean;
            if let (Some(mean), Some(std_dev)) = (normal_bits.mean, normal_bits.std_dev) {
                make_updates(normal_bits, NormalSpec { mean, std_dev });
            }
        });
    });

    let std_dev_updatable = Updatable::new(initial_std_dev, move |std_dev| {
        set_normal_bits.update(|normal_bits| {
            normal_bits.std_dev = *std_dev;
            if let (Some(mean), Some(std_dev)) = (normal_bits.mean, normal_bits.std_dev) {
                make_updates(normal_bits, NormalSpec { mean, std_dev });
            }
        });
    });


    let loss_updatable = Updatable::new(initial_loss, move |loss| {
        normal_bits.with(|normal_bits| {
            let correction = 1.70175;
            let cdf = |z: f64| {
                (correction*(z - normal_bits.mean.unwrap())/normal_bits.std_dev.unwrap()).exp() / ( 1.0 + (correction*(z - normal_bits.mean.unwrap())/normal_bits.std_dev.unwrap()).exp() )
            };
            console_log(&format!("Probability of having {}% or less: {}%", loss.unwrap_or_default(), cdf(loss.unwrap_or_default())*100.0));
            
        });
    });

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
                    /> ")"
                </div>
            </div>
            <div inner_html=move || { normal_bits.with(|normal_bits| normal_bits.pdf_drawing_svg.clone()) }></div>
            <div inner_html=move || { normal_bits.with(|normal_bits| normal_bits.cdf_drawing_svg.clone()) }></div>
                    <NumericInput
                        placeholder=Some("CDF Sample".to_string())
                        modification=Some(Modification::PrefixAndSuffix {
                            prefix: "loss= ".into(),
                            suffix: "%".into(),
                        })
                        non_negative=non_negative_mean
                        updatable=loss_updatable
                        size=9
                        max_len=14
                    />
                    <output
                    />
        </fieldset>
    }

    // ω <fn normal_spec_component>
}

// α <mod-def normal_spec_component>
// ω <mod-def normal_spec_component>
