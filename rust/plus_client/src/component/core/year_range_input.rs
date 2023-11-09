//! Module for year_range_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::AppContext;
use crate::Updatable;
use leptos::component;
use leptos::use_context;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::SignalGet;
use plus_lookup::I18nYearRangeInput;
use plus_modeled::YearRange;
use std::ops::RangeInclusive;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a _start_ and _end_ year.
///
///   * **updatable** - The [YearRange] being edited
///   * **range** - Range of valid values for input.
///   * **align_left** - If set, numeric text aligned to left.
///   * _return_ - View for year_range_input
#[component]
pub fn YearRangeInput(
    /// The [YearRange] being edited
    updatable: Updatable<Option<YearRange>>,
    /// Range of valid values for input.
    #[prop(default=None)]
    range: Option<RangeInclusive<u32>>,
    /// If set, numeric text aligned to left.
    #[prop(default = false)]
    align_left: bool,
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-yri";
    let lang_selector = use_context::<AppContext>().unwrap().lang_selector;
    let i18n_start_placeholder =
        move || I18nYearRangeInput::StartPlaceholder(lang_selector.get()).to_string();
    let i18n_end_placeholder =
        move || I18nYearRangeInput::EndPlaceholder(lang_selector.get()).to_string();
    crate::log_component!("`YearRangeInput`");
    // α <fn year_range_input>

    use crate::CssClasses;
    use crate::Year;
    use crate::YearInput;
    use leptos::store_value;
    use leptos::Signal;

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

    let updatable_stored_value = store_value(YearRangeData {
        start,
        end,
        updatable,
    });

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

    let range_for_start = range.clone();
    let start_year_updatable = Updatable::new(start, move |new_start| {
        updatable_stored_value.update_value(|year_range_data| {
            year_range_data.start = *new_start;
            if range_for_start
                .as_ref()
                .map(|range| range.contains(&new_start.unwrap()))
                .unwrap_or(true)
            {
                signal_pair(year_range_data);
            }
        })
    });

    let range_for_end = range.clone();
    let end_year_updatable = Updatable::new(end, move |new_end| {
        updatable_stored_value.update_value(|year_range_data| {
            year_range_data.end = *new_end;
            if range_for_end
                .as_ref()
                .map(|range| range.contains(&new_end.unwrap()))
                .unwrap_or(true)
            {
                signal_pair(year_range_data);
            }
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

    // ω <fn year_range_input>
    view! {
        <div class=SELF_CLASS>
            // α <plus-yri-view>

            <YearInput
                input_class=Some(CssClasses::YriStart.to_string())
                placeholder=Signal::derive(move || i18n_start_placeholder())
                updatable=start_year_updatable
                year_range=year_range
                live_clamp=true
                align_left=align_left
            />
            <YearInput
                input_class=Some(CssClasses::YriEnd.to_string())
                placeholder=Signal::derive(move || i18n_end_placeholder())
                updatable=end_year_updatable
                year_range=year_range
                live_clamp=true
                align_left=align_left
            />

        // ω <plus-yri-view>
        </div>
    }
}

// α <mod-def year_range_input>
// ω <mod-def year_range_input>
