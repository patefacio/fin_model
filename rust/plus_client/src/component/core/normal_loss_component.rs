//! Module for normal_loss_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::AppContext;
use leptos::component;
use leptos::expect_context;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::MaybeSignal;
use leptos::SignalGet;
use plus_modeled::NormalSpec;
use std::rc::Rc;

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
    use plus_lookup::i18n::normal_loss_component::*;
    pub const SELF_CLASS: &str = "plus-nlc";
    let lang_selector = expect_context::<Rc<AppContext>>().lang_selector;
    let i18n_gain_pct = move || i18n_gain_pct(lang_selector.get());
    let i18n_prob_pct = move || i18n_prob_pct(lang_selector.get());
    let i18n_prob_abbrev = move || i18n_prob_abbrev(lang_selector.get());
    let i18n_cdf_sample = move || i18n_cdf_sample(lang_selector.get());
    let i18n_gain_prefix = move || i18n_gain_prefix(lang_selector.get());
    let i18n_loss_table = move || i18n_loss_table(lang_selector.get());
    let component_id = crate::component_id!("`NormalLossComponent`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn normal_loss_component>

    use crate::scale_by;
    use crate::ClientCssClasses;
    use crate::DistributionCdf;
    use crate::Modification;
    use crate::NumericInput;
    use crate::Updatable;
    use leptos::create_signal;
    use leptos::For;
    use leptos::Signal;
    use leptos::SignalUpdate;
    use leptos::SignalWith;
    use leptos::SignalWithUntracked;

    let (sample_loss_read, sample_loss_write) = create_signal(
        normal_spec.with_untracked(|normal_spec| normal_spec.cdf_sigmoid_approx(0.0)),
    );
    let sample_loss_updatable = Updatable::new(Some(0.0), move |loss| {
        sample_loss_write.update(|sample_loss| *sample_loss = *loss);
    });

    // ω <fn normal_loss_component>
    view! {
        <div class=SELF_CLASS>
            // α <plus-nlc-view>

            <div class=ClientCssClasses::NlcLblCtnr.as_str()>
                <h4>
                    {move || {
                        normal_spec
                            .with(|normal_spec| {
                                format!("{} {}", i18n_loss_table(), normal_spec)
                            })
                    }}

                </h4>
            </div>
            <div class=ClientCssClasses::HeaderRight.as_str()>{i18n_gain_pct}</div>
            <div class=ClientCssClasses::HeaderRight.as_str()>{i18n_prob_pct}</div>
            <For
                each=move || loss_vec.get()
                key=|item| { format!("{item:?}") }
                children=move |cdf_input| {
                    view! {
                        <div class=ClientCssClasses::TxtRightPadLeft
                            .as_str()>{move || { format!("{:.2}%", scale_by(cdf_input, 2)) }}</div>
                        <div class=ClientCssClasses::TxtRightPadLeft
                            .as_str()>
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
            <div class=ClientCssClasses::TxtRightPadLeft.as_str()>
                <NumericInput
                    placeholder=Signal::derive(i18n_cdf_sample)
                    modification=Some(Modification::PrefixAndSuffix {
                        prefix: i18n_gain_prefix(),
                        suffix: "%".into(),
                    })

                    updatable=sample_loss_updatable
                    size=12
                    max_len=14
                />
            </div>
            <div class=ClientCssClasses::TxtRightPadLeft
                .as_str()>
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
                                                    "{}({:.2}%) => {:.2}%", i18n_prob_abbrev(), scale_by(loss,
                                                    2), scale_by(probability, 2)
                                                )
                                            },
                                        )
                                })
                        })
                }}

            </div>
        // ω <plus-nlc-view>
        </div>
    }
}

// α <mod-def normal_loss_component>
// ω <mod-def normal_loss_component>
