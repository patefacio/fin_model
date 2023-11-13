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
    use crate::MultiButtonData;
    use crate::MultiButtonSelect;
    use crate::ToggleState;
    use crate::ViewSide;
    use leptos::use_context;
    use leptos::SignalGet;

    let lang_selector = use_context::<AppContext>().unwrap().lang_selector;

    let prefix_lang = move |s| prefix_lang_flag(lang_selector.get(), s);

    let multi_button_example = move |side: ViewSide| {
        let people_lbl = prefix_lang("People");
        let valuables_lbl = prefix_lang("Valuables");
        let accounts_lbl = prefix_lang("Accounts");
        let button_data = vec![
            MultiButtonData::new(
                ButtonSelection::new(
                    "persons_button.png".into(),
                    people_lbl.clone(),
                    ToggleState::Selected,
                ),
                view! {
                {    crate::log_component!("`!!!!!!!!!! PERSON BUTTON !!!!!!!!`"); }

                                        <div>
                                        <h1>{people_lbl}</h1>
                                        </div>
                                }
                .into_view(),
            ),
            MultiButtonData::new(
                ButtonSelection::new(
                    "worths_button.png".into(),
                    valuables_lbl.clone(),
                    ToggleState::Deselected,
                ),
                view! {
                        <div>
                        <h1>{valuables_lbl}</h1>
                        </div>
                }
                .into_view(),
            ),
            MultiButtonData::new(
                ButtonSelection::new(
                    "accounts_button.png".into(),
                    accounts_lbl.clone(),
                    ToggleState::Selected,
                ),
                view! {
                    <div>
                    <h1>{accounts_lbl}</h1>
                    </div>
                }
                .into_view(),
            ),
        ];

        tracing::warn!("Instantiating MultiButtonSelect with {button_data:?}");

        view! {
            <div class="title">{move || format!("Multi-Button Select ({side:?})")}</div>
            <MultiButtonSelect button_data=button_data button_bar_side=side/>
        }
        .into_view()
    };

    // ω <fn ccd_multi_button>
    view! {
          // α <plus-cmb-view>
          <div class="ccd-section">{move || multi_button_example(ViewSide::Top)}</div>
          <div class="ccd-section">{move || multi_button_example(ViewSide::Left)}</div>
          <div class="ccd-section">{move || multi_button_example(ViewSide::Bottom)}</div>
          <div class="ccd-section">{move || multi_button_example(ViewSide::Right)}</div>
          // ω <plus-cmb-view>
    }
}

// α <mod-def ccd_multi_button>
// ω <mod-def ccd_multi_button>
