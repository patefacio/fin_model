//! Module for person_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::Person;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a person's details
///
///   * **cx** - Context
///   * **updatable** - The [Person] being edited
///   * _return_ - View for person_component
#[component]
pub fn PersonComponent(
    /// Context
    cx: Scope,
    /// The [Person] being edited
    updatable: Updatable<Option<Person>>,
) -> impl IntoView {
    // α <fn person_component>
    todo!("Implement `person_component`")
    // ω <fn person_component>
}

// α <mod-def person_component>
// ω <mod-def person_component>
