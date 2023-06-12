//! Module for numeric_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::utils::constants::{LEFT_KEY, RIGHT_KEY};
use crate::utils::numeric_text::{digit_position, format_number_lenient};
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
use leptos::{create_effect, create_node_ref, store_value, ReadSignal, SignalWith};
use leptos_dom::{console_log, html::Input};
use web_sys::KeyboardEvent;

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a prefix or suffix modification for a numeric input.
/// Non-reactive strings are supported for both prefix and suffix.
/// Reactive prefixes are supported for cases where like
/// currency which may need reactivity. The modification text appears
/// in the html input of the component.
pub enum Modification {
    /// A prefix for the number.
    Prefix(String),
    /// A reactive prefix for the number.
    ReactivePrefix(ReadSignal<String>),
    /// A suffix for the number.
    Suffix(String),
}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// A component for accepting numeric input.
/// Only valid numeric characters (ascii) are allowed. Numbers are automatically formatted.
///
///   * **cx** - Context
///   * **updatable** - Signal updated as numeric input is updated.
///   * **input_class** - Class to decorate input element for styling
///   * **modification** - Optional modification (e.g. suffix/prefix)
///   * **non_negative** - If set, negative values are disallowed.
///   * **placeholder** - Placeholder shown if entry is empty.
///   * _return_ - View for numeric_input
#[component]
pub fn NumericInput(
    /// Context
    cx: Scope,
    /// Signal updated as numeric input is updated.
    updatable: Updatable<Option<f64>>,
    /// Class to decorate input element for styling
    #[prop(default=None)]
    input_class: Option<String>,
    /// Optional modification (e.g. suffix/prefix)
    #[prop(default=None)]
    modification: Option<Modification>,
    /// If set, negative values are disallowed.
    #[prop(default = false)]
    non_negative: bool,
    /// Placeholder shown if entry is empty.
    #[prop(default=None)]
    placeholder: Option<String>,
) -> impl IntoView {
    // α <fn numeric_input>

    use leptos::IntoAttribute;

    // Get the initial value for the year if provided. Set to empty string if
    // not provided.
    let initial_value = if let Some(initial_value) = updatable.value.as_ref() {
        initial_value.to_string()
    } else {
        String::default()
    };

    let node_ref = create_node_ref::<Input>(cx);
    let modification = store_value(cx, modification);
    let mut updatable = updatable;

    let update_value = move || {
        modification.with_value(|modification| {
            let input_ref = node_ref.get().expect("Input node");
            let mut selection_start = input_ref
                .selection_start()
                .unwrap_or_default()
                .unwrap_or_default();

            let mut value = input_ref.value();

            if non_negative {
                while let Some(neg_pos) = value.find('-') {
                    if neg_pos < selection_start as usize {
                        selection_start -= 1;
                    }
                    value.remove(neg_pos);
                }
            }

            let (value, mut new_value, numeric_to_caret) =
                format_number_lenient(&value, selection_start);

            leptos_dom::console_log(&format!(
                "Format result {:?}",
                (value, &new_value, numeric_to_caret)
            ));

            if let Some(modification) = modification.as_ref() {
                new_value = modification.modify(&new_value);

                input_ref.set_value(&new_value);
                let new_position = modification.position_in_number(
                    new_value.len(),
                    digit_position(&new_value, numeric_to_caret) as usize,
                ) as u32;

                _ = input_ref.set_selection_range(new_position, new_position);
            } else {
                input_ref.set_value(&new_value);
            }

            updatable.update(|number| *number = value);
        });
    };

    let update_value = leptos::store_value(cx, update_value);

    let key_movement = move |ev: KeyboardEvent| {
        let key_code = ev.key_code();
        console_log(&format!("Examining key {key_code}"));
        match key_code {
            LEFT_KEY | RIGHT_KEY => modification.with_value(|modification| {
                if let Some(modification) = modification {
                    let input_ref = node_ref.get().expect("Input node");
                    let mut selection_start = input_ref
                        .selection_start()
                        .unwrap_or_default()
                        .unwrap_or_default();

                    selection_start = modification
                        .position_in_number(input_ref.value().len(), selection_start as usize)
                        as u32;

                    if key_code == LEFT_KEY {}
                    _ = input_ref.set_selection_range(selection_start, selection_start);
                    ev.stop_immediate_propagation();
                    //ev.prevent_default()
                }
            }),
            _ => (),
        }
    };

    create_effect(cx, move |_| {
        modification.with_value(|modification| {
            if let Some(Modification::ReactivePrefix(reactive)) = modification {
                reactive.with(|_| {
                    if let Some(_) = node_ref.get() {
                        update_value.update_value(|update_value| update_value())
                    }
                })
            }
        })
    });

    view! {
        cx,

        <input
            class=input_class
            node_ref=node_ref
            on:keydown=key_movement
            on:input = move |_| update_value.update_value(|update_value| update_value())
            placeholder=placeholder.unwrap_or_default()
            value=initial_value
            type="text"
        />


    }
    // ω <fn numeric_input>
}

////////////////////////////////////////////////////////////////////////////////////
// --- type impls ---
////////////////////////////////////////////////////////////////////////////////////
impl Modification {
    /// Returns either input position or if input position falls
    /// within the prefix/suffix then the position just after/before the prefix or suffix.
    /// This function can be used to ensure the caret does not move into the prefix/suffix.
    ///
    ///   * **input_len** - Length of input
    ///   * **position** - Position to ensure is constrained to numeric part of input.
    ///   * _return_ - A position within the numeric part of the input - i.e. not in the prefix/suffix.
    pub fn position_in_number(&self, input_len: usize, position: usize) -> usize {
        // α <fn Modification::position_in_number>
        debug_assert!(position <= input_len);
        let constrained = match &self {
            Modification::ReactivePrefix(p) => p.with(|p| p.len().max(position)),
            Modification::Prefix(p) => p.len().max(position),
            Modification::Suffix(s) => (input_len - s.len()).min(position),
        };
        console_log(&format!("Constrained {position} to {constrained}"));
        constrained
        // ω <fn Modification::position_in_number>
    }

    /// Attaches the modification (prefix/suffix) to the input
    ///
    ///   * **input** - Input to modify with prefix/suffix.
    ///   * _return_ - Modified input
    pub fn modify(&self, input: &str) -> String {
        // α <fn Modification::modify>
        let mut modified = input.to_string();

        leptos_dom::console_log(&format!("modifying {input}"));
        let result = match &self {
            Modification::ReactivePrefix(p) => p.with(|p| {
                debug_assert!(!modified.contains(p));
                modified.insert_str(0, p);
                modified
            }),
            Modification::Prefix(p) => {
                debug_assert!(!modified.contains(p));
                modified.insert_str(0, p);
                modified
            }
            Modification::Suffix(s) => {
                debug_assert!(!modified.contains(s));
                modified.push_str(s);
                modified
            }
        };
        result
        // ω <fn Modification::modify>
    }
}

/// Unit tests for `numeric_input`
#[cfg(test)]
pub mod unit_tests {

    /// Test type Modification
    mod test_modification {
        ////////////////////////////////////////////////////////////////////////////////////
        // --- module uses ---
        ////////////////////////////////////////////////////////////////////////////////////
        use test_log::test;

        ////////////////////////////////////////////////////////////////////////////////////
        // --- functions ---
        ////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn position_in_number() {
            // α <fn test Modification::position_in_number>
            todo!("Test position_in_number")
            // ω <fn test Modification::position_in_number>
        }

        #[test]
        fn modify() {
            // α <fn test Modification::modify>
            todo!("Test modify")
            // ω <fn test Modification::modify>
        }

        // α <mod-def test_modification>
        // ω <mod-def test_modification>
    }

    // α <mod-def unit_tests>
    // ω <mod-def unit_tests>
}

// α <mod-def numeric_input>
// ω <mod-def numeric_input>
