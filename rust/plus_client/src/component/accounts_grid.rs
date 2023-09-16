//! Module for accounts_grid leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::AppContext;
use crate::CollectionGrid;
use crate::CollectionGridEditType;
use crate::Updatable;
use leptos::use_context;
use leptos::IntoAttribute;
use leptos::SignalGet;
use leptos::StoredValue;
use leptos::WriteSignal;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;
use leptos_dom::View;
use plus_lookup::I18nAccountsGrid;
use plus_modeled::Account;

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Shared context required to support constraints on individual account edits.
/// Constraints and Dossier effects:
///
/// - No two accounts can have the same name
/// - All holdings with the same symbol name must share the same growth.
/// - Holding links must be kept in sync
#[derive(Debug, Clone, Default)]
pub struct AccountSharedContext {}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Display and edit support for list of accounts.
///
///   * **accounts_updatable** - The accounts to edit
///   * **shared_context_updatable** - The shared context
///   * _return_ - View for accounts_grid
#[component]
pub fn AccountsGrid(
    /// The accounts to edit
    accounts_updatable: Updatable<Vec<Account>>,
    /// The shared context
    shared_context_updatable: Updatable<AccountSharedContext>,
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-ag";
    let lang_selector = use_context::<AppContext>().unwrap().lang_selector;
    let i18n_accounts = move || I18nAccountsGrid::Accounts(lang_selector.get()).to_string();
    let i18n_grid_help = move || I18nAccountsGrid::GridHelp(lang_selector.get()).to_string();
    // α <fn accounts_grid>

    use crate::CollectionGridComponent;
    use leptos::SignalWith;

    // ω <fn accounts_grid>
    view! {
        <div class=SELF_CLASS>
            // α <plus-ag-view>
            <div class="grid-label">{i18n_accounts}</div>
            <p inner_html=i18n_grid_help></p>
            <CollectionGridComponent
                rows_updatable=accounts_updatable
                shared_context_updatable=shared_context_updatable
            />
        // ω <plus-ag-view>
        </div>
    }
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl CollectionGrid for Account {
    type SharedContext = AccountSharedContext;
    /// Get the display fields for the element.
    ///
    ///   * _return_ - The fields as elements
    fn get_fields(&self) -> Vec<View> {
        // α <fn CollectionGrid::get_fields for Account>

        use leptos::IntoStyle;
        use plus_lookup::I18nEnums;
        use plus_modeled::AccountType;

        let lang_selector = use_context::<AppContext>()
            .map(|dossier_context| dossier_context.lang_selector)
            .unwrap();

        let account_type = I18nEnums::AccountType(
            lang_selector.get(),
            &AccountType::from_i32(self.account_type).unwrap_or_default(),
        )
        .to_string();

        vec![
            view! { <div class="cgc-header-cell" style:text-align="right">{self.name.clone()}</div> }.into_view(),
            view! { <div class="cgc-header-cell" style:text-align="right">{account_type}</div> }.into_view(),
            view! { <div class="cgc-header-cell" style:text-align="right">{"MV-TODO".to_string()}</div> }.into_view(),
        ]
        // ω <fn CollectionGrid::get_fields for Account>
    }

    /// Get the header for the rows.
    ///
    ///   * _return_ - The header
    fn get_header() -> Vec<String> {
        // α <fn CollectionGrid::get_header for Account>
        let lang_selector = use_context::<AppContext>().unwrap().lang_selector;

        vec![
            I18nAccountsGrid::Name(lang_selector.get()).to_string(),
            I18nAccountsGrid::Type(lang_selector.get()).to_string(),
            I18nAccountsGrid::Mv(lang_selector.get()).to_string(),
        ]
        // ω <fn CollectionGrid::get_header for Account>
    }

    /// Get the text for `Add New Item`.
    ///
    ///   * _return_ - The add item label
    fn get_add_item_label() -> String {
        // α <fn CollectionGrid::get_add_item_label for Account>

        let lang_selector = use_context::<AppContext>().unwrap().lang_selector;
        I18nAccountsGrid::NewAccount(lang_selector.get()).to_string()

        // ω <fn CollectionGrid::get_add_item_label for Account>
    }

    /// Get key that uniquely identifies the element.
    ///
    ///   * _return_ - The key for the element
    fn get_key(&self) -> String {
        // α <fn CollectionGrid::get_key for Account>
        self.name.clone()
        // ω <fn CollectionGrid::get_key for Account>
    }

    /// Create new element to edit
    ///
    ///   * _return_ - New element
    fn new() -> Self {
        // α <fn CollectionGrid::new for Account>
        Account::default()
        // ω <fn CollectionGrid::new for Account>
    }

    /// Create a view to edit the row
    ///
    ///   * **edit_type** - Type of edit
    ///   * **row_stored_value** - Row to edit.
    ///   * **shared_context_stored_value** - Updatable containing the shared context.
    ///   * _return_ - The edit view
    fn edit_row(
        edit_type: CollectionGridEditType,
        row_stored_value: StoredValue<Self>,
        shared_context_stored_value: StoredValue<Self::SharedContext>,
    ) -> View {
        // α <fn CollectionGrid::edit_row for Account>

        use crate::AccountComponent;

        view! {
            <AccountComponent
                account_stored_value=row_stored_value
                shared_context_stored_value=shared_context_stored_value
            />
        }
        .into_view()
        // ω <fn CollectionGrid::edit_row for Account>
    }

    /// Return true if row edit satisfies any shared context constraints
    ///
    ///   * **edited_row** - The edited row
    ///   * **shared_context** - The current shared context
    ///   * _return_ - An error message if not acceptable change, None otherwise
    fn accept_row_edit(
        edited_row: &Self,
        shared_context: &mut Self::SharedContext,
    ) -> Option<String> {
        // α <fn CollectionGrid::accept_row_edit for Account>
        todo!("Implement `accept_row_edit`")
        // ω <fn CollectionGrid::accept_row_edit for Account>
    }
}

// α <mod-def accounts_grid>
// ω <mod-def accounts_grid>
