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

    /* 
        use crate::utils::updatable::Updatable;
        use leptos::{create_rw_signal, SignalUpdate, SignalGet};
        let initial_normal_spec = rw_signal.read_only()();
        let mean_rw_signal = Updatable::new(initial_normal_spec, )
        create_rw_signal(cx, initial_normal_spec.map(|ns| ns.mean));
        let std_dev_rw_signal = create_rw_signal(cx, initial_normal_spec.map(|ns| ns.std_dev));


        let on_mean_updated = mean_rw_signal.get()
            Updatable::new(Some(initial_normal_spec.mean), move |mean: &Option<f64>| {
                if let Some(&mean) = mean.as_ref() {
                    rw_signal.update(|ns| ns.mean = mean);
                }
            });

        let on_std_dev_updated = Updatable::new(
            Some(initial_normal_spec.std_dev),
            move |std_dev: &Option<f64>| {
                if let Some(&std_dev) = std_dev.as_ref() {
                    rw_signal.update(|ns| ns.std_dev = std_dev);
                }
            },
        );
    */
    
    
    view! {
        cx,

            <fieldset class="nsg">
            <legend>"Normal Growth"</legend>
            <div class="form">
    
    
                <div>
                
                    "N("
                    <div>
                        <NumericInput placeholder=Some("mean".to_string()) 
                        modification=Some(Modification::Suffix("%".into()))
                        non_negative=true
                        updatable=Updatable::new(Some(42.0), |n| console_log(&format!("Number changed to {n:?}")))
                     />   
                        
                    </div>
                    ", "
                    <div>
                        <NumericInput placeholder=Some("std. dev".to_string())
                        modification=Some(Modification::Suffix("%".into()))
                        non_negative=true
                        updatable=Updatable::new(Some(42.0), |n| console_log(&format!("Number changed to {n:?}")))
                        />
                        
                    </div>
                    ")"
                </div>

            </div>
            </fieldset>
        }

    // ω <fn normal_spec_component>
}

// α <mod-def normal_spec_component>


// ω <mod-def normal_spec_component>
