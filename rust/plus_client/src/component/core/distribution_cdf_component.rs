//! Module for distribution_cdf_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::component;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::MaybeSignal;
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
    pub const SELF_CLASS: &str = "plus-dcc";
    let component_id = crate::component_id!("`DistributionCdfComponent`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn distribution_cdf_component>

    use crate::DistributionCdf;
    use leptos::SignalGet;

    let _plot = move || {
        let normal_spec = normal_spec.get();
        normal_spec.get_cdf_chart(200)
    };

    // ω <fn distribution_cdf_component>
    view! { <div class=SELF_CLASS inner_html=_plot></div> }
}

// α <mod-def distribution_cdf_component>
// ω <mod-def distribution_cdf_component>
