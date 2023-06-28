//! Module for dossier_item_index_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::DossierHoldingIndex;
use plus_modeled::DossierItemIndex;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a single index that is one of:
/// - An index of a [Worth](plus_modeled::Worth). 0-based index into the worths
///   in the balance sheet.
/// - An index of a [FlowSpec](plus_modeled::FlowSpec). 0-based index into the flows
///   in the dossier.
/// - An index referencing a specific holding - via
///   [DossierHoldingIndex](plus_modeled::DossierHoldingIndex)
///
///   * **cx** - Context
///   * **updatable** - TODO Document Param(updatable)
///   * **item_placeholder** - Placeholder for holding item
///   * _return_ - View for dossier_item_index_component
#[component]
pub fn DossierItemIndexComponent(
    /// Context
    cx: Scope,
    /// TODO Document Param(updatable)
    updatable: Updatable<Option<DossierItemIndex>>,
    /// Placeholder for holding item
    #[prop(default="Item #".to_string())]
    item_placeholder: String,
) -> impl IntoView {
    // α <fn dossier_item_index_component>

    use std::cell::RefCell;
    use std::rc::Rc;

    let updatable = Rc::new(RefCell::new(updatable));

    use crate::Updatable;
    use plus_modeled::core::dossier_item_index::ItemIndex;

    let initial_item_index = updatable
        .as_ref()
        .borrow()
        .value
        .as_ref()
        .map(|dhi| dhi.item_index);

    let updatable_for_item_index = Rc::clone(&updatable);
    let item_index_updatable = Updatable::new(initial_item_index, move |new_input| {
        if let Some(new_input) = new_input.clone() {
            updatable_for_item_index
                .as_ref()
                .borrow_mut()
                .update_and_then_signal(|ii| {
                    if let Some(ii) = ii {
                        console_log(&format!("Setting account for DHI -> {new_input:?}"));
                        ii.item_index = new_input;
                    } else {
                        console_log(&format!("Setting empty DHI for first -> {new_input:?}"));
                        *ii = Some(DossierItemIndex {
                            item_index: Some(ItemIndex::WorthIndex(32)),
                        })
                    }
                });
        }
        console_log(&format!("New Account -> {new_input:?}"));
    });
    match item_index_updatable.value {
        Some(Some(ItemIndex::WorthIndex(x))) => console_log(&format!("Worth Index")),
        _ => console_log(&format!("Not Worth Index")),
    }

    view! { cx,
        <h3>"TODO DossierItemIndex"</h3>
        <label>
            <input type="text"/>
        </label>
    }
    // ω <fn dossier_item_index_component>
}

// α <mod-def dossier_item_index_component>
// ω <mod-def dossier_item_index_component>
