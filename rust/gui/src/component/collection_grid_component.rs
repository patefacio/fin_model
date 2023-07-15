//! Module for collection_grid_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
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
    fn get_header() -> Vec<String>;

    /// Get the display fields for the element.
    ///
    ///   * **cx** - The context for the fields
    ///   * _return_ - The fields as elements
    fn get_fields(&self, cx: Scope) -> Vec<View>;

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
///   * **read_only** - If true just display (default false)
///   * _return_ - View for collection_grid_component
#[component]
pub fn CollectionGridComponent<T>(
    /// Context
    cx: Scope,
    /// Items to show
    updatable: Updatable<Vec<T>>,
    /// If true just display (default false)
    #[prop(default = false)]
    read_only: bool,
) -> impl IntoView
where
    T: 'static + Clone + Debug + CollectionGrid,
{
    // α <fn collection_grid_component>

    use leptos::store_value;
    use leptos::For;
    use leptos::IntoView;
    use leptos::View;
    use leptos_dom::html::Div;
    use leptos_dom::HtmlElement;

    let updatable = store_value(cx, updatable);

    let header = {
        let mut fields = <T as CollectionGrid>::get_header();
        if !read_only {
            fields.insert(0, String::default());
            fields.insert(0, String::default());
        }
        fields
            .into_iter()
            .map(|column_header| {
                view! { cx, <div class="header">{column_header}</div> }
            })
            .collect::<Vec<HtmlElement<Div>>>()
    };

    view! { cx,
        <div style="display: grid; grid-template-columns: 1.8rem 1.8rem 1fr 1fr 1fr 1fr;">
            {header}
            <For
                each=move || updatable.with_value(|updatable| updatable.value.clone())
                key=|item| { item.get_key() }
                view=move |cx, item| {
                    view! { cx,
                        {
                            let mut user_fields = item.get_fields(cx);
                            if !read_only {
                            user_fields
                                .insert(
                                    0,
                                    view! { cx, <button>"🗑"</button> }
                                        .into_view(cx),
                                );
                            user_fields
                                .insert(
                                    0,
                                    view! { cx, <button>"✍"</button> }
                                        .into_view(cx),
                                );
                            }
                            user_fields
                        }
                    }
                }
            />
        </div>
    }

    // ω <fn collection_grid_component>
}

// α <mod-def collection_grid_component>
// ω <mod-def collection_grid_component>
