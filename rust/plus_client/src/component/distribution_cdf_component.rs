//! Module for distribution_cdf_component leptos function/component

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
/// Given a [NormalSpec] provides an SVG plot of its _cdf_.
///
///   * **normal_spec** - The normal to plot
///   * _return_ - View for distribution_cdf_component
#[component]
pub fn DistributionCdfComponent(
    /// The normal to plot
    normal_spec: MaybeSignal<NormalSpec>,
) -> impl IntoView {
    // α <fn distribution_cdf_component>

    use crate::DistributionCdf;
    use leptos::IntoAttribute;
    use leptos::SignalGet;

    let plot = move || {
        let normal_spec = normal_spec.get();
        normal_spec.get_cdf_chart(200)
    };

    view! { <div style="display: flex; justify-content: center;" inner_html=plot></div> }

    // ω <fn distribution_cdf_component>
}

// α <mod-def distribution_cdf_component>
// ω <mod-def distribution_cdf_component>
