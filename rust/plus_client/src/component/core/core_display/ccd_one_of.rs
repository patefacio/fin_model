//! Module for ccd_one_of leptos function/component

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
/// Show the [OneOfComponent]
///
///   * **show_update** - Function to display state updates
///   * _return_ - View for ccd_one_of
#[component]
pub fn CcdOneOf(
    /// Function to display state updates
    show_update: WriteSignal<String>,
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-coo; ccd-section-2col";
    let component_id = crate::component_id!("`CcdOneOf`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn ccd_one_of>

    use super::prefix_lang_flag;
    use crate::AppContext;
    use crate::OneOfComponent;
    use crate::SelectDirection;
    use crate::Updatable;
    use leptos::expect_context;
    use leptos::SignalGet;
    use leptos::SignalSet;
    use std::rc::Rc;

    let lang_selector = expect_context::<Rc<AppContext>>().lang_selector;

    #[derive(Debug, Clone, Copy, PartialEq, EnumVariantNames, EnumIter)]
    enum LostInSpace {
        Will,
        Penny,
        Judy,
        Don,
        John,
        Maureen,
        DrSmith,
        Robot,
    }

    let selection = LostInSpace::Judy;

    let content_maker = move |selection: LostInSpace| {
        format!(
            "My favorite is {}",
            prefix_lang_flag(lang_selector.get(), &&format!("{selection:?}"))
        )
        .into_view()
    };

    let label_maker = move |selection: LostInSpace| {
        prefix_lang_flag(lang_selector.get(), &format!("{selection:?}")).into_view()
    };

    // ω <fn ccd_one_of>
    view! {
        <div class=SELF_CLASS>
            // α <plus-coo-view>

            <div class="title">"One Of Select"</div>
            <div class="ccd-one-of-ctnr">
                <OneOfComponent
                    updatable=Updatable::new(
                        selection,
                        move |new_selection| {
                            show_update.set(format!("Selection updated -> {new_selection:?}"))
                        },
                    )

                    name="lis-ltr".to_string()
                    content_maker
                    label_maker
                />
                <OneOfComponent
                    updatable=Updatable::new(
                        selection,
                        move |new_selection| {
                            show_update.set(format!("Selection updated -> {new_selection:?}"))
                        },
                    )

                    name="lis-ttb".to_string()
                    direction=SelectDirection::TopToBottom
                    content_maker
                    label_maker
                />
            </div>

        // ω <plus-coo-view>
        </div>
    }
}

// α <mod-def ccd_one_of>
// ω <mod-def ccd_one_of>
