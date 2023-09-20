//! Module for enum_select leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::SelectDirection;
use crate::Updatable;
use leptos::MaybeSignal;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;
use std::boxed::Box;
use std::cmp::PartialEq;
use std::fmt::Debug;
use strum::{IntoEnumIterator, VariantNames};

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Provides a select drop-down from simple enums as modeled by `protobof` or similar.
/// Rust enums do not provide many convenient variant access methods out of the box.
/// We use `protobuf` as our modeling tool which provides for the simplest of rust
/// enums (i.e. enums where all variants are _UnitStruct_ values). Other rust packages
/// like `strum` fill in the gap by providing derive macros that give functionality
/// to iterate over the names of the variants. The _tonic_ `protobuf` generator
/// provides ability to specify additional derive macros and our enums have
/// [EnumVariantNames](strum_macros::EnumVariantNames) added so the
/// [VariantNames](strum::VariantNames) are available to the macros.
/// With this setup, any modeled enums can be provided as a list with this component.
///
///
///   * **updatable** - Value of enum that will be changed on selection.
///   * **column_count** - Number of columns to display in the grid of options.
///   * **direction** - Specifies whether items flows from top to bottom or left to right.
///   * **filter** - Filter on enum variants
///   * **label** - Convert enum type to label - for when VariantNames does not cut it
///   * **disabled** - Signal allowing the disabling of the select button.
///   * _return_ - View for enum_select
#[component]
pub fn EnumSelect<E>(
    /// Value of enum that will be changed on selection.
    updatable: Updatable<E>,
    /// Number of columns to display in the grid of options.
    #[prop(default = 3)]
    column_count: usize,
    /// Specifies whether items flows from top to bottom or left to right.
    #[prop(default=SelectDirection::LeftToRight)]
    direction: SelectDirection,
    /// Filter on enum variants
    #[prop(default=None)]
    filter: Option<Box<dyn Fn(&E) -> bool>>,
    /// Convert enum type to label - for when VariantNames does not cut it
    #[prop(default=None)]
    label: Option<Box<dyn Fn(&E) -> String>>,
    /// Signal allowing the disabling of the select button.
    #[prop(into, optional)]
    disabled: MaybeSignal<bool>,
) -> impl IntoView
where
    E: Clone + Debug + VariantNames + IntoEnumIterator + PartialEq + 'static,
{
    // α <fn enum_select>

    use crate::{InitialValue, MultiColumnSelect, SelectOption};

    let filter_set: Vec<_> = E::iter()
        .enumerate()
        .filter_map(|(i, e)| {
            let included = filter.as_ref().map(|filter| filter(&e)).unwrap_or(true);
            if included {
                Some((
                    e.clone(),
                    if let Some(label) = label.as_ref() {
                        label(&e)
                    } else {
                        E::VARIANTS[i].to_string()
                    },
                ))
            } else {
                None
            }
        })
        .collect();

    // Iterate over enum variants to find the index of the initial value
    let initial_index = filter_set
        .iter()
        .position(|(e, _label)| *e == updatable.value)
        .unwrap_or_default();

    let options: Vec<_> = filter_set
        .iter()
        .map(|(_e, label)| SelectOption::Label(label.to_string()))
        .collect();

    struct ESData<E> {
        updatable: Updatable<E>,
        filter_set: Vec<(E, String)>,
    }

    let es_data_stored_value = leptos::store_value(ESData {
        updatable,
        filter_set,
    });

    let menu_select = move |value: String| {
        es_data_stored_value.update_value(|es_data| {
            es_data.updatable.update_and_then_signal(|selection| {
                for (e, label) in es_data.filter_set.iter() {
                    if *label == value {
                        *selection = e.clone();
                        break;
                    }
                }
            })
        });
    };

    view! {
        <MultiColumnSelect
            options=options
            column_count=column_count
            initial_value=Some(InitialValue::SelectionIndex(initial_index))
            on_select=menu_select
            direction=direction
            disabled=disabled
        />
    }

    // ω <fn enum_select>
}

// α <mod-def enum_select>
// ω <mod-def enum_select>
