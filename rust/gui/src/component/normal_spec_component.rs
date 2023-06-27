//! Module for normal_spec_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::component::numeric_input::{Modification, NumericInput};
use crate::Updatable;
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
///   * _return_ - View for normal_spec_component
#[component]
pub fn NormalSpecComponent(
    /// Context
    cx: Scope,
    /// The normal spec being edited
    updatable: Updatable<Option<NormalSpec>>,
) -> impl IntoView {
    // α <fn normal_spec_component>

    use crate::utils::distribution_pdf::DistributionPdf;
    use leptos::create_signal;
    use leptos::IntoAttribute;
    use leptos::ReadSignal;
    use leptos::SignalUpdate;
    use leptos::SignalWith;
    use leptos::WriteSignal;

    struct NormalBits {
        mean: Option<f64>,
        std_dev: Option<f64>,
        updatable: Updatable<Option<NormalSpec>>,
        drawing_svg: String,
    }

    let initial_mean = updatable
        .value
        .as_ref()
        .map(|option_of_normal_spec| option_of_normal_spec.mean * 100.0);

    let initial_std_dev = updatable
        .value
        .as_ref()
        .map(|normal_spec| normal_spec.std_dev * 100.0);

    let (normal_bits, set_normal_bits) = create_signal(
        cx,
        NormalBits {
            mean: initial_mean,
            std_dev: initial_std_dev,
            updatable,
            drawing_svg: initial_mean
                .and_then(|mean| {
                    initial_std_dev.map(|std_dev| NormalSpec { mean, std_dev }.get_chart(200))
                })
                .unwrap_or_default(),
        },
    );

    let mean_updatable = Updatable::new(initial_mean, move |mean| {
        set_normal_bits.update(|normal_bits| {
            normal_bits.mean = *mean;
            if let (Some(mean), Some(std_dev)) = (normal_bits.mean, normal_bits.std_dev) {
                let mut new_normal = NormalSpec { mean, std_dev };
                normal_bits.drawing_svg = new_normal.get_chart(200);
                // Before signalling undo the 100x
                new_normal.mean /= 100.0;
                new_normal.std_dev /= 100.0;
                normal_bits
                    .updatable
                    .update_and_then_signal(move |normal_spec| *normal_spec = Some(new_normal));
            }
        });
    });

    let std_dev_updatable = Updatable::new(initial_std_dev, move |std_dev| {
        set_normal_bits.update(|normal_bits| {
            normal_bits.std_dev = *std_dev;
            if let (Some(mean), Some(std_dev)) = (normal_bits.mean, normal_bits.std_dev) {
                let mut new_normal = NormalSpec { mean, std_dev };
                normal_bits.drawing_svg = new_normal.get_chart(200);
                // Before signalling undo the 100x
                new_normal.mean /= 100.0;
                new_normal.std_dev /= 100.0;
                normal_bits
                    .updatable
                    .update_and_then_signal(|normal_spec| *normal_spec = Some(new_normal));
            }
        });
    });

    view! { cx,
        <fieldset class="nsg">
            <legend>"Normal Growth"</legend>
            <div class="form">
                <div style="display: inline-flex">
                    "N("
                    <NumericInput
                        placeholder=Some("mean".to_string())
                        modification=Some(Modification::PrefixAndSuffix {
                            prefix: "μ=".into(),
                            suffix: "%".into(),
                        })
                        non_negative=true
                        updatable=mean_updatable
                    /> ", "
                    <NumericInput
                        placeholder=Some("std. dev".to_string())
                        modification=Some(Modification::PrefixAndSuffix {
                            prefix: "σ=".into(),
                            suffix: "%".into(),
                        })
                        non_negative=true
                        updatable=std_dev_updatable
                    /> ")"
                </div>
            </div>
            <div inner_html=move || { normal_bits.with(|normal_bits| normal_bits.drawing_svg.clone()) }></div>
        </fieldset>
    }

    // ω <fn normal_spec_component>
}

// α <mod-def normal_spec_component>
// ω <mod-def normal_spec_component>
