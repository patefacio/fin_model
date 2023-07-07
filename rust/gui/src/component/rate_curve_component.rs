//! Module for rate_curve_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::RateCurve;
use plus_modeled::YearRange;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models an **ordered** series of [YearValue](plus_modeled::YearValue) pairs that together constitute a
/// piece-wise step function. The component ensures the ordering (i.e. the years in the
/// vector of [YearValue](plus_modeled::YearValue) are strictly increasing)
///
///
///   * **cx** - Context
///   * **updatable** - The [RateCurve] being edited
///   * **year_range** - Range of valid years.
///   * _return_ - View for rate_curve_component
#[component]
pub fn RateCurveComponent(
    /// Context
    cx: Scope,
    /// The [RateCurve] being edited
    updatable: Updatable<RateCurve>,
    /// Range of valid years.
    #[prop(default=YearRange{ start: 1900, end: 2400 })]
    year_range: YearRange,
) -> impl IntoView {
    // α <fn rate_curve_component>
    use crate::utils::plot_data::PlotData;
    use crate::PercentInput;
    use crate::Updatable;
    use crate::Year;
    use crate::YearInput;
    use leptos::create_rw_signal;
    use leptos::create_signal;
    use leptos::For;
    use leptos::*;
    use plus_modeled::YearValue;

    console_log("Constructing RateCurveComponent!!");

    let mut updatable = updatable;

    fn clean_curve(points: &mut Vec<YearValue>) {
        // First sort the data by year to ensure points are ordered
        points.sort_by(|a, b| a.year.cmp(&b.year));
        // Remove duplicates  where dupe is defined as years being equal.
        // If year appears multiple times, keep the last value.
        let mut last_inserted: Option<Year> = None;
        let mut deduped = Vec::with_capacity(points.len());

        points.iter().for_each(|year_value| {
            if let Some(last_inserted) = last_inserted {
                if last_inserted == year_value.year {
                    let last_value_ref: &mut YearValue = deduped.last_mut().unwrap();
                    last_value_ref.value = year_value.value;
                } else {
                    deduped.push(*year_value);
                }
            } else {
                deduped.push(*year_value);
            }
            last_inserted = Some(year_value.year);
        });

        *points = deduped;
    }

    clean_curve(&mut updatable.value.curve);
    console_log(&format!("Sorted data -> {:?}", updatable.value));
    let (updatable, set_updatable) = create_signal(cx, updatable);
    let (entry_complete, set_entry_complete) = create_signal(cx, (None, None));
    let (add_enabled, set_add_enabled) = create_signal(cx, false);
    let clear_fields = create_rw_signal(cx, false);

    view! { cx,
        <div class="rate-curve-data">
        <div style="display: grid; grid-template-columns: 1fr 1fr 1fr;">
            <For
                each=move || updatable.with(|updatable| updatable.value.curve.clone())
                key=|year_value| { year_value.year }
                view=move |cx, year_value| {
                    let (disabled, _set_disabled) = create_signal(cx, true);
                    let remove_me = move |_event| {
                        set_updatable
                            .update(|updatable| {
                                if let Some(found_index)
                                    = updatable
                                        .value
                                        .curve
                                        .iter()
                                        .position(|elm_year_value| {
                                            elm_year_value.year == year_value.year
                                        })
                                {
                                    updatable.value.curve.remove(found_index);
                                }
                            });
                    };
                    view! { cx,
                        <button on:click=remove_me>"🗑"</button>
                        <YearInput
                            disabled=Some(disabled)
                            updatable=Updatable::new(
                                Some(year_value.year),
                                move |year| {
                                    set_entry_complete.update(|entry_complete| entry_complete.0 = *year);
                                    set_add_enabled
                                        .update(|add_enabled| {
                                            *add_enabled = entry_complete
                                                .with(|entry_complete| {
                                                    entry_complete.0.is_some() && entry_complete.1.is_some()
                                                });
                                        });
                                },
                            )
                            placeholder=Some("year".to_string())
                        />
                        <PercentInput
                            updatable=Updatable::new(
                                Some(year_value.value),
                                move |percent| {
                                    console_log(&format!("Percent is updating => {percent:?}"));
                                    set_entry_complete.update(|entry_complete| entry_complete.1 = *percent);
                                    set_add_enabled
                                        .update(|add_enabled| {
                                            *add_enabled = entry_complete
                                                .with(|entry_complete| {
                                                    entry_complete.0.is_some() && entry_complete.1.is_some()
                                                });
                                        });
                                },
                            )
                            placeholder=Some("rate".to_string())
                        />
                    }
                }
            />
            <button
                disabled=move || !add_enabled.get()
                on:click=move |_| {
                    set_updatable
                        .update(move |updatable| {
                            entry_complete
                                .with(|entry_complete| {
                                    updatable
                                        .value
                                        .curve
                                        .push(YearValue {
                                            year: entry_complete.0.unwrap(),
                                            value: entry_complete.1.unwrap(),
                                        });
                                    clean_curve(&mut updatable.value.curve);
                                    console_log("Finished adding curve point, clearing fields!");
                                    clear_fields.set(true);
                                });
                        })
                }
            >
                "+"
            </button>
            <YearInput
                updatable=Updatable::new(
                    None,
                    move |year| {
                        console_log(&format!("Year is updating => {year:?}"));
                        set_entry_complete.update(|entry_complete| entry_complete.0 = *year);
                        set_add_enabled
                            .update(|add_enabled| {
                                *add_enabled = entry_complete
                                    .with(|entry_complete| {
                                        entry_complete.0.is_some() && entry_complete.1.is_some()
                                    });
                                console_log(&format!("Checking entry complete => {add_enabled:?}"));
                            });
                    },
                )
                placeholder=Some("year".to_string())
                clear_input=Some(clear_fields)
                year_range=year_range
                live_clamp=true
            />
            <PercentInput
                updatable=Updatable::new(
                    None,
                    move |percent| {
                        console_log(&format!("Percent is updating => {percent:?}"));
                        set_entry_complete.update(|entry_complete| entry_complete.1 = *percent);
                        set_add_enabled
                            .update(|add_enabled| {
                                *add_enabled = entry_complete
                                    .with(|entry_complete| {
                                        console_log(
                                            &format!("Checking entry complete => {entry_complete:?}"),
                                        );
                                        entry_complete.0.is_some() && entry_complete.1.is_some()
                                    });
                                console_log(&format!("Checking entry complete => {add_enabled:?}"));
                            });
                    },
                )
                placeholder=Some("rate".to_string())
            />
        </div>
        </div>
        <div inner_html=move || { updatable.with(|updatable| updatable.value.plot()) }></div>
    }

    // ω <fn rate_curve_component>
}

// α <mod-def rate_curve_component>
// ω <mod-def rate_curve_component>
