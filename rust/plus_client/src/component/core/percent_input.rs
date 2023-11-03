//! Module for percent_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Modification;
use crate::NumericInput;
use crate::Updatable;
use leptos::MaybeSignal;
use leptos::ReadSignal;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;
use std::ops::RangeInclusive;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Provides a [NumericInput] with a percent suffix modification.
///
///   * **updatable** - Called when input is updated.
///   * **placeholder** - Placeholder shown if entry is empty.
///   * **size** - The size attribute, which one hopes would make the size of the
/// input field roughly that number of characters. But YMMV.
///   * **max_len** - The maximum number of characters for the percent input.
///
///   * **range** - Range of valid values for input.
///   * **non_negative** - If set, negative values are disallowed.
///   * **on_enter** - Called if user hits enter, passes current input value.
///   * **clear_input** - Signal requesting to clear the input.
///   * **disabled** - Signal allowing the disabling of the input.
///   * _return_ - View for percent_input
#[component]
pub fn PercentInput(
    /// Called when input is updated.
    updatable: Updatable<Option<f64>>,
    /// Placeholder shown if entry is empty.
    #[prop(into, optional)]
    placeholder: String,
    /// The size attribute, which one hopes would make the size of the
    /// input field roughly that number of characters. But YMMV.
    #[prop(default = 9)]
    size: u32,
    /// The maximum number of characters for the percent input.
    #[prop(default = 8)]
    max_len: u32,
    /// Range of valid values for input.
    #[prop(default=None)]
    range: Option<RangeInclusive<f64>>,
    /// If set, negative values are disallowed.
    #[prop(default = false)]
    non_negative: bool,
    /// Called if user hits enter, passes current input value.
    #[prop(default=None)]
    on_enter: Option<Box<dyn FnMut(String)>>,
    /// Signal requesting to clear the input.
    #[prop(default=None)]
    clear_input: Option<ReadSignal<()>>,
    /// Signal allowing the disabling of the input.
    #[prop(into, optional)]
    disabled: MaybeSignal<bool>,
) -> impl IntoView {
    crate::log_component!("`PercentInput`");
    // α <fn percent_input>

    use crate::scale_by;

    let scaled_value = updatable.value.map(|value| scale_by(value, 2));
    let mut updatable = updatable;
    let numeric_updatable = Updatable::new(scaled_value, move |new_scaled_value| {
        let actual_value = new_scaled_value.map(|v| v / 100.0);
        updatable.update_and_then_signal(|new_value| {
            *new_value = actual_value;
        });
    });

    view! {
        <NumericInput
            updatable=numeric_updatable
            on_enter=on_enter
            modification=Some(Modification::Suffix("%".into()))
            non_negative=non_negative
            placeholder=placeholder
            max_len=max_len
            clear_input=clear_input
            size=size
            range=range.map(|range| range.start() * 100.0..=range.end() * 100.0)
            disabled=disabled
        />
    }

    // ω <fn percent_input>
}

// α <mod-def percent_input>
// ω <mod-def percent_input>
