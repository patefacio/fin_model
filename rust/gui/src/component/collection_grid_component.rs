//! Module for collection_grid_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::RwSignal;
use leptos::View;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use std::boxed::Box;
use std::fmt::Debug;

////////////////////////////////////////////////////////////////////////////////////
// --- traits ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a collection to be displayed in a [CollectionGridComponent].
/// Supports adding and removing entries and displaying as many _fields_ of
/// each element as the trait implementation dictates.
pub trait CollectionGrid {
    /// Get the number of columns.
    ///
    ///   * _return_ - Number of columns
    fn get_column_count() -> usize;

    /// The header for the collection.
    ///
    ///   * _return_ - The header as a list of elements
    fn get_header() -> Vec<View>;

    /// Get the display fields for the element.
    ///
    ///   * _return_ - The fields as elements
    fn get_fields(&self) -> Vec<View>;

    /// Get key that uniquely identifies the element.
    ///
    ///   * _return_ - The key for the element
    fn get_key(&self) -> String;

    /// Create new element to edit
    ///
    ///   * _return_ - New element
    fn new() -> Self;

    /// Create a view to edit the element
    ///
    ///   * **element** - Read/write signal containing the element to edit.
    /// This component will update the vector whenever the element is signaled
    /// by finding the proper element in the vector and replacing it with the update.
    ///   * _return_ - The edit view
    fn edit_element(element: RwSignal<Box<Self>>) -> View;
}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a collection of items that has:
/// - A fixed column header
/// - Values that can be displayed as a fixed column set of fields
///
/// It is called grid component because it is styled with a grid.
///
///   * **cx** - Context
///   * **updatable** - Items to show
///   * _return_ - View for collection_grid_component
#[component]
pub fn CollectionGridComponent<T>(
    /// Context
    cx: Scope,
    /// Items to show
    updatable: Updatable<Vec<T>>,
) -> impl IntoView
where
    T: 'static + Debug + CollectionGrid,
{
    // α <fn collection_grid_component>

    use leptos::store_value;

    let collection = store_value(cx, updatable);

    todo!("Implement `collection_grid_component`")
    // ω <fn collection_grid_component>
}

// α <mod-def collection_grid_component>
// ω <mod-def collection_grid_component>
