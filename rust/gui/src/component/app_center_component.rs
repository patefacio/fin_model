//! Module for app_center_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Main content
///
///   * **cx** - Context
///   * _return_ - View for app_center_component
#[component]
pub fn AppCenterComponent(
    /// Context
    cx: Scope,
) -> impl IntoView {
    // α <fn app_center_component>

    use crate::AccountSharedContext;
    use crate::AccountsGrid;
    use crate::Updatable;
    use plus_modeled::Account;
    use plus_modeled::AccountType;
    use plus_modeled::Holding;
    use plus_modeled::YearCurrencyValue;

    let holdings = vec![
        Holding {
            instrument_name: "SPY".to_string(),
            quantity: 755.3,
            unit_valuation: Some(YearCurrencyValue {
                year: 2020,
                currency: 0,
                value: 440.1,
            }),
            cost_basis: 320_000.0,
            ..Default::default()
        },
        Holding {
            instrument_name: "IWM".to_string(),
            quantity: 1000.0,
            unit_valuation: Some(YearCurrencyValue {
                year: 2020,
                currency: 0,
                value: 180.1,
            }),
            cost_basis: 150_000.0,
            ..Default::default()
        },
        Holding {
            instrument_name: "NVDA".to_string(),
            quantity: 500.3,
            unit_valuation: Some(YearCurrencyValue {
                year: 2020,
                currency: 0,
                value: 420.1,
            }),
            cost_basis: 140_000.0,
            ..Default::default()
        },
    ];

    let holdings = (0..3)
        .map(|i| {
            holdings.iter().map(move |h| Holding {
                instrument_name: format!("{} -> {i}", h.instrument_name),
                ..h.clone()
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    let accounts = [
        ("Interactive Brokers".to_string(), AccountType::Taxable),
        ("VG Retirement".to_string(), AccountType::TraditionalIra),
        ("VG 401k".to_string(), AccountType::TraditionalIrs401K),
        ("VG Roth".to_string(), AccountType::RothIrs401K),
    ]
    .map(|(name, account_type)| Account {
        name: name.to_string(),
        account_type: account_type as i32,
        holdings: holdings.clone(),
        value: 0.0,
        owner_index: 0,
        default_item_growth: None,
    })
    .to_vec();

    let accounts_updatable = Updatable::new(accounts, move |accounts| {
        log!("Accounts updated");
    });

    let shared_context_updatable = Updatable::new(AccountSharedContext::default(), move |sc| {
        log!("Accounts shared context updated -> {sc:?}");
    });

    view! {
        cx,
        <div class="app-center">
        <AccountsGrid
        accounts_updatable=accounts_updatable
        shared_context_updatable=shared_context_updatable
        />
        </div>
    }

    // ω <fn app_center_component>
}

// α <mod-def app_center_component>
// ω <mod-def app_center_component>
