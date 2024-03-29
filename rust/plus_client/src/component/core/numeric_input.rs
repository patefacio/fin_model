//! Module for numeric_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::utils::constants::{
    BACKSPACE_KEY, DELETE_KEY, DOWN_KEY, ENTER_KEY, LEFT_KEY, RIGHT_KEY, UP_KEY,
};
use crate::utils::numeric_text::{digit_position, format_number_lenient};
use crate::Updatable;
use leptos::component;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::MaybeSignal;
use leptos::{create_effect, create_node_ref, create_signal, store_value, ReadSignal, SignalWith};
use leptos_dom::html::Input;
use std::ops::RangeInclusive;
use web_sys::KeyboardEvent;

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a prefix or suffix modification for a numeric input.
/// Non-reactive strings are supported for both prefix and suffix.
/// Reactive prefixes are supported for cases where like
/// currency which may need reactivity. The modification text appears
/// in the html input of the component.
#[derive(Debug, Clone)]
pub enum Modification {
    /// A prefix for the number.
    Prefix(MaybeSignal<String>),
    /// A suffix for the number.
    Suffix(String),
    /// A prefix and suffix for the number.
    PrefixAndSuffix {
        /// Prefix for the number
        prefix: String,
        /// Suffix for the number
        suffix: String,
    },
}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// A component for accepting numeric input.
/// Only valid numeric characters (ascii) are allowed. Numbers are automatically formatted.
///
///   * **updatable** - Signal updated as numeric input is updated.
///   * **input_class** - Class to decorate input element for styling
///   * **modification** - Optional modification (e.g. suffix/prefix)
///   * **non_negative** - If set, negative values are disallowed.
///   * **align_left** - If set, numeric text aligned to left.
///   * **placeholder** - Placeholder shown if entry is empty.
///   * **size** - The size attribute, which one hopes would make the size of the
/// input field roughly that number of characters. But YMMV.
///   * **max_len** - The maximum number of characters for the input.
///   * **range** - Range of valid values for input.
///   * **on_enter** - Called if user hits enter, passes current input value.
///   * **clear_input** - Signal requesting to clear the input.
///   * **no_decimal** - Indicates decimals disallowed.
///   * **disabled** - Signal allowing the disabling of the select button.
///   * **validator** - Called on update to check if value is valid.
///   * **parent_override** - A way for parent to control the value in the input.
///   * _return_ - View for numeric_input
#[component]
pub fn NumericInput(
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
    /// If set, numeric text aligned to left.
    #[prop(default = false)]
    align_left: bool,
    /// Placeholder shown if entry is empty.
    #[prop(default=MaybeSignal::Static(String::from("value")), into)]
    placeholder: MaybeSignal<String>,
    /// The size attribute, which one hopes would make the size of the
    /// input field roughly that number of characters. But YMMV.
    #[prop(default = 9)]
    size: u32,
    /// The maximum number of characters for the input.
    #[prop(default = 12)]
    max_len: u32,
    /// Range of valid values for input.
    #[prop(default=None)]
    range: Option<RangeInclusive<f64>>,
    /// Called if user hits enter, passes current input value.
    #[prop(default=None)]
    on_enter: Option<Box<dyn FnMut(String)>>,
    /// Signal requesting to clear the input.
    #[prop(default=None)]
    clear_input: Option<ReadSignal<()>>,
    /// Indicates decimals disallowed.
    #[prop(default = false)]
    no_decimal: bool,
    /// Signal allowing the disabling of the select button.
    #[prop(into, optional)]
    disabled: MaybeSignal<bool>,
    /// Called on update to check if value is valid.
    #[prop(default=None)]
    validator: Option<Box<dyn FnMut(f64) -> bool>>,
    /// A way for parent to control the value in the input.
    #[prop(default=None)]
    parent_override: Option<ReadSignal<f64>>,
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-ni";
    let component_id = crate::component_id!("`NumericInput`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn numeric_input>

    use crate::LenientFormatted;
    use leptos::IntoClass;
    use leptos::IntoStyle;
    use leptos::SignalGet;
    use leptos::SignalSet;
    use std::cell::RefCell;
    use std::rc::Rc;

    let mut is_valid = true;

    let mut validator = validator;

    let dynamic_prefix_signal = match modification {
        Some(Modification::Prefix(MaybeSignal::Dynamic(signal))) => Some(signal),
        _ => None,
    };

    let initial_value = updatable
        .value
        .as_ref()
        .map(|initial_value| {
            let initial_value_string = initial_value.to_string();
            let (initial_value, initial_value_txt) =
                match format_number_lenient(&initial_value_string, 0) {
                    LenientFormatted::Zero(new_value, _) => (0.0, new_value),
                    LenientFormatted::NonZeroValue(value, new_value, _) => (value, new_value),
                    _ => (*initial_value, initial_value_string),
                };

            let custom_valid = validator
                .as_mut()
                .map(|validator| (validator.as_mut())(initial_value))
                .unwrap_or(true);

            is_valid = custom_valid
                && range
                    .as_ref()
                    .map(|range| range.contains(&initial_value))
                    .unwrap_or(true);
            modification
                .as_ref()
                .map(|modification| modification.modify(&initial_value_txt))
                .unwrap_or_else(|| initial_value_txt)
                .chars()
                .take(max_len as usize)
                .collect::<String>()
        })
        .unwrap_or_default();

    let (is_valid_read, is_valid_write) = create_signal(is_valid);

    #[derive(Debug, Copy, Clone)]
    enum TrackedKey {
        DeleteKey,
        BackspaceKey,
        Other(u32),
    }

    struct NumericInputData {
        updatable: Updatable<Option<f64>>,
        modification: Option<Modification>,
        range: Option<RangeInclusive<f64>>,
        on_enter: Option<Rc<RefCell<Box<dyn FnMut(String)>>>>,
        validator: Option<Rc<RefCell<Box<dyn FnMut(f64) -> bool>>>>,
        tracked_key: Option<TrackedKey>,
    }

    let numeric_input_data = NumericInputData {
        updatable,
        modification,
        range,
        on_enter: on_enter.map(|on_enter| Rc::new(RefCell::new(on_enter))),
        validator: validator.map(|validator| Rc::new(RefCell::new(validator))),
        tracked_key: None,
    };

    let numeric_input_data_stored_value = store_value(numeric_input_data);
    let node_ref = create_node_ref::<Input>();

    create_effect(move |_| {
        if let Some(clear_input) = clear_input {
            clear_input.track();
            if let Some(input_ref) = node_ref.get() {
                input_ref.set_value("");
            }
        };
    });

    let update_value = move |mut value: String| {
        if let Some(input_ref) = node_ref.get_untracked() {
            numeric_input_data_stored_value.update_value(|numeric_input_data| {
                let mut selection_start = input_ref
                    .selection_start()
                    .unwrap_or_default()
                    .unwrap_or_default();

                if non_negative {
                    while let Some(neg_pos) = value.find('-') {
                        if neg_pos < selection_start as usize {
                            selection_start -= 1;
                        }
                        value.remove(neg_pos);
                    }
                }

                if no_decimal {
                    if let Some(dot_pos) = value.find('.') {
                        value.replace_range(dot_pos.., "");
                        selection_start = selection_start.min(value.len() as u32);
                    }
                }

                selection_start = selection_start.min(value.len() as u32);

                // `format_number_lenient` will return the input with all non-digit
                // characters stripped in `new_value` excluding separator (',').
                // The value passed in will likely have a prefix or suffix which
                // is now *not present* in `new_value`
                let lenient_formatted = format_number_lenient(&value, selection_start);

                if let LenientFormatted::Incomplete(_text, _position) = &lenient_formatted {
                    return;
                }

                let (value, mut new_value, numeric_to_caret) = match lenient_formatted {
                    LenientFormatted::Zero(new_value, numeric_to_caret) => {
                        (0.0, new_value, numeric_to_caret)
                    }
                    LenientFormatted::NonZeroValue(value, new_value, numeric_to_caret) => {
                        (value, new_value, numeric_to_caret)
                    }
                    _ => unreachable!("Early exited for incomplete number"),
                };

                if let Some(modification) = numeric_input_data.modification.as_ref() {
                    if !new_value.is_empty() {
                        new_value = modification.modify(&new_value);

                        // Update the input with the improved value
                        _ = input_ref.set_value(&new_value);
                        // find out where the cursor should go
                        let final_position = modification.position_in_number(
                            new_value.len(),
                            digit_position(&new_value, numeric_to_caret) as usize,
                        ) as u32;
                        _ = input_ref.set_selection_range(final_position, final_position);
                    }
                } else {
                    let final_position = if new_value == "0" {
                        1
                    } else {
                        digit_position(&new_value, numeric_to_caret)
                    };
                    input_ref.set_value(&new_value);
                    _ = input_ref.set_selection_range(final_position, final_position);
                }

                let custom_valid = numeric_input_data
                    .validator
                    .as_mut()
                    .map(|validator| (validator.borrow_mut().as_mut())(value))
                    .unwrap_or(true);

                let is_in_range = numeric_input_data
                    .range
                    .as_ref()
                    .map(move |range| range.contains(&value))
                    .unwrap_or(true);

                is_valid_write.set(custom_valid && is_in_range);

                numeric_input_data
                    .updatable
                    .update_and_then_signal(|number| *number = Some(value));
            });
        }
    };

    let update_value = leptos::store_value(update_value);

    create_effect(move |_| {
        if let Some(parent_override) = parent_override {
            parent_override.track();
            if let Some(input_ref) = node_ref.get() {
                let override_value = parent_override.get().to_string();
                input_ref.set_value(&override_value);
                update_value.with_value(|update_value| update_value(override_value));
            }
        };
    });

    let key_movement = move |ev: KeyboardEvent| {
        let key_code = ev.key_code();

        match key_code {
            DELETE_KEY => numeric_input_data_stored_value.update_value(|numeric_input_data| {
                numeric_input_data.tracked_key = Some(TrackedKey::DeleteKey);
            }),

            BACKSPACE_KEY => numeric_input_data_stored_value.update_value(|numeric_input_data| {
                numeric_input_data.tracked_key = Some(TrackedKey::BackspaceKey);
            }),
            _ => {
                numeric_input_data_stored_value.update_value(|numeric_input_data| {
                    numeric_input_data.tracked_key = Some(TrackedKey::Other(key_code))
                });
            }
        };

        match key_code {
            LEFT_KEY | RIGHT_KEY => {
                numeric_input_data_stored_value.with_value(|numeric_input_data| {
                    if let Some(modification) = numeric_input_data.modification.as_ref() {
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
                    }
                })
            }

            ENTER_KEY => {
                numeric_input_data_stored_value.update_value(|numeric_input_data| {
                    if let Some(on_enter) = numeric_input_data.on_enter.as_mut() {
                        if let Some(input_ref) = node_ref.get() {
                            (on_enter.borrow_mut().as_mut())(input_ref.value());
                        }
                    }
                });
            }

            UP_KEY | DOWN_KEY => {
                use crate::utils::numeric_text::{get_number, update_digit};

                let input_ref = node_ref.get().expect("Input node");
                let mut value = input_ref.value();
                let selection_start = input_ref
                    .selection_start()
                    .unwrap_or_default()
                    .unwrap_or_default();
                let shift_key = ev.shift_key();

                if !shift_key {
                    let updated_number =
                        numeric_input_data_stored_value.with_value(|numeric_input_data| {
                            if let Some(mut number) = if let Some(modification) =
                                numeric_input_data.modification.as_ref()
                            {
                                modification.get_number(&value)
                            } else {
                                get_number(&value)
                            } {
                                number += match key_code {
                                    UP_KEY => 1.0,
                                    DOWN_KEY => {
                                        // Ensure decrement does not go negative if non_negative
                                        if non_negative && number == 0.0 {
                                            0.0
                                        } else {
                                            -1.0
                                        }
                                    }
                                    _ => 0.0,
                                };
                                Some(number.to_string())
                            } else {
                                None
                            }
                        });

                    if let Some(updated_number) = updated_number {
                        if updated_number != value {
                            // Now that we have the new value with incremented/decremented digit,
                            // put cursor at end of number.
                            let new_start_position = value.len() as u32;
                            let _ = input_ref
                                .set_selection_range(new_start_position, new_start_position);
                            update_value.with_value(|update_value| update_value(updated_number));
                        }
                    }
                } else if selection_start > 0 {
                    let digit_index = (selection_start - 1) as usize;
                    update_digit(&mut value, matches!(key_code, UP_KEY), digit_index);
                    update_value.with_value(|update_value| update_value(value));
                }
                ev.prevent_default();
            }

            _ => (),
        }
    };

    if let Some(signal) = dynamic_prefix_signal {
        create_effect(move |_| {
            signal.track();
            if let Some(input_ref) = node_ref.get_untracked() {
                update_value.with_value(|update_value| update_value(input_ref.value()));
            }
        });
    };

    // ω <fn numeric_input>
    view! {
        <div class=SELF_CLASS>
            // α <plus-ni-view>

            <input
                class=input_class
                class:invalid=move || { !is_valid_read.get() }
                style:text-align=move || { if align_left { "left" } else { "right" } }
                node_ref=node_ref
                on:keydown=key_movement
                on:input=move |_| {
                    if let Some(input_ref) = node_ref.get_untracked() {
                        let input_value = input_ref.value();
                        let is_effectively_empty = numeric_input_data_stored_value
                            .with_value(|numeric_input_data| {
                                numeric_input_data
                                    .modification
                                    .as_ref()
                                    .map(|modification| {
                                        let modified_value = modification.modify("");
                                        modified_value == input_value
                                    })
                                    .unwrap_or_default()
                            });
                        if is_effectively_empty {
                            input_ref.set_value("");
                        } else {
                            update_value
                                .update_value(|update_value| update_value(input_ref.value()));
                        }
                    }
                }

                placeholder=placeholder
                value=initial_value
                maxlength=max_len
                size=size
                type="text"
                disabled=disabled
            />

        // ω <plus-ni-view>
        </div>
    }
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
            Modification::Prefix(maybe_signal) => {
                maybe_signal.with(|p| p.chars().count().max(position))
            }
            Modification::Suffix(s) => (input_len - s.chars().count()).min(position),
            Modification::PrefixAndSuffix { prefix, suffix } => {
                let suffix_len = suffix.chars().count();
                if input_len < suffix_len {
                    suffix_len
                } else {
                    input_len - suffix.chars().count()
                }
                .min(position)
                .max(prefix.chars().count())
            }
        };
        constrained

        // ω <fn Modification::position_in_number>
    }

    /// Gets the number from the input string.
    /// First assume prefix and/or suffix are present and gets the number positionally.
    /// If that fails, indicating input text has no or different prefix/suffixes,
    /// this pulls the first string of numbers encountered. This is problematic if
    /// prefix or suffix has numbers themselves.
    ///
    ///   * **input** - Input to get number from
    ///   * _return_ - The number in between any prefix/suffix
    #[inline]
    pub fn get_number(&self, input: &str) -> Option<f64> {
        // α <fn Modification::get_number>

        use crate::utils::numeric_text::get_number;

        let number_range = match &self {
            Modification::Prefix(maybe_signal) => maybe_signal.with(|p| {
                if input.starts_with(p) {
                    p.len()..input.len()
                } else {
                    0..input.len()
                }
            }),
            Modification::Suffix(s) => {
                if input.ends_with(s) {
                    0..(input.len() - s.len())
                } else {
                    0..input.len()
                }
            }
            Modification::PrefixAndSuffix { prefix, suffix } => {
                if input.starts_with(prefix) {
                    if input.ends_with(suffix) {
                        prefix.len()..input.len()
                    } else {
                        0..input.len()
                    }
                } else {
                    if input.ends_with(suffix) {
                        0..(input.len() - suffix.len())
                    } else {
                        0..input.len()
                    }
                }
            }
        };

        get_number(&input[number_range])

        // ω <fn Modification::get_number>
    }

    /// Attaches the modification (prefix/suffix) to the input
    ///
    ///   * **input** - Input to modify with prefix/suffix.
    ///   * _return_ - Modified input
    pub fn modify(&self, input: &str) -> String {
        // α <fn Modification::modify>
        use leptos::SignalWithUntracked;
        let mut modified = input.to_string();
        let result = match &self {
            Modification::Prefix(maybe_signal) => {
                maybe_signal.with_untracked(|p| debug_assert!(!modified.contains(p)));
                maybe_signal.with_untracked(|p| modified.insert_str(0, &p));
                modified
            }
            Modification::Suffix(s) => {
                debug_assert!(!modified.contains(s));
                modified.push_str(s);
                modified
            }
            Modification::PrefixAndSuffix { prefix, suffix } => {
                modified.insert_str(0, prefix);
                modified.push_str(suffix);
                modified
            }
        };
        result
        // ω <fn Modification::modify>
    }

    /// Returns the count of *chars* in prefix
    ///
    ///   * _return_ - Number of characters in prefix
    pub fn prefix_count(&self) -> usize {
        // α <fn Modification::prefix_count>
        use leptos::SignalWithUntracked;
        match &self {
            Modification::Prefix(maybe_signal) => {
                maybe_signal.with_untracked(|p| p.chars().count())
            }
            Modification::PrefixAndSuffix { prefix, suffix: _ } => prefix.chars().count(),
            Modification::Suffix(_) => 0,
        }
        // ω <fn Modification::prefix_count>
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

            let prefix_modification = Modification::Prefix(MaybeSignal::Static("USD:".to_string()));
            assert_eq!(5, prefix_modification.position_in_number(8, 5));
            assert_eq!(4, prefix_modification.position_in_number(8, 1));

            let suffix_modification = Modification::Suffix("%".to_string());
            assert_eq!(4, suffix_modification.position_in_number(5, 5));
            assert_eq!(3, suffix_modification.position_in_number(7, 3));

            // ω <fn test Modification::position_in_number>
        }

        #[test]
        fn get_number() {
            // α <fn test Modification::get_number>

            let prefix_suffix_modification = Modification::PrefixAndSuffix {
                prefix: "prefix:".into(),
                suffix: ":suffix".into(),
            };
            assert_eq!(
                Some(123456.0),
                prefix_suffix_modification.get_number("prefix: 123,456.0 :suffix")
            );
            assert_eq!(
                Some(123456.0),
                prefix_suffix_modification.get_number("prefix: 123,456 :suffix")
            );
            assert_eq!(
                Some(-123456.0),
                prefix_suffix_modification.get_number("prefix:-123,456.0:suffix")
            );

            let modification = Modification::Prefix(MaybeSignal::Static("$".to_string()));
            assert_eq!(Some(780000.0), modification.get_number("$780,000"));
            // ω <fn test Modification::get_number>
        }

        #[test]
        fn modify() {
            // α <fn test Modification::modify>
            let prefix_modification = Modification::Prefix(MaybeSignal::Static("$".to_string()));
            assert_eq!(
                "$123,456".to_string(),
                prefix_modification.modify("123,456")
            );
            assert_eq!("$145".to_string(), prefix_modification.modify("145"));
            assert_eq!("$123456".to_string(), prefix_modification.modify("123456"));

            let suffix_modification = Modification::Suffix("%".to_string());
            assert_eq!("3.5%".to_string(), suffix_modification.modify("3.5"));
            assert_eq!("26%".to_string(), suffix_modification.modify("26"));

            let prefix_suffix_modification = Modification::PrefixAndSuffix {
                prefix: "σ=".into(),
                suffix: "%".into(),
            };
            assert_eq!(
                "σ=3.5%".to_string(),
                prefix_suffix_modification.modify("3.5")
            );
            assert_eq!("σ=26%".to_string(), prefix_suffix_modification.modify("26"));

            // ω <fn test Modification::modify>
        }

        #[test]
        fn prefix_count() {
            // α <fn test Modification::prefix_count>

            let unicode_prefix_mod = Modification::PrefixAndSuffix {
                prefix: "σ=".into(),
                suffix: "%".into(),
            };

            match &unicode_prefix_mod {
                Modification::PrefixAndSuffix { prefix, suffix: _ } => {
                    assert!(prefix.len() > 2);
                }
                _ => unreachable!(),
            }

            assert_eq!(2, unicode_prefix_mod.prefix_count());

            // ω <fn test Modification::prefix_count>
        }

        // α <mod-def test_modification>
        use crate::Modification;
        use leptos::MaybeSignal;
        // ω <mod-def test_modification>
    }

    // α <mod-def unit_tests>

    // ω <mod-def unit_tests>
}

// α <mod-def numeric_input>
// ω <mod-def numeric_input>
