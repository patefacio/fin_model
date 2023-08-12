//! Module for numeric_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::utils::constants::{
    BACKSPACE_KEY, DELETE_KEY, DOWN_KEY, ENTER_KEY, LEFT_KEY, RIGHT_KEY, UP_KEY,
};
use crate::utils::numeric_text::{digit_position, format_number_lenient};
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::MaybeSignal;
use leptos::{component, view, IntoView, Scope};
use leptos::{create_effect, create_node_ref, create_signal, store_value, ReadSignal, SignalWith};
#[allow(unused_imports)]
use leptos_dom::console_log;
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
///   * **cx** - Context
///   * **updatable** - Signal updated as numeric input is updated.
///   * **input_class** - Class to decorate input element for styling
///   * **modification** - Optional modification (e.g. suffix/prefix)
///   * **non_negative** - If set, negative values are disallowed.
///   * **align_left** - If set, numeric text aligned to left.
///   * **placeholder** - Placeholder shown if entry is empty.
///   * **size** - The size attribute, which one hopes would make the size of the
/// input field roughly that number of characters. But YMMV.
///
///   * **max_len** - The maximum number of characters for the input.
///
///   * **range** - Range of valid values for input.
///   * **on_enter** - Called if user hits enter, passes current input value.
///   * **clear_input** - Signal requesting to clear the input.
///   * **no_decimal** - Indicates decimals disallowed.
///   * **disabled** - Signal allowing the disabling of the input.
///   * **validator** - Called on update to check if value is valid.
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
    /// If set, numeric text aligned to left.
    #[prop(default = false)]
    align_left: bool,
    /// Placeholder shown if entry is empty.
    #[prop(default=None)]
    placeholder: Option<String>,
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
    /// Signal allowing the disabling of the input.
    #[prop(default=None)]
    disabled: Option<ReadSignal<bool>>,
    /// Called on update to check if value is valid.
    #[prop(default=None)]
    validator: Option<Box<dyn FnMut(f64) -> bool>>,
) -> impl IntoView {
    // α <fn numeric_input>

    use crate::LenientFormatted;
    use leptos::IntoAttribute;
    use leptos::IntoClass;
    use leptos::IntoStyle;
    use leptos::SignalGet;
    use leptos::SignalSet;
    use std::cell::RefCell;
    use std::rc::Rc;

    let mut is_valid = true;

    let mut validator = validator;

    let initial_value = updatable
        .value
        .as_ref()
        .map(|initial_value| {
            let initial_value_string = initial_value.to_string();

            let (initial_value, mut initial_value_txt) =
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

    let (is_valid, set_is_valid) = create_signal(cx, is_valid);

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

    let numeric_input_data = store_value(cx, numeric_input_data);

    let _log_me = move |s: &str| {
        numeric_input_data.with_value(|nid| {
            log!(
                "`{s}` ({cx:?}) ->\n\tN:<{:?}>\n\tMod:<{:?}>\n\tRange:<{:?}>",
                nid.updatable.value,
                nid.modification,
                nid.range
            )
        })
    };

    let node_ref = create_node_ref::<Input>(cx);
    let component_is_initialized = move || node_ref.get().is_some();

    create_effect(cx, move |_| {
        if let Some(clear_input) = clear_input {
            clear_input.track();
            if let Some(input_ref) = node_ref.get() {
                input_ref.set_value("");
            }
        };
    });

    let update_value = move || {
        numeric_input_data.update_value(|numeric_input_data| {
            let input_ref = node_ref.get_untracked().expect("Input node");
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

            if no_decimal {
                if let Some(dot_pos) = value.find('.') {
                    value.replace_range(dot_pos.., "");
                    selection_start = selection_start.min(value.len() as u32);
                }
            }

            // `format_number_lenient` will return the input with all non-digit
            // characters stripped in `new_value` excluding separator (',').
            // The value passed in will likely have a prefix or suffix which
            // is now *not present* in `new_value`
            let lenient_formatted = format_number_lenient(&value, selection_start);

            if let LenientFormatted::Incomplete(_text, _position) = &lenient_formatted {
                log!("Incomplete number -> {lenient_formatted:?}");
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
                let final_position = digit_position(&new_value, numeric_to_caret);
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

            set_is_valid.set(custom_valid && is_in_range);

            numeric_input_data
                .updatable
                .update_and_then_signal(|number| *number = Some(value));
        });
    };

    let update_value = leptos::store_value(cx, update_value);

    let key_movement = move |ev: KeyboardEvent| {
        let key_code = ev.key_code();

        match key_code {
            DELETE_KEY => numeric_input_data.update_value(|numeric_input_data| {
                numeric_input_data.tracked_key = Some(TrackedKey::DeleteKey);
            }),

            BACKSPACE_KEY => numeric_input_data.update_value(|numeric_input_data| {
                numeric_input_data.tracked_key = Some(TrackedKey::BackspaceKey);
            }),
            _ => {
                numeric_input_data.update_value(|numeric_input_data| {
                    numeric_input_data.tracked_key = Some(TrackedKey::Other(key_code))
                });
            }
        };

        match key_code {
            LEFT_KEY | RIGHT_KEY => numeric_input_data.with_value(|numeric_input_data| {
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
            }),

            ENTER_KEY => {
                numeric_input_data.update_value(|numeric_input_data| {
                    if let Some(on_enter) = numeric_input_data.on_enter.as_mut() {
                        if let Some(input_ref) = node_ref.get() {
                            (on_enter.borrow_mut().as_mut())(input_ref.value());
                        }
                    }
                });
            }

            UP_KEY | DOWN_KEY => {
                let input_ref = node_ref.get().expect("Input node");
                let mut value = input_ref.value();
                let selection_start = input_ref
                    .selection_start()
                    .unwrap_or_default()
                    .unwrap_or_default();

                if selection_start > 0 {
                    let digit_index = (selection_start - 1) as usize;
                    if let Some(c) = value.chars().nth(digit_index) {
                        if c.is_ascii_digit() {
                            let digit = c.to_digit(10).unwrap();
                            let new_digit = char::from_digit(
                                if key_code == UP_KEY {
                                    digit + 1
                                } else {
                                    if digit == 0 {
                                        9
                                    } else {
                                        digit - 1
                                    }
                                } % 10,
                                10,
                            )
                            .unwrap();
                            value.replace_range(
                                value
                                    .char_indices()
                                    .nth(digit_index)
                                    .map(|(pos, _ch)| (pos..pos + 1))
                                    .unwrap(),
                                &new_digit.to_string(),
                            );

                            input_ref.set_value(&value);
                            let _ = input_ref.set_selection_range(selection_start, selection_start);
                            update_value.with_value(|update_value| update_value());
                        }
                    }
                }
                ev.prevent_default();
            }

            _ => (),
        }
    };

    create_effect(cx, move |_| {
        let mut should_update = false;
        numeric_input_data.with_value(|numeric_input_data| {
            if let Some(Modification::Prefix(MaybeSignal::Dynamic(_))) =
                numeric_input_data.modification.as_ref()
            {
                should_update = true;
            }
        });

        if component_is_initialized() && should_update {
            update_value.with_value(|update_value| update_value());
        }
    });

    view! { cx,
        <input
            class=input_class
            class:invalid=move || { !is_valid.get() }
            style:text-align=move || { if align_left { "left" } else { "right" } }
            node_ref=node_ref
            on:keydown=key_movement
            on:input=move |_| {
                if let Some(input_ref) = node_ref.get_untracked() {
                    let input_value = input_ref.value();
                    let is_effectively_empty = numeric_input_data
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
                        update_value.update_value(|update_value| update_value());
                    }
                }
            }

            placeholder=placeholder.unwrap_or_default()
            value=initial_value
            maxlength=max_len
            size=size
            type="text"
            disabled=move || disabled.map(|disabled| disabled.get()).unwrap_or_default()
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
