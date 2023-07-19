//! TODO: Document Module(holdings_impl)

use crate::DistributionCdf;
////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Table;
use crate::Updatable;
use leptos::view;
use leptos::IntoView;
use leptos::RwSignal;
use leptos::Scope;
use leptos_dom::View;
use plus_modeled::NormalSpec;

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl Table for NormalSpec {
    /// Get the number of columns.
    ///
    ///   * _return_ - Number of columns
    fn get_column_count() -> usize {
        // α <fn CollectionGrid::get_column_count for Holding>
        2
        // ω <fn CollectionGrid::get_column_count for Holding>
    }

    /// The header for the collection.
    ///
    ///   * _return_ - The header as a list of elements
    fn get_header() -> Vec<String> {
        // α <fn CollectionGrid::get_header for Holding>
        [
            "Amount gain or less".to_string(),
            "Probability".to_string(),
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
            view! { cx, <div style:text-align="right">{self.cdf_sigmoid_approx(1.0)}</div> },
            view! { cx, <div style:text-align="right">{self.cdf_sigmoid_approx(1.0)}</div> },
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
        self.mean.to_string()
        // ω <fn CollectionGrid::get_key for Holding>
    }

    /// Create new element to edit
    ///
    ///   * _return_ - New element
    fn new() -> Self {
        // α <fn CollectionGrid::new for Holding>
        NormalSpec::default()
        // ω <fn CollectionGrid::new for Holding>
    }

    /// Create a view to edit the element
    ///
    ///   * **cx** - Context
    ///   * **updatable** - Read/write signal containing the element to edit.
    /// This component will update the vector whenever the element is signaled
    /// by finding the proper element in the vector and replacing it with the update.
    ///   * _return_ - The edit view
    fn edit_element(cx: Scope, updatable: Updatable<Self>) -> View {
        // α <fn CollectionGrid::edit_element for Holding>

        use crate::NormalSpecComponent;
        use crate::Updatable;
        use leptos::log;
        use leptos::SignalWith;
        use plus_modeled::Holding;

        let on_cancel = || log!("Holding edit canceled");

        updatable.value = Some(updatable.value);

        view! { cx, <NormalSpecComponent
            updatable=updatable
        /> }
        .into_view(cx)

        // ω <fn CollectionGrid::edit_element for Holding>
    }
}

// α <mod-def holdings_impl>
// ω <mod-def holdings_impl>
