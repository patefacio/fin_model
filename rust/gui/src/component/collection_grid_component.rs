//! Module for collection_grid_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::View;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use std::fmt::Debug;

////////////////////////////////////////////////////////////////////////////////////
// --- traits ---
////////////////////////////////////////////////////////////////////////////////////
/// TODO: Document Trait(collection_grid)
pub trait CollectionGrid {
    /// Type in collection
    type T;

    /// The header for the collection.
    ///
    ///   * _return_ - The header as a list of elements
    fn get_header() -> Vec<View>;

    /// Get the display fields for the element.
    ///
    ///   * **element** - The element to get fields
    ///   * _return_ - The fields as elements
    fn get_fields(element: &Self::T) -> Vec<View>;
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
    T: Debug + CollectionGrid,
{
    // α <fn collection_grid_component>
    todo!("Implement `collection_grid_component`")
    // ω <fn collection_grid_component>
}

// α <mod-def collection_grid_component>
// ω <mod-def collection_grid_component>
