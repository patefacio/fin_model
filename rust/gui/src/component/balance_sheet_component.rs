//! Module for balance_sheet_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::BalanceSheet;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models the balance sheet, which includes worths, holdings, and any shared instrument
/// growth objects.
///
///   * **cx** - Context
///   * **updatable** - The edited model
///   * _return_ - View for balance_sheet_component
#[component]
pub fn BalanceSheetComponent(
    /// Context
    cx: Scope,
    /// The edited model
    updatable: Updatable<BalanceSheet>,
) -> impl IntoView {
    // α <fn balance_sheet_component>

    view! { cx,
        <fieldset class="balance_sheet">
            <legend>"Balance Sheet"</legend>
        </fieldset>
    }

    // ω <fn balance_sheet_component>
}

// α <mod-def balance_sheet_component>
// ω <mod-def balance_sheet_component>
