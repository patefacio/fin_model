//! Module for item_growth_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::ReadSignal;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::DossierItemType;
use plus_modeled::FlowDirection;
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
///   * **updatable** - The current value of the system id
///   * **dossier_item_type** - Indicates the categories to display for selection (e.g. Worth, Holding, ...)
///   * **growth_item_mappings** - Provides mapping of assumptions that the categories default to.
/// This map provides the component the ability to display for
/// reference the current associated growth, _assuming `standard`
/// outlook.
///   * **flow_direction** - Specialize the types of flows by direction for the case of DossierItemType::Flow.
///   * _return_ - View for item_growth_component
#[component]
pub fn ItemGrowthComponent<'a>(
    /// The current value of the system id
    updatable: Updatable<ItemGrowth>,
    /// Indicates the categories to display for selection (e.g. Worth, Holding, ...)
    dossier_item_type: DossierItemType,
    /// Provides mapping of assumptions that the categories default to.
    /// This map provides the component the ability to display for
    /// reference the current associated growth, _assuming `standard`
    /// outlook.
    growth_item_mappings: &'a GrowthItemMappings,
    /// Specialize the types of flows by direction for the case of DossierItemType::Flow.
    #[prop(default=None)]
    flow_direction: Option<ReadSignal<FlowDirection>>,
) -> impl IntoView {
    // α <fn item_growth_component>

    use crate::AppContext;
    use crate::CollapsibleComponent;
    use crate::GrowthAssumptionComponent;
    use crate::ItemGrowthSelect;
    use leptos::store_value;
    use leptos::use_context;
    use leptos::SignalGet;
    use plus_lookup::I18nEnums;
    use plus_modeled::get_flow_direction;
    use plus_modeled::ItemGrowth;
    use plus_modeled::LangSelector;
    use plus_modeled::SystemGrowthId;
    use plus_modeled::SystemId;
    use std::collections::BTreeMap;

    let initial_system_id = updatable
        .value
        .system_growth_id
        .as_ref()
        .and_then(|system_growth_id| {
            system_growth_id.system_id.map(|system_id| match system_id {
                SystemId::HoldingItemId(i) => i,
                SystemId::InstrumentId(i) => i,
                SystemId::WorthItemId(i) => i,
                SystemId::FlowItemId(i) => i,
            })
        })
        .unwrap_or_default() as i32;

    let updatable_store_value = store_value(updatable);

    let set_system_id = move |id: u32| {
        let new_id = match dossier_item_type {
            DossierItemType::Holding => SystemId::HoldingItemId(id),
            DossierItemType::Flow => SystemId::FlowItemId(id),
            DossierItemType::Worth => SystemId::WorthItemId(id),
            DossierItemType::Instrument => SystemId::InstrumentId(id),
        };

        updatable_store_value.update_value(|updatable| {
            updatable.update_and_then_signal(|item_growth| {
                if let Some(system_growth_id) = item_growth.system_growth_id.as_mut() {
                    system_growth_id.system_id = Some(new_id);
                } else {
                    *item_growth = ItemGrowth {
                        system_growth_id: Some(SystemGrowthId {
                            system_id: Some(new_id),
                        }),
                        ..Default::default()
                    }
                }
            })
        })
    };

    let flow_type_filter = flow_direction.map(|flow_direction| {
        let flow_type_filter: Box<dyn Fn(&FlowType) -> bool> =
            Box::new(move |flow_type: &FlowType| {
                let desired_direction = flow_direction.get();
                let flow_direction = get_flow_direction(flow_type);
                flow_direction == desired_direction
            });

        (flow_direction, flow_type_filter)
    });

    let column_count = 2;
    let lang_selector = use_context::<AppContext>().unwrap().lang_selector;

    let category_select = match dossier_item_type {
        DossierItemType::Holding => view! {
            <ItemGrowthSelect
                updatable=Updatable::new(
                    HoldingType::from_i32(initial_system_id).unwrap_or_default(),
                    move |e| { set_system_id(*e as u32) },
                )

                growth_mapping=BTreeMap::new()
                column_count=column_count
                label=Box::new(move |e| I18nEnums::HoldingType(lang_selector.get(), e).to_string())
            />
        }
        .into_view(),
        DossierItemType::Flow => view! {
            <ItemGrowthSelect
                updatable=Updatable::new(
                    FlowType::from_i32(initial_system_id).unwrap_or_default(),
                    move |e| { set_system_id(*e as u32) },
                )

                growth_mapping=BTreeMap::new()
                column_count=column_count
                flow_filter=flow_type_filter
                label=Box::new(move |e| I18nEnums::FlowType(lang_selector.get(), e).to_string())
            />
        }
        .into_view(),
        DossierItemType::Worth => view! {
            <ItemGrowthSelect
                updatable=Updatable::new(
                    WorthType::from_i32(initial_system_id).unwrap_or_default(),
                    move |e| { set_system_id(*e as u32) },
                )

                growth_mapping=BTreeMap::new()
                column_count=column_count
                label=Box::new(move |e| I18nEnums::WorthType(lang_selector.get(), e).to_string())
            />
        }
        .into_view(),
        DossierItemType::Instrument => view! {
            <ItemGrowthSelect
                updatable=Updatable::new(
                    FlowType::from_i32(initial_system_id).unwrap_or_default(),
                    move |e| { set_system_id(*e as u32) },
                )

                growth_mapping=BTreeMap::new()
                column_count=column_count
                label=Box::new(move |e| I18nEnums::FlowType(lang_selector.get(), e).to_string())
            />
        }
        .into_view(),
    };

    let growth_assumption_updatable = move || {
        Updatable::new(
            updatable_store_value.with_value(|updatable| {
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

    view! {
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
