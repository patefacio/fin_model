//! Module for item_growth_select leptos function/component

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
use plus_modeled::NormalSpec;
use std::collections::BTreeMap;
use std::fmt::Debug;
use strum::{IntoEnumIterator, VariantNames};

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Special select that supports selecting a growth item that includes
/// in the select option text the current growth distribution of the
/// items. For example, if showing `UsEquityMarket` it would show something
/// like `UsEquityMarket N(mu=10%, simga=20%)`. This makes the display
/// larger but provides a way for the user to see the growth impact
/// of the selection
///
///   * **cx** - Context
///   * **updatable** - The enum being selected (eg [HoldingType](plus_modeled::HoldingType),
/// [WorthType](plus_modeled::WorthType), etc).
///   * **growth_mapping** - Mapping of enum to its growth.
///   * **column_count** - Number of columns to display in the grid of options.
///   * **direction** - Specifies whether items flows from top to bottom or left to right.
///   * _return_ - View for item_growth_select
#[component]
pub fn ItemGrowthSelect<E>(
    /// Context
    cx: Scope,
    /// The enum being selected (eg [HoldingType](plus_modeled::HoldingType),
    /// [WorthType](plus_modeled::WorthType), etc).
    updatable: Updatable<E>,
    /// Mapping of enum to its growth.
    growth_mapping: BTreeMap<E, NormalSpec>,
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
    // α <fn item_growth_select>

    use crate::{InitialValue, MultiColumnSelect, SelectOption};

    // Iterate over enum variants to find the index of the initial value
    let (initial_index, _) = E::iter()
        .enumerate()
        .find(|(_, variant)| *variant == updatable.value)
        .unwrap();

    let stored_updatable = leptos::store_value(cx, updatable);

    let options: Vec<_> = E::VARIANTS
        .iter()
        .map(|variant| SelectOption::KeyLabel {
            key: variant.to_string(),
            label: NormalSpec {
                mean: 0.1,
                std_dev: 0.2,
            }
            .to_string(),
        })
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
        <div class="ig">
            <MultiColumnSelect
                options=options
                column_count=column_count
                initial_value=Some(InitialValue::SelectionIndex(initial_index))
                on_select=menu_select
                direction=direction
            />
            <div>
                <div class="ig-growth-label">
                    <div class="info-label">
                        {move || {
                            format!("Growth: {}", NormalSpec { mean : 0.1, std_dev : 0.2 })
                        }}

                    </div>
                </div>
            </div>
        </div>
    }

    // ω <fn item_growth_select>
}

// α <mod-def item_growth_select>
// ω <mod-def item_growth_select>
