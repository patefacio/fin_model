//! Module for item_growth_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::FlowType;
use plus_modeled::HoldingType;
use plus_modeled::ItemGrowth;

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// Discriminates on the set of {`Worth`, `Holding`, `Flow`}
pub enum DossierItemType {
    /// Indicates categories appropriate for worths.
    WorthItem,
    /// Indicates categories appropriate for holdings.
    HoldingItem,
    /// Indicates categories appropriate for flow specs.
    FlowItem,
}

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
///   * **dossier_item_type** - Indicates the categories to display for selection
///   * _return_ - View for item_growth_component
#[component]
pub fn ItemGrowthComponent(
    /// Context
    cx: Scope,
    /// The current value of the system id
    updatable: Updatable<Option<ItemGrowth>>,
    /// Indicates the categories to display for selection
    dossier_item_type: DossierItemType,
) -> impl IntoView {
    // α <fn item_growth_component>
    todo!("Implement `item_growth_component`")
    // ω <fn item_growth_component>
}

// α <mod-def item_growth_component>
// ω <mod-def item_growth_component>
