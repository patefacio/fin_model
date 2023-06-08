//! Module for year_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::utils::updatable::Updatable;
use crate::utils::year_clamp::YearClamp;
use fin_model::core::YearRange;
use leptos::create_node_ref;
use leptos::{component, view, IntoView, Scope};
use leptos_dom::{console_log, html::Input};

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// A component for specifying a year, constrained by a year range.
///
///   * **cx** - Context
///   * **input_class** - Class to decorate input element for styling
///   * **year_range** - Range of valid years.
///   * **updatable** - Value and callback
///   * _return_ - View for year_input
#[component]
pub fn YearInput(
    /// Context
    cx: Scope,
    /// Class to decorate input element for styling
    #[prop(default=None)]
    input_class: Option<String>,
    /// Range of valid years.
    #[prop(default=YearRange{ start: 1900, end: 2300 })]
    year_range: YearRange,
    /// Value and callback
    updatable: Updatable<Option<u32>>,
) -> impl IntoView {
    // α <fn year_input>

    use leptos::IntoAttribute;

    // Get the initial value for the year if provided. Set to empty string if
    // not provided.
    let initial_value = if let Some(initial_value) = updatable.value.as_ref() {
        initial_value.to_string()
    } else {
        String::default()
    };
    
    let node_ref = create_node_ref::<Input>(cx);
    let mut updatable = updatable;
    let year_clamp = YearClamp::new(year_range);

    // Bunch of not great code here has been elided. The elided code and
    // therefore the code we want to write will ensure the value in the input
    // field is valid. It will get the value - which is a String (because input
    // fields always accept text which maps to Strings) - and then ensures the
    // provided number is within the range.
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

        leptos_dom::console_log(&format!("YearInput: filtered -> {value:?}"));

        if value.is_empty() {
            // No characters in the input are valid digits - the value is now None
            updatable.update(|year| *year = None);
            leptos_dom::console_log(&format!("YearInput: setting value to -> None"));
            input_ref.set_value("");
        } else {
            // We have valid digits, pass to clamp to ensure value is in range.
            // Update the input by setting the value to the clamped_year. This may
            // be off-putting and we should discuss. Imagine typing "2303" and on
            // entering that last 3 what gets displayed is "2300". It will certainly
            // ensure the value is in range but maybe the approach is heavy handed.
            let clamped  = year_clamp.clamp(&value);
            updatable.update(|year| *year = Some(clamped.as_u32));
            input_ref.set_value(&clamped.as_string);
            leptos_dom::console_log(&format!("YearInput: setting value to -> {clamped:?}"));
        }
    };

    view! { cx,
        <input
            node_ref=node_ref
            on:input = move |_| update_value()
            // This sets the value in a static manner: 
            // https://leptos-rs.github.io/leptos/view/05_forms.html?highlight=prop%3Avalue#uncontrolled-inputs:~:text=%22%20%7Bname%7D%3C/p%3E%0A%7D-,Uncontrolled%20Inputs,-In%20an%20%22uncontrolled
            value=initial_value
            type="text"
        />
    }

    // ω <fn year_input>
}

// α <mod-def year_input>

// ω <mod-def year_input>
