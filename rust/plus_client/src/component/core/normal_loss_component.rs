//! Module for normal_loss_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::MaybeSignal;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;
use plus_modeled::NormalSpec;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Component showing potential loss for a normal distribution.
///
///   * **normal_spec** - The normal to plot
///   * **loss_vec** - Vector of returns to report probability of loss
///   * _return_ - View for normal_loss_component
#[component]
pub fn NormalLossComponent(
    /// The normal to plot
    normal_spec: MaybeSignal<NormalSpec>,
    /// Vector of returns to report probability of loss
    #[prop(default=MaybeSignal::Static(vec![0.7, 0.3, 0.1, 0.05, 0.01, 0.0, -0.01, -0.05, -0.1, -0.3, -0.7]))]
    loss_vec: MaybeSignal<Vec<f64>>,
) -> impl IntoView {
    crate::log_component!("`NormalLossComponent`");
    // α <fn normal_loss_component>

    use crate::scale_by;
    use crate::CssClasses;
    use crate::DistributionCdf;
    use crate::Modification;
    use crate::NumericInput;
    use crate::Updatable;
    use leptos::create_signal;
    use leptos::For;
    use leptos::IntoAttribute;
    use leptos::SignalGet;
    use leptos::SignalUpdate;
    use leptos::SignalWith;
    use leptos::SignalWithUntracked;

    let (sample_loss_read, sample_loss_write) = create_signal(
        normal_spec.with_untracked(|normal_spec| normal_spec.cdf_sigmoid_approx(0.0)),
    );
    let sample_loss_updatable = Updatable::new(Some(0.0), move |loss| {
        sample_loss_write.update(|sample_loss| *sample_loss = *loss);
    });

    view! {
        <div style="display: grid; grid-template-columns: 1fr 1fr">
            <div style="grid-column-start: 1; grid-column-end: 3; text-align: center">
                <h4>
                    {move || {
                        normal_spec
                            .with(|normal_spec| { format!("Loss Table For {}", normal_spec) })
                    }}

                </h4>
            </div>
            <div class=CssClasses::HeaderRight.to_string()>"Gain < (%)"</div>
            <div class=CssClasses::HeaderRight.to_string()>"Probability(%)"</div>
            <For
                each=move || loss_vec.get()
                key=|item| { format!("{item:?}") }
                children=move |cdf_input| {
                    view! {
                        <div style="text-align: right;">
                            {move || { format!("{:.2}%", scale_by(cdf_input, 2)) }}
                        </div>
                        <div style="text-align: right;">
                            {move || {
                                normal_spec
                                    .with(|normal_spec| {
                                        normal_spec
                                            .cdf_sigmoid_approx(cdf_input)
                                            .map_or_else(
                                                || String::default(),
                                                |loss| format!("{:.2}%", scale_by(loss, 2)),
                                            )
                                    })
                            }}

                        </div>
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
                    placeholder="CDF Sample"
                    modification=Some(Modification::PrefixAndSuffix {
                        prefix: "gain < ".into(),
                        suffix: "%".into(),
                    })

                    updatable=sample_loss_updatable
                    size=12
                    max_len=14
                />
            </div>
            <div style="text-align: right">
                {move || {
                    normal_spec
                        .with(move |normal_spec| {
                            sample_loss_read
                                .get()
                                .map(|loss| {
                                    let loss = scale_by(loss, -2);
                                    normal_spec
                                        .cdf_sigmoid_approx(loss)
                                        .map_or_else(
                                            || String::default(),
                                            |probability| {
                                                format!(
                                                    "Prob({:.2}%) => {:.2}%", scale_by(loss, 2),
                                                    scale_by(probability, 2)
                                                )
                                            },
                                        )
                                })
                        })
                }}

            </div>
        </div>
    }

    // ω <fn normal_loss_component>
}

// α <mod-def normal_loss_component>
// ω <mod-def normal_loss_component>
