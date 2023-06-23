//! Module for dossier_correlation_matrix_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::DossierCorrelationMatrix;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a correlation matrix where all pair-wise indices are represented as keys.
///
///   * **cx** - Context
///   * **updatable** - The [DossierCorrelationMatrix] being edited
///   * _return_ - View for dossier_correlation_matrix_component
#[component]
pub fn DossierCorrelationMatrixComponent(
    /// Context
    cx: Scope,
    /// The [DossierCorrelationMatrix] being edited
    updatable: Updatable<DossierCorrelationMatrix>,
) -> impl IntoView {
    // α <fn dossier_correlation_matrix_component>

    use leptos::create_signal;
    use leptos::For;
    use leptos::IntoAttribute;
    use plus_modeled::core::dossier_item_index::ItemIndex;

    let index_pairs = updatable.value.mappings.iter();

    let (indices, set_indices) = create_signal(cx, updatable.value.mappings);

    view! {
        cx,

        <For
            each=indices
            key=|entry| { format!("{entry:?}")}
            view=|cx, element| {
                let row_index = element.row_index.as_ref().map(
                    |row_index| match row_index.item_index {
                        Some(ItemIndex::WorthIndex(w)) => format!("W({w})"),
                        Some(ItemIndex::FlowIndex(f)) => format!("F({f})"),
                        Some(ItemIndex::HoldingIndex(dhi)) => format!("H({dhi:?})"),
                        None => "Oops".to_string()
                    }
                );
                let column_index = element.column_index.as_ref();
                let correlation = element.correlation;
                view!{ cx, 
                    <div style="display=inline;">
                    <div
                        inner_html=format!("({row_index:?}")
                    />
                    <div
                        inner_html=format!("({column_index:?}")
                    />
                    <div
                        inner_html=format!("{correlation}")
                    />
                    </div>
                    <hr/>
                }
            }
        />

        <h5>"The matrix"</h5>
    }

    // ω <fn dossier_correlation_matrix_component>
}

// α <mod-def dossier_correlation_matrix_component>
// ω <mod-def dossier_correlation_matrix_component>
