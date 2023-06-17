//! Module for item_growth_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
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

    use crate::EnumSelect;
    use crate::GrowthAssumptionComponent;

    let category_select = match dossier_item_type {
        DossierItemType::Holding => view! { cx,
            <EnumSelect
                updatable = Updatable::new(HoldingType::UsEquityMarket, |e| {
                    console_log("Holding selection updated -> {e:?}");
                })
        />
        }.into_view(cx),
        DossierItemType::Flow => view! { cx, 
            <EnumSelect
                updatable = Updatable::new(FlowType::CollegeExpense, |e| {
                    console_log("Flow selection updated -> {e:?}");
                })
        />
        }.into_view(cx),
        DossierItemType::Worth => view! { cx, 
            <EnumSelect
                updatable = Updatable::new(WorthType::FamilyFarm, |e| {
                    console_log("Worth selection updated -> {e:?}");
                })
            />
        }.into_view(cx),
        DossierItemType::Instrument => view! { cx, 
            <EnumSelect
                updatable = Updatable::new(FlowType::CollegeExpense, |e| {
                    console_log("Flow selection updated -> {e:?}");
                })
            />
        }.into_view(cx)
    };

    let growth_assumption = updatable.value.growth_assumption.unwrap_or_default();

    view! {
        cx,

        <fieldset class="igc">
        <legend>"Item Growth"</legend>

        <div>
        {category_select}
        </div>

        <GrowthAssumptionComponent
            updatable=Updatable::new(
                growth_assumption,
                |ga| {
                    console_log(&format!("growth_assumption updated -> {ga:?}"));
                }
            )
        />
        </fieldset>
    }

    // ω <fn item_growth_component>
}

// α <mod-def item_growth_component>
// ω <mod-def item_growth_component>
