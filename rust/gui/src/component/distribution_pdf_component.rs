//! Module for distribution_pdf_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
#[allow(unused_imports)]
use leptos::log;
use leptos::MaybeSignal;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::NormalSpec;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Given a [NormalSpec] provides an SVG plot of its _pdf_.
///
///   * **cx** - Context
///   * **normal_spec** - The normal to plot
///   * _return_ - View for distribution_pdf_component
#[component]
pub fn DistributionPdfComponent(
    /// Context
    cx: Scope,
    /// The normal to plot
    normal_spec: MaybeSignal<NormalSpec>,
) -> impl IntoView {
    // α <fn distribution_pdf_component>

    use crate::DistributionPdf;
    use leptos::IntoAttribute;
    use leptos::SignalGet;

    let plot = move || {
        let normal_spec = normal_spec.get();
        normal_spec.get_pdf_chart(200)
    };

    view! { cx, <div inner_html=plot></div> }

    // ω <fn distribution_pdf_component>
}

// α <mod-def distribution_pdf_component>
// ω <mod-def distribution_pdf_component>
