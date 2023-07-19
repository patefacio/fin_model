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
pub trait CollectionGrid: Sized {
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
    ///   * **cx** - Context
    ///   * **updatable** - Read/write signal containing the element to edit.
    /// This component will update the vector whenever the element is signaled
    /// by finding the proper element in the vector and replacing it with the update.
    ///   * _return_ - The edit view
    fn edit_element(cx: Scope, updatable: Updatable<Self>) -> View;
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

    use leptos::create_rw_signal;
    use leptos::store_value;
    use leptos::Component;
    use leptos::For;
    use leptos::IntoAttribute;
    use leptos::IntoView;
    use leptos::Show;
    use leptos::SignalUpdate;
    use leptos::SignalWith;
    use leptos::View;
    use leptos_dom::html::Div;
    use leptos_dom::Element;
    use leptos_dom::HtmlElement;

    #[derive(Eq, PartialEq, Debug)]
    enum ComponentState {
        Display,
        EditNew,
        EditSelection { selection_key: String },
    }

    struct CGCData<T> {
        updatable: Updatable<Vec<T>>,
        component_state: ComponentState,
    }

    let cgc_data = create_rw_signal(
        cx,
        CGCData {
            updatable,
            component_state: ComponentState::Display,
        },
    );

    let disabled =
        move || cgc_data.with(|cgc_data| cgc_data.component_state != ComponentState::Display);

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
                each=move || cgc_data.with(|cgc_data| cgc_data.updatable.value.clone())
                key=|item| { item.get_key() }
                view=move |cx, item| {
                    use std::rc::Rc;
                    let item = Rc::new(item);
                    let mut user_fields = item.get_fields(cx);
                    let key = item.get_key();
                    let this_row_edit = move || {
                        cgc_data
                            .with(|cgc_data| {
                                let key = key.clone();
                                log!("Checking on selection key vs `{key}`");
                                match &cgc_data.component_state {
                                    ComponentState::EditSelection { selection_key } => {
                                        *selection_key == key
                                    }
                                    _ => false,
                                }
                            })
                    };

                    let this_row_updated =move
                        |item: &_| {
                            log!("Item updated {item:?}");
                        };
                    

                    let key = item.get_key();
                    let cloned_item = Rc::clone(&item);
                    view! { cx,
                        {
                            if !read_only {
                                user_fields
                                    .insert(
                                        0,
                                        view! { cx,
                                            <button on:click=|_| { log!("Trashcan clicked!") } disabled=disabled>
                                                "🗑"
                                            </button>
                                        }
                                            .into_view(cx),
                                    );
                                user_fields
                                    .insert(
                                        0,
                                        view! { cx,
                                            <button
                                                on:click=move |_| {
                                                    cgc_data
                                                        .update(|cgc_data| {
                                                            cgc_data
                                                                .component_state = ComponentState::EditSelection {
                                                                selection_key: key.clone(),
                                                            };
                                                            log!("Updated component_state -> {:?}", cgc_data.component_state);
                                                        });
                                                    log!("Edit clicked!");
                                                }
                                                disabled=disabled
                                            >
                                                "✍"
                                            </button>
                                        }
                                            .into_view(cx),
                                    );
                            }
                            user_fields
                        }
                        <Show when=move || this_row_edit() fallback=|_| ()>
                            <div class="cgc-editable">
                                {    
                                
                                    <T as CollectionGrid>::edit_element(cx, Updatable::new((*item).clone(), this_row_updated))
                                }
                            </div>
                        </Show>
                    }
                }
            /> <button>
                <strong>"+"</strong>
            </button>
            <div class="cgc-insert" style="grid-column-start: 2; grid-column-end: 7;"></div>
        </div>
    }

    // ω <fn collection_grid_component>
}

// α <mod-def collection_grid_component>
// ω <mod-def collection_grid_component>
