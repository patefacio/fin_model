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
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl CollectionGrid for Account {
    type SharedContext = ();
    /// Get the number of columns.
    ///
    ///   * _return_ - Number of columns
    fn get_column_count() -> usize {
        // α <fn CollectionGrid::get_column_count for Account>
        todo!("Implement `get_column_count`")
        // ω <fn CollectionGrid::get_column_count for Account>
    }

    /// The header for the collection.
    ///
    ///   * _return_ - The header as a list of elements
    fn get_header() -> Vec<String> {
        // α <fn CollectionGrid::get_header for Account>
        todo!("Implement `get_header`")
        // ω <fn CollectionGrid::get_header for Account>
    }

    /// Get the display fields for the element.
    ///
    ///   * **cx** - The context for the fields
    ///   * _return_ - The fields as elements
    fn get_fields(&self, cx: Scope) -> Vec<View> {
        // α <fn CollectionGrid::get_fields for Account>
        todo!("Implement `get_fields`")
        // ω <fn CollectionGrid::get_fields for Account>
    }

    /// Get key that uniquely identifies the element.
    ///
    ///   * _return_ - The key for the element
    fn get_key(&self) -> String {
        // α <fn CollectionGrid::get_key for Account>
        todo!("Implement `get_key`")
        // ω <fn CollectionGrid::get_key for Account>
    }

    /// Create new element to edit
    ///
    ///   * _return_ - New element
    fn new() -> Self {
        // α <fn CollectionGrid::new for Account>
        todo!("Implement `new`")
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
        todo!("Implement `edit_element`")
        // ω <fn CollectionGrid::edit_element for Account>
    }
}

// α <mod-def accounts_impl>
// ω <mod-def accounts_impl>
