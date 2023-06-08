//! Module for balance_sheet_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use fin_model::balance_sheet::BalanceSheet;
use leptos::{component, view, IntoView, Scope};

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models the balance sheet, which includes worths, holdings, and any shared instrument
/// growth objects.
///
///   * **cx** - Context
///   * _return_ - View for balance_sheet_component
#[component]
pub fn BalanceSheetComponent(
    /// Context
    cx: Scope,
) -> impl IntoView {
    // α <fn balance_sheet_component>

    view! {
        cx,
        <fieldset class="balance_sheet">
        <legend>"Balance Sheet"</legend>

        </fieldset>
    }

    // ω <fn balance_sheet_component>
}

// α <mod-def balance_sheet_component>
// ω <mod-def balance_sheet_component>
