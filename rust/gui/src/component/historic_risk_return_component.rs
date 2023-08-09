//! Module for historic_risk_return_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView, Scope};
use leptos::{For, MaybeSignal};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::NormalSpec;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Given a [NormalSpec] provides an SVG plot showing the `(risk/return)` along side
/// various historic `(risk/return)` values for equities, bonds, ...
///
///   * **cx** - Context
///   * **normal_spec** - The normal to plot
///   * _return_ - View for historic_risk_return_component
#[component]
pub fn HistoricRiskReturnComponent(
    /// Context
    cx: Scope,
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

    let holding_color_list = HISTORIC_RISK_RETURN_SAMPLES
        .iter()
        .map(|hv| (hv.color.clone(), hv.label.clone()))
        .collect::<Vec<_>>();

    // let color_list = HISTORIC_RISK_RETURN_SAMPLES
    //     .iter()
    //     .map(|hv| hv.clone().color)
    //     .collect::<Vec<_>>();

    let plot = move || normal_spec.with(|ns| ns.get_historic_plot(&*HISTORIC_RISK_RETURN_SAMPLES));
    view! { cx,
        <div style="display:grid; grid-template-rows: 1fr 3fr 1fr; max-width: 600px">
            <div style="grid-row-start:2; grid-row-end:4;">
                <div style="display: grid; grid-template-columns: 1fr">
                    <div style="text-align: center; margin-bottom: 0.75em;" class="header">
                        "Holding Type"
                    </div>
                    <For
                        each=move || holding_color_list.clone()
                        key=|hrr| hrr.1.clone()
                        view=move |cx, (color, label)| {
                            view! { cx,
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
                                        color.0, color.1, color.2
                                    )></div>
                                    <div style="position: relative;">
                                        <div style="position: absolute; bottom: 16.5%; left: 0; white-space:nowrap;">
                                            {move || label.clone()}
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
