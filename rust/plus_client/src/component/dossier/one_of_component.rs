//! Module for one_of_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::SelectDirection;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::Debug;
use strum::{IntoEnumIterator, VariantNames};

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Provides a component to toggle between values in an enumeration.
///
///   * **selection** - The selections to choose from
///   * **name** - Used for the name of radio buttons
///   * **labels** - Labels to show
///   * **views** - Function to create view from option
///   * **direction** - Specifies whether items flows from top to bottom or left to right.
///   * _return_ - View for one_of_component
#[component]
pub fn OneOfComponent<E, L, F, IV>(
    /// The selections to choose from
    selection: E,
    /// Used for the name of radio buttons
    name: String,
    /// Labels to show
    #[prop(default=None)]
    labels: Option<L>,
    /// Function to create view from option
    views: F,
    /// Specifies whether items flows from top to bottom or left to right.
    #[prop(default=SelectDirection::LeftToRight)]
    direction: SelectDirection,
) -> impl IntoView
where
    E: Clone + Debug + VariantNames + IntoEnumIterator + PartialEq + 'static,
    L: Fn(&E) -> String + 'static,
    F: FnMut(&E) -> IV + 'static,
    IV: IntoView,
{
    // α <fn one_of_component>

    use leptos::create_rw_signal;
    use leptos::store_value;
    use leptos::IntoAttribute;
    use leptos::SignalGet;
    use leptos::SignalUpdate;
    use leptos::View;
    use std::iter::zip;

    let one_of_name = name;
    let views_store_value = store_value(views);
    let labels_store_value = store_value(labels);
    let value = create_rw_signal(selection.clone());
    let column_count = E::VARIANTS.len();
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

    let make_radio_button = move |i: usize, e: E, name: String| -> View {
        let handler_e = e.clone();
        let checked_e = e.clone();
        let id = name.to_lowercase().replace(" ", "-");
        let click_handler = move |_| value.update(|value| *value = handler_e.clone());
        let label = labels_store_value.with_value(|labels| {
            if let Some(labels) = labels {
                labels(&e)
            } else {
                format!("{selection:?}")
            }
        });

        let (class, location) = match direction {
            SelectDirection::LeftToRight => ("one-of-rb-ltr", format!("grid-column: {} / {};", i+1, i+2)),
            SelectDirection::TopToBottom => ("one-of-rb-ttb", format!("grid-row: {} / {};", i+1, i+2))
        };

        view! {
            <div class=class style=format!("margin-bottom: 0.3em; {location}")>
                <label>
                    <input
                        style="vertical-align: baseline;"
                        id=id.clone()
                        on:click=click_handler
                        name=one_of_name.clone()
                        type="radio"
                        value=id.clone()
                        checked=move || { value.get() == checked_e }
                    />
                    {label}
                </label>
            </div>
        }
        .into_view()
    };

    let radio_buttons = zip(E::iter(), E::VARIANTS)
        .enumerate()
        .map(|(i, (e, &label))| make_radio_button(i, e, label.to_string()))
        .collect::<Vec<_>>();

    let content_view = move || {
        let mut result = None;
        let content_view = views_store_value
            .update_value(|views| {
                result = Some(
                    views(&value.get()).into_view()
                );
            });
        result
    };

    let content_location = match direction {
        SelectDirection::LeftToRight => format!("grid-row: 2 / 3; grid-column: 1 / {}", column_count + 1),
        SelectDirection::TopToBottom => format!("grid-column: 2 / 3; grid-row: 1 / {}", column_count + 1)
    };

    view! {
        <div class="one-of" style=container_style>
        {radio_buttons}
        <hr/>
        <div style=content_location >
        {content_view}
        </div>
        </div>
    }
    // ω <fn one_of_component>
}

// α <mod-def one_of_component>
// ω <mod-def one_of_component>
