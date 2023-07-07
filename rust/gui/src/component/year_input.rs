//! Module for year_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::IntegerClamp;
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
use leptos::{create_node_ref, ReadSignal, RwSignal};
#[allow(unused_imports)]
use leptos_dom::console_log;
use leptos_dom::html::Input;
use plus_modeled::core::YearRange;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// A component for specifying a year, constrained by a year range.
///
///   * **cx** - Context
///   * **input_class** - Class to decorate input element for styling
///   * **year_range** - Range of valid years.
///   * **updatable** - Value and callback
///   * **placeholder** - Placeholder shown if entry is empty.
///   * **disabled** - Signal allowing the disabling of the input.
///   * **clear_input** - Signal requesting to clear the input.
///   * **live_clamp** - If set will force values to be within year range as they are typed.
/// With this true if the user enters a year with the proper number
/// of digits it will be within range. But it may be disorienting
/// as the numbers showing up in input may not be those typed.
///   * _return_ - View for year_input
#[component]
pub fn YearInput(
    /// Context
    cx: Scope,
    /// Class to decorate input element for styling
    #[prop(default=None)]
    input_class: Option<String>,
    /// Range of valid years.
    #[prop(default=YearRange{ start: 1900, end: 2400 })]
    year_range: YearRange,
    /// Value and callback
    updatable: Updatable<Option<u32>>,
    /// Placeholder shown if entry is empty.
    #[prop(default=None)]
    placeholder: Option<String>,
    /// Signal allowing the disabling of the input.
    #[prop(default=None)]
    disabled: Option<ReadSignal<bool>>,
    /// Signal requesting to clear the input.
    #[prop(default=None)]
    clear_input: Option<RwSignal<bool>>,
    /// If set will force values to be within year range as they are typed.
    /// With this true if the user enters a year with the proper number
    /// of digits it will be within range. But it may be disorienting
    /// as the numbers showing up in input may not be those typed.
    #[prop(default = false)]
    live_clamp: bool,
) -> impl IntoView {
    // α <fn year_input>

    use crate::ParsedNum;
    use leptos::create_signal;
    use leptos::IntoAttribute;
    use leptos::SignalGet;
    use leptos::*;

    // Determine if year is in the provided range
    let year_is_valid = move |year| {
        console_log(&format!("Checking {year} against {year_range:?}"));
        year >= year_range.start && year <= year_range.end
    };

    // Track whether year is valid to give hint to user - reactive to update class
    let (is_in_range, set_is_in_range) = create_signal(
        cx,
        updatable
            .value
            .map(|year| year_is_valid(year))
            .unwrap_or_default(),
    );

    // Get the initial value for the year if provided. Set to empty string if
    // not provided.
    let initial_value = if let Some(initial_value) = updatable.value.as_ref() {
        initial_value.to_string()
    } else {
        String::default()
    };

    let node_ref = create_node_ref::<Input>(cx);
    let mut updatable = updatable;
    let year_clamp = if live_clamp {
        Some(IntegerClamp::new(year_range.start..=year_range.end))
    } else {
        None
    };

    // TODO: Figure out a way for this control to own and manage the input but
    // also for the parent to, on request, clear the contents.
    let clear_requested = move || {
        console_log(&format!("Checking on clearing input {clear_input:?}!!"));

        if let Some(clear_input) = clear_input {
            if clear_input.get() {
                console_log("Requested to clear input");

                if let Some(input_ref) = node_ref.get() {
                    console_log(&format!("Clearing input from {}!!", input_ref.value()));
                    input_ref.set_value("");
                }
            }

            clear_input.set(false);
        }
    };

    let mut update_value = move || {
        // First get the HtmlElement<Input>, think of it as a handle that provides
        // access to the input element in the DOM that holds our number.
        let input_ref = node_ref.get().expect("Year input node");

        // From the input element get the current value, which is just the
        // text the user has typed in the input field so far.
        let mut value = input_ref.value();

        // First cull any non numeric characters. Years must be all ascii digits,
        // no negative or decimal allowed. (Could this be faster?)
        value = value.chars().filter(|c| c.is_ascii_digit()).collect();

        console_log(&format!("YearInput: filtered -> {value:?}"));

        if value.is_empty()
            || clear_input
                .map(|clear_input| {
                    console_log("What side effect");
                    clear_input.get()
                })
                .unwrap_or_default()
        {
            // No characters in the input are valid digits - the value is now None
            updatable.update_and_then_signal(|year| *year = None);
            console_log(&format!("YearInput: setting value to -> None"));
            input_ref.set_value("");
            // if let Some(clear_input) = clear_input {
            //     clear_input.set(false);
            // }
        } else {
            // We have valid digits, pass to clamp to ensure value is in range.
            // Update the input by setting the value to the clamped_year. This may
            // be off-putting and we should discuss. Imagine typing "2303" and on
            // entering that last 3 what gets displayed is "2300". It will certainly
            // ensure the value is in range but maybe the approach is heavy handed.
            let clamped = year_clamp
                .as_ref()
                .map(|year_clamp| year_clamp.clamp(&value))
                .unwrap_or_else(|| ParsedNum::from_str(&value));
            updatable.update_and_then_signal(|year| *year = Some(clamped.as_u32));
            set_is_in_range.set(year_is_valid(clamped.as_u32));
            input_ref.set_value(&clamped.as_string);
            console_log(&format!("YearInput: setting value to -> {clamped:?}"));
        }
    };

    view! { cx,
        <input
            node_ref=node_ref
            class="year-input"
            class=input_class
            class:invalid=move || { !is_in_range.get() }
            on:input=move |_| update_value()
            value=initial_value
            size=5
            maxlength=4
            placeholder=placeholder
            type="text"
            disabled=disabled.map(|disabled| disabled.get()).unwrap_or_default()
            hat=move || {
                let _ = clear_requested();
                1
            }
        />
    }

    // ω <fn year_input>
}

// α <mod-def year_input>

// ω <mod-def year_input>
