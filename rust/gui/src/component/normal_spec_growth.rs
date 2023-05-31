//! Module for normal_spec_growth leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::component::numeric_input::{Modification, NumericInput};
use crate::utils::updatable::Updatable;
use fin_model::core::NormalSpec;
use leptos::{component, tracing, view, IntoView, Scope};

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a normal specification -> N(mu, sigma).
///
///   * **cx** - Context
///   * **updatable** - Value and signalling callback
///   * _return_ - View for normal_spec_growth
#[component]
pub fn NormalSpecGrowth<F>(
    /// Context
    cx: Scope,
    /// Value and signalling callback
    updatable: Updatable<NormalSpec, F>,
) -> impl IntoView
where
    F: FnMut(&NormalSpec) + 'static,
{
    // α <fn normal_spec_growth>

    use std::rc::Rc;
    use std::cell::RefCell;
    let updatable = Rc::new(RefCell::new(updatable));
    let mean_updatable = updatable.clone();
    let on_mean_updated = move |mean| {
        let mut current = mean_updatable.borrow_mut();
        current.update(|v| { v.mean = mean; });
    };

    let on_std_dev_updated = move |std_dev| {
        let mut current = updatable.borrow_mut();
        current.update(|v| { v.std_dev = std_dev; });
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
