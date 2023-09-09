//! Module for distribution_policy_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::DistributionPolicy;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Component for the distributions of a security
///
///   * **updatable** - The distribution policy being edited
///   * _return_ - View for distribution_policy_component
#[component]
pub fn DistributionPolicyComponent(
    /// The distribution policy being edited
    updatable: Updatable<Option<DistributionPolicy>>,
) -> impl IntoView {
    // α <fn distribution_policy_component>

    use crate::BondSpecComponent;
    use crate::DistributionSpecComponent;
    use crate::OneOfComponent;
    use leptos::store_value;
    use plus_modeled::BondSpec;
    use plus_modeled::DistributionPolicy;
    use plus_modeled::DistributionSpec;

    #[derive(Debug, Clone, PartialEq, EnumVariantNames, EnumIter)]
    enum Policy {
        None,
        Distributions,
        Bond,
    }

    let selection = match &updatable.value {
        Some(DistributionPolicy::BondSpec(_)) => Policy::Bond,
        Some(DistributionPolicy::DistributionSpec(_)) => Policy::Distributions,
        None => Policy::None,
    };

    let label_maker = |policy: &Policy| match policy {
        Policy::None => "No Distributions".to_string(),
        Policy::Distributions => "Distributions".to_string(),
        Policy::Bond => "Bond Spec".to_string(),
    };

    let updatable_store_value = store_value(updatable);

    let distribution_spec_view = move || {
        let current_distribution_spec =
            updatable_store_value.with_value(|updatable| match updatable.value.as_ref() {
                Some(DistributionPolicy::DistributionSpec(ds)) => ds.clone(),
                _ => DistributionSpec::default(),
            });
        let distribution_spec_updatable = Updatable::new(
            Some(current_distribution_spec),
            move |new_distribution_spec| {
                updatable_store_value.update_value(|updatable| {
                    updatable.update_and_then_signal(|updater| {
                        *updater = Some(DistributionPolicy::DistributionSpec(
                            new_distribution_spec.as_ref().cloned().unwrap_or_default(),
                        ));
                    })
                })
            },
        );
        view! { <DistributionSpecComponent updatable=distribution_spec_updatable/> }.into_view()
    };

    let bond_spec_view = move || {
        let current_bond_spec =
            updatable_store_value.with_value(|updatable| match updatable.value.as_ref() {
                Some(DistributionPolicy::BondSpec(bs)) => bs.clone(),
                _ => BondSpec::default(),
            });

        let bond_spec_updatable = Updatable::new(Some(current_bond_spec), move |new_bond_spec| {
            let bs = new_bond_spec.as_ref().cloned().unwrap_or_default();
            let new_policy = Some(DistributionPolicy::BondSpec(bs));

            updatable_store_value.update_value(|updatable| {
                updatable.update_and_then_signal(|updater| {
                    *updater = new_policy.clone();
                })
            })
        });

        view! { <BondSpecComponent updatable=bond_spec_updatable/> }.into_view()
    };

    let views = move |policy: &Policy| match policy {
        Policy::Distributions => distribution_spec_view(),
        Policy::Bond => bond_spec_view(),
        Policy::None => ().into_view(),
    };

    view! {
        <OneOfComponent
            selection=selection
            name="distribution-policy".to_string()
            views=views
            labels=Some(label_maker)
        />
    }

    // ω <fn distribution_policy_component>
}

// α <mod-def distribution_policy_component>
// ω <mod-def distribution_policy_component>
