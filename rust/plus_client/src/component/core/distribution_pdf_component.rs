//! Module for distribution_pdf_component leptos function/component

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
/// Given a [NormalSpec] provides an SVG plot of its _pdf_.
///
///   * **normal_spec** - The normal to plot
///   * **disabled** - Signal allowing the disabling of the input.
///   * _return_ - View for distribution_pdf_component
#[component]
pub fn DistributionPdfComponent(
    /// The normal to plot
    normal_spec: MaybeSignal<NormalSpec>,
    /// Signal allowing the disabling of the input.
    #[prop(default = false)]
    disabled: bool,
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-dist-pdf-comp";
    let component_id = crate::component_id!("`DistributionPdfComponent`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn distribution_pdf_component>

    use crate::DistributionPdf;
    use leptos::SignalGet;

    let _plot = move || {
        let normal_spec = normal_spec.get();
        if !disabled {
            normal_spec.get_pdf_chart(200)
        } else {
            " ".to_string()
        }
    };

    // ω <fn distribution_pdf_component>
    view! { <div class=SELF_CLASS inner_html=_plot></div> }
}

// α <mod-def distribution_pdf_component>
// ω <mod-def distribution_pdf_component>
