//! Module for historic_risk_return_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::AppContext;
use leptos::use_context;
use leptos::IntoAttribute;
use leptos::MaybeSignal;
use leptos::SignalGet;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;
use plus_lookup::I18nHistoricRiskReturnComponent;
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
    pub const SELF_CLASS: &str = "plus-hrrc";
    let lang_selector = use_context::<AppContext>().unwrap().lang_selector;
    let i18n_holding_type =
        move || I18nHistoricRiskReturnComponent::HoldingType(lang_selector.get()).to_string();
    crate::log_component!("`HistoricRiskReturnComponent`");
    // α <fn historic_risk_return_component>

    use crate::utils::historic_risk_return::HistoricRiskReturnPlot;
    use crate::utils::historic_risk_return::HISTORIC_RISK_RETURN_SAMPLES;
    use crate::CssClasses;
    use leptos::For;
    use leptos::IntoAttribute;
    use leptos::SignalWith;

    let plot =
        move || normal_spec.with(|ns| ns.get_historic_plot(&*HISTORIC_RISK_RETURN_SAMPLES, false));

    // ω <fn historic_risk_return_component>
    view! {
        <div class=SELF_CLASS>
            // α <plus-hrrc-view>

            <div>
                <div class=CssClasses::GridLbl.to_string()>{i18n_holding_type}</div>
                <For
                    each=move || HISTORIC_RISK_RETURN_SAMPLES.clone()
                    key=|hrr| hrr.label.clone()
                    children=move |hrr| {
                        view! {
                            <div style="display:flex; text-align: right;">
                                <div style=format!(
                                    "height: 15px;
                                            width: 15px;
                                            margin: 0.35em;
                                            text-align:right;
                                            bottom:0;
                                            left: 0;
                                            background-color: rgb({}, {}, {});
                                            border-radius: 50%;
                                            display: inline-block;",
                                    hrr.color.0, hrr.color.1, hrr.color.2
                                )></div>
                                <div style="position: relative;">
                                    <div style="position: absolute; bottom: 16.5%; left: 0; white-space:nowrap;">
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
            <div style="grid-row-start:1; grid-row-end:4;" inner_html=plot></div>

        // ω <plus-hrrc-view>
        </div>
    }
}

// α <mod-def historic_risk_return_component>
// ω <mod-def historic_risk_return_component>
