//! Module for bond_spec_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::DateInput;
use crate::NumericInput;
use crate::RateCurveComponent;
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::BondSpec;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models bond specification
///
///   * **cx** - Context
///   * **updatable** - The [BondSpec] being edited.
///   * _return_ - View for bond_spec_component
#[component]
pub fn BondSpecComponent(
    /// Context
    cx: Scope,
    /// The [BondSpec] being edited.
    updatable: Updatable<Option<BondSpec>>,
) -> impl IntoView {
    // α <fn bond_spec_component>

    let i = updatable.value.unwrap();
    
    view! { cx,
        <h2>"BOND SPEC"</h2>
        <h3>"Face Value"</h3>
        <NumericInput updatable=Updatable::new(
            Some(i.face_value),
            move |rc| {
                log!("Rate Curve -> {rc:?}");
            },
        )/>
        <h3>"Annual Coupon"</h3>
        <RateCurveComponent updatable=Updatable::new(
            i.annual_coupon.unwrap_or_default(),
            move |rc| {
                log!("Rate Curve -> {rc:?}");
            },
        )/>
        <h3>"Maturity Date"</h3>
        <DateInput updatable=Updatable::new(
            i.maturity,
            move |rc| {
                log!("Date -> {rc:?}");
            },
        )/>
    }

    // ω <fn bond_spec_component>
}

// α <mod-def bond_spec_component>
// ω <mod-def bond_spec_component>
