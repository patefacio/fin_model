//! Module for year_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::IntegerClamp;
use crate::Updatable;
use leptos::component;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::MaybeSignal;
use leptos::ReadSignal;
use leptos_dom::html::Input;
use plus_modeled::core::YearRange;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// A component for specifying a year, constrained by a year range.
///
///   * **input_class** - Class to decorate input element for styling
///   * **year_range** - Range of valid years.
///   * **updatable** - Value and callback
///   * **placeholder** - Placeholder shown if entry is empty.
///   * **disabled** - Signal allowing the disabling of the input.
///   * **clear_input** - Signal requesting to clear the input.
///   * **set_focus** - Signal requesting to set focus.
///   * **live_clamp** - If set will force values to be within year range as they are typed.
/// With this true if the user enters a year with the proper number
/// of digits it will be within range. But it may be disorienting
/// as the numbers showing up in input may not be those typed.
///   * **align_left** - If set, numeric text aligned to left.
///   * _return_ - View for year_input
#[component]
pub fn YearInput(
    /// Class to decorate input element for styling
    #[prop(default=None)]
    input_class: Option<String>,
    /// Range of valid years.
    #[prop(default=YearRange{ start: 1900, end: 2400 })]
    year_range: YearRange,
    /// Value and callback
    updatable: Updatable<Option<u32>>,
    /// Placeholder shown if entry is empty.
    #[prop(default=MaybeSignal::Static(String::from("Year")), into)]
    placeholder: MaybeSignal<String>,
    /// Signal allowing the disabling of the input.
    #[prop(into, optional)]
    disabled: MaybeSignal<bool>,
    /// Signal requesting to clear the input.
    #[prop(default=None)]
    clear_input: Option<ReadSignal<()>>,
    /// Signal requesting to set focus.
    #[prop(default=None)]
    set_focus: Option<ReadSignal<()>>,
    /// If set will force values to be within year range as they are typed.
    /// With this true if the user enters a year with the proper number
    /// of digits it will be within range. But it may be disorienting
    /// as the numbers showing up in input may not be those typed.
    #[prop(default = false)]
    live_clamp: bool,
    /// If set, numeric text aligned to left.
    #[prop(default = false)]
    align_left: bool,
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-yi";
    crate::log_component!("`YearInput`");
    // α <fn year_input>

    use crate::utils::constants::{DOWN_KEY, UP_KEY};
    use crate::CssClasses;
    use crate::ParsedNum;
    use leptos::create_effect;
    use leptos::create_node_ref;
    use leptos::create_rw_signal;
    use leptos::create_signal;
    use leptos::IntoClass;
    use leptos::IntoStyle;
    use leptos::SignalGet;
    use leptos::SignalSet;
    use leptos::SignalUpdate;
    use web_sys::KeyboardEvent;

    // Determine if year is in the provided range
    let year_is_valid = move |year| year >= year_range.start && year <= year_range.end;

    // Track whether year is valid to give hint to user - reactive to update class
    let (is_in_range_read, is_in_range_write) = create_signal(
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

    let node_ref = create_node_ref::<Input>();
    let mut updatable = updatable;
    let year_clamp = if live_clamp {
        Some(IntegerClamp::new(year_range.start..=year_range.end))
    } else {
        None
    };

    create_effect(move |_| {
        if let Some(clear_input) = clear_input.as_ref() {
            let _ = clear_input.get();
            if let Some(input_ref) = node_ref.get() {
                input_ref.set_value("");
            }
        }
    });

    create_effect(move |_| {
        if let Some(set_focus) = set_focus.as_ref() {
            let _ = set_focus.get();
            if let Some(input_ref) = node_ref.get() {
                let _ = input_ref.focus();
            }
        }
    });

    let update_value = move || {
        // First get the HtmlElement<Input>, think of it as a handle that provides
        // access to the input element in the DOM that holds our number.
        let input_ref = node_ref.get().expect("Year input node");

        // From the input element get the current value, which is just the
        // text the user has typed in the input field so far.
        let mut value = input_ref.value();

        // First cull any non numeric characters. Years must be all ascii digits,
        // no negative or decimal allowed. (Could this be faster?)
        value = value.chars().filter(|c| c.is_ascii_digit()).collect();

        if value.is_empty() {
            // No characters in the input are valid digits - the value is now None
            updatable.update_and_then_signal(|year| *year = None);
            input_ref.set_value("");
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
            is_in_range_write.set(year_is_valid(clamped.as_u32));
            input_ref.set_value(&clamped.as_string);
        }
    };

    let update_value = create_rw_signal(update_value);

    let key_down = move |ev: KeyboardEvent| {
        let key_code = ev.key_code();
        match key_code {
            UP_KEY | DOWN_KEY => {
                let input_ref = node_ref.get().expect("Input node");
                let mut value = input_ref.value();
                if let Ok(year) = value.parse::<i32>() {
                    value = (year + if key_code == UP_KEY { 1 } else { -1 }).to_string();
                    input_ref.set_value(&value);
                    update_value.update(|update_value| update_value());
                    ev.prevent_default();
                }
            }
            _ => {}
        }
    };

    // ω <fn year_input>
    view! {
        <div class=SELF_CLASS>
            // α <plus-yi-view>

            <input
                node_ref=node_ref
                class=CssClasses::YiInput.as_str()
                class=input_class
                class:invalid=move || { !is_in_range_read.get() }
                on:input=move |_| update_value.update(|update_value| update_value())
                on:keydown=key_down
                value=initial_value
                size=5
                maxlength=4
                style:text-align=move || { if align_left { "left" } else { "right" } }
                placeholder=placeholder
                type="text"
                disabled=disabled
            />
        // ω <plus-yi-view>
        </div>
    }
}

// α <mod-def year_input>

// ω <mod-def year_input>
