//! Module for dossier_correlation_entry_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;
use plus_modeled::DossierCorrelationEntry;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a correlation value between _dossier items_.
///
///   * **updatable** - The [DossierCorrelationEntry] being edited
///   * _return_ - View for dossier_correlation_entry_component
#[component]
pub fn DossierCorrelationEntryComponent(
    /// The [DossierCorrelationEntry] being edited
    updatable: Updatable<Option<DossierCorrelationEntry>>,
) -> impl IntoView {
    // α <fn dossier_correlation_entry_component>

    use std::cell::RefCell;
    use std::rc::Rc;

    let updatable = Rc::new(RefCell::new(updatable));

    use crate::NumericInput;
    use crate::Updatable;

    let initial_row = updatable
        .as_ref()
        .borrow()
        .value
        .as_ref()
        .map(|DCE| DCE.row_index);

    let updatable_for_row = Rc::clone(&updatable);
    let row_updatable = Updatable::new(initial_row, move |new_input| {
        if let Some(new_input) = new_input.clone() {
            updatable_for_row
                .as_ref()
                .borrow_mut()
                .update_and_then_signal(|DCE| {
                    if let Some(DCE) = DCE {
                        log!("Setting value on row -> {new_input:?}");
                        DCE.row_index = new_input
                    } else {
                        log!("Setting empty row on first change of value -> {new_input:?}");

                        *DCE = Some(DossierCorrelationEntry {
                            row_index: new_input,
                            column_index: new_input,
                            correlation: 0.0,
                        })
                    }
                });
        }
        log!("New value -> {new_input:?}");
    });

    view! { <h3>"TODO DossierCorrelationEntryComponent"</h3> }

    // ω <fn dossier_correlation_entry_component>
}

// α <mod-def dossier_correlation_entry_component>
// ω <mod-def dossier_correlation_entry_component>
