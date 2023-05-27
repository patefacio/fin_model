//! TODO: Document Module(normal_spec_growth)

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::component::numeric_input::{Modification, NumericInput};
use crate::utils::updatable::Updatable;
use fin_model::core::NormalSpec;
use leptos::{component, tracing, view, IntoView, Scope};
use std::borrow::BorrowMut;
use std::rc::Rc;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a normal distribution -> N(mu, sigma).
///
///   * **cx** - Context
///   * **updatable** - Value and callback
///   * _return_ - View for normal_spec_growth
#[component]
pub fn NormalSpecGrowth<F>(
    /// Context
    cx: Scope,
    /// Value and callback
    updatable: Updatable<NormalSpec, F>,
) -> impl IntoView
where
    F: Fn(&NormalSpec) + 'static,
{
    // α <fn normal_spec_growth>

    let updatable = Rc::new(updatable);
    let on_mean_updated = move |mean| {
        // TODO update updateable

        let current = updatable.value.borrow();
        let mut new_value = current.clone();
        new_value.mean = mean;
        updatable.update(new_value);
        // let mut new_value = updatable.value.clone();
        // new_value.mean = mean;
    };

    let on_std_dev_updated = move |std_dev| {
        // TODO update updatable
    };

    view! {
        cx,

        <fieldset class="nsg">
        <legend>"Normal Growth"</legend>
        <div class="form">

        <div>
            <label for="mean">"Mean (μ)"</label>
            <NumericInput
                placeholder=Some("e.g. 10.00%".into())
                on_update=on_mean_updated
                modification=Some(Modification::Suffix("%".into()))
                non_negative=true
            />
        </div>

        <div>
            <label for="std-dev">"Std. Dev. (σ)"</label>
            <NumericInput
                placeholder=Some("e.g. 20.00%".into())
                on_update=on_std_dev_updated
                modification=Some(Modification::Suffix("%".into()))
                non_negative=true
            />
        </div>
        </div>
        </fieldset>
    }
    // ω <fn normal_spec_growth>
}

// α <mod-def normal_spec_growth>
// ω <mod-def normal_spec_growth>
