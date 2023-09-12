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
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::core::NormalSpec;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a normal specification -> N(mu, sigma).
///
///   * **updatable** - The normal spec being edited
///   * **non_negative_mean** - If set, negative values are disallowed for the mean.
///   * _return_ - View for normal_spec_component
#[component]
pub fn NormalSpecComponent(
    /// The normal spec being edited
    updatable: Updatable<Option<NormalSpec>>,
    /// If set, negative values are disallowed for the mean.
    #[prop(default = false)]
    non_negative_mean: bool,
) -> impl IntoView {
    // α <fn normal_spec_component>

    use crate::scale_by;
    use crate::CollapsibleComponent;
    use crate::DistributionCdfComponent;
    use crate::DistributionPdfComponent;
    use crate::HistoricRiskReturnComponent;
    use crate::NormalLossComponent;
    use leptos::create_signal;
    use leptos::store_value;
    use leptos::MaybeSignal;
    use leptos::Show;
    use leptos::SignalUpdate;
    use leptos::SignalWith;

    enum GraphDisplay {
        Pdf,
        Cdf,
        Historic,
    }
    struct NormalBits {
        mean: Option<f64>,
        std_dev: Option<f64>,
        updatable: Updatable<Option<NormalSpec>>,
    }

    let initial_mean = updatable
        .value
        .as_ref()
        .map(|option_of_normal_spec| scale_by(option_of_normal_spec.mean, 2));

    let initial_std_dev = updatable
        .value
        .as_ref()
        .map(|normal_spec| scale_by(normal_spec.std_dev, 2));

    let (spec_signal, set_spec_signal) =
        create_signal(updatable.value.as_ref().map(|ns| *ns).unwrap_or_default());

    let normal_bits_store_value = store_value(NormalBits {
        mean: initial_mean,
        std_dev: initial_std_dev,
        updatable,
    });

    let signal_update = move || {
        normal_bits_store_value.update_value(|normal_bits| {
            if let (Some(mean), Some(std_dev)) = (normal_bits.mean, normal_bits.std_dev) {
                let normal_spec = NormalSpec {
                    mean: scale_by(mean, -2),
                    std_dev: scale_by(std_dev, -2),
                };
                normal_bits.updatable.value = Some(normal_spec);
                normal_bits.updatable.signal();
                set_spec_signal.update(|ns| {
                    *ns = normal_spec;
                });
            }
        });
    };

    let mean_updatable = Updatable::new(initial_mean, move |mean| {
        normal_bits_store_value.update_value(|normal_bits| {
            normal_bits.mean = *mean;
        });
        signal_update();
    });

    let std_dev_updatable = Updatable::new(initial_std_dev, move |std_dev| {
        normal_bits_store_value.update_value(|normal_bits| {
            normal_bits.std_dev = *std_dev;
            if normal_bits.std_dev.unwrap_or_default() == 0.0 {
                normal_bits.std_dev = None
            };
        });
        signal_update();
    });

    let (enable_graphs, set_disable_graphs) = create_signal(GraphDisplay::Historic);

    let show_pdf = move |_| {
        set_disable_graphs.update(|v| {
            *v = GraphDisplay::Pdf;
        })
    };
    let show_cdf = move |_| {
        set_disable_graphs.update(|v| {
            *v = GraphDisplay::Cdf;
        })
    };
    let show_hist = move |_| {
        set_disable_graphs.update(|v| {
            *v = GraphDisplay::Historic;
        })
    };

    const INPUT_SIZE: u32 = 7;

    view! {
        <div class="ns">
            <fieldset class="nsg">
                <div style="display: flex;">
                    <div style="display: inline;">
                        <span>"N("</span>
                        <NumericInput
                            placeholder="mean"
                            modification=Some(Modification::PrefixAndSuffix {
                                prefix: "μ=".into(),
                                suffix: "%".into(),
                            })

                            non_negative=non_negative_mean
                            updatable=mean_updatable
                            size=INPUT_SIZE
                            max_len=INPUT_SIZE
                        />
                        <span>", "</span>
                        <NumericInput
                            placeholder="std. dev"
                            modification=Some(Modification::PrefixAndSuffix {
                                prefix: "σ=".into(),
                                suffix: "%".into(),
                            })

                            non_negative=true
                            updatable=std_dev_updatable
                            size=INPUT_SIZE
                            max_len=INPUT_SIZE
                        />
                        <span>")"</span>
                        <div class="explore-normal">
                            <CollapsibleComponent
                                collapsed_header="Explore Normal Detail".to_string()
                                expanded_header=Some("Hide Normal Detail".to_string())
                                is_expanded=false
                            >
                                <NormalLossComponent normal_spec=MaybeSignal::Dynamic(
                                    spec_signal.into(),
                                )/>
                                <hr/>
                                <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; margin-top: 10px">
                                    <div>
                                        <label>
                                            <input
                                                on:click=show_hist
                                                type="radio"
                                                id="historic"
                                                name="chart-select"
                                                value="Historic"
                                                checked
                                            />
                                            "Historic"
                                        </label>
                                    </div>
                                    <div>
                                        <label>
                                            <input
                                                on:click=show_pdf
                                                type="radio"
                                                id="pdf"
                                                name="chart-select"
                                                value="PDF"
                                            />
                                            "PDF"
                                        </label>
                                    </div>
                                    <div>
                                        <label>
                                            <input
                                                on:click=show_cdf
                                                type="radio"
                                                id="cdf"
                                                name="chart-select"
                                                value="CDF"
                                            />
                                            "CDF"
                                        </label>
                                    </div>
                                </div>
                                <div style="margin-top: 10px">
                                    <Show
                                        when=move || {
                                            enable_graphs.with(|v| matches!(v, GraphDisplay::Historic))
                                        }

                                        fallback=|| ()
                                    >
                                        <HistoricRiskReturnComponent normal_spec=MaybeSignal::Dynamic(
                                            spec_signal.into(),
                                        )/>
                                    </Show>
                                    <Show
                                        when=move || {
                                            enable_graphs.with(|v| matches!(v, GraphDisplay::Pdf))
                                        }

                                        fallback=|| ()
                                    >
                                        <DistributionPdfComponent normal_spec=MaybeSignal::Dynamic(
                                            spec_signal.into(),
                                        )/>
                                    </Show>
                                    <Show
                                        when=move || {
                                            enable_graphs.with(|v| matches!(v, GraphDisplay::Cdf))
                                        }

                                        fallback=|| ()
                                    >
                                        <DistributionCdfComponent normal_spec=MaybeSignal::Dynamic(
                                            spec_signal.into(),
                                        )/>
                                    </Show>
                                </div>

                            </CollapsibleComponent>
                        </div>
                    </div>

                </div>
            </fieldset>
        </div>
    }

    // ω <fn normal_spec_component>
}

// α <mod-def normal_spec_component>
// ω <mod-def normal_spec_component>