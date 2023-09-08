//! Module for accounts_grid leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::CollectionGrid;
use crate::CollectionGridEditType;
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::StoredValue;
use leptos::WriteSignal;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use leptos_dom::View;
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
///   * **cx** - Context
///   * **accounts_updatable** - The accounts to edit
///   * **shared_context_updatable** - The shared context
///   * _return_ - View for accounts_grid
#[component]
pub fn AccountsGrid(
    /// Context
    cx: Scope,
    /// The accounts to edit
    accounts_updatable: Updatable<Vec<Account>>,
    /// The shared context
    shared_context_updatable: Updatable<AccountSharedContext>,
) -> impl IntoView {
    // α <fn accounts_grid>

    use crate::AppContext;
    use crate::CollectionGridComponent;
    use leptos::use_context;

    let lang_selector = use_context::<AppContext>(cx).unwrap().lang_selector;

    view! { cx,
        <h3>"Accounts"</h3>

        <p>"Enter all financial accounts in this section."</p>
        <CollectionGridComponent
            rows_updatable=accounts_updatable
            shared_context_updatable=shared_context_updatable
            // TODO: i18n
            header=vec!["Account".to_string(), "Type".to_string(), "Market Value".to_string(),]
            add_item_label="Add New Account".to_string()
        />
    }

    // ω <fn accounts_grid>
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl CollectionGrid for Account {
    type SharedContext = AccountSharedContext;
    /// Get the display fields for the element.
    ///
    ///   * **cx** - The context for the fields
    ///   * _return_ - The fields as elements
    fn get_fields(&self, cx: Scope) -> Vec<View> {
        // α <fn CollectionGrid::get_fields for Account>

        log!("GETTING FIELDS OF {cx:?} -> {}", self.name);

        use crate::AppContext;
        use leptos::use_context;
        use leptos::IntoStyle;
        use leptos::SignalGet;
        use plus_lookup::I18nEnums;
        use plus_modeled::AccountType;
        use plus_modeled::LangSelector;

        let lang_selector = use_context::<AppContext>(cx)
            .map(|dossier_context| dossier_context.lang_selector)
            .unwrap();

        let account_type = I18nEnums::AccountType(
            lang_selector.get(),
            &AccountType::from_i32(self.account_type).unwrap_or_default(),
        )
        .to_string();

        vec![
            view! { cx, <div class="cgc-header-cell" style:text-align="right">{self.name.clone()}</div> }.into_view(cx),
            view! { cx, <div class="cgc-header-cell" style:text-align="right">{account_type}</div> }.into_view(cx),
            view! { cx, <div class="cgc-header-cell" style:text-align="right">{"MV-TODO".to_string()}</div> }.into_view(cx),
        ]
        // ω <fn CollectionGrid::get_fields for Account>
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
    ///   * **cx** - Context
    ///   * **edit_type** - Type of edit
    ///   * **row_stored_value** - Row to edit.
    ///   * **shared_context_stored_value** - Updatable containing the shared context.
    ///   * _return_ - The edit view
    fn edit_row(
        cx: Scope,
        edit_type: CollectionGridEditType,
        row_stored_value: StoredValue<Self>,
        shared_context_stored_value: StoredValue<Self::SharedContext>,
    ) -> View {
        // α <fn CollectionGrid::edit_row for Account>

        use crate::AccountComponent;

        log!(
            "RERUNNING EDIT ACCOUNT ROW {}",
            row_stored_value.with_value(|a| a.name.clone())
        );

        view! { cx,
            <AccountComponent
                account_stored_value=row_stored_value
                shared_context_stored_value=shared_context_stored_value
            />
        }
        .into_view(cx)
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
