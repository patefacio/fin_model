//! Module for historic_risk_return_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::AppContext;
use leptos::component;
use leptos::use_context;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::MaybeSignal;
use leptos::SignalGet;
use plus_modeled::NormalSpec;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Given a [NormalSpec] provides an SVG plot showing the `(risk/return)` along side
/// various historic `(risk/return)` values for equities, bonds, ...
///
///   * **normal_spec** - The normal to plot
///   * _return_ - View for historic_risk_return_component
#[component]
pub fn HistoricRiskReturnComponent(
    /// The normal to plot
    normal_spec: MaybeSignal<NormalSpec>,
) -> impl IntoView {
    use plus_lookup::i18n::historic_risk_return_component::*;
    pub const SELF_CLASS: &str = "plus-hrrc";
    let lang_selector = use_context::<AppContext>().unwrap().lang_selector;
    let i18n_holding_type = move || i18n_holding_type(lang_selector.get());
    crate::log_component!("`HistoricRiskReturnComponent`");
    // α <fn historic_risk_return_component>

    use crate::utils::historic_risk_return::HistoricRiskReturnPlot;
    use crate::utils::historic_risk_return::HISTORIC_RISK_RETURN_SAMPLES;
    use crate::CssClasses;
    use leptos::For;
    use leptos::SignalWith;

    let plot =
        move || normal_spec.with(|ns| ns.get_historic_plot(&*HISTORIC_RISK_RETURN_SAMPLES, false));

    // ω <fn historic_risk_return_component>
    view! {
        <div class=SELF_CLASS>
            // α <plus-hrrc-view>

            <div>
                <div class=CssClasses::GridLbl.as_str()>{i18n_holding_type}</div>
                <For
                    each=move || HISTORIC_RISK_RETURN_SAMPLES.clone()
                    key=|hrr| hrr.label.clone()
                    children=move |hrr| {
                        view! {
                            <div class=CssClasses::HrrcCtnr.as_str()>
                                <div
                                    class=CssClasses::HrrcDot.as_str()
                                    style=format!(
                                        "background-color: rgb({}, {}, {});", hrr.color.0, hrr.color
                                        .1, hrr.color.2
                                    )
                                >
                                </div>
                                <div class=CssClasses::HrrcLblCtnr.as_str()>
                                    <div class=CssClasses::HrrcLbl
                                        .as_str()>
                                        {move || {
                                            format!(
                                                "{} - {}", hrr.label.clone(), NormalSpec { mean : hrr
                                                .risk_return.1, std_dev : hrr.risk_return.0 }
                                            )
                                        }}

                                    </div>
                                </div>
                            </div>
                        }
                    }
                />

            </div>
            <div class=CssClasses::HrrcPlot.as_str() inner_html=plot></div>

        // ω <plus-hrrc-view>
        </div>
    }
}

// α <mod-def historic_risk_return_component>
// ω <mod-def historic_risk_return_component>
