//! Module for multi_button_select leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::ButtonSelection;
use crate::ToggleImageButton;
use crate::ViewSide;
use leptos::IntoAttribute;
use leptos::View;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Data for a single button including its state which is managed by the
/// [MultiButtonSelect](crate::MultiButtonSelect) and its corresponding view
#[derive(Debug, Clone, Default)]
pub struct MultiButtonData {
    /// The image and state of the button
    pub button_selection: ButtonSelection,
    /// The view to display when the button is _selected_.
    pub view: View,
}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Provides selection of view from a list of toggle-buttons to present
/// from a list of views.
///
///   * **button_data** - The buttons data to display in the managed toolbar
///   * **button_bar_side** - Side of view the buttons appear.
/// Top and bottom orient buttons horizontally.
/// Left and right orient buttons vertically.
///   * _return_ - View for multi_button_select
#[component]
pub fn MultiButtonSelect(
    /// The buttons data to display in the managed toolbar
    button_data: Vec<MultiButtonData>,
    /// Side of view the buttons appear.
    /// Top and bottom orient buttons horizontally.
    /// Left and right orient buttons vertically.
    button_bar_side: ViewSide,
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-mbs";
    crate::log_component!("`MultiButtonSelect`");
    // α <fn multi_button_select>

    use crate::CssClasses;
    use crate::ToggleState;
    use crate::Updatable;
    use leptos::create_signal;
    use leptos::store_value;
    use leptos::Show;
    use leptos::SignalSet;
    use leptos::SignalWith;

    tracing::warn!("Creating mbs on {button_data:?}");

    let (mbs_grid, toolbar_span, view_span, toolbar_class) = match button_bar_side {
        ViewSide::Top => (
            "display: grid; grid-template-rows: 1fr auto; grid-template-columns: 1fr auto;",
            "grid-row: 1; grid-column: 1 / span 2;",
            "grid-row: 2; grid-column: 1 / span 2",
            CssClasses::BtnTbTop.to_string(),
        ),
        ViewSide::Right => (
            "display: grid; grid-template-rows: 1fr auto; grid-template-columns: 1fr auto;",
            "grid-column: 2; grid-row: 1 / span 2;",
            "grid-column: 1; grid-row: 1 / span 2;",
            CssClasses::BtnTbRight.to_string(),
        ),
        ViewSide::Bottom => (
            "display: grid; grid-template-rows: 1fr auto; grid-template-columns: 1fr auto;",
            "grid-row: 2; grid-column: 1 / span 2;",
            "grid-row: 1; grid-column: 1 / span 2",
            CssClasses::BtnTbBottom.to_string(),
        ),
        ViewSide::Left => (
            "display: grid; grid-template-rows: 1fr auto; grid-template-columns: auto 1fr;",
            "grid-column: 1; grid-row: 1 / span 2;",
            "grid-column: 2; grid-row: 1 / span 2;",
            CssClasses::BtnTbLeft.to_string(),
        ),
    };

    let button_data_stored_value = store_value(
        button_data
            .iter()
            .map(|button_data| button_data.button_selection.toggle_state)
            .collect::<Vec<_>>(),
    );
    let (state_changed_read, state_changed_write) = create_signal(());

    let button_view_is_shown = move |i: usize| {
        state_changed_read.track();
        button_data_stored_value.with_value(|toggle_state| toggle_state[i] == ToggleState::Selected)
    };

    let toggle_view = move |i: usize, new_state: ToggleState| {
        button_data_stored_value.update_value(|toggle_states| toggle_states[i] = new_state)
    };

    // Before moving the button data into leptos store, swap out the views provided with
    // an empty view to provide ownership to the container view below.
    let mut button_data = button_data;
    let (button_views, content_views): (Vec<_>, Vec<_>) = button_data
        .into_iter()
        .enumerate()
        .map(|(i, mut button_data)| {
            use std::mem::swap;
            let mut displayed_view = ().into_view();
            swap(&mut displayed_view, &mut button_data.view);
            let mut button_selection = ButtonSelection::default();
            swap(&mut button_selection, &mut button_data.button_selection);

            tracing::warn!("Creating content for {button_selection:?} and {displayed_view:?}");
            let content_view = view! {
                <Show when=move || button_view_is_shown(i) fallback=|| ()>
                    {displayed_view.clone()}
                </Show>
            }
            .into_view();

            let button_view = view! {
                <ToggleImageButton updatable=Updatable::new(
                    button_selection,
                    move |button_selection| {
                        tracing::warn!("Button {button_data:?} toggled to {button_selection:?}");
                        toggle_view(i, button_selection.toggle_state);
                        state_changed_write.set(());
                    },
                )/>
            }
            .into_view();

            (button_view, content_view)
        })
        .unzip();

    // ω <fn multi_button_select>
    view! {
        <div class=SELF_CLASS>
            // α <plus-mbs-view>

            <div style=mbs_grid>
                <div class=toolbar_class style=toolbar_span>
                    {button_views}
                </div>
                <div class=CssClasses::MbsView.to_string() style=view_span>
                    {content_views}
                </div>
            </div>

        // ω <plus-mbs-view>
        </div>
    }
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl MultiButtonData {
    /// Create new instance of MultiButtonData
    ///
    ///   * **button_selection** - The image and state of the button
    ///   * **view** - The view to display when the button is _selected_.
    ///   * _return_ - The new instance
    pub fn new(button_selection: ButtonSelection, view: View) -> MultiButtonData {
        MultiButtonData {
            button_selection,
            view,
        }
    }
}

// α <mod-def multi_button_select>
// ω <mod-def multi_button_select>
