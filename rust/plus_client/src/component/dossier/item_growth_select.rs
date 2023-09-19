//! Module for item_growth_select leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::SelectDirection;
use crate::Updatable;
use leptos::ReadSignal;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;
use plus_modeled::FlowDirection;
use plus_modeled::NormalSpec;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::hash::Hash;
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
///   * **updatable** - The enum being selected (eg [HoldingType](plus_modeled::HoldingType),
/// [WorthType](plus_modeled::WorthType), etc).
///   * **growth_mapping** - Mapping of enum to its growth.
///   * **column_count** - Number of columns to display in the grid of options.
///   * **direction** - Specifies whether items flows from top to bottom or left to right.
///   * **flow_filter** - Filter on enum variants
///   * **label** - Convert enum type to label
///   * _return_ - View for item_growth_select
#[component]
pub fn ItemGrowthSelect<E>(
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
    /// Filter on enum variants
    #[prop(default=None)]
    flow_filter: Option<(ReadSignal<FlowDirection>, Box<dyn Fn(&E) -> bool>)>,
    /// Convert enum type to label
    label: Box<dyn Fn(&E) -> String>,
) -> impl IntoView
where
    E: Clone + Debug + VariantNames + IntoEnumIterator + PartialEq + Eq + Hash + 'static,
{
    // α <fn item_growth_select>

    use crate::{InitialValue, MultiColumnSelect, SelectOption};
    use leptos::store_value;
    use leptos::SignalWith;
    use leptos::StoredValue;
    use std::collections::HashMap;

    fn to_key_label(key: &str) -> SelectOption {
        SelectOption::KeyLabel {
            key: key.to_string(),
            label: NormalSpec {
                mean: 0.1,
                std_dev: 0.2,
            }
            .to_string(),
        }
    }

    struct IGSData<E> {
        updatable: Updatable<E>,
        label_to_value: HashMap<String, E>,
        value_to_label: HashMap<E, String>,
    }

    let filter_signal = flow_filter.as_ref().map(|flow_filter| flow_filter.0);
    let (label_to_value, value_to_label): (Vec<_>, Vec<_>) = E::iter()
        .enumerate()
        .map(|(i, e)| {
            let label = label(&e);
            ((label.clone(), e.clone()), (e.clone(), label))
        })
        .unzip();

    let igs_data_stored_value = store_value(IGSData {
        updatable,
        label_to_value: label_to_value.into_iter().collect(),
        value_to_label: value_to_label.into_iter().collect(),
    });

    let mcs_select = move || {
        if let Some(filter_change) = filter_signal {
            filter_change.track();
        }

        let mut initial_value = None;

        let options = {
            let options = igs_data_stored_value.with_value(|igs_data| {
                let mut current_index = 0;
                E::iter()
                    .enumerate()
                    .filter_map(|(i, e)| {
                        let included = flow_filter
                            .as_ref()
                            .map(|(_flow_direction, filter)| filter(&e))
                            .unwrap_or(true);

                        if included {
                            if e == igs_data.updatable.value {
                                initial_value = Some(InitialValue::SelectionIndex(current_index));
                            }
                            current_index += 1;
                            Some(to_key_label(&igs_data.value_to_label.get(&e).unwrap()))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            });

            options
        };

        let menu_select = move |label: String| {
            igs_data_stored_value.update_value(|igs_data| {
                igs_data.updatable.update_and_then_signal(|selection| {
                    *selection = igs_data.label_to_value.get(&label).cloned().unwrap();
                })
            });
        };

        view! {
            <MultiColumnSelect
                options=options
                column_count=column_count
                initial_value=initial_value
                on_select=menu_select
                direction=direction
            />
        }
    };

    view! {
        <div class="ig">
            {move || mcs_select()} <div>
                <div class="ig-growth-label">
                    <div class="info-label">
                        {move || { format!("{}", NormalSpec { mean : 0.1, std_dev : 0.2 }) }}

                    </div>
                </div>
            </div>
        </div>
    }

    // ω <fn item_growth_select>
}

// α <mod-def item_growth_select>
// ω <mod-def item_growth_select>
