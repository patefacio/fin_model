//! Module for ccd_multi_button leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::component;
use leptos::view;
use leptos::IntoView;
use leptos::WriteSignal;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Show the [MultiButtonSelect]
///
///   * **show_update** - Function to display state updates
///   * _return_ - View for ccd_multi_button
#[component]
pub fn CcdMultiButton(
    /// Function to display state updates
    show_update: WriteSignal<String>,
) -> impl IntoView {
    let component_id = crate::component_id!("`CcdMultiButton`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn ccd_multi_button>

    use super::prefix_lang_flag;
    use crate::AppContext;
    use crate::ButtonData;
    use crate::ButtonImage;
    use crate::CurrencySelect;
    use crate::MbsGroupingConstraint;
    use crate::MultiButtonSelect;
    use crate::ToggleState;
    use crate::Updatable;
    use crate::ViewSide;
    use leptos::expect_context;
    use leptos::MaybeSignal;
    use leptos::Signal;
    use leptos::SignalGet;
    use leptos::SignalSet;
    use plus_modeled::Currency;
    use std::rc::Rc;

    let lang_selector = expect_context::<Rc<AppContext>>().lang_selector;
    let prefix_lang = move |s| prefix_lang_flag(lang_selector.get(), s);
    let multi_button_example = move |side: ViewSide, grouping_constraint: MbsGroupingConstraint| {
        let button_selections = vec![
            (
                ButtonData::new(
                    ButtonImage::from_image_ref("Family.png"),
                    MaybeSignal::Dynamic(Signal::derive(move || prefix_lang("People"))),
                ),
                ToggleState::Selected,
            ),
            (
                ButtonData::new(
                    ButtonImage::from_image_ref("Real-assets.png"),
                    MaybeSignal::Dynamic(Signal::derive(move || prefix_lang("Valuables"))),
                ),
                ToggleState::Deselected,
            ),
            (
                ButtonData::new(
                    ButtonImage::from_image_ref("Financial-accounts.png"),
                    MaybeSignal::Dynamic(Signal::derive(move || prefix_lang("Accounts"))),
                ),
                ToggleState::Deselected,
            ),
        ];

        let content_maker = move |i| match i {
            0 => view! {
                <div>
                    <h1>{prefix_lang("People")}</h1>
                </div>
            }
            .into_view(),
            1 => view! {
                <div class="cmb-valuables">
                    <label>
                        {prefix_lang("Valuables")}
                        <CurrencySelect updatable=Updatable::new(
                            Currency::Usd,
                            move |new_currency| {
                                show_update
                                    .set(
                                        format!("Updated `Valuables` currency to {new_currency:?}"),
                                    )
                            },
                        )/>
                    </label>
                </div>
            }
            .into_view(),
            2 => view! {
                <div>
                    <h1>{prefix_lang("Accounts")}</h1>
                </div>
            }
            .into_view(),
            _ => panic!("Invalid view index"),
        };

        view! {
            <div class="title">
                {format!("Multi-Button Select ({side:?}) ({grouping_constraint:?})")}
            </div>
            <MultiButtonSelect
                button_selections
                content_maker
                button_bar_side=side
                grouping_constraint
            />
        }
        .into_view()
    };

    view! {
        <div class="ccd-section">
            {move || multi_button_example(ViewSide::Top, MbsGroupingConstraint::NoConstraint)}
        </div>
        <div class="ccd-section">
            {move || multi_button_example(ViewSide::Left, MbsGroupingConstraint::NoConstraint)}
        </div>
        <div class="ccd-section">
            {move || multi_button_example(ViewSide::Bottom, MbsGroupingConstraint::OneOrNone)}
        </div>
        <div class="ccd-section">
            {move || multi_button_example(ViewSide::Right, MbsGroupingConstraint::ExactlyOne)}
        </div>
    }

    // ω <fn ccd_multi_button>
}

// α <mod-def ccd_multi_button>
// ω <mod-def ccd_multi_button>
