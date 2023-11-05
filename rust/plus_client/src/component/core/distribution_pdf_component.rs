//! Module for distribution_pdf_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::MaybeSignal;
use leptos::{component, view, IntoView};
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
    crate::log_component!("`DistributionPdfComponent`");
    // α <fn distribution_pdf_component>

    use crate::DistributionPdf;
    use leptos::IntoAttribute;
    use leptos::SignalGet;

    let plot = move || {
        let normal_spec = normal_spec.get();
        if !disabled {
            normal_spec.get_pdf_chart(200)
        } else {
            " ".to_string()
        }
    };

    view! { <div style="display: flex; justify-content: center;" inner_html=plot></div> }

    // ω <fn distribution_pdf_component>
}

// α <mod-def distribution_pdf_component>
// ω <mod-def distribution_pdf_component>
