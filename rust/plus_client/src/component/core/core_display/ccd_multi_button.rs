//! Module for ccd_multi_button leptos function/component

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
/// Show the [MultiButtonSelect]
///
///   * **show_update** - Function to display state updates
///   * _return_ - View for ccd_multi_button
#[component]
pub fn CcdMultiButton(
    /// Function to display state updates
    show_update: WriteSignal<String>,
) -> impl IntoView {
    crate::log_component!("`CcdMultiButton`");
    // α <fn ccd_multi_button>

    use super::prefix_lang_flag;
    use crate::AppContext;
    use crate::ButtonSelection;
    use crate::CurrencySelect;
    use crate::MultiButtonSelect;
    use crate::ToggleState;
    use crate::Updatable;
    use crate::ViewSide;
    use leptos::use_context;
    use leptos::SignalGet;
    use leptos::SignalSet;
    use plus_modeled::Currency;

    let lang_selector = use_context::<AppContext>().unwrap().lang_selector;

    let prefix_lang = move |s| prefix_lang_flag(lang_selector.get(), s);

    let multi_button_example = move |side: ViewSide| {

        let button_selections = vec![
            ButtonSelection::new(
                "persons_button.png".into(),
                prefix_lang("People"),
                ToggleState::Selected,
            ),
            ButtonSelection::new(
                "worths_button.png".into(),
                prefix_lang("Valuables"),
                ToggleState::Deselected,
            ),
            ButtonSelection::new(
                "accounts_button.png".into(),
                prefix_lang("Accounts"),
                ToggleState::Selected,
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
            2 => {
                tracing::warn!("Recreating accounts view!!");
                view! {
                    <div>
                        <h1>{prefix_lang("Accounts")}</h1>
                    </div>
                }
                .into_view()
            }
            _ => panic!("Invalid view index"),
        };

        view! {
            <div class="title">{move || format!("Multi-Button Select ({side:?})")}</div>
            <MultiButtonSelect
                button_selections
                content_maker
                button_bar_side=side
            />
        }
        .into_view()
    };

    // ω <fn ccd_multi_button>
    view! {
        <div class="ccd-section">{move || multi_button_example(ViewSide::Top)}</div>
        <div class="ccd-section">{move || multi_button_example(ViewSide::Left)}</div>
        <div class="ccd-section">{move || multi_button_example(ViewSide::Bottom)}</div>
        <div class="ccd-section">{move || multi_button_example(ViewSide::Right)}</div>
    }
}

// α <mod-def ccd_multi_button>
// ω <mod-def ccd_multi_button>
