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

    use super::nested_widget_grid::NestedWidget;
    use super::nested_widget_grid::NestedWidgetGrid;
    use super::nested_widget_grid::NwgSharedContext;
    use super::nested_widget_grid::RustItem;
    use super::sample_widget_grid::Color;
    use super::sample_widget_grid::SampleWidget;
    use super::sample_widget_grid::SwgSharedContext;
    use crate::SampleWidgetGrid;
    use crate::Updatable;
    use leptos::SignalSet;

    let colors = vec![Color::Red, Color::Green, Color::Blue];
    let widgets_1 = (0usize..15usize)
        .map(|i| SampleWidget {
            name: format!("Widget {i}"),
            color: colors[i % 3usize],
            years: Some(i as u32),
        })
        .collect::<Vec<_>>();

    let rust_items = vec![
        RustItem::Function,
        RustItem::Struct,
        RustItem::Enumeration,
        RustItem::Closure,
    ];

    let nested_widgets = (0usize..10usize)
        .map(|i| NestedWidget {
            name: format!("Nested Widget {i}"),
            address: format!("{i} Widget Way"),
            favorite_movie: format!("Oscar Winner {i}"),
            favorite_item: rust_items[i % 4usize],
        })
        .collect::<Vec<_>>();

    // ω <fn ccd_collection_grid>
    view! {
        <div class=SELF_CLASS>
            // α <plus-ccg-view>

            <div class="ccd-section">

                <div style="display: flex">
                    <SampleWidgetGrid
                        updatable=Updatable::new(
                            widgets_1,
                            move |widgets| show_update.set(format!("{widgets:?}")),
                        )

                        shared_context_updatable=Updatable::new(SwgSharedContext {}, |_| {})
                    />

                    <NestedWidgetGrid
                        updatable=Updatable::new(
                            nested_widgets,
                            move |widgets| show_update.set(format!("{widgets:?}")),
                        )

                        shared_context_updatable=Updatable::new(NwgSharedContext {}, |_| {})
                    />

                </div>
            </div>

        // ω <plus-ccg-view>
        </div>
    }
}

// α <mod-def ccd_collection_grid>
// ω <mod-def ccd_collection_grid>
