//! Module for one_of_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::SelectDirection;
use crate::Updatable;
use leptos::component;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::View;
use std::cmp::PartialEq;
use std::fmt::Debug;
use strum::{IntoEnumIterator, VariantNames};

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Provides a component to toggle between values in an enumeration.
///
///   * **updatable** - The current selection
///   * **name** - Used for the name of radio buttons
///   * **label_maker** - Labels and views to show
///   * **content_maker** - Content view maker
///   * **direction** - Specifies whether items flows from top to bottom or left to right.
///   * _return_ - View for one_of_component
#[component]
pub fn OneOfComponent<E, LM, CM>(
    /// The current selection
    updatable: Updatable<E>,
    /// Used for the name of radio buttons
    name: String,
    /// Labels and views to show
    label_maker: LM,
    /// Content view maker
    content_maker: CM,
    /// Specifies whether items flows from top to bottom or left to right.
    #[prop(default=SelectDirection::LeftToRight)]
    direction: SelectDirection,
) -> impl IntoView
where
    E: Clone + Copy + Debug + VariantNames + IntoEnumIterator + PartialEq + 'static,
    LM: Fn(E) -> View + Clone + 'static,
    CM: Fn(E) -> View + Clone + 'static,
{
    pub const SELF_CLASS: &str = "plus-ooc";
    crate::log_component!("`OneOfComponent`");
    // α <fn one_of_component>

    use crate::CssClasses;
    use leptos::create_rw_signal;
    use leptos::SignalUpdate;
    use leptos::SignalWith;
    use std::rc::Rc;

    let column_count = E::VARIANTS.len();
    let name = Rc::new(name);

    let updatable_rw_signal = create_rw_signal(updatable);

    let container_style = match direction {
        SelectDirection::LeftToRight => format!(
            "display: grid; grid-template-columns: {}; margin: 3px;",
            "1fr ".repeat(column_count)
        ),
        SelectDirection::TopToBottom => format!(
            "display: grid; grid-template-rows: {}; margin: 3px;",
            "1fr ".repeat(column_count)
        ),
    };

    let radio_buttons = move || {
        // TODO: lang_selector.track();
        E::iter()
            .enumerate()
            .map(|(i, e)| {
                let label = label_maker(e);
                let html_id = format!("{}-{i}", name.clone());
                let name = name.as_str().to_string();
                // The class and location are dynamically created
                let (class, location) = match direction {
                    SelectDirection::LeftToRight => (
                        CssClasses::OocRbLtr.as_str(),
                        format!("grid-column: {} / {};", i + 1, i + 2),
                    ),
                    SelectDirection::TopToBottom => (
                        CssClasses::OocRbTtb.as_str(),
                        format!("grid-row: {} / {};", i + 1, i + 2),
                    ),
                };

                let click_handler = move |_| {
                    updatable_rw_signal
                        .update(|updatable| updatable.update_and_then_signal(|value| *value = e))
                };

                view! {
                    <div class=class style=format!("margin-bottom: 0.3em; {location}")>
                        <label>
                            <input
                                style="vertical-align: baseline;"
                                id=html_id.clone()
                                on:click=click_handler
                                name=name
                                type="radio"
                                value=html_id.clone()
                                checked=move || {
                                    updatable_rw_signal.with(|updatable| updatable.value == e)
                                }
                            />

                            {label}
                        </label>
                    </div>
                }
            })
            .collect::<Vec<_>>()
            .into_view()
    };

    let content_view = move || updatable_rw_signal.with(|updatable| content_maker(updatable.value));

    let content_location = match direction {
        SelectDirection::LeftToRight => {
            format!("grid-row: 2 / 3; grid-column: 1 / {}", column_count + 1)
        }
        SelectDirection::TopToBottom => {
            format!("grid-column: 2 / 3; grid-row: 1 / {}", column_count + 1)
        }
    };

    // ω <fn one_of_component>
    view! {
        <div class=SELF_CLASS>
            // α <plus-ooc-view>
            <div class=CssClasses::OocCtnr.as_str() style=container_style>
                {radio_buttons}
                <hr/>
                <div style=content_location>{content_view}</div>
            </div>
        // ω <plus-ooc-view>
        </div>
    }
}

// α <mod-def one_of_component>
// ω <mod-def one_of_component>
