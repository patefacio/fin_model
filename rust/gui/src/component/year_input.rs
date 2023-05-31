//! Module for year_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::utils::updatable::Updatable;
use fin_model::core::YearRange;
use leptos::{component, create_node_ref, tracing, view, IntoView, Scope};
use leptos_dom::{console_log, html::Input};

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// A component for specifying a year, constrained by a year range.
///
///   * **cx** - Context
///   * **year_range** - Range of valid years.
///   * **updatable** - Value and callback
///   * _return_ - View for year_input
#[component]
pub fn YearInput<F>(
    /// Context
    cx: Scope,
    /// Range of valid years.
    #[prop(default=YearRange{ start: 1900, end: 2300 })]
    year_range: YearRange,
    /// Value and callback
    updatable: Updatable<u32, F>,
) -> impl IntoView
where
    F: FnMut(&u32) + 'static,
{
    // α <fn year_input>

    let node_ref = create_node_ref::<Input>(cx);
    let mut updatable = updatable;

    let mut update_value = move || {
        let input_ref = node_ref.get().expect("Year input node");
        let mut value = input_ref.value();
        let first_char_in_start = first_digit(year_range.start);
        let first_char_in_end = first_digit(year_range.end);
        value = value
            .chars()
            .enumerate()
            .filter_map(|(i, c)| if c.is_ascii_digit() { 
                if i == 0 {
                    let first_char = c.max(first_char_in_start).min(first_char_in_end);
                    leptos_dom::console_log(&format!("fcs({first_char_in_start}), fce({first_char_in_end}) ({i},{c})->`{first_char}`"));
                    Some(first_char)
                } else {
                    Some(c) 
                }
            } else { None })
            .take(4)
            .collect();
        if value.len() == 4 {
            let i = value.parse::<u32>().expect("Valid int");
            updatable.update(|value| *value = i);
        }
        input_ref.set_value(&value);
    };

    view! { cx,
        <input
            node_ref=node_ref
            on:input = move |_| update_value()
            type="text"
        />
    }

    // ω <fn year_input>
}

// α <mod-def year_input>

fn first_digit(number: u32) -> char {
    let mut number = number;

    // While the number is greater than 9, divide it by 10.
    while number > 9 {
        number /= 10;
    }

    // Return the first digit of the number.
    return number.to_string().chars().next().expect("Valid digit");
}

// ω <mod-def year_input>
