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
use plus_modeled::core::dossier_item_index::ItemIndex;

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
///   * _return_ - View for dossier_item_index_component
#[component]
pub fn DossierItemIndexComponent(
    /// Context
    cx: Scope,
    /// TODO Document Param(updatable)
    updatable: Updatable<Option<DossierItemIndex>>,

    #[prop(default="Item #".to_string())] item_placeholder: String,

) -> impl IntoView {

    use std::cell::RefCell;
    use std::rc::Rc;

    let updatable = Rc::new(RefCell::new(updatable));

    use crate::NumericInput;
    use crate::Updatable;

    let _temp = DossierItemIndex{item_index: Some(ItemIndex::WorthIndex(2)) };
    
    let index_updatable = Updatable::new(Some(32.0), move |new_input| {
        //
        console_log(&format!("New Item Index -> {new_input:?}"));
    });

    // α <fn dossier_item_index_component>
    view! {
        cx,
        <h3>"TODO DossierItemIndex"</h3>
        
        <NumericInput
            updatable=index_updatable
            placeholder=Some(item_placeholder)
        />


    }
    // ω <fn dossier_item_index_component>
}

// α <mod-def dossier_item_index_component>
// ω <mod-def dossier_item_index_component>
