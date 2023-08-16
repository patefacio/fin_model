//! Module for account_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::AccountSharedContext;
use crate::CollectionGridState;
use crate::Updatable;
use crate::UpdatablePair;
use crate::UpdatePairType;
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::Account;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// A single account
///
///   * **cx** - Context
///   * **updatable** - The account to edit with shared context
///   * **on_cancel** - Called if edit is canceled
///   * _return_ - View for account_component
#[component]
pub fn AccountComponent<F>(
    /// Context
    cx: Scope,
    /// The account to edit with shared context
    updatable: UpdatablePair<Account, AccountSharedContext>,
    /// Called if edit is canceled
    on_cancel: F,
) -> impl IntoView
where
    F: 'static + FnMut(),
{
    // α <fn account_component>

    use crate::CollectionGridComponent;
    use crate::CollectionGridState;
    use crate::EnumSelect;
    use crate::HoldingSharedContext;
    use crate::OkCancel;
    use crate::OkCancelComponent;
    use crate::UpdatablePair;
    use crate::UpdatePairType;
    use leptos::create_signal;
    use leptos::store_value;
    use leptos::IntoAttribute;
    use leptos::Show;
    use leptos::SignalWith;
    use plus_modeled::AccountType;

    let on_cancel = on_cancel;
    let initial_holdings = updatable.first_value.holdings.clone();
    let account = &updatable.first_value;
    let account_name = account.name.clone();
    let account_type = AccountType::from_i32(account.account_type).unwrap();
    let updatable = store_value(cx, updatable);
    let on_cancel = store_value(cx, on_cancel);

    let holding_type_updatable = Updatable::new(account_type, move |&account_type| {
        updatable.update_value(|updatable| updatable.first_value.account_type = account_type as i32)
    });

    let on_ok_cancel = move |ok_cancel| match ok_cancel {
        OkCancel::Ok => {
            updatable.update_value(|updatable| {
                log!("OK on Account -> ({})", updatable.first_value);
                updatable.signal()
            });
        }
        OkCancel::Cancel => {
            on_cancel.update_value(|on_cancel| on_cancel());
        }
    };

    let (state_change, set_state_change) = create_signal(cx, CollectionGridState::Display);

    let show_ok_cancel = move || {
        state_change.with(|state_change| {
            log!("Account edit state change -> {state_change:?}");
            match state_change {
                CollectionGridState::Display => true,
                _ => false,
            }
        })
    };

    let holdings_updatable = UpdatablePair::new(
        initial_holdings,
        HoldingSharedContext::default(),
        move |(new_holdings, new_shared, update_type)| {
            log!(
                "Updating Acct:{} holdings!",
                updatable.with_value(|pair| pair.first_value.name.clone())
            );
            match update_type {
                UpdatePairType::UpdateFirst | UpdatePairType::UpdateBoth => {
                    updatable.update_value(|pair| {
                        pair.update(|(account, shared)| {
                            account.holdings = new_holdings.clone();
                            UpdatePairType::UpdateFirst
                        })
                    })
                }
                _ => {
                    log!("Ignoring account update of type {update_type:?}");
                }
            }
            log!("Holdings updated -> {new_holdings:?}");
        },
    );

    view! { cx,
        <fieldset>
            <legend>"Account"</legend>
            <div class="form">
                <div class="form-row">
                    <label>"Name" <input value=account_name/></label>
                    <label>
                        "Account Type" <div style="display: inline-block;">
                            <EnumSelect updatable=holding_type_updatable/>
                        </div>
                    </label>
                </div>
            </div>
            <div class="account-holdings">
                <div>
                    <div class="info-label">"Account Holdings"</div>
                    <CollectionGridComponent
                        updatable=holdings_updatable
                        on_state_change=Some(set_state_change)
                    />
                </div>
            </div>
            <Show when=show_ok_cancel fallback=|_| ()>
                <div class="ok-cancel-bar">
                    <OkCancelComponent on_ok_cancel=on_ok_cancel/>
                </div>
            </Show>
        </fieldset>
    }

    // ω <fn account_component>
}

// α <mod-def account_component>
// ω <mod-def account_component>
