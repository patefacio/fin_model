//! Module for nested_widget_grid leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::CollectionGrid;
use crate::CollectionGridEditType;
use crate::Updatable;
use leptos::component;
use leptos::view;
use leptos::IntoView;
use leptos::StoredValue;
use leptos_dom::View;

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// Supported items
#[derive(Debug, Copy, Clone)]
pub enum RustItem {
    /// A function
    Function,
    /// A struct
    Struct,
    /// An enum
    Enumeration,
    /// A closure
    Closure,
}

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Shared context for [NestedWidgetGrid]
#[derive(Debug, Clone)]
pub struct NwgSharedContext {}

/// Nested widget
#[derive(Debug, Clone)]
pub struct NestedWidget {
    /// Name of widget
    pub name: String,
    /// Address
    pub address: String,
    /// Favorite movie
    pub favorite_movie: String,
    /// Favorite item
    pub favorite_item: RustItem,
}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Display and edit support for list of widgets.
///
///   * **updatable** - Widgets to edit
///   * **shared_context_updatable** - The shared context
///   * _return_ - View for nested_widget_grid
#[component]
pub fn NestedWidgetGrid(
    /// Widgets to edit
    updatable: Updatable<Vec<NestedWidget>>,
    /// The shared context
    shared_context_updatable: Updatable<NwgSharedContext>,
) -> impl IntoView {
    let component_id = crate::component_id!("`NestedWidgetGrid`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn nested_widget_grid>

    use crate::AppContext;
    use crate::ClientCssClasses;
    use crate::CollectionGridComponent;
    use leptos::expect_context;
    use leptos::IntoAttribute;
    use leptos::SignalGet;
    use std::rc::Rc;

    let lang_selector = expect_context::<Rc<AppContext>>().lang_selector;
    let i18n_worths = move || plus_lookup::i18n::worths_grid::i18n_worths(lang_selector.get());

    view! {
        <div>
            <div class=ClientCssClasses::GridLbl.as_str()>{i18n_worths}</div>
            <CollectionGridComponent
                rows_updatable=updatable
                shared_context_updatable=shared_context_updatable
            />
        </div>
    }

    // ω <fn nested_widget_grid>
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl CollectionGrid for NestedWidget {
    type SharedContext = NwgSharedContext;
    /// Get number of fields provided by `get_fields` to be displayed.
    /// Used to build the `grid-template-columns` style. Two additional fields
    /// are added (_edit button_, _delete button_).
    ///
    ///   * _return_ - Number of fields
    fn get_fields_len() -> usize {
        // α <fn CollectionGrid::get_fields_len for NestedWidget>
        4
        // ω <fn CollectionGrid::get_fields_len for NestedWidget>
    }

    /// Get the display fields for the element.
    ///
    ///   * _return_ - The fields as elements
    fn get_fields(&self) -> Vec<View> {
        // α <fn CollectionGrid::get_fields for NestedWidget>
        use crate::ClientCssClasses;
        use leptos::IntoAttribute;

        let favorite_item = format!("{:?}", self.favorite_item);

        vec![
            view! { <div class=ClientCssClasses::CgcCell.as_str()>{self.name.clone()}</div> }.into_view(),
            view! { <div class=ClientCssClasses::CgcCell.as_str()>{self.address.clone()}</div> }
                .into_view(),
            view! { <div class=ClientCssClasses::CgcCell.as_str()>{self.favorite_movie.clone()}</div> }
                .into_view(),
            view! { <div class=ClientCssClasses::CgcCell.as_str()>{favorite_item}</div> }.into_view(),
        ]

        // ω <fn CollectionGrid::get_fields for NestedWidget>
    }

    /// Get the header for the rows.
    ///
    ///   * _return_ - The header
    fn get_header() -> Vec<String> {
        // α <fn CollectionGrid::get_header for NestedWidget>
        vec![
            "Name".to_string(),
            "Address".to_string(),
            "Favorite Movie".to_string(),
            "Favorite Item".to_string(),
        ]
        // ω <fn CollectionGrid::get_header for NestedWidget>
    }

    /// Get the text for `Add New Item`.
    ///
    ///   * _return_ - The add item label
    fn get_add_item_label() -> String {
        // α <fn CollectionGrid::get_add_item_label for NestedWidget>
        "Add New Nested Widget".into()
        // ω <fn CollectionGrid::get_add_item_label for NestedWidget>
    }

    /// Get key that uniquely identifies the element.
    ///
    ///   * _return_ - The key for the element
    fn get_key(&self) -> String {
        // α <fn CollectionGrid::get_key for NestedWidget>
        self.name.clone()
        // ω <fn CollectionGrid::get_key for NestedWidget>
    }

    /// Create new element to edit
    ///
    ///   * _return_ - New element
    fn new() -> Self {
        // α <fn CollectionGrid::new for NestedWidget>
        NestedWidget {
            name: "Some Nested Widget".into(),
            address: "23 Widget Way".into(),
            favorite_movie: "The Fugitive".into(),
            favorite_item: RustItem::Enumeration,
        }
        // ω <fn CollectionGrid::new for NestedWidget>
    }

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
    ) -> View {
        // α <fn CollectionGrid::edit_row for NestedWidget>
        view! {
            <div>

                {format!(
                    "Edit Nested Sample Widget {edit_type:?} {:?}", row_stored_value.get_value()
                )}

            </div>
        }
        .into_view()
        // ω <fn CollectionGrid::edit_row for NestedWidget>
    }

    /// Return true if row edit satisfies any shared context constraints
    ///
    ///   * **edited_row** - The edited row
    ///   * **shared_context** - The current shared context
    ///   * _return_ - An error message if not acceptable change, None otherwise
    fn accept_row_edit(
        edited_row: &Self,
        shared_context: &mut Self::SharedContext,
    ) -> Option<String> {
        // α <fn CollectionGrid::accept_row_edit for NestedWidget>

        // TODO: Use the edit_type
        tracing::warn!("TODO: Use {edited_row:?}");
        // TODO: Use the edit_type
        tracing::warn!("TODO: Use {shared_context:?}");

        todo!("Implement `accept_row_edit`")

        // ω <fn CollectionGrid::accept_row_edit for NestedWidget>
    }
}

// α <mod-def nested_widget_grid>
// ω <mod-def nested_widget_grid>
