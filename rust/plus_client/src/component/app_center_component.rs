//! Module for app_center_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::console_log;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Main content
///
///   * _return_ - View for app_center_component
#[component]
pub fn AppCenterComponent() -> impl IntoView {
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
        <div class="app-center">
            <AccountsGrid
                accounts_updatable=accounts_updatable
                shared_context_updatable=shared_context_updatable
            />
            <Parent/>
        </div>
    }

    // ω <fn app_center_component>
}

// α <mod-def app_center_component>

#[component]
pub fn Parent() -> impl IntoView {
    use crate::EnumSelect;
    use crate::NumericInput;
    use crate::Updatable;
    use leptos::create_rw_signal;
    use leptos::Signal;
    use leptos::SignalGet;
    use leptos::SignalUpdate;
    use plus_modeled::AccountType;

    let child_id_rw_signal = create_rw_signal(1);

    let create_new_child = move || {
        let id = child_id_rw_signal.get();
        view! { <Child id=id/> }
    };

    view! {
        <h4>"Parent"</h4>
        <button on:click=move |_| {
            child_id_rw_signal.update(|id| *id += 1);
        }>"Make Child"</button>
        {move || create_new_child()}
        <EnumSelect
            updatable=Updatable::new(
                AccountType::Taxable,
                |account_type| log!("Account type updated -> {account_type:?}"),
            )

            disabled=Signal::derive(move || child_id_rw_signal.get() % 2 == 0)
        />
        <NumericInput updatable=Updatable::new(Some(1.0), |n| log!("Number now {n:?}"))/>
    }
}

#[component]
pub fn Child(id: u32) -> impl IntoView {
    use leptos::on_cleanup;
    view! {
        {
            on_cleanup(move || log!("Cleaning up child {id}"));
        }

        <h5>{format!("Child {id}")}</h5>
    }
}

// ω <mod-def app_center_component>
