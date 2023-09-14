//! Module for dossier_holding_index_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;
use plus_modeled::DossierHoldingIndex;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a composite index referring to the account (0-indexed) and
/// _optionally_ the holding within the account (0-indexed).
///
///   * **updatable** - The [DossierHoldingIndex] being edited
///   * **account_placeholder** - Placeholder for account input
///   * **holding_placeholder** - Placeholder for holding input
///   * _return_ - View for dossier_holding_index_input
#[component]
pub fn DossierHoldingIndexInput(
    /// The [DossierHoldingIndex] being edited
    updatable: Updatable<Option<DossierHoldingIndex>>,
    /// Placeholder for account input
    #[prop(default="account".to_string())]
    account_placeholder: String,
    /// Placeholder for holding input
    #[prop(default="holding".to_string())]
    holding_placeholder: String,
) -> impl IntoView {
    // α <fn dossier_holding_index_input>

    use std::cell::RefCell;
    use std::rc::Rc;

    let updatable = Rc::new(RefCell::new(updatable));

    use crate::NumericInput;
    use crate::Updatable;

    let initial_account = updatable
        .as_ref()
        .borrow()
        .value
        .as_ref()
        .map(|dhi| dhi.account_index as f64);
    let initial_holding = updatable
        .as_ref()
        .borrow()
        .value
        .as_ref()
        .map(|dhi| dhi.holding_index.unwrap() as f64);

    let updatable_for_account = Rc::clone(&updatable);
    let account_updatable = Updatable::new(initial_account, move |new_input| {
        if let Some(new_input) = new_input.clone() {
            updatable_for_account
                .as_ref()
                .borrow_mut()
                .update_and_then_signal(|dhi| {
                    if let Some(dhi) = dhi {
                        log!("Setting account for DHI -> {new_input:?}");
                        dhi.account_index = new_input as u32;
                    } else {
                        log!("Setting empty DHI for first -> {new_input:?}");
                        *dhi = Some(DossierHoldingIndex {
                            account_index: new_input as u32,
                            holding_index: Some(0),
                        })
                    }
                });
        }
        log!("New Account -> {new_input:?}");
    });

    let updatable_for_holding = Rc::clone(&updatable);
    let holding_updatable = Updatable::new(initial_holding, move |new_input| {
        if let Some(new_input) = new_input.clone() {
            updatable_for_holding
                .as_ref()
                .borrow_mut()
                .update_and_then_signal(|dhi| {
                    if let Some(dhi) = dhi {
                        log!("Setting holding for DHI -> {new_input:?}");
                        dhi.holding_index = Some(new_input as u32);
                    } else {
                        log!("Setting empty DHI for first -> {new_input:?}");
                        *dhi = Some(DossierHoldingIndex {
                            account_index: 0,
                            holding_index: Some(new_input as u32),
                        })
                    }
                });
        }
        log!("New Holding -> {new_input:?}");
    });

    view! {
        <h3>"TEST DossierHoldingIndex"</h3>
        <NumericInput updatable=account_updatable placeholder=account_placeholder/>
        <NumericInput updatable=holding_updatable placeholder=holding_placeholder/>
    }
    // ω <fn dossier_holding_index_input>
}

// α <mod-def dossier_holding_index_input>
// ω <mod-def dossier_holding_index_input>
