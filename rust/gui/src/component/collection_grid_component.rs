//! Module for collection_grid_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::UpdatablePair;
#[allow(unused_imports)]
use leptos::log;
use leptos::RwSignal;
use leptos::SignalUpdate;
use leptos::View;
use leptos::WriteSignal;
use leptos::{component, view, IntoView, Scope};
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
///   * **header** - Header for the grid
///   * **on_state_change** - Enables parent to track state changes.
/// For example, parent may want different behavior when editing an entry
/// versus just displaying the rows.
///   * **add_item_label** - Label to show on add button
///   * _return_ - View for collection_grid_component
#[component]
pub fn CollectionGridComponent<T, S>(
    /// Context
    cx: Scope,
    /// Items to show
    updatable: UpdatablePair<Vec<T>, S>,
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

    use crate::UpdatePairType;
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

    struct CGCData<T, S> {
        updatable: UpdatablePair<Vec<T>, S>,
        component_state: CollectionGridState,
    }

    // Entry_signals is the map of keys to the signal associated with the index of the row,
    // which enables update of view for specific row.
    let signals = store_value(
        cx,
        updatable
            .first_value
            .iter()
            .enumerate()
            .map(|(i, row)| (row.get_key(), create_rw_signal(cx, i)))
            .collect::<HashMap<String, RwSignal<usize>>>(),
    );

    let add_item_label = move || add_item_label.clone();

    // Component data containing the vector we manage and the current state
    let cgc_data_signal = create_rw_signal(
        cx,
        CGCData {
            updatable,
            component_state: CollectionGridState::Display,
        },
    );

    // If we are not in the _display_ state we are either editing a new entry or are editing
    // a specific row. In either case we want the other rows to be disables
    let is_disabled = move || {
        cgc_data_signal.with(|cgc_data| cgc_data.component_state != CollectionGridState::Display)
    };

    let signal_state_change = move |new_state: CollectionGridState| {
        if let Some(on_state_change) = on_state_change {
            on_state_change.update(|state| *state = new_state)
        }
    };

    let set_enabled = move || {
        log!("Setting state back to display");
        cgc_data_signal.update(|cgc_data| cgc_data.component_state = CollectionGridState::Display);
        signal_state_change(CollectionGridState::Display);
    };

    let set_new_item_edit = move || {
        log!("Setting state back to display");
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
                view! { cx, <div class="header">{column_header}</div> }
            })
            .collect::<Vec<HtmlElement<Div>>>()
    };

    let editable_style =
        move || format!("grid-column-start: 1; grid-column-end: {grid_column_end}");

    // Reactive count of elements
    let num_elements =
        move || cgc_data_signal.with(|cgc_data| cgc_data.updatable.first_value.len());

    // Get the key associated with the row index into the vec
    let nth_key = move |n: usize| {
        cgc_data_signal.with_untracked(|cgc_data| {
            cgc_data
                .updatable
                .first_value
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
                    let rows = &mut cgc_data.updatable.first_value;
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
            CollectionGridState::EditSelection { selection_key } => selection_key == &*key,
            _ => false,
        })
    };

    let this_row_updated =
        move |(row, shared_context, update_type): (&T, &S, crate::UpdatePairType)| {
            if let Some(key_signal) = key_signal(&row.get_key()) {
                let index = key_signal.get_untracked();
                cgc_data_signal.update_untracked(|cgc_data| {
                    cgc_data.updatable.update_and_then_signal(|(rows, shared)| {
                        if let Some(row_) = rows.get_mut(index) {
                            log!(
                                "Collection grid updating row {index} -> `{}` with\n{row:?}",
                                row.get_key()
                            );
                            *row_ = row.clone();
                            update_type
                        } else {
                            UpdatePairType::UpdateNone
                        }
                    });
                });
                set_enabled();
                key_signal.update(|i| log!("Signalling {i} for {row:?}"));
            } else {
                panic!("No signal for row `{}`", row.get_key());
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
                .first_value
                .get(index_by_key(&*key))
                .cloned()
                .unwrap()
        })
    };

    let shared_context_cloned =
        move || cgc_data_signal.with_untracked(|cgc_data| cgc_data.updatable.second_value.clone());

    let show_row_editor = move |key: &str| {
        let key = key.to_string();
        let edit_key = key.clone();
        view! { cx,
            <Show when=move || { is_this_row_edit(&key) } fallback=|_| ()>
                <div class="cgc-editable" style=editable_style>
                    {<T as CollectionGrid>::edit_element(
                        cx,
                        UpdatablePair::new(
                            row_cloned(&edit_key),
                            shared_context_cloned(),
                            this_row_updated,
                        ),
                        this_row_canceled,
                    )}

                </div>
            </Show>
        }
    };

    let show_new_row_editor = move || {
        view! { cx,
            <div
                class="cgc-insert"
                style=format!("grid-column-start: 2; grid-column-end: {};", grid_column_end)
            ></div>
            <Show when=move || is_new_item_edit() fallback=|_| ()>
                <div class="cgc-editable" style=editable_style>
                    {<T as CollectionGrid>::edit_element(
                        cx,
                        UpdatablePair::new(
                            <T as CollectionGrid>::new(),
                            shared_context_cloned(),
                            this_row_updated,
                        ),
                        this_row_canceled,
                    )}

                </div>
            </Show>
        }
    };

    view! { cx,
        <div
            class="collection-grid"
            style=format!("display: grid; grid-template-columns: {grid_template_columns}")
        >
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
                                                if let Some(row) = cgc_data.updatable.first_value.get(index)
                                                {
                                                    let mut user_fields = row.get_fields(cx);
                                                    for user_field in user_fields.iter() {
                                                        let inactive_edit_key = Rc::clone(&key);
                                                        if let Some(element) = user_field.as_element().cloned() {
                                                            let html_element = element.into_html_element(cx);
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

                                                    view! { cx,
                                                        {user_fields.into_view(cx)}
                                                        {show_row_editor(&key)}
                                                    }
                                                        .into_view(cx)
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
            />

            <Show when=move || !is_new_item_edit() fallback=|_| ()>
                <button
                    class="cgc-add-row"
                    style=format!("grid-column-start: 0; grid-column-end: {grid_column_end};")
                    on:click=move |_| { set_new_item_edit() }
                    disabled=move || is_disabled()
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
