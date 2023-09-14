//! Module for one_of_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
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
    let container_style = format!(
        "display: grid; grid-template-columns: {}; margin: 3px;",
        "1fr ".repeat(column_count)
    );

    let make_radio_button = move |e: E, name: String| -> View {
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

        view! {
            <div style="margin-bottom: 0.3em;">
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
        .map(|(e, &label)| make_radio_button(e, label.to_string()))
        .collect::<Vec<_>>();

    view! {
        <div style=container_style>{radio_buttons}</div>
        <hr/>
        {move || {
            let mut result = None;
            views_store_value
                .update_value(|views| {
                    result = Some(views(&value.get()).into_view());
                });
            result
        }}
    }
    // ω <fn one_of_component>
}

// α <mod-def one_of_component>
// ω <mod-def one_of_component>
