//! TODO: Document Module(accounts_impl)

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::CollectionGrid;
use crate::Updatable;
use crate::UpdatablePair;
use leptos::view;
use leptos::IntoView;
use leptos::Scope;
use leptos_dom::View;
use plus_modeled::Account;

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// TODO: Document Struct(account_shared_context)
#[derive(Debug, Clone, Default)]
pub struct AccountSharedContext {}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl CollectionGrid for Account {
    type SharedContext = AccountSharedContext;
    /// Get the number of columns.
    ///
    ///   * _return_ - Number of columns
    fn get_column_count() -> usize {
        // α <fn CollectionGrid::get_column_count for Account>
        3
        // ω <fn CollectionGrid::get_column_count for Account>
    }

    /// The header for the collection.
    ///
    ///   * _return_ - The header as a list of elements
    fn get_header() -> Vec<String> {
        // α <fn CollectionGrid::get_header for Account>
        vec![
            "Account".to_string(),
            "Type".to_string(),
            "Market Value".to_string(),
        ]
        // ω <fn CollectionGrid::get_header for Account>
    }

    /// Get the display fields for the element.
    ///
    ///   * **cx** - The context for the fields
    ///   * _return_ - The fields as elements
    fn get_fields(&self, cx: Scope) -> Vec<View> {
        // α <fn CollectionGrid::get_fields for Account>

        use leptos::IntoStyle;
        use plus_modeled::AccountType;

        vec![
            view! { cx, <div class="account-header-cell" style:text-align="right">{self.name.clone()}</div> }.into_view(cx),
            view! { cx, <div class="account-header-cell" style:text-align="right">{AccountType::from_i32(self.account_type).unwrap().as_str_name().to_string()}</div> }.into_view(cx),
            view! { cx, <div class="account-header-cell" style:text-align="right">{"MV".to_string()}</div> }.into_view(cx),
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

    /// Create a view to edit the element
    ///
    ///   * **cx** - Context
    ///   * **updatable** - Updatable containing the element to edit.
    /// This component will update the vector whenever the element is signaled
    /// by finding the proper element in the vector and replacing it with the update.
    ///   * **on_cancel** - Called if edit is canceled.
    ///   * _return_ - The edit view
    fn edit_element<F>(
        cx: Scope,
        updatable: UpdatablePair<Self, Self::SharedContext>,
        on_cancel: F,
    ) -> View
    where
        F: 'static + FnMut(&str),
    {
        // α <fn CollectionGrid::edit_element for Account>

        use crate::AccountComponent;
        let key = updatable.first_value.get_key();
        let mut on_cancel = on_cancel;
        let on_cancel = move || on_cancel(&key);

        view! { cx, <AccountComponent updatable=updatable on_cancel=on_cancel/> }
        .into_view(cx)

        // ω <fn CollectionGrid::edit_element for Account>
    }
}

// α <mod-def accounts_impl>
// ω <mod-def accounts_impl>
