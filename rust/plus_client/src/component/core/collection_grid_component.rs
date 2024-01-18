//! Module for collection_grid_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::OkCancel;
use crate::Updatable;
use leptos::component;
use leptos::view;
use leptos::IntoView;
use leptos::RwSignal;
use leptos::StoredValue;
use leptos::View;
use leptos::WriteSignal;
use std::collections::HashMap;
use std::collections::HashSet;
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

/// Result for an edit complete operation
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum EditCompleteResult {
    /// A selection edit has modified the key (i.e. name) such that it collides with another
    /// previously defined row's key.
    RejectNameCollision {
        /// Some detail about name collision
        message: String,
    },
    /// The edit_complete operation was accepted
    Accepted,
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

    /// Get number of fields provided by `get_fields` to be displayed.
    /// Used to build the `grid-template-columns` style. Two additional fields
    /// are added (_edit button_, _delete button_).
    ///
    ///   * _return_ - Number of fields
    fn get_fields_len() -> usize;

    /// Get the display fields for the element.
    ///
    ///   * _return_ - The fields as elements
    fn get_fields(&self) -> Vec<View>;

    /// Get the header for the rows.
    ///
    ///   * _return_ - The header
    fn get_header() -> Vec<String>;

    /// Get the text for `Add New Item`.
    ///
    ///   * _return_ - The add item label
    fn get_add_item_label() -> String;

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
        #[allow(unused)] shared_context_stored_value: StoredValue<Self::SharedContext>,
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
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// CollectionGridComponent data kept as stored value
pub struct CgcData<T, S>
where
    T: 'static,
    S: 'static,
{
    /// The vector of rows
    pub rows_updatable: Updatable<Vec<T>>,
    /// The shared context to signal
    pub shared_context_updatable: Updatable<S>,
    /// Row being edited
    pub row_stored_value: StoredValue<T>,
    /// The shared context **being edited**.
    /// When an edit is accepted this is pushed into the updatable and parent is signaled.
    pub shared_context_stored_value: StoredValue<S>,
    /// Maps key to row signal
    pub row_signals: HashMap<String, RwSignal<usize>>,
    /// The component state
    pub component_state: CollectionGridState,
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
///   * **on_state_change** - Enables parent to track state changes.
/// For example, parent may want different behavior when editing an entry
/// versus just displaying the rows.
///   * _return_ - View for collection_grid_component
#[component]
pub fn CollectionGridComponent<T, S>(
    /// Items to show
    rows_updatable: Updatable<Vec<T>>,
    /// Shared context
    shared_context_updatable: Updatable<S>,
    /// Enables parent to track state changes.
    /// For example, parent may want different behavior when editing an entry
    /// versus just displaying the rows.
    #[prop(default=None)]
    on_state_change: Option<WriteSignal<CollectionGridState>>,
) -> impl IntoView
where
    T: 'static + Clone + Debug + CollectionGrid<SharedContext = S>,
    S: 'static + Clone + Debug,
{
    let component_id = crate::component_id!("`CollectionGridComponent`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn collection_grid_component>

    use crate::AppContext;
    use crate::ClientCssClasses;
    use crate::OkCancel;
    use crate::OkCancelComponent;
    use leptos::create_rw_signal;
    use leptos::expect_context;
    use leptos::store_value;
    use leptos::For;
    #[allow(unused)]
    use leptos::IntoAttribute;
    use leptos::IntoView;
    use leptos::Show;
    use leptos::SignalGet;
    use leptos::SignalGetUntracked;
    use leptos::SignalSet;
    use leptos::SignalUpdate;
    use leptos::SignalWith;
    use leptos_dom::html::Div;
    use leptos_dom::HtmlElement;
    use std::rc::Rc;

    // This is used to ensure only one collection has an ok/cancel enabled at a time.
    // Because grids nest (e.g. AccountsGrid has AccountComponents each of which has
    // HoldingsGrid). When user opens an account and then a holding within it, without
    // this logic there would be to <Ok/Cancel> bars showing which could be quite confusing.
    // This ensures only the innermost <Ok/Cancel> bar is shown.
    let app_context = expect_context::<Rc<AppContext>>();
    let lang_selector = app_context.lang_selector;
    let display_currency = app_context.display_currency;
    let grid_edit_active_count = app_context.grid_edit_active_count;

    let add_to_active_count = move || {
        grid_edit_active_count.update(|count| {
            tracing::debug!("Added active count to {}", *count + 1);
            *count += 1;
        })
    };

    let remove_from_active_count = move || {
        grid_edit_active_count.update(|count| {
            tracing::debug!("Removed active count to {}", *count - 1);
            *count -= 1;
        })
    };

    let initial_grid_edit_active_count = grid_edit_active_count.get_untracked() + 1;
    tracing::debug!(
        "CollectionGrid creation with initial active count {initial_grid_edit_active_count}"
    );
    let ok_cancel_enabled = move || grid_edit_active_count.get() == initial_grid_edit_active_count;

    let add_item_label = move || {
        use leptos::SignalWith;
        lang_selector.track();
        <T as CollectionGrid>::get_add_item_label()
    };

    // Used to signal state change (e.g. going from Display to EditNew or EditSelection and back)
    let state_change_signal = create_rw_signal(());

    // Used to signal vector cardinality change
    let row_count_signal = create_rw_signal(rows_updatable.value.len());

    // Component data containing the vector we manage and the current state
    let cgc_data_stored_value = store_value(CgcData::new(rows_updatable, shared_context_updatable));

    // Current row count, subject to reactivity on row_count_signal
    let row_count_reactive = move || {
        //use leptos::SignalWith;
        //row_count_signal.track();
        //let row_count = cgc_data_stored_value.with_value(|cgc_data| cgc_data.rows_updatable.value.len());
        let row_count = row_count_signal.get();
        tracing::debug!("Reacting to new row count {row_count}");
        row_count
    };

    // Signals update of row count - called when adding/deleting rows
    let row_count_updated = move || {
        tracing::debug!("Row count update on CGC");
        let new_row_count =
            cgc_data_stored_value.with_value(|cgc_data| cgc_data.rows_updatable.value.len());
        tracing::debug!("Row count updated to {new_row_count}");
        row_count_signal.set(new_row_count);
    };

    // Current state subject to reactivity on state_change_signal
    let state_change_reactive = move || {
        use leptos::SignalWith;
        state_change_signal.track();
        let new_state =
            cgc_data_stored_value.with_value(|cgc_data| cgc_data.component_state.clone());
        tracing::debug!("STATE CHANGE REACTIVE: State has changed to {new_state:?}");
        if let Some(on_state_change) = on_state_change {
            on_state_change.update(|state| *state = new_state.clone());
        }
        new_state
    };

    // The component goes into a _disabled_ state when editing a selected or new entry.
    // In this state the edit and delete buttons of all the rows are disabled and the
    // Ok/Cancel bar for the current edit is shown.
    let is_disabled_reactive = move || {
        use leptos::SignalWith;
        state_change_signal.track();
        let result = cgc_data_stored_value
            .with_value(|cgc_data| cgc_data.component_state != CollectionGridState::Display);
        result
    };

    // Called to get a new item and enter into CollectionGridState::NewEdit.
    // Signals the state change
    let set_new_item_edit = move || {
        cgc_data_stored_value.update_value(|cgc_data| cgc_data.edit_new_item());
        state_change_signal.set(());
    };

    let header_len = <T as CollectionGrid>::get_fields_len() + 2;

    // A header for the component, including empty fields for our `Edit` and `Delete` buttons,
    // so the shape matches that of the displayed rows
    let header_reactive = move || {
        use leptos::SignalWith;
        lang_selector.track();
        let mut fields = <T as CollectionGrid>::get_header();
        fields.insert(0, String::default());
        fields.insert(0, String::default());
        fields
            .into_iter()
            .map(|column_header| {
                view! { <div class=ClientCssClasses::HeaderRight.as_str()>{column_header}</div> }
            })
            .collect::<Vec<HtmlElement<Div>>>()
    };

    // The line ending the grid is 3 (2 for the two buttons and 1 for indexing) plus number columns
    let grid_column_end = 1 + header_len;
    // The number of columns is 2 for edit and delete buttons and one for each cell in the header
    let grid_template_columns = format!("1.8rem 1.8rem repeat({}, 1fr)", header_len - 2);

    let editable_style =
        move || format!("grid-column-start: 1; grid-column-end: {grid_column_end}");

    // Get the key associated with the row index into the vec - **Not Reactive**
    let nth_key = move |n: usize| cgc_data_stored_value.with_value(|cgc_data| cgc_data.nth_key(n));

    // Signal to update the view of element identified by key
    let key_index_signal = move |key: &str| {
        cgc_data_stored_value.with_value(|cgc_data| cgc_data.key_index_signal(key))
    };

    // Delete the entry corresponding to the key.
    let delete_by_key = move |key: &str| {
        cgc_data_stored_value.update_value(|cgc_data| cgc_data.delete_item(key));
        row_count_updated();
        tracing::debug!(
            "Deleted `{key}` BT -> {:?}",
            std::backtrace::Backtrace::capture()
        );
    };

    let make_edit_button = move |key: &str| {
        let key = key.to_string();
        view! {
            <button
                on:click=move |_| {
                    let key = key.clone();
                    cgc_data_stored_value
                        .update_value(|cgc_data| {
                            tracing::debug!("EDITING `{key}` for button press!");
                            cgc_data.edit_item(&key);
                        });
                    cgc_data_stored_value
                        .with_value(|cgc_data| {
                            let key_index_signal = cgc_data.key_index_signal(&key);
                            tracing::debug!(
                                "Signaling `{key}` {} to refresh -> {key_index_signal:?}",
                                key_index_signal.get_untracked()
                            );
                            key_index_signal
                        })
                        .update(|_| ());
                    tracing::debug!("SIGNALLING STATE CHANGE!");
                    state_change_signal.set(());
                }

                disabled=move || is_disabled_reactive()
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
                    row_count_updated()
                }

                disabled=move || is_disabled_reactive()
            >
                "🗑"
            </button>
        }
        .into_view()
    };

    let on_ok_cancel = move |ok_cancel: OkCancel| {
        let mut edit_complete_result = EditCompleteResult::Accepted;
        // Try to complete the edit. Save the edit completion result
        let mut active_signal = None;

        cgc_data_stored_value.update_value(|cgc_data| {
            active_signal = cgc_data.active_signal();
            edit_complete_result = cgc_data.edit_complete(ok_cancel);
        });

        if edit_complete_result == EditCompleteResult::Accepted {
            remove_from_active_count();
            // Signal state change
            state_change_signal.set(());

            if let Some(active_signal) = active_signal {
                // There was an active edit, signal to hide its edit view
                active_signal.update(|_| ());
            } else {
                // Was a new edit, signal cardinality change
                row_count_updated();
            }
        } else {
            tracing::debug!("Edit complete failed -> {edit_complete_result:?}");
        }
    };

    let edit_row_view = move || {
        add_to_active_count();

        let (row_stored_value, shared_context_stored_value) =
            cgc_data_stored_value.with_value(|cgc_data| {
                (
                    cgc_data.row_stored_value,
                    cgc_data.shared_context_stored_value,
                )
            });

        view! {
            <div class=ClientCssClasses::CgcEditable.as_str() style=editable_style>
                {<T as CollectionGrid>::edit_row(
                    CollectionGridEditType::RowEdit,
                    row_stored_value,
                    shared_context_stored_value,
                )}

                <Show when=move || ok_cancel_enabled() fallback=|| ()>
                    <div class=ClientCssClasses::OkCancelBar.as_str()>
                        <OkCancelComponent on_ok_cancel=on_ok_cancel/>
                    </div>
                </Show>
            </div>
        }
        .into_view()
    };

    let show_new_row_editor = move || {
        view! {
            <Show
                when=move || state_change_reactive() == CollectionGridState::EditNew
                fallback=|| ()
            >
                {edit_row_view}
            </Show>
        }
    };

    view! {
        <div
            class=ClientCssClasses::CollectionGrid.as_str()
            style=format!("grid-template-columns: {grid_template_columns}")
        >
            {header_reactive}
            <For
                each=move || { 0..row_count_reactive() }
                key=move |&i| { nth_key(i) }
                children=move |i| {
                    let initial_key = nth_key(i);
                    let key_index_signal = key_index_signal(&initial_key);
                    let is_this_row_edit = move || {
                        let index = key_index_signal.get();
                        let key = nth_key(index);
                        cgc_data_stored_value.with_value(|cgc_data| cgc_data.is_active_key(&key))
                    };
                    view! {
                        {move || {
                            lang_selector.track();
                            display_currency.track();
                            let index = key_index_signal.get();
                            let key = nth_key(index);
                            let mut user_fields = cgc_data_stored_value
                                .with_value(|cgc_data| {
                                    let row = cgc_data.rows_updatable.value.get(index).unwrap();
                                    row.get_fields()
                                });
                            user_fields.insert(0, make_delete_button(&key));
                            user_fields.insert(0, make_edit_button(&key));
                            user_fields.into_view()
                        }}

                        <Show when=move || { is_this_row_edit() } fallback=|| ()>
                            {edit_row_view}
                        </Show>
                    }
                }
            />

            <Show when=move || !is_disabled_reactive() fallback=|| ()>
                <button
                    class=ClientCssClasses::CgcAddRow.as_str()
                    style=format!("grid-column-end: {grid_column_end};")
                    on:click=move |_| { set_new_item_edit() }
                >

                    {add_item_label()}
                </button>
            </Show>
            {show_new_row_editor}
        </div>
    }

    // ω <fn collection_grid_component>
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl<T, S> CgcData<T, S>
where
    T: 'static + CollectionGrid + Debug + Clone,
    S: 'static + Debug + Clone,
{
    /// Create new data with default `T` and client updatables
    ///
    ///   * **rows_updatable** - Items to show
    ///   * **shared_context_updatable** - Shared context
    ///   * _return_ - New data instance
    pub fn new(
        rows_updatable: Updatable<Vec<T>>,
        shared_context_updatable: Updatable<S>,
    ) -> CgcData<T, S> {
        // α <fn CgcData[T,S]::new>

        use leptos::create_rw_signal;
        use leptos::store_value;

        let mut keys: HashSet<String> = HashSet::with_capacity(rows_updatable.value.len());

        let row_signals = rows_updatable
            .value
            .iter()
            .enumerate()
            .map(|(i, t)| {
                keys.insert(t.get_key());
                (t.get_key(), create_rw_signal(i))
            })
            .collect();

        let shared_context_stored_value = store_value(shared_context_updatable.value.clone());

        CgcData {
            rows_updatable,
            shared_context_updatable,
            row_stored_value: store_value(<T as CollectionGrid>::new()),
            shared_context_stored_value,
            row_signals,
            component_state: CollectionGridState::Display,
        }

        // ω <fn CgcData[T,S]::new>
    }

    /// Edit item identified by key
    ///
    ///   * **key** - Identifies the row to edit
    ///   * _return_ - The new state with the key of the active row
    #[inline]
    pub fn edit_item(&mut self, key: &str) -> CollectionGridState {
        // α <fn CgcData[T,S]::edit_item>

        use leptos::SignalGetUntracked;
        self.component_state = CollectionGridState::EditSelection {
            selection_key: key.into(),
        };

        let index = self.key_index_signal(key).get_untracked();

        // Get the row from vector and store as active row
        self.row_stored_value.update_value(|row| {
            *row = self
                .rows_updatable
                .value
                .get(index)
                .expect("Row {index} found")
                .clone()
        });

        self.component_state.clone()
        // ω <fn CgcData[T,S]::edit_item>
    }

    /// Edit a new item
    #[inline]
    pub fn edit_new_item(&mut self) {
        // α <fn CgcData[T,S]::edit_new_item>

        self.row_stored_value
            .update_value(|row| *row = <T as CollectionGrid>::new());

        self.component_state = CollectionGridState::EditNew;

        // ω <fn CgcData[T,S]::edit_new_item>
    }

    /// Retrieves the key of edit row.
    /// This is only available when a selected row is being edited.
    /// In display and new edit state there is no active key.
    ///
    ///   * **key** - Key to compare to active
    ///   * _return_ - The active key
    #[inline]
    pub fn is_active_key(&self, key: &str) -> bool {
        // α <fn CgcData[T,S]::is_active_key>
        match &self.component_state {
            CollectionGridState::EditSelection { selection_key } => key == selection_key,
            _ => false,
        }
        // ω <fn CgcData[T,S]::is_active_key>
    }

    /// Returns signal tied to the active edit row, if exists.
    /// When editing a new row there is no active edit row.
    /// When editing a selected row, that row is the active edit row
    /// and the signal returned is connected to it.
    ///
    ///   * _return_ - If editing a row, the signal tied to that row.
    #[inline]
    pub fn active_signal(&mut self) -> Option<RwSignal<usize>> {
        // α <fn CgcData[T,S]::active_signal>

        match &self.component_state {
            CollectionGridState::EditSelection { selection_key } => Some(
                self.row_signals
                    .get(selection_key)
                    .expect("Row signal for active row `{selection_key}`")
                    .clone(),
            ),
            _ => None,
        }

        // ω <fn CgcData[T,S]::active_signal>
    }

    /// Delete the item identified by key
    ///
    ///   * **key** - Identifies row to delete
    pub fn delete_item(&mut self, key: &str) {
        // α <fn CgcData[T,S]::delete_item>

        use leptos::SignalGetUntracked;
        use leptos::SignalUpdateUntracked;

        self.log_state(&format!("pre-delete of `{key}`"));

        if let Some(position) = self
            .row_signals
            .get(key)
            .cloned()
            .map(|signal| signal.get_untracked())
        {
            let rows = &mut self.rows_updatable.value;
            rows.remove(position);
            let end = rows.len();
            // After removing the row we need to iterate over all subsequent
            // rows and decrement their index into the vector so they point
            // at the proper entry.
            let elements_after = &rows[position..end];
            for (i, row) in elements_after.iter().enumerate() {
                let key = row.get_key();
                if let Some(row_signal) = self.row_signals.get_mut(&key) {
                    row_signal.update_untracked(|index| *index = position + i);
                }
            }
            self.row_signals.remove(key);
            self.log_state(&format!("post-delete of `{key}`"));
        } else {
            panic!("Could not find item `{key}` to delete!");
        }

        // ω <fn CgcData[T,S]::delete_item>
    }

    /// Processes the completed edit based on ok/cancel status
    ///
    ///   * **ok_cancel** - The exit status of the edit
    ///   * _return_ - Result status of the operation
    pub fn edit_complete(&mut self, ok_cancel: OkCancel) -> EditCompleteResult {
        // α <fn CgcData[T,S]::edit_complete>

        use leptos::create_rw_signal;
        use leptos::SignalGetUntracked;

        let mut result = EditCompleteResult::Accepted;

        match ok_cancel {
            OkCancel::Ok => {
                match &self.component_state {
                    // Ok for selected edit
                    CollectionGridState::EditSelection { selection_key } => {
                        let row_signal = self
                            .row_signals
                            .get(selection_key)
                            .expect("Active key `{selection_key}` has a signal");

                        let index = row_signal.get_untracked();
                        let rows_updatable = &mut self.rows_updatable;
                        let row_signals = &mut self.row_signals;
                        let row_stored_value = &mut self.row_stored_value;

                        rows_updatable.update(|rows| {
                            if let Some(row_) = rows.get_mut(index) {
                                let updated_row = row_stored_value.get_value();
                                let updated_key = updated_row.get_key();
                                let key_name_change = &updated_key != selection_key;
                                let name_collision =
                                    key_name_change && row_signals.contains_key(&updated_key);

                                if name_collision {
                                    result = EditCompleteResult::RejectNameCollision {
                                        message: format!(
                                            "New name `{updated_key}` collides with existing item"
                                        ),
                                    };
                                } else {
                                    if key_name_change {
                                        let row_signal = row_signals
                                            .remove(selection_key)
                                            .expect("Old key `{selection_key}` exists");
                                        row_signals.insert(updated_key, row_signal);
                                    }
                                    *row_ = updated_row;
                                }
                            } else {
                                unreachable!("Unable to find row for {index}!");
                            }
                        });
                    }
                    // Ok for **new** edit
                    _ => {
                        let new_key = self.row_stored_value.with_value(|row| row.get_key());
                        self.row_signals
                            .insert(new_key, create_rw_signal(self.rows_updatable.value.len()));
                        self.rows_updatable
                            .update(|rows| rows.push(self.row_stored_value.get_value()));
                    }
                };
            }
            OkCancel::Cancel => {
                tracing::debug!("OkCancel::Cancel received!")
            }
        };

        if result == EditCompleteResult::Accepted {
            self.rows_updatable.signal();
            self.component_state = CollectionGridState::Display;
        }

        result

        // ω <fn CgcData[T,S]::edit_complete>
    }

    /// Key of nth item
    ///
    ///   * **n** - Index of item to retrieve key
    ///   * _return_ - Key of the `nth` item.
    #[inline]
    pub fn nth_key(&self, n: usize) -> String {
        // α <fn CgcData[T,S]::nth_key>

        self.rows_updatable
            .value
            .get(n)
            .map(|row| row.get_key())
            .expect(&format!("Signal present at {n}"))

        // ω <fn CgcData[T,S]::nth_key>
    }

    /// The index signal for the key
    ///
    ///   * **key** - The key to retrieve signal
    ///   * _return_ - Index signal for the key.
    #[inline]
    pub fn key_index_signal(&self, key: &str) -> RwSignal<usize> {
        // α <fn CgcData[T,S]::key_index_signal>

        *self
            .row_signals
            .get(key)
            .expect(&format!("Row index signal `{key}`"))

        // ω <fn CgcData[T,S]::key_index_signal>
    }

    /// Dump current state
    ///
    ///   * **label** - Prefix label for log statement
    pub fn log_state(&self, label: &str) {
        // α <fn CgcData[T,S]::log_state>

        use leptos::SignalGetUntracked;

        let component_state = format!("{:?}", self.component_state);

        let signals = self
            .row_signals
            .iter()
            .map(|(key, signal)| format!("\t`{key}` -> {}", signal.get_untracked()))
            .collect::<Vec<_>>()
            .join("\n");

        let rows = self
            .rows_updatable
            .value
            .iter()
            .enumerate()
            .map(|(i, row)| format!("\t{i} -> {}", row.get_key()))
            .collect::<Vec<_>>()
            .join("\n");

        tracing::debug!(
            "{label}: {}",
            [component_state, signals, rows].to_vec().join("\n")
        );

        // ω <fn CgcData[T,S]::log_state>
    }
}

/// Unit tests for `collection_grid_component`
#[cfg(test)]
pub mod unit_tests {

    /// Test type CgcData<T,S>
    mod test_cgc_data_ts {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn new() {
            // α <fn test CgcData[T,S]::new>

            with_runtime(|| {
                let cgc_data = make_cgc_data();
                assert_eq!(Holding::default(), cgc_data.row_stored_value.get_value());
                assert_eq!((), cgc_data.shared_context_stored_value.get_value());
                assert_eq!(CollectionGridState::Display, cgc_data.component_state);
            });

            // ω <fn test CgcData[T,S]::new>
        }

        #[test]
        fn edit_item() {
            // α <fn test CgcData[T,S]::edit_item>

            with_runtime(|| {
                let mut cgc_data = make_cgc_data();
                let state = cgc_data.edit_item("SPY");
                assert!(matches!(
                    &state,
                    CollectionGridState::EditSelection {
                        selection_key
                    } if selection_key == "SPY"
                ));
                assert_eq!(spy_holding(), cgc_data.row_stored_value.get_value());
            });

            // ω <fn test CgcData[T,S]::edit_item>
        }

        #[test]
        fn edit_new_item() {
            // α <fn test CgcData[T,S]::edit_new_item>
            with_runtime(|| {
                let mut cgc_data = make_cgc_data();
                cgc_data.edit_new_item();
                assert!(matches!(
                    &cgc_data.component_state,
                    CollectionGridState::EditNew
                ));
                assert_eq!(
                    <Holding as CollectionGrid>::new(),
                    cgc_data.row_stored_value.get_value()
                );
            });
            // ω <fn test CgcData[T,S]::edit_new_item>
        }

        #[test]
        fn is_active_key() {
            // α <fn test CgcData[T,S]::is_active_key>
            with_runtime(|| {
                let mut cgc_data = make_cgc_data();
                assert!(!cgc_data.is_active_key("SPY"));
                let _state = cgc_data.edit_item("SPY");
                assert!(cgc_data.is_active_key("SPY"));
                cgc_data.edit_complete(OkCancel::Ok);
                assert!(!cgc_data.is_active_key("SPY"));
            });
            // ω <fn test CgcData[T,S]::is_active_key>
        }

        #[test]
        fn active_signal() {
            // α <fn test CgcData[T,S]::active_signal>
            with_runtime(|| {
                let mut cgc_data = make_cgc_data();
                assert!(cgc_data.active_signal().is_none());
                let _state = cgc_data.edit_item("SPY");
                assert_eq!(
                    0,
                    cgc_data
                        .active_signal()
                        .expect("SPY@0 active")
                        .get_untracked()
                );
                cgc_data.edit_complete(OkCancel::Cancel);
                let _state = cgc_data.edit_item("QQQ");
                assert_eq!(
                    1,
                    cgc_data
                        .active_signal()
                        .expect("QQQ@0 active")
                        .get_untracked()
                );
                cgc_data.edit_complete(OkCancel::Cancel);
                assert!(cgc_data.active_signal().is_none());
            });

            // ω <fn test CgcData[T,S]::active_signal>
        }

        #[test]
        fn delete_item() {
            // α <fn test CgcData[T,S]::delete_item>

            with_runtime(|| {
                let mut cgc_data = make_cgc_data();
                assert!(cgc_data.active_signal().is_none());
                assert_eq!(3, cgc_data.rows_updatable.value.len());
                assert_eq!(3, cgc_data.row_signals.len());
                assert_eq!(2, cgc_data.key_index_signal("DIA").get_untracked());
                cgc_data.delete_item("QQQ");
                assert_eq!(2, cgc_data.rows_updatable.value.len());
                assert_eq!(2, cgc_data.row_signals.len());
                // Was SPY-0, QQQ-1, DIA-2. After delete of QQQ it is SPY-0, DIA-1
                assert_eq!(1, cgc_data.key_index_signal("DIA").get_untracked());
                assert_eq!(None, cgc_data.active_signal());
            });

            // ω <fn test CgcData[T,S]::delete_item>
        }

        #[test]
        fn edit_complete() {
            // α <fn test CgcData[T,S]::edit_complete>

            with_runtime(|| {
                let mut cgc_data = make_cgc_data();
                cgc_data.edit_item("SPY");
                cgc_data
                    .row_stored_value
                    .update_value(|holding| holding.quantity = 99.0);
                // Not yet completed
                assert_eq!(100.0, cgc_data.rows_updatable.value[0].quantity);
                cgc_data.edit_complete(OkCancel::Ok);
                assert_eq!(99.0, cgc_data.rows_updatable.value[0].quantity);
                cgc_data.edit_item("QQQ");
                cgc_data
                    .row_stored_value
                    .update_value(|holding| holding.quantity = 99.0);
                assert_eq!(50.0, cgc_data.rows_updatable.value[1].quantity);
                cgc_data.edit_complete(OkCancel::Cancel);
                // Since edit was canceled - no change in quantity
                assert_eq!(50.0, cgc_data.rows_updatable.value[1].quantity);
            });

            // ω <fn test CgcData[T,S]::edit_complete>
        }

        #[test]
        fn nth_key() {
            // α <fn test CgcData[T,S]::nth_key>

            with_runtime(|| {
                let mut cgc_data = make_cgc_data();
                assert_eq!("SPY", cgc_data.nth_key(0));
                assert_eq!("QQQ", cgc_data.nth_key(1));
                assert_eq!("DIA", cgc_data.nth_key(2));
                cgc_data.delete_item("QQQ");
                assert_eq!("SPY", cgc_data.nth_key(0));
                assert_eq!("DIA", cgc_data.nth_key(1));
            });

            // ω <fn test CgcData[T,S]::nth_key>
        }

        #[test]
        fn key_index_signal() {
            // α <fn test CgcData[T,S]::key_index_signal>

            with_runtime(|| {
                let mut cgc_data = make_cgc_data();
                assert_eq!(0, cgc_data.key_index_signal("SPY").get_untracked());
                assert_eq!(1, cgc_data.key_index_signal("QQQ").get_untracked());
                assert_eq!(2, cgc_data.key_index_signal("DIA").get_untracked());
                // TODO: Would be nice to show that this panics - but compile issues prevent it
                // let should_panic = || cgc_data.key_index_signal("NVDA");
                // assert!(std::panic::catch_unwind(should_panic).is_err());
                cgc_data.edit_new_item();
                cgc_data
                    .row_stored_value
                    .update_value(|row| row.instrument_name = "NVDA".into());
                cgc_data.edit_complete(OkCancel::Ok);
                assert_eq!(3, cgc_data.key_index_signal("NVDA").get_untracked());
            });

            // ω <fn test CgcData[T,S]::key_index_signal>
        }

        // α <mod-def test_cgc_data_ts>
        use super::*;
        use leptos::SignalGetUntracked;
        use plus_modeled::Holding;

        fn spy_holding() -> Holding {
            Holding {
                instrument_name: "SPY".into(),
                quantity: 100.0,
                ..Default::default()
            }
        }

        fn qqq_holding() -> Holding {
            Holding {
                instrument_name: "QQQ".into(),
                quantity: 50.0,
                ..Default::default()
            }
        }

        fn dia_holding() -> Holding {
            Holding {
                instrument_name: "DIA".into(),
                quantity: 75.0,
                ..Default::default()
            }
        }

        fn test_holdings() -> Vec<Holding> {
            vec![spy_holding(), qqq_holding(), dia_holding()]
        }

        fn make_cgc_data() -> CgcData<Holding, ()> {
            CgcData::new(
                Updatable::new(test_holdings(), |_| {}),
                Updatable::new((), |_| {}),
            )
        }

        fn with_runtime<F: FnOnce()>(f: F) {
            let runtime = leptos::create_runtime();
            f();
            runtime.dispose();
        }
        // ω <mod-def test_cgc_data_ts>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def collection_grid_component>
// ω <mod-def collection_grid_component>
