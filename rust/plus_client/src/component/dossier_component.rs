//! Module for dossier_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::AppContext;
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::use_context;
use leptos::IntoAttribute;
use leptos::SignalGet;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_lookup::I18nDossierComponent;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Component to edit a single [Dossier](plus_modeled::Dossier)
///
///   * **updatable** - The dossier to edit
///   * _return_ - View for dossier_component
#[component]
pub fn DossierComponent(
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-dc";
    let lang_selector = use_context::<AppContext>().unwrap().lang_selector;
    let i18n_people = move || I18nDossierComponent::People(lang_selector.get()).to_string();
    let i18n_worths = move || I18nDossierComponent::Worths(lang_selector.get()).to_string();
    let i18n_accounts = move || I18nDossierComponent::Accounts(lang_selector.get()).to_string();
    let i18n_cash_flows = move || I18nDossierComponent::CashFlows(lang_selector.get()).to_string();
    let i18n_taxes = move || I18nDossierComponent::Taxes(lang_selector.get()).to_string();
    let i18n_assumptions =
        move || I18nDossierComponent::Assumptions(lang_selector.get()).to_string();
    // α <fn dossier_component>

    use crate::AccountSharedContext;
    use crate::AccountsGrid;
    use crate::AppContext;
    use crate::Updatable;
    use leptos::create_signal;
    use leptos::store_value;
    use leptos::use_context;
    use leptos::IntoAttribute;
    use leptos::Show;
    use leptos::SignalGet;
    use leptos::SignalSet;
    use leptos::SignalUpdate;
    use plus_modeled::Holding;
    use std::collections::HashMap;

    let grid_edit_active_count = use_context::<AppContext>().unwrap().grid_edit_active_count;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
    enum ShowContent {
        ShowPeople,
        ShowWorths,
        ShowAccounts,
        ShowCashFlows,
        ShowTaxes,
        ShowAssumptions,
        #[default]
        ShowNothing,
    }

    let (show_content, set_show_content) = create_signal(ShowContent::ShowNothing);
    let show_people = move || set_show_content.update(|s| *s = ShowContent::ShowPeople);
    let show_worths = move || set_show_content.update(|s| *s = ShowContent::ShowWorths);
    let show_accounts = move || set_show_content.update(|s| *s = ShowContent::ShowAccounts);
    let show_cash_flows = move || set_show_content.update(|s| *s = ShowContent::ShowCashFlows);
    let show_taxes = move || set_show_content.update(|s| *s = ShowContent::ShowTaxes);
    let show_assumptions = move || set_show_content.update(|s| *s = ShowContent::ShowAssumptions);

    let persons_view = move || {
        view! {
            <h2>"TODO -> " {move || i18n_people()}</h2>
        }
    };

    let worths_view = move || {
        view! {
            <h2>"TODO -> " {move || i18n_worths()}</h2>
        }
    };

    let accounts_view = move || {
        let (accounts, account_shared_context) = spoof_accounts();

        let accounts_updatable = Updatable::new(accounts, move |accounts| {
            log!("Accounts updated");
        });

        let shared_context_updatable = Updatable::new(account_shared_context, move |sc| {
            log!("Accounts shared context updated -> {sc:?}");
        });

        view! {
            <AccountsGrid
                accounts_updatable=accounts_updatable
                shared_context_updatable=shared_context_updatable
            />
        }
    };

    let flow_specs_view = move || {
        view! {
            <h2>"TODO -> "  {move || i18n_cash_flows()}</h2>
        }
    };

    let taxes_view = move || {
        view! {
            <h2>"TODO -> "  {move || i18n_taxes()}</h2>
        }
    };

    let assumptions_view = move || {
        view! {
            <h2>"TODO -> "  {move || i18n_assumptions()}</h2>
        }
    };

    let buttons_disabled = move || grid_edit_active_count.get() > 0;
    let people_button_disabled = move || grid_edit_active_count.get() > 0;

    // ω <fn dossier_component>
    view! {
        <div class=SELF_CLASS>
            // α <plus-dc-view>
            <div class="dossier-toolbar">
                <div class="label-button">
                    <button
                        on:click=move |_| show_people()
                        disabled=move || people_button_disabled()
                    >
                        <img src="persons_button.png" border="0px"/>
                    </button>
                    <label>{i18n_people}</label>
                </div>
                <div class="label-button">
                    <button on:click=move |_| show_worths() disabled=move || buttons_disabled()>
                        <img src="worths_button.png" border="0px"/>
                    </button>
                    <label>{i18n_worths}</label>
                </div>
                <div class="label-button">
                    <button on:click=move |_| show_accounts() disabled=move || buttons_disabled()>
                        <img src="accounts_button.png" border="0px"/>
                    </button>
                    <label>{i18n_accounts}</label>
                </div>
                <div class="label-button">
                    <button on:click=move |_| show_cash_flows() disabled=move || buttons_disabled()>
                        <img src="cash_flows_button.png" border="0px"/>
                    </button>
                    <label>{i18n_cash_flows}</label>
                </div>
                <div class="label-button">
                    <button on:click=move |_| show_taxes() disabled=move || buttons_disabled()>
                        <img src="taxes_button.png" border="0px"/>
                    </button>
                    <label>{i18n_taxes}</label>
                </div>
                <div class="label-button">
                    <button
                        on:click=move |_| show_assumptions()
                        disabled=move || buttons_disabled()
                    >
                        <img src="assumptions_button.png" border="0px"/>
                    </button>
                    <label>{i18n_assumptions}</label>
                </div>
            </div>

            <hr/>
            <div class="dossier-show">
                <Show when=move || show_content.get() == ShowContent::ShowPeople fallback=|| ()>
                    {persons_view}
                </Show>

                <Show when=move || show_content.get() == ShowContent::ShowWorths fallback=|| ()>
                    {worths_view}
                </Show>

                <Show when=move || show_content.get() == ShowContent::ShowAccounts fallback=|| ()>
                    {accounts_view}
                </Show>

                <Show when=move || show_content.get() == ShowContent::ShowCashFlows fallback=|| ()>
                    {flow_specs_view}
                </Show>

                <Show when=move || show_content.get() == ShowContent::ShowTaxes fallback=|| ()>
                    {taxes_view}
                </Show>

                <Show
                    when=move || show_content.get() == ShowContent::ShowAssumptions
                    fallback=|| ()
                >
                    {assumptions_view}
                </Show>
            </div>

            // ω <plus-dc-view>
        </div>
    }
}

// α <mod-def dossier_component>

use plus_modeled::{Holding, YearCurrencyValue, Account, AccountType};
use crate::AccountSharedContext;

fn spoof_accounts() -> (Vec<Account>, AccountSharedContext) {
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
            instrument_name: "QQQ".to_string(),
            quantity: 100.0,
            unit_valuation: Some(YearCurrencyValue {
                year: 2020,
                currency: 0,
                value: 376.97,
            }),
            cost_basis: 40_000.0,
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
        Holding {
            instrument_name: "XBI".to_string(),
            quantity: 300.0,
            unit_valuation: Some(YearCurrencyValue {
                year: 2020,
                currency: 0,
                value: 78.83,
            }),
            cost_basis: 36_323.0,
            ..Default::default()
        },
    ];

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

    (accounts, AccountSharedContext::default())
}

// ω <mod-def dossier_component>
