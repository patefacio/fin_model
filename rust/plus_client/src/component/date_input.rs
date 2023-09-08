//! Module for date_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::Date;
use plus_modeled::YearRange;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a date of format _MM/DD/YYYY_.
///
///   * **cx** - Context
///   * **updatable** - The [Date] being edited
///   * **year_range** - Range of valid years for date input.
///   * **placeholder** - Placeholder shown if entry is empty.
///   * **input_class** - Class to decorate input element for styling
///   * _return_ - View for date_input
#[component]
pub fn DateInput(
    /// Context
    cx: Scope,
    /// The [Date] being edited
    updatable: Updatable<Option<Date>>,
    /// Range of valid years for date input.
    #[prop(default=Some(YearRange{ start: 1900, end: 2350 }))]
    year_range: Option<YearRange>,
    /// Placeholder shown if entry is empty.
    #[prop(default=None)]
    placeholder: Option<String>,
    /// Class to decorate input element for styling
    #[prop(default=None)]
    input_class: Option<String>,
) -> impl IntoView {
    // α <fn date_input>

    use crate::utils::constants::{LEFT_KEY, RIGHT_KEY, TAB_KEY};
    use crate::LiveParsedDate;
    use leptos::create_node_ref;
    use leptos::create_signal;
    use leptos::store_value;
    use leptos::IntoAttribute;
    use leptos::IntoClass;
    use leptos::SignalGet;
    use leptos::SignalSet;
    use leptos_dom::html::Input;
    use web_sys::KeyboardEvent;

    struct DateData {
        updatable: Updatable<Option<Date>>,
        live_parsed_date: LiveParsedDate,
    }

    let mut is_in_range = false;

    let live_parsed_date = updatable
        .value
        .as_ref()
        .map(|date| {
            let mut live_parsed_date = LiveParsedDate::from_date(date, year_range);
            // Parse the date passed in to ensure it is within range
            let (_, date, _) = live_parsed_date
                .parse_date(&format!("{}/{}/{}", date.month, date.day, date.year), 0);
            is_in_range = date.is_some();
            live_parsed_date
        })
        .unwrap_or_else(|| LiveParsedDate::new(year_range));

    // Track whether year is valid to give hint to user - reactive to update class
    let (is_in_range, set_is_in_range) = create_signal(cx, is_in_range);

    let initial_value = if updatable.value.is_some() {
        live_parsed_date.formatted.clone()
    } else {
        String::default()
    };

    let date_data = store_value(
        cx,
        DateData {
            updatable,
            live_parsed_date,
        },
    );

    let node_ref = create_node_ref::<Input>(cx);
    let update_value = move || {
        let input_ref = node_ref.get().expect("Date input node");
        let value = input_ref.value();
        let position = input_ref
            .selection_start()
            .unwrap_or_default()
            .unwrap_or_default();

        date_data.update_value(|date_data| {
            let live_parsed_date = &mut date_data.live_parsed_date;
            let (new_value, new_date, new_position) = live_parsed_date.parse_date(&value, position);
            input_ref.set_value(&new_value);
            _ = input_ref.set_selection_range(new_position, new_position);
            if let Some(new_date) = new_date.as_ref() {
                date_data.updatable.update_and_then_signal(|date| {
                    set_is_in_range.set(true);
                    *date = Some(new_date.clone());
                });
            } else {
                set_is_in_range.set(false);
            }
        });
    };

    let key_down_handler = move |ev: KeyboardEvent| {
        let key_code = ev.key_code();
        let is_shift = ev.shift_key();

        match key_code {
            TAB_KEY | LEFT_KEY | RIGHT_KEY => {
                // Interpret tab as jump from field to next
                let input_ref = node_ref.get().expect("Date input node");
                let value = input_ref.value();

                let position = input_ref
                    .selection_end()
                    .unwrap_or_default()
                    .unwrap_or_default();

                match key_code {
                    TAB_KEY if !value.is_empty() => {
                        // Shift-tab means jump to prior field and tab means jump to next field
                        if is_shift {
                            if position >= 6 {
                                _ = input_ref.set_selection_range(3, 3);
                                update_value();
                                ev.prevent_default();
                            } else if position >= 3 {
                                _ = input_ref.set_selection_range(0, 0);
                                update_value();
                                ev.prevent_default();
                            }
                        } else if position < 3 {
                            _ = input_ref.set_selection_range(3, 3);
                            update_value();
                            ev.prevent_default();
                        } else if position < 6 {
                            _ = input_ref.set_selection_range(6, 6);
                            update_value();
                            ev.prevent_default();
                        }
                    }
                    RIGHT_KEY => {
                        date_data.with_value(|date_data| {
                            let position = input_ref
                                .selection_start()
                                .unwrap_or_default()
                                .unwrap_or_default()
                                + 1;
                            if let Some(i) = date_data.live_parsed_date.formatted.find('_') {
                                let capped_position = position.min(i as u32);
                                _ = input_ref.set_selection_range(capped_position, capped_position);
                                ev.prevent_default();
                            }
                        });
                    }
                    _ => (),
                }
            }

            _ => (),
        }
    };

    view! { cx,
        <input
            node_ref=node_ref
            class="date-input"
            class=input_class
            class:invalid=move || { !is_in_range.get() }
            on:input=move |_| update_value()
            on:keydown=key_down_handler
            value=initial_value
            size=12
            placeholder=placeholder
            type="text"
        />
    }
    // ω <fn date_input>
}

// α <mod-def date_input>
// ω <mod-def date_input>
