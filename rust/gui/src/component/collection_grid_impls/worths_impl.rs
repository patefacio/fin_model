//! TODO: Document Module(worths_impl)

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
use plus_modeled::Worth;

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl CollectionGrid for Worth {
    type SharedContext = ();
    /// Get the number of columns.
    ///
    ///   * _return_ - Number of columns
    fn get_column_count() -> usize {
        // α <fn CollectionGrid::get_column_count for Worth>
        todo!("Implement `get_column_count`")
        // ω <fn CollectionGrid::get_column_count for Worth>
    }

    /// The header for the collection.
    ///
    ///   * _return_ - The header as a list of elements
    fn get_header() -> Vec<String> {
        // α <fn CollectionGrid::get_header for Worth>
        todo!("Implement `get_header`")
        // ω <fn CollectionGrid::get_header for Worth>
    }

    /// Get the display fields for the element.
    ///
    ///   * **cx** - The context for the fields
    ///   * _return_ - The fields as elements
    fn get_fields(&self, cx: Scope) -> Vec<View> {
        // α <fn CollectionGrid::get_fields for Worth>
        todo!("Implement `get_fields`")
        // ω <fn CollectionGrid::get_fields for Worth>
    }

    /// Get key that uniquely identifies the element.
    ///
    ///   * _return_ - The key for the element
    fn get_key(&self) -> String {
        // α <fn CollectionGrid::get_key for Worth>
        todo!("Implement `get_key`")
        // ω <fn CollectionGrid::get_key for Worth>
    }

    /// Create new element to edit
    ///
    ///   * _return_ - New element
    fn new() -> Self {
        // α <fn CollectionGrid::new for Worth>
        todo!("Implement `new`")
        // ω <fn CollectionGrid::new for Worth>
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
        // α <fn CollectionGrid::edit_element for Worth>
        todo!("Implement `edit_element`")
        // ω <fn CollectionGrid::edit_element for Worth>
    }
}

// α <mod-def worths_impl>
// ω <mod-def worths_impl>
