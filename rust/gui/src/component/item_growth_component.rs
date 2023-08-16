//! Module for item_growth_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::DossierItemType;
use plus_modeled::FlowType;
use plus_modeled::GrowthItemMappings;
use plus_modeled::HoldingType;
use plus_modeled::ItemGrowth;
use plus_modeled::WorthType;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// A component for specifying growth of a specific item like,
/// [Worth](plus_modeled::Worth), [Holding](plus_modeled::Holding) (i.e. a financial instrument),
/// [FlowSpec](plus_modeled::FlowSpec) payment, etc.
///
/// Ideally all, or the vast majority, of growth entries for a user's
/// model will come from the named system categories. There are categories associated with
/// worths, holdings, and flow specs and are displayed in a [MultiColumnSelect](crate::MultiColumnSelect) which
/// the user *must choose* the type most appropriate for the item. The _dossier_item_type_ argument is used
/// to specify which collection of categories to select from.
///
/// In terms of flexibility we want to provide the user the ability to specify their own growth,
/// (i.e. override the system selection).
///
/// Additionally they will be able be able to specify it in the form of a
/// Normal(mean, std_dev) or a Rate Curve (i.e. _pinned curve_).
/// This component provides the selection of the system categorization and uses
/// the [GrowthAssumptionComponent](crate::GrowthAssumptionComponent) to collect the overrides.
///
///   * **cx** - Context
///   * **updatable** - The current value of the system id
///   * **dossier_item_type** - Indicates the categories to display for selection (e.g. Worth, Holding, ...)
///   * **growth_item_mappings** - Provides mapping of assumptions that the categories default to.
/// This map provides the component the ability to display for
/// reference the current associated growth, _assuming `standard`
/// outlook.
///   * _return_ - View for item_growth_component
#[component]
pub fn ItemGrowthComponent<'a>(
    /// Context
    cx: Scope,
    /// The current value of the system id
    updatable: Updatable<ItemGrowth>,
    /// Indicates the categories to display for selection (e.g. Worth, Holding, ...)
    dossier_item_type: DossierItemType,
    /// Provides mapping of assumptions that the categories default to.
    /// This map provides the component the ability to display for
    /// reference the current associated growth, _assuming `standard`
    /// outlook.
    growth_item_mappings: &'a GrowthItemMappings,
) -> impl IntoView {
    // α <fn item_growth_component>

    use crate::CollapsibleComponent;
    use crate::GrowthAssumptionComponent;
    use crate::ItemGrowthSelect;
    use leptos::store_value;
    use std::collections::BTreeMap;

    let column_count = 2;

    let category_select = match dossier_item_type {
        DossierItemType::Holding => view! { cx,
            <ItemGrowthSelect
                updatable=Updatable::new(
                    HoldingType::UsEquityMarket,
                    |e| {
                        log!("Holding selection updated -> {e:?}");
                        tracing::debug!("Holding selection updated -> {e:?}");
                    },
                )

                growth_mapping=BTreeMap::new()
                column_count=column_count
            />
        }
        .into_view(cx),
        DossierItemType::Flow => view! { cx,
            <ItemGrowthSelect
                updatable=Updatable::new(
                    FlowType::CollegeExpense,
                    |e| {
                        log!("Flow selection updated -> {e:?}");
                    },
                )

                growth_mapping=BTreeMap::new()
                column_count=column_count
            />
        }
        .into_view(cx),
        DossierItemType::Worth => view! { cx,
            <ItemGrowthSelect
                updatable=Updatable::new(
                    WorthType::FamilyFarm,
                    |e| {
                        log!("Worth selection updated -> {e:?}");
                    },
                )

                growth_mapping=BTreeMap::new()
                column_count=column_count
            />
        }
        .into_view(cx),
        DossierItemType::Instrument => view! { cx,
            <ItemGrowthSelect
                updatable=Updatable::new(
                    FlowType::CollegeExpense,
                    |e| {
                        log!("Flow selection updated -> {e:?}");
                    },
                )

                growth_mapping=BTreeMap::new()
                column_count=column_count
            />
        }
        .into_view(cx),
    };

    let updatable_value = store_value(cx, updatable);

    let growth_assumption_updatable = move || {
        Updatable::new(
            updatable_value.with_value(|updatable| {
                updatable
                    .value
                    .growth_assumption
                    .as_ref()
                    .cloned()
                    .unwrap_or_default()
            }),
            |ga| {
                log!("growth_assumption updated -> {ga:?}");
            },
        )
    };

    view! { cx,
        <div>
            <div class="icg-select">{category_select}</div>
            <CollapsibleComponent
                collapsed_header="Override System Growth".to_string()
                expanded_header=Some("Use System Growth".to_string())
                is_expanded=false
            >
                <GrowthAssumptionComponent updatable=growth_assumption_updatable()/>
            </CollapsibleComponent>
        </div>
    }

    // ω <fn item_growth_component>
}

// α <mod-def item_growth_component>
// ω <mod-def item_growth_component>
