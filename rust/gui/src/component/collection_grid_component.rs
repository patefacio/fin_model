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
    ///   * **on_cancel** - Called if edit is canceled.
    ///   * _return_ - The edit view
    fn edit_element<F>(cx: Scope, updatable: Updatable<Self>, on_cancel: F) -> View
    where
        F: 'static + FnMut(&str);
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
    use leptos::For;
    use leptos::IntoAttribute;
    use leptos::IntoView;
    use leptos::Show;
    use leptos::SignalGet;
    use leptos::SignalGetUntracked;
    use leptos::SignalUpdate;
    use leptos::SignalUpdateUntracked;
    use leptos::SignalWith;
    use leptos::SignalWithUntracked;
    use leptos_dom::html::Div;
    use leptos_dom::HtmlElement;
    use std::collections::HashMap;
    use std::rc::Rc;

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

    // Entry_signals is the map of keys to the signal associated with the index of the row,
    // which enables update of view for specific row.
    let signals = store_value(
        cx,
        updatable
            .value
            .iter()
            .enumerate()
            .map(|(i, row)| (row.get_key(), create_rw_signal(cx, i)))
            .collect::<HashMap<String, RwSignal<usize>>>(),
    );

    // Component data containing the vector we manage and the current state
    let cgc_data_signal = create_rw_signal(
        cx,
        CGCData {
            updatable,
            component_state: ComponentState::Display,
        },
    );

    // If we are not in the _display_ state we are either editing a new entry or are editing
    // a specific row. In either case we want the other rows to be disables
    let is_disabled = move || {
        cgc_data_signal.with(|cgc_data| cgc_data.component_state != ComponentState::Display)
    };

    let set_enabled = move || {
        log!("Setting state back to display");
        cgc_data_signal.update(|cgc_data| cgc_data.component_state = ComponentState::Display)
    };

    let set_new_item_edit = move || {
        log!("Setting state back to display");
        cgc_data_signal.update(|cgc_data| cgc_data.component_state = ComponentState::EditNew)
    };

    let is_new_item_edit = move || {
        cgc_data_signal.with(|cgc_data| cgc_data.component_state == ComponentState::EditNew)
    };

    // A header for the component, including empty fields for our `Edit` and `Delete` buttons,
    // so the shape matches that of the displayed rows
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

    // Reactive count of elements
    let num_elements = move || cgc_data_signal.with(|cgc_data| cgc_data.updatable.value.len());

    // Get the key associated with the row index into the vec
    let nth_key = move |n: usize| {
        cgc_data_signal.with_untracked(|cgc_data| {
            cgc_data
                .updatable
                .value
                .get(n)
                .map(|row| row.get_key().clone())
        })
    };

    // Signal to update the view of element identified by key
    let key_signal = move |key: &str| signals.with_value(|signals| signals.get(key).cloned());

    // Delete the entry corresponding to the key.
    let delete_by_key = move |key: &str| {
        // Reach into the map of signals to get the index of the corresponding entry
        if let Some(position) =
            signals.with_value(|signals| signals.get(key).cloned().map(|signal| signal.get()))
        {
            // Call `update` since this removal of a row will need to be reflected in the
            // <For... component that is tracking `cgc_data`
            cgc_data_signal.update(|cgc_data| {
                signals.update_value(|signals| {
                    let rows = &mut cgc_data.updatable.value;
                    rows.remove(position);
                    let end = rows.len();
                    // After removing the row we need to iterate over all subsequent
                    // rows and decrement their index into the vector so they point
                    // at the proper entry.
                    let elements_after = &rows[position..end];
                    for (i, row) in elements_after.iter().enumerate() {
                        if let Some(row_signal) = signals.get_mut(&row.get_key()) {
                            row_signal.update_untracked(|index| *index = position + i);
                        }
                    }
                });
            });
        }
    };

    // Get the index into the vector of the row identified by key - **non-reactive**
    let index_by_key =
        move |key: &str| signals.with_value(|signals| signals.get(key).unwrap().get_untracked());

    let make_edit_button = move |key: &str| {
        let key = key.to_string();

        view! { cx,
            <button
                on:click=move |_| {
                    cgc_data_signal
                        .update(|cgc_data| {
                            cgc_data
                                .component_state = ComponentState::EditSelection {
                                selection_key: key.clone(),
                            };
                        });
                    if let Some(signal) = key_signal(&key) {
                        signal.update(|_| {});
                    }
                }
                disabled=move || is_disabled()
            >
                "✍"
            </button>
        }
        .into_view(cx)
    };

    let make_delete_button = move |key: &str| {
        let key = key.to_string();
        view! { cx,
            <button
                on:click=move |_| {
                    log!("Trashcan clicked!");
                    delete_by_key(&key);
                }
                disabled=move || is_disabled()
            >
                "🗑"
            </button>
        }
        .into_view(cx)
    };

    let is_this_row_edit = move |key: &str| {
        cgc_data_signal.with_untracked(|cgc_data| match &cgc_data.component_state {
            ComponentState::EditSelection { selection_key } => selection_key == &*key,
            _ => false,
        })
    };

    let this_row_updated = move |row: &T| {
        if let Some(key_signal) = key_signal(&row.get_key()) {
            let index = key_signal.get_untracked();
            cgc_data_signal.update_untracked(|cgc_data| {
                if let Some(row_) = cgc_data.updatable.value.get_mut(index) {
                    *row_ = row.clone();
                }
            });
            set_enabled();
            key_signal.update(|i| log!("Signalling {i} for {row:?}"));
        }
    };

    let this_row_canceled = move |key: &str| {
        set_enabled();
        if let Some(signal) = key_signal(key) {
            signal.update(|_| {});
        }
    };

    let row_cloned = move |key: &str| {
        cgc_data_signal.with_untracked(move |cgc_data| {
            cgc_data
                .updatable
                .value
                .get(index_by_key(&*key))
                .cloned()
                .unwrap()
        })
    };

    let show_row_editor = move |key: &str| {
        let key = key.to_string();
        let edit_key = key.clone();
        view! { cx,
            <Show when=move || { is_this_row_edit(&key) } fallback=|_| ()>
                <div class="cgc-editable">
                    {<T as CollectionGrid>::edit_element(
                        cx,
                        Updatable::new(row_cloned(&edit_key), this_row_updated),
                        this_row_canceled,
                    )}
                </div>
            </Show>
        }
    };

    let show_new_row_editor = move || {
        view! { cx,
            <div class="cgc-insert" style="grid-column-start: 2; grid-column-end: 7;"></div>
            <Show when=move || is_new_item_edit() fallback=|_| ()>
                <div class="cgc-editable">
                    {<T as CollectionGrid>::edit_element(
                        cx,
                        Updatable::new(<T as CollectionGrid>::new(), this_row_updated),
                        this_row_canceled,
                    )}
                </div>
            </Show>
        }
    };

    view! { cx,
        <div style="display: grid; grid-template-columns: 1.8rem 1.8rem 1fr 1fr 1fr 1fr;">
            {header}
            <For
                each=move || { 0..num_elements() }
                key=move |&i| { nth_key(i) }
                view=move |cx, i| {
                    let key = Rc::new(nth_key(i).unwrap());
                    if let Some(key_signal) = key_signal(&key) {
                        view! { cx,
                            {move || {
                                let key = Rc::clone(&key);
                                key_signal
                                    .with(move |&index| {
                                        let key = Rc::clone(&key);
                                        cgc_data_signal
                                            .with_untracked(move |cgc_data| {
                                                let key = Rc::clone(&key);
                                                if let Some(row) = cgc_data.updatable.value.get(index) {
                                                    let mut user_fields = row.get_fields(cx);
                                                    if !read_only {
                                                        user_fields.insert(0, make_delete_button(&key));
                                                        user_fields.insert(0, make_edit_button(&key));
                                                    }
                                                    [user_fields.into_view(cx), show_row_editor(&key)].into_view(cx)
                                                } else {
                                                    ().into_view(cx)
                                                }
                                            })
                                    })
                            }}
                        }
                            .into_view(cx)
                    } else {
                        ().into_view(cx)
                    }
                }
            /> <button on:click=move |_| { set_new_item_edit() } disabled=move || is_disabled()>
                <strong>"+"</strong>
            </button> {show_new_row_editor}
        </div>
    }

    // ω <fn collection_grid_component>
}

// α <mod-def collection_grid_component>
// ω <mod-def collection_grid_component>
