//! Module for percent_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::component::numeric_input::{Modification, NumericInput, NumericInputProps};
use crate::Updatable;
use leptos::create_signal;
#[allow(unused_imports)]
use leptos::log;
use leptos::ReadSignal;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use std::ops::RangeInclusive;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Provides a [NumericInput] with a percent suffix modification.
///
///   * **cx** - Context
///   * **updatable** - Called when input is updated.
///   * **placeholder** - Placeholder for the year field
///   * **size** - The size attribute, which one hopes would make the size of the
/// input field roughly that number of characters. But YMMV.
///
///   * **max_len** - The maximum number of characters for the percent input.
///
///   * **range** - Range of valid values for input.
///   * **non_negative** - If set, negative values are disallowed.
///   * **on_enter** - Called if user hits enter, passes current input value.
///   * **clear_input** - Signal requesting to clear the input.
///   * _return_ - View for percent_input
#[component]
pub fn PercentInput(
    /// Context
    cx: Scope,
    /// Called when input is updated.
    updatable: Updatable<Option<f64>>,
    /// Placeholder for the year field
    #[prop(default=None)]
    placeholder: Option<String>,
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
) -> impl IntoView {
    // α <fn percent_input>

    use crate::utils::scale_by::scale_by;

    let scaled_value = updatable.value.map(|value| scale_by(value, 2));
    let mut updatable = updatable;
    let numeric_updatable = Updatable::new(scaled_value, move |new_scaled_value| {
        let actual_value = new_scaled_value.map(|v| v / 100.0);
        updatable.update_and_then_signal(|new_value| {
            *new_value = actual_value;
        });
    });

    view! { cx,
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
        />
    }

    // ω <fn percent_input>
}

// α <mod-def percent_input>
// ω <mod-def percent_input>
