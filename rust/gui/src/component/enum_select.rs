//! Module for enum_select leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::SelectDirection;
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
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
///   * **cx** - Context
///   * **updatable** - Value of enum that will be changed on selection.
///   * **column_count** - Number of columns to display in the grid of options.
///   * **direction** - Specifies whether items flows from top to bottom or left to right.
///   * _return_ - View for enum_select
#[component]
pub fn EnumSelect<E>(
    /// Context
    cx: Scope,
    /// Value of enum that will be changed on selection.
    updatable: Updatable<E>,
    /// Number of columns to display in the grid of options.
    #[prop(default = 3)]
    column_count: usize,
    /// Specifies whether items flows from top to bottom or left to right.
    #[prop(default=SelectDirection::LeftToRight)]
    direction: SelectDirection,
) -> impl IntoView
where
    E: Debug + VariantNames + IntoEnumIterator + PartialEq + 'static,
{
    // α <fn enum_select>

    use crate::component::multi_column_select::{InitialValue, MultiColumnSelect, SelectOption};

    // Iterate over enum variants to find the index of the initial value
    let (initial_index, _) = E::iter()
        .enumerate()
        .find(|(_, variant)| *variant == updatable.value)
        .unwrap();

    let stored_updatable = leptos::store_value(cx, updatable);

    let options: Vec<_> = E::VARIANTS
        .iter()
        .map(|variant| SelectOption::Label(variant.to_string()))
        .collect();

    let menu_select = move |value: String| {
        stored_updatable.update_value(|updatable| {
            updatable.update_and_then_signal(|selection| {
                for (i, e) in E::iter().enumerate() {
                    if E::VARIANTS[i] == value {
                        *selection = e;
                        break;
                    }
                }
            })
        });
    };

    view! { cx,
        <MultiColumnSelect
            options=options
            column_count=column_count
            initial_value=Some(InitialValue::SelectionIndex(initial_index))
            on_select=menu_select
            direction=direction
        />
    }

    // ω <fn enum_select>
}

// α <mod-def enum_select>
// ω <mod-def enum_select>
