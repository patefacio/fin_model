//! Module for normal_spec_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::component::numeric_input::{Modification, NumericInput};
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::core::NormalSpec;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a normal specification -> N(mu, sigma).
///
///   * **cx** - Context
///   * **updatable** - The normal spec being edited
///   * _return_ - View for normal_spec_component
#[component]
pub fn NormalSpecComponent(
    /// Context
    cx: Scope,
    /// The normal spec being edited
    updatable: Updatable<Option<NormalSpec>>,
) -> impl IntoView {
    // α <fn normal_spec_component>

    use std::cell::RefCell;
    use std::rc::Rc;

    let updatable = Rc::new(RefCell::new(updatable));

    let initial_mean = updatable
        .as_ref()
        .borrow()
        .value
        .as_ref()
        .map(|option_of_normal_spec| option_of_normal_spec.mean);

    let initial_std_dev = updatable
        .as_ref()
        .borrow()
        .value
        .as_ref()
        .map(|normal_spec| normal_spec.std_dev);

    let updatable_for_mean = Rc::clone(&updatable);
    let mean_updatable = Updatable::new(initial_mean, move |mean| {
        if let Some(mean) = mean.clone() {
            updatable_for_mean
                .as_ref()
                .borrow_mut()
                .update_and_then_signal(|normal_spec| {
                    let inner_value = if let Some(normal_spec) = normal_spec {
                        normal_spec.mean = mean;
                        *normal_spec
                    } else {
                        NormalSpec { mean, std_dev: 0.0 }
                    };

                    *normal_spec = Some(inner_value)
                });
        }
    });

    let std_dev_updatable = Updatable::new(initial_std_dev, move |std_dev| {
        if let Some(std_dev) = std_dev.clone() {
            updatable
                .as_ref()
                .borrow_mut()
                .update_and_then_signal(|normal_spec| {
                    let inner_value = if let Some(normal_spec) = normal_spec {
                        normal_spec.std_dev = std_dev;
                        *normal_spec
                    } else {
                        NormalSpec { mean: 0.0, std_dev }
                    };

                    *normal_spec = Some(inner_value)
                });
        }
    });

    view! {
    cx,

        <fieldset class="nsg">
        <legend>"Normal Growth"</legend>
        <div class="form">


            <div style="display: inline-flex" >

                "N("
                    <NumericInput
                        placeholder=Some("mean".to_string())
                        modification=Some(Modification::PrefixAndSuffix{
                            prefix: "μ=".into(),
                            suffix: "%".into()
                        })
                        non_negative=true
                        updatable=mean_updatable
                    />
                ", "
                    <NumericInput
                        placeholder=Some("std. dev".to_string())
                        modification=Some(Modification::PrefixAndSuffix{
                            prefix: "σ=".into(),
                            suffix: "%".into()
                        })
                        non_negative=true
                        updatable=std_dev_updatable
                    />
                ")"
            </div>

        </div>
        </fieldset>
    }

    // ω <fn normal_spec_component>
}

// α <mod-def normal_spec_component>

// ω <mod-def normal_spec_component>
