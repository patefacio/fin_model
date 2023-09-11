//! Module for collection_grid_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::RwSignal;
use leptos::StoredValue;
use leptos::View;
use leptos::WriteSignal;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::console_log;
use std::boxed::Box;
use std::fmt::Debug;

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// The state of the [GridCollectionComponent]
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum CollectionGridState {
    /// The component is showing the entries as rows in a _grid_.
    Display,
    /// The component is editing a new item.
    EditNew,
    /// Currently editing row identified by `selection_key`.
    EditSelection {
        /// Key of item being edited
        selection_key: String,
    },
}

/// Specifies if new row edit or existing row edit
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum CollectionGridEditType {
    /// Edit of row in grid
    RowEdit,
    /// Edit of row to be added to grid
    NewRowEdit,
}

////////////////////////////////////////////////////////////////////////////////////
// --- traits ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a collection to be displayed in a [CollectionGridComponent].
/// Supports adding and removing entries and displaying as many _fields_ of
/// each element as the trait implementation dictates.
pub trait CollectionGrid: Sized {
    /// Data shared among all edited items.
    type SharedContext: Debug;

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
    ) -> View;

    /// Return true if row edit satisfies any shared context constraints
    ///
    ///   * **edited_row** - The edited row
    ///   * **shared_context** - The current shared context
    ///   * _return_ - An error message if not acceptable change, None otherwise
    fn accept_row_edit(
        edited_row: &Self,
        shared_context: &mut Self::SharedContext,
    ) -> Option<String>;
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
///   * **rows_updatable** - Items to show
///   * **shared_context_updatable** - Shared context
///   * **header** - Header for the grid
///   * **on_state_change** - Enables parent to track state changes.
/// For example, parent may want different behavior when editing an entry
/// versus just displaying the rows.
///   * **add_item_label** - Label to show on add button
///   * _return_ - View for collection_grid_component
#[component]
pub fn CollectionGridComponent<T, S>(
    /// Items to show
    rows_updatable: Updatable<Vec<T>>,
    /// Shared context
    shared_context_updatable: Updatable<S>,
    /// Header for the grid
    header: Vec<String>,
    /// Enables parent to track state changes.
    /// For example, parent may want different behavior when editing an entry
    /// versus just displaying the rows.
    #[prop(default=None)]
    on_state_change: Option<WriteSignal<CollectionGridState>>,
    /// Label to show on add button
    #[prop(default="Add New Item".to_string())]
    add_item_label: String,
) -> impl IntoView
where
    T: 'static + Clone + Debug + CollectionGrid<SharedContext = S>,
    S: 'static + Clone + Debug,
{
    // α <fn collection_grid_component>

    use crate::AppContext;
    use crate::OkCancel;
    use crate::OkCancelComponent;
    use leptos::create_rw_signal;
    use leptos::on_cleanup;
    use leptos::store_value;
    use leptos::use_context;
    use leptos::For;
    use leptos::IntoAttribute;
    use leptos::IntoView;
    use leptos::Show;
    use leptos::SignalGet;
    use leptos::SignalGetUntracked;
    use leptos::SignalSet;
    use leptos::SignalUpdate;
    use leptos::SignalUpdateUntracked;
    use leptos::SignalWith;
    use leptos::SignalWithUntracked;
    use leptos_dom::html::Div;
    use leptos_dom::HtmlElement;
    use std::collections::HashMap;
    use std::rc::Rc;

    /// This is used to ensure only one collection has an ok/cancel enabled at a time.
    let grid_edit_active_count = use_context::<AppContext>().unwrap().grid_edit_active_count;

    let add_to_active_count = move || {
        grid_edit_active_count.update(|count| {
            *count += 1;
        })
    };

    let remove_from_active_count = move || {
        grid_edit_active_count.update(|count| {
            *count -= 1;
        })
    };

    // Add a default new element
    let row_stored_value = store_value(<T as CollectionGrid>::new());
    let shared_context_stored_value = store_value(shared_context_updatable.value.clone());
    let active_key_stored_value: StoredValue<Option<String>> = store_value(None);
    let initial_grid_edit_active_count = grid_edit_active_count.get_untracked() + 1;
    let ok_cancel_enabled = move || {
        grid_edit_active_count.get() == initial_grid_edit_active_count
    };
    let row_count_signal = create_rw_signal(rows_updatable.value.len());

    struct CGCData<T, S> {
        rows_updatable: Updatable<Vec<T>>,
        shared_context_updatable: Updatable<S>,
        component_state: CollectionGridState,
    }

    // Entry_signals is the map of keys to the signal associated with the index of the row,
    // which enables update of view for specific row.
    let signals = store_value(
        rows_updatable
            .value
            .iter()
            .enumerate()
            .map(|(i, row)| (row.get_key(), create_rw_signal(i)))
            .collect::<HashMap<String, RwSignal<usize>>>(),
    );

    let add_item_label = move || add_item_label.clone();

    // Component data containing the vector we manage and the current state
    let cgc_data_signal = create_rw_signal(CGCData {
        rows_updatable,
        shared_context_updatable,
        component_state: CollectionGridState::Display,
    });

    // If we are not in the _display_ state we are either editing a new entry or are editing
    // a specific row. In either case we want the other rows to be disables
    let is_disabled = move || {
        cgc_data_signal.with(|cgc_data| cgc_data.component_state != CollectionGridState::Display)
    };

    let signal_state_change = move |new_state: CollectionGridState| {
        match new_state {
            CollectionGridState::Display => remove_from_active_count(),
            _ => add_to_active_count(),
        }
        if let Some(on_state_change) = on_state_change {
            on_state_change.update(|state| *state = new_state)
        }
    };

    let set_enabled = move || {
        cgc_data_signal.update(|cgc_data| cgc_data.component_state = CollectionGridState::Display);
        signal_state_change(CollectionGridState::Display);
    };

    let set_new_item_edit = move || {
        row_stored_value.update_value(|row| *row = <T as CollectionGrid>::new());
        active_key_stored_value.update_value(|active_key| *active_key = None);
        cgc_data_signal.update(|cgc_data| cgc_data.component_state = CollectionGridState::EditNew);
        signal_state_change(CollectionGridState::EditNew);
    };

    let is_new_item_edit = move || {
        cgc_data_signal.with(|cgc_data| cgc_data.component_state == CollectionGridState::EditNew)
    };

    // The line ending the grid is 3 (2 for the two buttons and 1 for indexing) plus number columns
    let grid_column_end = 3 + header.len();
    let grid_template_columns = format!("1.8rem 1.8rem repeat({}, 1fr)", header.len());

    // A header for the component, including empty fields for our `Edit` and `Delete` buttons,
    // so the shape matches that of the displayed rows
    let header = {
        let mut fields = header;
        fields.insert(0, String::default());
        fields.insert(0, String::default());
        fields
            .into_iter()
            .map(|column_header| {
                view! { <div class="header">{column_header}</div> }
            })
            .collect::<Vec<HtmlElement<Div>>>()
    };

    let editable_style =
        move || format!("grid-column-start: 1; grid-column-end: {grid_column_end}");

    // Reactive count of elements
    let num_elements_tracked = move || {
        let num_elements = row_count_signal.get();
        num_elements
    };

    let num_elements_untracked = move || row_count_signal.get_untracked();

    // Get the key associated with the row index into the vec
    let nth_key = move |n: usize| {
        cgc_data_signal.with_untracked(|cgc_data| {
            cgc_data
                .rows_updatable
                .value
                .get(n)
                .map(|row| row.get_key())
        })
    };

    // Signal to update the view of element identified by key
    let key_signal = move |key: &str| signals.with_value(|signals| signals.get(key).cloned());

    // Delete the entry corresponding to the key.
    let delete_by_key = move |key: &str| {
        let rows_len = num_elements_untracked();
        // Reach into the map of signals to get the index of the corresponding entry
        if let Some(position) =
            signals.with_value(|signals| signals.get(key).cloned().map(|signal| signal.get()))
        {
            // Call `update` since this removal of a row will need to be reflected in the
            // <For... component that is tracking `cgc_data`
            cgc_data_signal.update(|cgc_data| {
                signals.update_value(|signals| {
                    let rows = &mut cgc_data.rows_updatable.value;
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
                    signals.remove(key);
                });
                cgc_data.rows_updatable.signal();
            });
            row_count_signal.set(rows_len - 1);
        }
    };

    // Get the index into the vector of the row identified by key - **non-reactive**
    let index_by_key =
        move |key: &str| signals.with_value(|signals| signals.get(key).unwrap().get_untracked());

    let make_edit_button = move |key: &str| {
        let key = key.to_string();

        view! {
            <button
                on:click=move |_| {
                    cgc_data_signal
                        .update(|cgc_data| {
                            let new_state = CollectionGridState::EditSelection {
                                selection_key: key.clone(),
                            };
                            cgc_data.component_state = new_state.clone();
                            signal_state_change(new_state);
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
        .into_view()
    };

    let make_delete_button = move |key: &str| {
        let key = key.to_string();
        view! {
            <button
                on:click=move |_| {
                    delete_by_key(&key);
                }

                disabled=move || is_disabled()
            >
                "🗑"
            </button>
        }
        .into_view()
    };

    let is_this_row_edit = move |key: &str| {
        cgc_data_signal.with_untracked(|cgc_data| match &cgc_data.component_state {
            CollectionGridState::EditSelection { selection_key } => selection_key == &*key,
            _ => false,
        })
    };

    let row_cloned = move |key: &str| {
        cgc_data_signal.with_untracked(move |cgc_data| {
            cgc_data
                .rows_updatable
                .value
                .get(index_by_key(&*key))
                .cloned()
                .unwrap()
        })
    };

    let on_ok_cancel = move |ok_cancel: OkCancel| {
        if let Some(key) = active_key_stored_value.get_value() {
            let key_signal = key_signal(&key).unwrap();

            match ok_cancel {
                OkCancel::Ok => {
                    let index = key_signal.get_untracked();
                    cgc_data_signal.update_untracked(|cgc_data| {
                        cgc_data.rows_updatable.update_and_then_signal(|rows| {
                            let updated_row = row_stored_value.get_value();
                            if let Some(row_) = rows.get_mut(index) {
                                *row_ = updated_row;
                            } else {
                                log!("UpdateNone!!");
                            }
                        });
                    });
                    set_enabled();
                }
                OkCancel::Cancel => {
                    set_enabled();
                }
            }
            key_signal.update(|i| {});
        } else {
            // Adding new entry
            match ok_cancel {
                OkCancel::Ok => {
                    let row_count = row_count_signal.get();
                    cgc_data_signal.update(|cgc_data| {
                        cgc_data.rows_updatable.update_and_then_signal(|rows| {
                            let new_row = row_stored_value.get_value();
                            let key = new_row.get_key();
                            rows.push(new_row);
                            signals.update_value(|signals| {
                                signals.insert(key, create_rw_signal(rows.len() - 1));
                            });
                        });
                    });
                    row_count_signal.set(row_count + 1);
                    set_enabled();
                }
                OkCancel::Cancel => {
                    set_enabled();
                }
            }
        }
    };

    let edit_row_view = move || {
        view! {
            <div class="cgc-editable" style=editable_style>
                {<T as CollectionGrid>::edit_row(
                    CollectionGridEditType::RowEdit,
                    row_stored_value,
                    shared_context_stored_value,
                )}

                <Show when=move || ok_cancel_enabled() fallback=|| ()>
                    <div class="ok-cancel-bar">
                        <OkCancelComponent on_ok_cancel=on_ok_cancel/>
                    </div>
                </Show>
            </div>
        }
        .into_view()
    };

    let show_row_editor = move |key: &str| {
        let key = key.to_string();
        let this_row_edit = is_this_row_edit(&key);
        if this_row_edit {
            active_key_stored_value.update_value(|active_key| *active_key = Some(key.clone()));
            let edit_key = key.clone();
            // First get the row data and store
            row_stored_value.update_value(|row| *row = row_cloned(&edit_key));
        }

        view! {
            <Show when=move || this_row_edit fallback=|| ()>
                {edit_row_view}
            </Show>
        }
    };

    let show_new_row_editor = move || {
        view! {
            <Show when=move || is_new_item_edit() fallback=|| ()>
                {edit_row_view}
            </Show>
        }
    };

    // TRY TO REFACTOR TO FUNCTION
    // fn get_row_fields<T>(row: &T) -> Vec<View> {
    //     let mut user_fields = row.get_fields();
    //     for user_field in user_fields.iter() {
    //         let inactive_edit_key = Rc::clone(&key);
    //         if let Some(element) = user_field.as_element().cloned() {
    //             let html_element = element.into_html_element();
    //             html_element.class("inactive-edit", move || {
    //                 is_disabled() && !is_this_row_edit(&inactive_edit_key)
    //             });
    //         }
    //     }
    //     user_fields.insert(0, make_delete_button(&key));
    //     user_fields.insert(0, make_edit_button(&key));
    //     user_fields
    // }

    view! {
        <div
            class="collection-grid"
            style=format!("display: grid; grid-template-columns: {grid_template_columns}")
        >
            {header}
            <For
                each=move || { 0..num_elements_tracked() }
                key=move |&i| { nth_key(i) }

                view=move |i| {
                    let key = Rc::new(nth_key(i).unwrap());
                    if let Some(key_signal) = key_signal(&key) {
                        view! {
                            {move || {
                                let key = Rc::clone(&key);
                                key_signal
                                    .with(move |&index| {
                                        let key = Rc::clone(&key);
                                        cgc_data_signal
                                            .with_untracked(move |cgc_data| {
                                                let key = Rc::clone(&key);
                                                let row = cgc_data.rows_updatable.value.get(index).unwrap();
                                                let mut user_fields = row.get_fields();
                                                for user_field in user_fields.iter() {
                                                    let inactive_edit_key = Rc::clone(&key);
                                                    if let Some(element) = user_field.as_element().cloned() {
                                                        let html_element = element.into_html_element();
                                                        html_element
                                                            .class(
                                                                "inactive-edit",
                                                                move || {
                                                                    is_disabled() && !is_this_row_edit(&inactive_edit_key)
                                                                },
                                                            );
                                                    }
                                                }
                                                user_fields.insert(0, make_delete_button(&key));
                                                user_fields.insert(0, make_edit_button(&key));
                                                view! {
                                                    {user_fields.into_view()}
                                                    {show_row_editor(&key)}
                                                }
                                                    .into_view()
                                            })
                                    })
                            }}
                        }
                            .into_view()
                    } else {
                        ().into_view()
                    }
                }
            />

            <Show when=move || !is_disabled() fallback=|| ()>
                <button
                    class="cgc-add-row"
                    style=format!("grid-column-start: 0; grid-column-end: {grid_column_end};")
                    on:click=move |_| { set_new_item_edit() }
                >
                    <strong>{add_item_label()}</strong>
                </button>
            </Show>
            {show_new_row_editor}
        </div>
    }

    // ω <fn collection_grid_component>
}

// α <mod-def collection_grid_component>
// ω <mod-def collection_grid_component>
