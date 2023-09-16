//! Module for account_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::AccountSharedContext;
use crate::AppContext;
use leptos::use_context;
use leptos::IntoAttribute;
use leptos::SignalGet;
use leptos::StoredValue;
use leptos::WriteSignal;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;
use plus_lookup::I18nAccountComponent;
use plus_modeled::Account;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// A single account
///
///   * **account_stored_value** - The account to edit with shared context
///   * **shared_context_stored_value** - The shared context for accounts
///   * _return_ - View for account_component
#[tracing::instrument(level = "warn")]
#[component]
pub fn AccountComponent(
    /// The account to edit with shared context
    account_stored_value: StoredValue<Account>,
    /// The shared context for accounts
    shared_context_stored_value: StoredValue<AccountSharedContext>,
) -> impl IntoView {
    tracing::debug!("Created account component for {:?}", account_stored_value.get_value());
    pub const SELF_CLASS: &str = "plus-ac";
    let lang_selector = use_context::<AppContext>().unwrap().lang_selector;
    let i18n_account = move || I18nAccountComponent::Account(lang_selector.get()).to_string();
    let i18n_name = move || I18nAccountComponent::Name(lang_selector.get()).to_string();
    let i18n_account_type =
        move || I18nAccountComponent::AccountType(lang_selector.get()).to_string();
    let i18n_holdings = move || I18nAccountComponent::Holdings(lang_selector.get()).to_string();
    // α <fn account_component>

    use crate::AppContext;
    use crate::EnumSelect;
    use crate::HoldingSharedContext;
    use crate::HoldingsGrid;
    use crate::Updatable;
    use leptos::create_node_ref;
    use leptos::create_signal;
    use leptos::store_value;
    use leptos::use_context;
    use leptos::IntoAttribute;
    use leptos::SignalGet;
    use leptos_dom::html::Input;
    use plus_lookup::I18nAccountComponent;
    use plus_lookup::I18nEnums;
    use plus_modeled::AccountType;

    let lang_selector = use_context::<AppContext>().unwrap().lang_selector;
    let account = account_stored_value.with_value(|account| account.clone());

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

    // ω <fn account_component>
    view! {
        <div class=SELF_CLASS>
            // α <plus-ac-view>
            <fieldset>
                <legend>{i18n_account}</legend>
                <div class="form">
                    <div class="form-row">
                        <label>
                            {i18n_name}
                            <input
                                value=account_name
                                node_ref=name_node_ref
                                on:input=on_name_input
                            />
                        </label>
                        <label>
                            {i18n_account_type} <div style="display: inline-block;">
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
                        <HoldingsGrid
                            holdings_updatable=holdings_updatable
                            shared_context_updatable=shared_context_updatable
                        />
                    </div>
                </div>
            </fieldset>
        // ω <plus-ac-view>
        </div>
    }
}

// α <mod-def account_component>
// ω <mod-def account_component>
