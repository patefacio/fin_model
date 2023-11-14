//! Module for multi_button_select leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::ButtonSelection;
use crate::ToggleImageButton;
use crate::ViewSide;
use leptos::component;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::View;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Provides selection of view from a list of toggle-buttons to present
/// from a list of views.
///
///   * **button_selections** - Initial display state for each button. Determines the number of buttons and views.
///   * **content_maker** - Creates the content view for entry `i`
///   * **button_bar_side** - Side of view the buttons appear.
/// Top and bottom orient buttons horizontally.
/// Left and right orient buttons vertically.
///   * _return_ - View for multi_button_select
#[component]
pub fn MultiButtonSelect<CM>(
    /// Initial display state for each button. Determines the number of buttons and views.
    button_selections: Vec<ButtonSelection>,
    /// Creates the content view for entry `i`
    content_maker: CM,
    /// Side of view the buttons appear.
    /// Top and bottom orient buttons horizontally.
    /// Left and right orient buttons vertically.
    button_bar_side: ViewSide,
) -> impl IntoView
where
    CM: Fn(usize) -> View + Clone + 'static,
{
    pub const SELF_CLASS: &str = "plus-mbs";
    crate::log_component!("`MultiButtonSelect`");
    // α <fn multi_button_select>

    use crate::CssClasses;
    use crate::CssShow;
    use crate::ToggleState;
    use crate::Updatable;
    use leptos::create_signal;
    use leptos::store_value;
    use leptos::Signal;
    use leptos::SignalSet;
    use leptos::SignalWith;

    let (mbs_grid_style, toolbar_span_style, view_span, toolbar_class) = match button_bar_side {
        ViewSide::Top => (
            "display: grid; grid-template-rows: auto 1fr; grid-template-columns: 1fr auto;",
            "grid-row: 1; grid-column: 1 / span 2;",
            "grid-row: 2; grid-column: 1 / span 2",
            CssClasses::BtnTbTop.as_str(),
        ),
        ViewSide::Right => (
            "display: grid; grid-template-rows: 1fr auto; grid-template-columns: 1fr auto;",
            "grid-column: 2; grid-row: 1 / span 2;",
            "grid-column: 1; grid-row: 1 / span 2;",
            CssClasses::BtnTbRight.as_str(),
        ),
        ViewSide::Bottom => (
            "display: grid; grid-template-rows: 1fr auto; grid-template-columns: 1fr auto;",
            "grid-row: 2; grid-column: 1 / span 2;",
            "grid-row: 1; grid-column: 1 / span 2",
            CssClasses::BtnTbBottom.as_str(),
        ),
        ViewSide::Left => (
            "display: grid; grid-template-rows: 1fr auto; grid-template-columns: auto 1fr;",
            "grid-column: 1; grid-row: 1 / span 2;",
            "grid-column: 2; grid-row: 1 / span 2;",
            CssClasses::BtnTbLeft.as_str(),
        ),
    };

    let button_count = button_selections.len();
    let (state_changed_read, state_changed_write) = create_signal(());
    let toggle_states_stored_value = store_value(
        button_selections
            .iter()
            .map(|button_selection| button_selection.toggle_state)
            .collect::<Vec<_>>(),
    );

    let button_views = button_selections
        .into_iter()
        .enumerate()
        .map(|(i, button_selection)| {
            view! {
                <ToggleImageButton updatable=Updatable::new(
                    button_selection,
                    move |button_selection| {
                        tracing::warn!(
                            "Button {button_selection:?} toggled to {button_selection:?}"
                        );
                        toggle_states_stored_value
                            .update_value(|toggle_states| {
                                toggle_states[i] = button_selection.toggle_state;
                            });
                        state_changed_write.set(());
                    },
                )/>
            }
            .into_view()
        })
        .collect::<Vec<_>>();

    // derived signal indicating if the button's view is shown
    let button_view_is_shown = move |i: usize| {
        state_changed_read.track();
        toggle_states_stored_value
            .with_value(|toggle_state| toggle_state[i] == ToggleState::Selected)
    };

    let content_maker_stored_value = store_value(content_maker);

    let content_views = (0..button_count)
        .into_iter()
        .map(|i| {
            view! {
                <CssShow
                    when=Signal::derive(move || button_view_is_shown(i))
                    display_type="block".into()
                >
                    {move || {
                        content_maker_stored_value.with_value(|content_maker| content_maker(i))
                    }}

                </CssShow>
            }
        })
        .collect::<Vec<_>>();

    // ω <fn multi_button_select>
    view! {
        <div class=SELF_CLASS>
            // α <plus-mbs-view>

            <div class=CssClasses::MbsInterior.as_str() style=mbs_grid_style>
                <div class=toolbar_class style=toolbar_span_style>
                    {button_views}
                </div>
                <div class=CssClasses::MbsView.as_str() style=view_span>
                    {content_views}
                </div>
            </div>

        // ω <plus-mbs-view>
        </div>
    }
}

// α <mod-def multi_button_select>

// ω <mod-def multi_button_select>
