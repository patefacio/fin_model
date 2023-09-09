//! Module for historic_risk_return_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
#[allow(unused_imports)]
use leptos::log;
use leptos::MaybeSignal;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::console_log;
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
    // α <fn historic_risk_return_component>
    use crate::utils::historic_risk_return::HistoricRiskReturn;
    use crate::utils::historic_risk_return::HistoricRiskReturnPlot;
    use crate::utils::historic_risk_return::HISTORIC_RISK_RETURN_SAMPLES;
    use leptos::For;
    use leptos::IntoAttribute;
    use leptos::SignalWith;

    let plot =
        move || normal_spec.with(|ns| ns.get_historic_plot(&*HISTORIC_RISK_RETURN_SAMPLES, false));
    view! {
        <div
            class="historic-legend"
            style="display:grid; grid-template-rows: 1fr 3fr 1fr; grid-template-columns: 1fr; max-width: 600px"
        >
            <div style="grid-row-start:2; grid-row-end:4; grid-column-start:0; grid-column-end:2; overflow: hidden;">
                <div>
                    <div style="text-align: center; margin-bottom: 0.75em;" class="header">
                        "Holding Type"
                    </div>
                    <For
                        each=move || HISTORIC_RISK_RETURN_SAMPLES.clone()
                        key=|hrr| hrr.label.clone()
                        view=move |hrr| {
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
            </div>
            <div style="grid-row-start:1; grid-row-end:4;" inner_html=plot></div>
        </div>
    }
    // ω <fn historic_risk_return_component>
}

// α <mod-def historic_risk_return_component>
// ω <mod-def historic_risk_return_component>
