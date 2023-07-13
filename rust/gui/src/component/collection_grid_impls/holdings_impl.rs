//! TODO: Document Module(holdings_impl)

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::CollectionGrid;
use leptos::view;
use leptos::IntoView;
use leptos::RwSignal;
use leptos::Scope;
use leptos_dom::View;
use plus_modeled::Holding;

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl CollectionGrid for Holding {
    /// Get the number of columns.
    ///
    ///   * _return_ - Number of columns
    fn get_column_count() -> usize {
        // α <fn CollectionGrid::get_column_count for Holding>
        4
        // ω <fn CollectionGrid::get_column_count for Holding>
    }

    /// The header for the collection.
    ///
    ///   * _return_ - The header as a list of elements
    fn get_header() -> Vec<String> {
        // α <fn CollectionGrid::get_header for Holding>
        [
            "Symbol".to_string(),
            "Quantity".to_string(),
            "Price".to_string(),
            "Cost".to_string(),
        ]
        .into_iter()
        .collect()
        // ω <fn CollectionGrid::get_header for Holding>
    }

    /// Get the display fields for the element.
    ///
    ///   * **cx** - The context for the fields
    ///   * _return_ - The fields as elements
    fn get_fields(&self, cx: Scope) -> Vec<View> {
        // α <fn CollectionGrid::get_fields for Holding>

        use leptos::IntoStyle;

        [
            view! { cx, <div style:text-align="right">{self.instrument_name.clone()}</div> },
            view! { cx, <div style:text-align="right">{self.quantity}</div> },
            view! { cx,
                <div style:text-align="right">
                    {self.unit_valuation.as_ref().map(|uv| uv.value).unwrap_or_default()}
                </div>
            },
            view! { cx, <div style:text-align="right">{self.cost_basis}</div> },
        ]
        .into_iter()
        .map(|item| item.into_view(cx))
        .collect()
        // ω <fn CollectionGrid::get_fields for Holding>
    }

    /// Get key that uniquely identifies the element.
    ///
    ///   * _return_ - The key for the element
    fn get_key(&self) -> String {
        // α <fn CollectionGrid::get_key for Holding>
        self.instrument_name.clone()
        // ω <fn CollectionGrid::get_key for Holding>
    }

    /// Create new element to edit
    ///
    ///   * _return_ - New element
    fn new() -> Self {
        // α <fn CollectionGrid::new for Holding>
        Holding::default()
        // ω <fn CollectionGrid::new for Holding>
    }

    /// Create a view to edit the element
    ///
    ///   * **element** - Read/write signal containing the element to edit.
    /// This component will update the vector whenever the element is signaled
    /// by finding the proper element in the vector and replacing it with the update.
    ///   * _return_ - The edit view
    fn edit_element(element: RwSignal<Box<Self>>) -> View {
        // α <fn CollectionGrid::edit_element for Holding>
        todo!("Implement `edit_element`")
        // ω <fn CollectionGrid::edit_element for Holding>
    }
}

// α <mod-def holdings_impl>
// ω <mod-def holdings_impl>
