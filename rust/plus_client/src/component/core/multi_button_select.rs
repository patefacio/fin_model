//! Module for multi_button_select leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::ButtonData;
use crate::ToggleImageButton;
use crate::ToggleState;
use crate::ViewSide;
use leptos::component;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::View;

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// Enumerates supported constraints of which may be grouped and displayed simultaneously
#[derive(Debug, Clone)]
pub enum MbsGroupingConstraint {
    /// All can be shown at once
    NoConstraint,
    /// Only show one at a time
    ExactlyOne,
    /// Show either one or none
    OneOrNone,
}

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Manages a vector of [ToggleState]'s subject to an [MbsGroupingConstraint]
#[derive(Debug, Clone)]
pub struct ConstrainedToggleStates {
    /// The set of toggle states
    pub toggle_states: Vec<ToggleState>,
    /// The constraints on the [ToggleState] entries
    pub constraint: MbsGroupingConstraint,
}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Provides selection of view from a list of toggle-buttons to present
/// from a list of views.
///
///   * **button_selections** - ButtonData to get image/label and initial display state for each button.
/// Determines the number of buttons and views.
///   * **content_maker** - Creates the content view for entry `i`
///   * **button_bar_side** - Side of view the buttons appear.
/// Top and bottom orient buttons horizontally.
/// Left and right orient buttons vertically.
///   * **grouping_constraint** - Constrain the number shown simultaneously
///   * _return_ - View for multi_button_select
#[component]
pub fn MultiButtonSelect<CM>(
    /// ButtonData to get image/label and initial display state for each button.
    /// Determines the number of buttons and views.
    button_selections: Vec<(ButtonData, ToggleState)>,
    /// Creates the content view for entry `i`
    content_maker: CM,
    /// Side of view the buttons appear.
    /// Top and bottom orient buttons horizontally.
    /// Left and right orient buttons vertically.
    button_bar_side: ViewSide,
    /// Constrain the number shown simultaneously
    #[prop(default=MbsGroupingConstraint::NoConstraint)]
    grouping_constraint: MbsGroupingConstraint,
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
    use leptos::create_rw_signal;
    use leptos::store_value;
    use leptos::Signal;
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
    let rw_signal = create_rw_signal(());

    let constrained_toggle_states_store_value = store_value(ConstrainedToggleStates::new(
        button_selections
            .iter()
            .map(|button_selection| button_selection.1)
            .collect(),
        grouping_constraint,
    ));

    let button_views: Vec<View> = button_selections
        .into_iter()
        .enumerate()
        .map(|(i, mut button_selection)| {
            use std::mem::take;
            let button_data = take(&mut button_selection.0);

            view! {
                <ToggleImageButton
                    button_data
                    writer=move |new_toggle_state| {
                        constrained_toggle_states_store_value
                            .update_value(|constrained_toggle_states| {
                                constrained_toggle_states.update(i, new_toggle_state)
                            })
                    }

                    reader=move || {
                        constrained_toggle_states_store_value
                            .with_value(|constrained_toggle_states| {
                                constrained_toggle_states.toggle_states[i]
                            })
                    }

                    rw_signal
                />
            }
            .into_view()
        })
        .collect::<Vec<_>>();

    // derived signal indicating if the button's view is shown
    let button_view_is_shown = move |i: usize| {
        rw_signal.track();
        constrained_toggle_states_store_value.with_value(|constrained_toggle_states| {
            constrained_toggle_states.toggle_states[i] == ToggleState::Selected
        })
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

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl ConstrainedToggleStates {
    /// Update the state of `i` to `new_state` adhering to constraint
    ///
    ///   * **i** - Index of vew to display/hide
    ///   * **new_state** - New state for view `i`.
    pub fn update(&mut self, i: usize, new_state: ToggleState) {
        // α <fn ConstrainedToggleStates::update>
        match new_state {
            ToggleState::Selected => match self.constraint {
                MbsGroupingConstraint::ExactlyOne | MbsGroupingConstraint::OneOrNone => {
                    for (j, toggle_state) in self.toggle_states.iter_mut().enumerate() {
                        // Deselect all but the one selected
                        if j != i {
                            *toggle_state = ToggleState::Deselected;
                        } else {
                            *toggle_state = ToggleState::Selected;
                        }
                        tracing::warn!(
                            "Selecting {:?}: entry({j}) set to {toggle_state:?}",
                            self.constraint
                        );
                    }
                }
                MbsGroupingConstraint::NoConstraint => {
                    self.toggle_states[i] = new_state;
                }
            },
            ToggleState::Deselected => match self.constraint {
                MbsGroupingConstraint::ExactlyOne => {
                    for (j, toggle_state) in self.toggle_states.iter_mut().enumerate() {
                        *toggle_state = ToggleState::Deselected;
                        tracing::warn!(
                            "Deselecting {:?}: entry({j}) set to {toggle_state:?}",
                            self.constraint
                        );
                    }
                    self.toggle_states[0] = ToggleState::Selected;
                }
                MbsGroupingConstraint::NoConstraint | MbsGroupingConstraint::OneOrNone => {
                    self.toggle_states[i] = new_state;
                }
            },
        }

        // ω <fn ConstrainedToggleStates::update>
    }

    /// Initializer
    ///
    ///   * **toggle_states** - The set of toggle states
    ///   * **constraint** - The constraints on the [ToggleState] entries
    ///   * _return_ - The constructed instance
    pub fn new(toggle_states: Vec<ToggleState>, constraint: MbsGroupingConstraint) -> Self {
        // α <new initialization>
        // ω <new initialization>
        Self {
            toggle_states,
            constraint,
        }
    }
}

/// Unit tests for `multi_button_select`
#[cfg(test)]
pub mod unit_tests {

    /// Test type ConstrainedToggleStates
    mod test_constrained_toggle_states {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn update() {
            // α <fn test ConstrainedToggleStates::update>
            todo!("Test update")
            // ω <fn test ConstrainedToggleStates::update>
        }

        // α <mod-def test_constrained_toggle_states>
        // ω <mod-def test_constrained_toggle_states>
    }

    // α <mod-def unit_tests>
    // ω <mod-def unit_tests>
}

// α <mod-def multi_button_select>

// ω <mod-def multi_button_select>
