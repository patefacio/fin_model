//! Module for ccd_collection_grid leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::component;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::WriteSignal;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Show the [CollectionGridComponent]
///
///   * **show_update** - Function to display state updates
///   * _return_ - View for ccd_collection_grid
#[component]
pub fn CcdCollectionGrid(
    /// Function to display state updates
    show_update: WriteSignal<String>,
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-ccg; ccd-section-2col";
    crate::log_component!("`CcdCollectionGrid`");
    // α <fn ccd_collection_grid>

    use super::sample_widget_grid::Color;
    use super::sample_widget_grid::SampleWidget;
    use super::sample_widget_grid::SwgSharedContext;
    use crate::SampleWidgetGrid;
    use leptos::SignalSet;

    use crate::Updatable;
    let updatable = Updatable::new(
        vec![
            SampleWidget {
                name: "Widget 1".into(),
                color: Color::Red,
                years: Some(10),
            },
            SampleWidget {
                name: "Widget 2".into(),
                color: Color::Green,
                years: Some(10),
            },
            SampleWidget {
                name: "Widget 3".into(),
                color: Color::Blue,
                years: Some(10),
            },
        ],
        move |widgets| show_update.set(format!("{widgets:?}")),
    );

    let shared_context_updatable = Updatable::new(SwgSharedContext {}, |_| {});

    // ω <fn ccd_collection_grid>
    view! {
        <div class=SELF_CLASS>
            // α <plus-ccg-view>

            <SampleWidgetGrid updatable shared_context_updatable/>

        // ω <plus-ccg-view>
        </div>
    }
}

// α <mod-def ccd_collection_grid>
// ω <mod-def ccd_collection_grid>
