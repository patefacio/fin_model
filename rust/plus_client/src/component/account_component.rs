//! Module for account_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::AccountSharedContext;
#[allow(unused_imports)]
use leptos::log;
use leptos::StoredValue;
use leptos::WriteSignal;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::Account;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// A single account
///
///   * **account_stored_value** - The account to edit with shared context
///   * **shared_context_stored_value** - The shared context for accounts
///   * _return_ - View for account_component
#[component]
pub fn AccountComponent(
    /// The account to edit with shared context
    account_stored_value: StoredValue<Account>,
    /// The shared context for accounts
    shared_context_stored_value: StoredValue<AccountSharedContext>,
) -> impl IntoView {
    // α <fn account_component>

    log!(
        "Creating new account component {}",
        account_stored_value.with_value(|a| a.name.clone())
    );

    leptos::on_cleanup(move || {
        log!(
            "CLEANING UP Account -> {}",
            account_stored_value.with_value(|a| a.name.clone())
        )
    });

    use crate::AppContext;
    use crate::CollectionGridComponent;
    use crate::CollectionGridState;
    use crate::EnumSelect;
    use crate::HoldingSharedContext;
    use crate::Updatable;
    use leptos::create_node_ref;
    use leptos::create_signal;
    use leptos::store_value;
    use leptos::use_context;
    use leptos::IntoAttribute;
    use leptos::SignalGet;
    use leptos_dom::html::Input;
    use plus_lookup::I18nEnums;
    use plus_modeled::AccountType;

    let lang_selector = use_context::<AppContext>().unwrap().lang_selector;

    account_stored_value.with_value(|account| {
        let name_node_ref = create_node_ref::<Input>();
        let account_name = account.name.clone();
        let initial_holdings = account.holdings.clone();
        let account_type = AccountType::from_i32(account.account_type).unwrap();

        let holding_type_updatable = Updatable::new(account_type, move |&account_type| {
            account_stored_value.update_value(|account| {
                account.account_type = account_type as i32;
            });
        });

        let holdings_updatable = Updatable::new(initial_holdings, move |new_holdings| {
            account_stored_value.update_value(|account| {
                account.holdings = new_holdings.clone();
                log!(
                    "Updating Acct:{} holdings -> {new_holdings:?}",
                    account.name
                );
            });
        });

        let shared_context_updatable =
            Updatable::new(HoldingSharedContext::default(), |shared_context| {
                log!("Shared context updated -> {shared_context:?}");
            });

        let on_name_input = move |_| {
            if let Some(input) = name_node_ref.get() {
                account_stored_value.update_value(|account| account.name = input.value());
            }
        };

        view! {
            <fieldset>
                <legend>"Account"</legend>
                <div class="form">
                    <div class="form-row">
                        <label>
                            "Name"
                            <input
                                value=account_name
                                node_ref=name_node_ref
                                on:input=on_name_input
                            />
                        </label>
                        <label>
                            "Account Type" <div style="display: inline-block;">
                                <EnumSelect
                                    updatable=holding_type_updatable
                                    label=Some(
                                        Box::new(move |e| {
                                            I18nEnums::AccountType(lang_selector.get(), e).to_string()
                                        }),
                                    )
                                />

                            </div>
                        </label>
                    </div>
                </div>
                <div class="account-holdings">
                    <div>
                        <div class="info-label">"Account Holdings"</div>
                        <CollectionGridComponent

                            {
                                on_cleanup(log!("CLEANING UP HOLDING GRID!"));
                            }

                            header=vec![
                                "Symbol".to_string(), "Market Value".to_string(), "Cost Basis"
                                .to_string(), "Unrealized (G/L)".to_string(),
                            ]

                            rows_updatable=holdings_updatable
                            shared_context_updatable=shared_context_updatable
                            add_item_label="Add New Holding".to_string()
                        />
                    </div>
                </div>
            </fieldset>
        }
    })

    // ω <fn account_component>
}

// α <mod-def account_component>
// ω <mod-def account_component>
