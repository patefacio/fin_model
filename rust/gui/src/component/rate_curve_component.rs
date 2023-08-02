//! Module for rate_curve_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use crate::Year;
#[allow(unused_imports)]
use leptos::log;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::RateCurve;
use plus_modeled::YearRange;
use plus_modeled::YearValue;

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
    use leptos::log;
    use leptos::store_value;
    use leptos::For;
    use leptos::IntoAttribute;
    use leptos::RwSignal;
    use leptos::SignalGet;
    use leptos::SignalGetUntracked;
    use leptos::SignalUpdate;
    use leptos::SignalUpdateUntracked;
    use leptos::SignalWith;
    use leptos::SignalWithUntracked;
    use plus_modeled::YearValue;
    use std::collections::HashMap;
    use std::ops::Range;

    let mut updatable = updatable;
    clean_curve(&mut updatable.value.curve);

    let (updatable, set_updatable) = create_signal(cx, updatable);
    let (curve, set_curve) = create_signal(cx, {
        let curve = updatable.with_untracked(|updatable| updatable.value.curve.clone());
        set_updatable.update(|updatable| {
            updatable.value.curve.clear();
        });
        curve
    });

    let (entry_complete, set_entry_complete) = create_signal(cx, (None, None));
    let (add_enabled, set_add_enabled) = create_signal(cx, false);
    let (clear_fields, set_clear_fields) = create_signal(cx, ());
    let (year_input_focus, set_year_input_focus) = create_signal(cx, ());

    let signal_parent_update = move || {
        curve.with_untracked(|curve| {
            set_updatable.update(|updatable| {
                updatable.update_and_then_signal(|client_curve| {
                    client_curve.curve = curve.clone();
                })
            })
        })
    };

    let signals = store_value(
        cx,
        curve
            .get_untracked()
            .iter()
            .enumerate()
            .map(|(i, year_value)| (year_value.year.to_owned(), create_rw_signal(cx, i)))
            .collect::<HashMap<u32, RwSignal<usize>>>(),
    );

    let on_accept = move || {
        if let (Some(new_year), Some(new_value)) = entry_complete.get() {
            set_curve.update_untracked(move |curve| {
                curve.push(YearValue {
                    year: new_year,
                    value: new_value,
                });
                let _deduped_years = clean_curve(curve);
            });
            let position = curve.with(|curve| {
                curve
                    .iter()
                    .enumerate()
                    .find(|(_, year_value)| year_value.year == new_year)
                    .unwrap()
                    .0
            });

            signals.update_value(|signals_map| {
                if let Some(current_signal) = signals_map.get(&new_year) {
                    log!("Replacing new year -> {new_year} at position-> {position:?}");
                    current_signal.update(|index| *index = position);
                } else {
                    log!("Inserting new year -> {new_year} at position-> {position:?}");
                    signals_map.insert(new_year, create_rw_signal(cx, position));
                }
            });
            set_curve.update(|_curve| {});
            set_clear_fields.update(|_| {});
            set_year_input_focus.update(|_| {});
            set_add_enabled.update(|enabled| *enabled = false);

            signal_parent_update()
        };
    };
    let on_accept_evt = move |_| on_accept();

    let num_elements = move || curve.with(|curve| curve.len());
    let nth_key = move |n: usize| {
        curve.with_untracked(|curve| curve.get(n).map(|year_value| year_value.year))
    };

    let key_signal =
        move |key: &u32| signals.with_value(|signals| signals.get(key).unwrap().clone());

    let delete_by_key = move |key: &u32| {
        if let Some(position) =
            signals.with_value(|signals| signals.get(key).cloned().map(|signal| signal.get()))
        {
            log!("Delete before curve update -> {key}");
            let mut position_to_end_range: Option<Range<usize>> = None;
            set_curve.update(|curve| {
                log!("Delete after curve update -> {key}");
                let _removed_value = curve.remove(position);
                let end = curve.len();
                position_to_end_range = Some(position..end);
            });

            signals.update_value(|signals| {
                curve.with(|curve| {
                    let elements_after = &curve[position_to_end_range.unwrap()];
                    for (i, year_value) in elements_after.iter().enumerate() {
                        let key = year_value.year;
                        let new_index = position + i;
                        if let Some(keyed_signal) = signals.get_mut(&key) {
                            keyed_signal.update(|index| {
                                log!("Updating key index -> {key}");
                                *index = new_index;
                            });
                        }
                    }
                });
            });
        }
    };

    let (disabled, _set_disabled) = create_signal(cx, true);
    let year_input = move |year_value: YearValue| {
        view! { cx,
            <YearInput
                input_class=Some("rcc-yi".to_string())
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
        }
    };

    let percent_input = move |year_value: YearValue| {
        log!("Percent input changing for year value to {year_value:?}");
        view! { cx,
            <PercentInput
                disabled=Some(disabled)
                updatable=Updatable::new(
                    Some(year_value.value),
                    move |percent| {
                        log!("Percent is updating => {percent:?}");
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
                on_enter=Some(
                    Box::new(move |_| {
                        on_accept();
                    }),
                )
            />
        }
    };

    view! { cx,
        <div class="rate-curve-data">
            <div style="display: grid; grid-template-columns: 0.1fr 0.4fr 0.6fr;">
                <div class="header"></div>
                <div class="header">"Year"</div>
                <div class="header">"Rate(%)"</div>
                <For
                    each=move || 0..num_elements()
                    key=move |&i| nth_key(i)
                    view=move |cx, i| {
                        let key = nth_key(i).unwrap();
                        let key_signal = key_signal(&key);
                        let year_value = move || {
                            let year_value_index = key_signal.get();
                            curve
                                .with_untracked(|curve| {
                                    if let Some(year_value) = curve.get(year_value_index).cloned() {
                                        year_value
                                    } else {
                                        panic!("Can't find year_value for {year_value_index:?}");
                                    }
                                })
                        };
                        let delete_row = move |_event| {
                            delete_by_key(&key);
                            signal_parent_update();
                        };
                        view! { cx,
                            <button on:click=delete_row>"🗑"</button>
                            {move || year_input(year_value())}
                            {move || percent_input(year_value())}
                        }
                    }
                />
                <button disabled=move || !add_enabled.get() on:click=on_accept_evt>
                    "+"
                </button>
                <YearInput
                    updatable=Updatable::new(
                        None,
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
                    clear_input=Some(clear_fields)
                    set_focus=Some(year_input_focus)
                    year_range=year_range
                    live_clamp=true
                />
                <PercentInput
                    updatable=Updatable::new(
                        None,
                        move |percent| {
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
                    clear_input=Some(clear_fields)
                    on_enter=Some(
                        Box::new(move |_| {
                            on_accept();
                        }),
                    )
                />
            </div>
        </div>
        <div inner_html=move || { updatable.with(|updatable| updatable.value.plot()) }></div>
    }
}

// ω <fn rate_curve_component>


/// Sorts the [YearValue] entries by year and removes any duplicates
///
///   * **points** - Curve points to sort and dedup.
///   * _return_ - Vector of year's that were combined due to duplication.
pub fn clean_curve(points: &mut Vec<YearValue>) -> Vec<u32> {
    // α <fn clean_curve>
    // First sort the data by year to ensure points are ordered
    points.sort_by(|a, b| a.year.cmp(&b.year));
    let mut last_inserted: Option<Year> = None;
    let mut deduped = Vec::with_capacity(points.len());
    let mut removed_keys = Vec::with_capacity(2);

    points.iter().for_each(|year_value| {
        if let Some(last_inserted) = last_inserted {
            if last_inserted == year_value.year {
                let last_value_ref: &mut YearValue = deduped.last_mut().unwrap();
                last_value_ref.value = year_value.value;
                removed_keys.push(year_value.year);
            } else {
                deduped.push(*year_value);
            }
        } else {
            deduped.push(*year_value);
        }
        last_inserted = Some(year_value.year);
    });

    *points = deduped;

    removed_keys
    // ω <fn clean_curve>
}

/// Unit tests for `rate_curve_component`
#[cfg(test)]
pub mod unit_tests {
    ////////////////////////////////////////////////////////////////////////////////////
    // --- functions ---
    ////////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn test_clean_curve() {
        // α <fn test_clean_curve>

        let mut dirty_curve = vec![
            YearValue {
                year: 2021,
                value: 3.0,
            },
            YearValue {
                year: 2020,
                value: 2.0,
            },
            YearValue {
                year: 2020,
                value: 4.0,
            },
        ];
        let values_removed = clean_curve(&mut dirty_curve);
        assert_eq!(
            vec![
                YearValue {
                    year: 2020,
                    value: 4.0
                },
                YearValue {
                    year: 2021,
                    value: 3.0
                }
            ],
            dirty_curve
        );

        assert_eq!(vec![2020], values_removed);

        // ω <fn test_clean_curve>
    }

    // α <mod-def unit_tests>
    use super::*;
    // ω <mod-def unit_tests>
}

// α <mod-def rate_curve_component>
// ω <mod-def rate_curve_component>
