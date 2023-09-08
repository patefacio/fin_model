//! Module for year_value_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::YearValue;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a _year_ combined with a _value_.
///
///   * **cx** - Context
///   * **updatable** - The [YearValue] being edited
///   * **align_left** - If set, numeric text aligned to left.
///   * _return_ - View for year_value_input
#[component]
pub fn YearValueInput(
    /// Context
    cx: Scope,
    /// The [YearValue] being edited
    updatable: Updatable<Option<YearValue>>,
    /// If set, numeric text aligned to left.
    #[prop(default = false)]
    align_left: bool,
) -> impl IntoView {
    // α <fn year_value_input>

    use crate::Modification;
    use crate::NumericInput;
    use crate::Year;
    use crate::YearInput;
    use leptos::store_value;

    let year = updatable.value.as_ref().map(|year_value| year_value.year);
    let value = updatable.value.as_ref().map(|year_value| year_value.value);

    struct YearValueData {
        year: Option<Year>,
        value: Option<f64>,
        updatable: Updatable<Option<YearValue>>,
    }

    let stored_updatable = store_value(
        cx,
        YearValueData {
            year,
            value,
            updatable,
        },
    );

    fn signal_pair(year_value_data: &mut YearValueData) {
        if year_value_data.year.is_some() && year_value_data.value.is_some() {
            year_value_data
                .updatable
                .update_and_then_signal(|year_value| {
                    *year_value = Some(YearValue {
                        year: year_value_data.year.unwrap(),
                        value: year_value_data.value.unwrap(),
                    })
                });
        }
    }

    let year_updatable = Updatable::new(year, move |new_year| {
        stored_updatable.update_value(|year_value_data| {
            year_value_data.year = *new_year;
            signal_pair(year_value_data);
        })
    });

    let value_updatable = Updatable::new(value, move |new_value| {
        stored_updatable.update_value(|year_value_data| {
            year_value_data.value = *new_value;
            signal_pair(year_value_data);
        })
    });

    view! { cx,
        <div style="display: inline-flex">
            <YearInput
                input_class=Some("yvi-year".to_string())
                updatable=year_updatable
                placeholder=Some("year".to_string())
                align_left=align_left
            />
            <NumericInput
                input_class=Some("yvi-value".to_string())
                placeholder=Some("value".to_string())
                updatable=value_updatable
                non_negative=true
                modification=Some(Modification::Prefix(("$".into())))
                align_left=align_left
            />
        </div>
    }

    // ω <fn year_value_input>
}

// α <mod-def year_value_input>
// ω <mod-def year_value_input>
