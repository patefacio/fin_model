//! Module for sample_widget_grid leptos function/component

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
/// Supported colors
#[derive(Debug, Copy, Clone)]
pub enum Color {
    /// Red
    Red,
    /// Green
    Green,
    /// Blue
    Blue,
}

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Shared context for [SampleWidgetGrid]
#[derive(Debug, Clone)]
pub struct SwgSharedContext {}

/// Sample widget
#[derive(Debug, Clone)]
pub struct SampleWidget {
    /// Name of widget
    pub name: String,
    /// Color
    pub color: Color,
    /// Years in use
    pub years: Option<u32>,
}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Display and edit support for list of widgets.
///
///   * **updatable** - Widgets to edit
///   * **shared_context_updatable** - The shared context
///   * _return_ - View for sample_widget_grid
#[component]
pub fn SampleWidgetGrid(
    /// Widgets to edit
    updatable: Updatable<Vec<SampleWidget>>,
    /// The shared context
    shared_context_updatable: Updatable<SwgSharedContext>,
) -> impl IntoView {
    let component_id = crate::component_id!("`SampleWidgetGrid`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn sample_widget_grid>

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

    // ω <fn sample_widget_grid>
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl CollectionGrid for SampleWidget {
    type SharedContext = SwgSharedContext;
    /// Get number of fields provided by `get_fields` to be displayed.
    /// Used to build the `grid-template-columns` style. Two additional fields
    /// are added (_edit button_, _delete button_).
    ///
    ///   * _return_ - Number of fields
    fn get_fields_len() -> usize {
        // α <fn CollectionGrid::get_fields_len for SampleWidget>
        3
        // ω <fn CollectionGrid::get_fields_len for SampleWidget>
    }

    /// Get the display fields for the element.
    ///
    ///   * _return_ - The fields as elements
    fn get_fields(&self) -> Vec<View> {
        // α <fn CollectionGrid::get_fields for SampleWidget>

        use crate::ClientCssClasses;
        use leptos::IntoAttribute;

        let color = format!("{:?}", self.color);
        let years = format!("{}", self.years.unwrap_or_default());

        vec![
            view! { <div class=ClientCssClasses::CgcCell.as_str()>{self.name.clone()}</div> }
                .into_view(),
            view! { <div class=ClientCssClasses::CgcCell.as_str()>{color}</div> }.into_view(),
            view! { <div class=ClientCssClasses::CgcCell.as_str()>{years}</div> }.into_view(),
        ]

        // ω <fn CollectionGrid::get_fields for SampleWidget>
    }

    /// Get the header for the rows.
    ///
    ///   * _return_ - The header
    fn get_header() -> Vec<String> {
        // α <fn CollectionGrid::get_header for SampleWidget>
        vec!["Name".to_string(), "Color".to_string(), "Years".to_string()]
        // ω <fn CollectionGrid::get_header for SampleWidget>
    }

    /// Get the text for `Add New Item`.
    ///
    ///   * _return_ - The add item label
    fn get_add_item_label() -> String {
        // α <fn CollectionGrid::get_add_item_label for SampleWidget>
        "Add New Widget".into()
        // ω <fn CollectionGrid::get_add_item_label for SampleWidget>
    }

    /// Get key that uniquely identifies the element.
    ///
    ///   * _return_ - The key for the element
    fn get_key(&self) -> String {
        // α <fn CollectionGrid::get_key for SampleWidget>
        self.name.clone()
        // ω <fn CollectionGrid::get_key for SampleWidget>
    }

    /// Create new element to edit
    ///
    ///   * _return_ - New element
    fn new() -> Self {
        // α <fn CollectionGrid::new for SampleWidget>
        SampleWidget {
            name: "Some Sample".into(),
            color: Color::Blue,
            years: Some(42),
        }
        // ω <fn CollectionGrid::new for SampleWidget>
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
        // α <fn CollectionGrid::edit_row for SampleWidget>
        view! {
            <div>

                {format!("Edit Sample Widget {edit_type:?} {:?}", row_stored_value.get_value())}

            </div>
        }
        .into_view()
        // ω <fn CollectionGrid::edit_row for SampleWidget>
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
        // α <fn CollectionGrid::accept_row_edit for SampleWidget>

        // TODO: Use the edit_type
        tracing::warn!("TODO: Use {edited_row:?}");
        // TODO: Use the edit_type
        tracing::warn!("TODO: Use {shared_context:?}");

        todo!("Implement `accept_row_edit`")

        // ω <fn CollectionGrid::accept_row_edit for SampleWidget>
    }
}

// α <mod-def sample_widget_grid>
// ω <mod-def sample_widget_grid>
