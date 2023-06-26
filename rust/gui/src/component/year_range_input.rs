//! Module for year_range_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::YearRange;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a _start_ and _end_ year.
///
///   * **cx** - Context
///   * **updatable** - The [YearRange] being edited
///   * _return_ - View for year_range_input
#[component]
pub fn YearRangeInput(
    /// Context
    cx: Scope,
    /// The [YearRange] being edited
    updatable: Updatable<Option<YearRange>>,
) -> impl IntoView {
    // α <fn year_range_input>
    use crate::YearInput;
    use std::cell::RefCell;
    use std::rc::Rc;

    let updatable = Rc::new(RefCell::new(updatable));

    let initial_start_year = updatable
        .as_ref()
        .borrow()
        .value
        .as_ref()
        .map(|option_of_year_range| option_of_year_range.start);

    let initial_end_year = updatable
        .as_ref()
        .borrow()
        .value
        .as_ref()
        .map(|option_of_year_range| option_of_year_range.end);

    let updatable_for_start_year = Rc::clone(&updatable);
    let start_year_updatable = Updatable::new(initial_start_year, move |initial_start_year| {
        if let Some(initial_start_year) = initial_start_year.clone() {
            updatable_for_start_year
                .as_ref()
                .borrow_mut()
                .update_and_then_signal(|year_range| {
                    if let Some(year_range) = year_range {
                        year_range.start = initial_start_year;
                        
                    }; 
                });
        }
    });

    let updatable_for_end_year = Rc::clone(&updatable);
    let end_year_updatable = Updatable::new(initial_end_year, move |initial_end_year| {
        if let Some(initial_end_year) = initial_end_year.clone() {
            updatable_for_end_year
                .as_ref()
                .borrow_mut()
                .update_and_then_signal(|year_range| {
                    if let Some(year_range) = year_range {
                        year_range.end = initial_end_year;
                        
                    };
                });
        }
    });

    view! { cx,
        <h3>"TODO Year Range"</h3>
        <fieldset class="nsg">
            <legend>"Year Range"</legend>
            <div class="form">
                <div style="display: inline-flex">
                    "("
                    <YearInput
                        placeholder=Some("start".to_string())
                        updatable=start_year_updatable
                    /> ","
                    <YearInput placeholder=Some("end".to_string()) updatable=end_year_updatable/>
                    ")"
                </div>
            </div>
        </fieldset>
    }
    // ω <fn year_range_input>
}

// α <mod-def year_range_input>
// ω <mod-def year_range_input>
