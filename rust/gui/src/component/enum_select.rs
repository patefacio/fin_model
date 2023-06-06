//! Module for enum_select leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::utils::updatable::Updatable;
use leptos::{component, tracing, view, IntoView, Scope};
use std::fmt::Debug;
use strum::{IntoEnumIterator, VariantNames};

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a select drop-down from simple enums as modeled by `protobof` or similar.
///
///   * **cx** - Context
///   * **updatable** - Value of enum that will be changed on selection.
///   * _return_ - View for enum_select
#[component]
pub fn EnumSelect<E>(
    /// Context
    cx: Scope,
    /// Value of enum that will be changed on selection.
    updatable: Updatable<E>,
) -> impl IntoView
where
    E: Debug + VariantNames + IntoEnumIterator + 'static,
{
    // α <fn enum_select>

    use crate::component::multi_column_select::{InitialValue, MultiColumnSelect, SelectOption};

    let updatable = leptos::store_value(cx, updatable);

    let options: Vec<_> = E::VARIANTS
        .iter()
        .map(|variant| SelectOption::Label(variant.to_string()))
        .collect();

    let menu_select = move |value: String| {
        updatable.update_value(|updatable| {
            updatable.update(|selection| {
                for (i, e) in E::iter().enumerate() {
                    if E::VARIANTS[i] == value {
                        *selection = e;
                        break;
                    }
                }
            })
        });
    };

    view! {
        cx,
        <MultiColumnSelect
            options=options
            initial_value=Some(InitialValue::SelectionIndex(3))
            on_select=menu_select
        />
    }

    // ω <fn enum_select>
}

// α <mod-def enum_select>
// ω <mod-def enum_select>
