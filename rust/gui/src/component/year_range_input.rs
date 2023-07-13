//! Module for year_range_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::YearRange;
use std::ops::RangeInclusive;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a _start_ and _end_ year.
///
///   * **cx** - Context
///   * **updatable** - The [YearRange] being edited
///   * **range** - Range of valid values for input.
///   * **align_left** - If set, numeric text aligned to left.
///   * _return_ - View for year_range_input
#[component]
pub fn YearRangeInput(
    /// Context
    cx: Scope,
    /// The [YearRange] being edited
    updatable: Updatable<Option<YearRange>>,
    /// Range of valid values for input.
    #[prop(default=None)]
    range: Option<RangeInclusive<u32>>,
    /// If set, numeric text aligned to left.
    #[prop(default = false)]
    align_left: bool,
) -> impl IntoView {
    // α <fn year_range_input>

    use crate::Year;
    use crate::YearInput;
    use leptos::store_value;

    console_log(&format!("Creating YearRangeInput -> {cx:?}"));

    let start = updatable
        .value
        .as_ref()
        .map(|option_of_year_range| option_of_year_range.start);

    let end = updatable
        .value
        .as_ref()
        .map(|option_of_year_range| option_of_year_range.end);

    struct YearRangeData {
        start: Option<Year>,
        end: Option<Year>,
        updatable: Updatable<Option<YearRange>>,
    }

    let stored_updatable = store_value(
        cx,
        YearRangeData {
            start,
            end,
            updatable,
        },
    );

    fn signal_pair(year_range_data: &mut YearRangeData) {
        if year_range_data.start.is_some() && year_range_data.end.is_some() {
            year_range_data
                .updatable
                .update_and_then_signal(|year_range| {
                    *year_range = Some(YearRange {
                        start: year_range_data.start.unwrap(),
                        end: year_range_data.end.unwrap(),
                    })
                });
        }
    }

    let start_year_updatable = Updatable::new(start, move |new_start| {
        stored_updatable.update_value(|year_range_data| {
            year_range_data.start = *new_start;
            signal_pair(year_range_data);
        })
    });

    let end_year_updatable = Updatable::new(end, move |new_end| {
        stored_updatable.update_value(|year_range_data| {
            year_range_data.end = *new_end;
            signal_pair(year_range_data);
        })
    });

    let year_range = range
        .map(|range| YearRange {
            start: *range.start(),
            end: *range.end(),
        })
        .unwrap_or_else(|| YearRange {
            start: 1900,
            end: 2400,
        });

    view! { cx,
        <div style="display: inline-flex;">
            <YearInput
                input_class=Some("yri-start".to_string())
                placeholder=Some("start".to_string())
                updatable=start_year_updatable
                year_range=year_range
                live_clamp=true
                align_left=align_left
            />
            <YearInput
                input_class=Some("yri-end".to_string())
                placeholder=Some("end".to_string())
                updatable=end_year_updatable
                year_range=year_range
                live_clamp=true
                align_left=align_left
            /> 
        </div>
    }
    // ω <fn year_range_input>
}

// α <mod-def year_range_input>
// ω <mod-def year_range_input>
