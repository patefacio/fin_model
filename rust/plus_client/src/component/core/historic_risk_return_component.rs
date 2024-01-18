//! Module for historic_risk_return_component leptos function/component

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
    let lang_selector = expect_context::<Rc<AppContext>>().lang_selector;
    let i18n_holding_type = move || i18n_holding_type(lang_selector.get());
    let component_id = crate::component_id!("`HistoricRiskReturnComponent`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn historic_risk_return_component>

    use crate::utils::historic_risk_return::HistoricRiskReturnPlot;
    use crate::utils::historic_risk_return::HISTORIC_RISK_RETURN_SAMPLES;
    use crate::ClientCssClasses;
    use leptos::For;
    use leptos::SignalWith;

    let plot =
        move || normal_spec.with(|ns| ns.get_historic_plot(&*HISTORIC_RISK_RETURN_SAMPLES, false));

    // ω <fn historic_risk_return_component>
    view! {
        <div class=SELF_CLASS>
            // α <plus-hrrc-view>

            <div>
                <div class=ClientCssClasses::GridLbl.as_str()>{i18n_holding_type}</div>
                <For
                    each=move || HISTORIC_RISK_RETURN_SAMPLES.clone()
                    key=|hrr| hrr.label.clone()
                    children=move |hrr| {
                        view! {
                            <div class=ClientCssClasses::HrrcCtnr.as_str()>
                                <div
                                    class=ClientCssClasses::HrrcDot.as_str()
                                    style=format!(
                                        "background-color: rgb({}, {}, {});", hrr.color.0, hrr.color
                                        .1, hrr.color.2
                                    )
                                >
                                </div>
                                <div class=ClientCssClasses::HrrcLblCtnr.as_str()>
                                    <div class=ClientCssClasses::HrrcLbl
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
            <div class=ClientCssClasses::HrrcPlot.as_str() inner_html=plot></div>

        // ω <plus-hrrc-view>
        </div>
    }
}

// α <mod-def historic_risk_return_component>
// ω <mod-def historic_risk_return_component>
