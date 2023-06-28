//! Module for date_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::Date;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a date of format _MM/DD/YYYY_.
///
///   * **cx** - Context
///   * **updatable** - The [Date] being edited
///   * **placeholder** - Placeholder shown if entry is empty.
///   * _return_ - View for date_input
#[component]
pub fn DateInput(
    /// Context
    cx: Scope,
    /// The [Date] being edited
    updatable: Updatable<Option<Date>>,
    /// Placeholder shown if entry is empty.
    #[prop(default=None)]
    placeholder: Option<String>,
) -> impl IntoView {
    // α <fn date_input>
    const YEAR_SIZE: u32 = 4;

    use leptos::create_node_ref;
    use leptos::IntoAttribute;
    use leptos_dom::html::Input;

    let initial_value = if let Some(initial_value) = updatable.value.as_ref() {
        initial_value.day.to_string()
    } else {
        String::default()
    };

    let node_ref = create_node_ref::<Input>(cx);

    //let updatable = store_value(cx, updatable.value.unwrap_or(Date { year: 0, month: 0, day: 0 }));
    let mut updatable = updatable;

    fn size_clamp(mut s: String, lower_bound: u32, upper_bound: u32, fill: char) -> String {
        for _i in 0..(s.len() as i32 - upper_bound as i32) {
            s.pop();
        }
        for _i in 0..(lower_bound as i32 - s.len() as i32) {
            s.push(fill);
        }
        return s;
    }

    fn date_string(mut s: String) -> String {
        if s.len() > 2 {
            s.insert(2, '/');
        }
        if s.len() > 5 {
            s.insert(5, '/');
        }
        return s;
    }

    let mut update_value = move || {
        let input_ref = node_ref.get().expect("Date input node");
        let mut value = input_ref.value();

        console_log(&format!("{} size {}", value, value.len()));
        let last_letter = value.chars().last().unwrap_or(' ');

        if value.len() == 2 && last_letter == '/' {
            value.insert(0, '0');
        }
        if value.len() == 5 && last_letter == '/' {
            value.insert(3, '0')
        }

        value = value.chars().filter(|c| c.is_ascii_digit()).collect();
        value = size_clamp(date_string(value), 0, 6 + YEAR_SIZE, '0');

        if value.len() >= 2 && value[0..2].parse::<u32>().unwrap() > 12 {
            value.remove(0);
            value.remove(0);
            value.insert_str(0, "12")
        }
        if value.len() >= 5 && value[3..5].parse::<u32>().unwrap() > 31 {
            value.remove(3);
            value.remove(3);
            value.insert_str(3, "31")
        }

        let full_date = size_clamp(value.clone(), 6 + YEAR_SIZE, 6 + YEAR_SIZE, '0');
        //value = full_date[0..value.len()].to_string();

        leptos_dom::console_log(&format!("DateInput: filtered -> {value:?}"));

        if value.is_empty() {
            updatable.update_and_then_signal(|year| *year = None);
            leptos_dom::console_log(&format!("DateInput: setting value to -> None"));
            input_ref.set_value("");
        } else {
            updatable.update_and_then_signal(|year| {
                *year = Some(Date {
                    month: full_date[0..2].parse::<u32>().unwrap(),
                    day: full_date[3..5].parse::<u32>().unwrap(),
                    year: full_date[6..(6 + YEAR_SIZE as usize)]
                        .parse::<u32>()
                        .unwrap(),
                })
            });
            input_ref.set_value(&value);
            leptos_dom::console_log(&format!("DateInput: setting value to -> {value:?}"));
        }
    };
    view! { cx,
        <input
            node_ref=node_ref
            on:input=move |_| update_value()
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
