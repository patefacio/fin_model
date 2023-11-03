//! Module for year_value_series_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::AppContext;
use crate::Updatable;
use crate::Year;
use leptos::use_context;
use leptos::IntoAttribute;
use leptos::ReadSignal;
use leptos::SignalGet;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;
use plus_lookup::I18nYearValueSeriesComponent;
use plus_modeled::YearRange;
use plus_modeled::YearValue;

////////////////////////////////////////////////////////////////////////////////////
// --- enums ---
////////////////////////////////////////////////////////////////////////////////////
/// Enumerates the types of data being displayed
#[derive(Debug, Copy, Clone)]
pub enum YearValueSeriesType {
    /// The data are growth rates and displayed as percentages.
    RateCurve,
    /// The data are market values of provided currency
    MarketValue {
        /// The currency of the values.
        currency_read: ReadSignal<String>,
    },
}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models an **ordered** series of [YearValue](plus_modeled::YearValue) pairs that
/// together constitute a piece-wise step function.
///
/// The component ensures the ordering (i.e. the years in the
/// vector of [YearValue](plus_modeled::YearValue) are strictly increasing).
///
///   * **updatable** - The [RateCurve] being edited
///   * **year_range** - Range of valid years.
///   * **series_type** - Type of data displayed.
///   * _return_ - View for year_value_series_component
#[component]
pub fn YearValueSeriesComponent(
    /// The [RateCurve] being edited
    updatable: Updatable<Vec<YearValue>>,
    /// Range of valid years.
    #[prop(default=YearRange{ start: 1900, end: 2400 })]
    year_range: YearRange,
    /// Type of data displayed.
    #[prop(default=YearValueSeriesType::RateCurve)]
    series_type: YearValueSeriesType,
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-yvsc";
    let lang_selector = use_context::<AppContext>().unwrap().lang_selector;
    let i18n_show_rate_curve =
        move || I18nYearValueSeriesComponent::ShowRateCurve(lang_selector.get()).to_string();
    let i18n_hide_rate_curve =
        move || I18nYearValueSeriesComponent::HideRateCurve(lang_selector.get()).to_string();
    let i18n_rate_placeholder =
        move || I18nYearValueSeriesComponent::RatePlaceholder(lang_selector.get()).to_string();
    let i18n_rate_percent =
        move || I18nYearValueSeriesComponent::RatePercent(lang_selector.get()).to_string();
    let i18n_value = move || I18nYearValueSeriesComponent::Value(lang_selector.get()).to_string();
    let i18n_year = move || I18nYearValueSeriesComponent::Year(lang_selector.get()).to_string();
    let i18n_value_placeholder =
        move || I18nYearValueSeriesComponent::ValuePlaceholder(lang_selector.get()).to_string();
    let i18n_year_placeholder =
        move || I18nYearValueSeriesComponent::YearPlaceholder(lang_selector.get()).to_string();
    crate::log_component!("`YearValueSeriesComponent`");
    // α <fn year_value_series_component>
    use crate::utils::plot_data::PlotData;
    use crate::CollapsibleComponent;
    use crate::CssClasses;
    use crate::Modification;
    use crate::NumericInput;
    use crate::PercentInput;
    use crate::RateCurveData;
    use crate::Updatable;
    use crate::YearInput;
    use leptos::create_rw_signal;
    use leptos::create_signal;
    use leptos::store_value;
    use leptos::For;
    use leptos::MaybeSignal;
    use leptos::RwSignal;
    use leptos::Show;
    use leptos::SignalGetUntracked;
    use leptos::SignalUpdate;
    use leptos::SignalUpdateUntracked;
    use leptos::SignalWith;
    use leptos::SignalWithUntracked;
    use plus_modeled::YearValue;
    use std::collections::HashMap;
    use std::ops::Range;

    let mut updatable = updatable;
    clean_curve(&mut updatable.value);

    let is_rate_curve_data = matches!(series_type, YearValueSeriesType::RateCurve);
    let (updatable_read, updatable_write) = create_signal(updatable);
    let (curve_read, curve_write) =
        create_signal(updatable_read.with_untracked(|updatable| updatable.value.clone()));

    let (entry_complete_read, entry_complete_write) = create_signal((None, None));
    let (add_enabled_read, add_enabled_write) = create_signal(false);
    let (clear_fields_read, clear_fields_write) = create_signal(());
    let (year_input_focus_read, year_input_focus_write) = create_signal(());

    let signal_parent_update = move || {
        curve_read.with_untracked(|curve| {
            updatable_write.update(|updatable| {
                updatable.update_and_then_signal(|client_curve| {
                    *client_curve = curve.clone();
                })
            })
        })
    };

    let signals_stored_value = store_value(
        curve_read
            .get_untracked()
            .iter()
            .enumerate()
            .map(|(i, year_value)| (year_value.year.to_owned(), create_rw_signal(i)))
            .collect::<HashMap<u32, RwSignal<usize>>>(),
    );

    let on_accept = move || {
        if let (Some(new_year), Some(new_value)) = entry_complete_read.get() {
            tracing::info!("Accepting {new_year:?} and  {new_value:?}!");

            curve_write.update_untracked(move |curve| {
                curve.push(YearValue {
                    year: new_year,
                    value: new_value,
                });
                let _deduped_years = clean_curve(curve);
            });
            let position = curve_read.with(|curve| {
                curve
                    .iter()
                    .enumerate()
                    .find(|(_, year_value)| year_value.year == new_year)
                    .unwrap()
                    .0
            });

            signals_stored_value.update_value(|signals_map| {
                if let Some(current_signal) = signals_map.get(&new_year) {
                    current_signal.update(|index| *index = position);
                } else {
                    signals_map.insert(new_year, create_rw_signal(position));
                }
            });
            curve_write.update(|_curve| {});
            clear_fields_write.update(|_| {});
            year_input_focus_write.update(|_| {});
            add_enabled_write.update(|enabled| *enabled = false);

            signal_parent_update()
        };
    };
    let on_accept_evt = move |_| on_accept();

    let num_elements = move || curve_read.with(|curve| curve.len());
    let nth_key = move |n: usize| {
        curve_read.with_untracked(|curve| curve.get(n).map(|year_value| year_value.year))
    };

    let key_signal = move |key: &u32| {
        signals_stored_value.with_value(|signals| signals.get(key).unwrap().clone())
    };

    let delete_by_key = move |key: &u32| {
        if let Some(position) = signals_stored_value
            .with_value(|signals| signals.get(key).cloned().map(|signal| signal.get()))
        {
            let mut position_to_end_range: Option<Range<usize>> = None;
            curve_write.update(|curve| {
                let _removed_value = curve.remove(position);
                let end = curve.len();
                position_to_end_range = Some(position..end);
            });

            signals_stored_value.update_value(|signals| {
                curve_read.with(|curve| {
                    let elements_after = &curve[position_to_end_range.unwrap()];
                    for (i, year_value) in elements_after.iter().enumerate() {
                        let key = year_value.year;
                        let new_index = position + i;
                        if let Some(keyed_signal) = signals.get_mut(&key) {
                            keyed_signal.update(|index| {
                                *index = new_index;
                            });
                        }
                    }
                });
            });
        }
    };

    let (disabled_read, _disabled_write) = create_signal(true);
    let year_input = move |year_value: YearValue| {
        view! {
            <YearInput
                input_class=Some("rcc-yi".to_string())
                disabled=disabled_read
                updatable=Updatable::new(
                    Some(year_value.year),
                    move |year| {
                        tracing::info!("Updating display year to {year:?}");
                        entry_complete_write.update(|entry_complete| entry_complete.0 = *year);
                        add_enabled_write
                            .update(|add_enabled| {
                                *add_enabled = entry_complete_read
                                    .with(|entry_complete| {
                                        entry_complete.0.is_some() && entry_complete.1.is_some()
                                    });
                            });
                    },
                )

                placeholder=i18n_year_placeholder()
            />
        }
    };

    let percent_display = move |year_value: YearValue| {
        view! {
            <PercentInput
                disabled=disabled_read
                updatable=Updatable::new(
                    Some(year_value.value),
                    move |percent| {
                        tracing::info!("Updated display percent to {percent:?}");
                    },
                )

                placeholder=i18n_rate_placeholder()
                on_enter=Some(
                    Box::new(move |_| {
                        on_accept();
                    }),
                )
            />
        }
    };

    let value_display = move |year_value: YearValue| {
        view! {
            <NumericInput
                disabled=disabled_read
                updatable=Updatable::new(
                    Some(year_value.value),
                    move |value| {
                        tracing::info!("Updated display value to {value:?}");
                    },
                )

                modification=Some(
                    Modification::Prefix(
                        match series_type {
                            YearValueSeriesType::MarketValue { currency_read } => {
                                MaybeSignal::Dynamic(currency_read.into())
                            }
                            _ => panic!("Must be market value data with currency"),
                        },
                    ),
                )

                placeholder=i18n_value_placeholder()
                on_enter=Some(
                    Box::new(move |_| {
                        on_accept();
                    }),
                )
            />
        }
    };

    let percent_input = move || {
        view! {
            <PercentInput
                updatable=Updatable::new(
                    None,
                    move |percent| {
                        tracing::info!("Updating live percent to {percent:?}");
                        entry_complete_write.update(|entry_complete| entry_complete.1 = *percent);
                        add_enabled_write
                            .update(|add_enabled| {
                                *add_enabled = entry_complete_read
                                    .with(|entry_complete| {
                                        entry_complete.0.is_some() && entry_complete.1.is_some()
                                    });
                            });
                    },
                )

                placeholder=i18n_rate_placeholder()
                clear_input=Some(clear_fields_read)
                on_enter=Some(
                    Box::new(move |_| {
                        on_accept();
                    }),
                )
            />
        }
        .into_view()
    };

    let value_input = move || {
        view! {
            <NumericInput
                updatable=Updatable::new(
                    None,
                    move |value| {
                        tracing::info!("Updating live value to {value:?}");
                        entry_complete_write.update(|entry_complete| entry_complete.1 = *value);
                        add_enabled_write
                            .update(|add_enabled| {
                                *add_enabled = entry_complete_read
                                    .with_untracked(|entry_complete| {
                                        entry_complete.0.is_some() && entry_complete.1.is_some()
                                    });
                            });
                    },
                )

                modification=Some(
                    Modification::Prefix(
                        match series_type {
                            YearValueSeriesType::MarketValue { currency_read } => {
                                MaybeSignal::Dynamic(currency_read.into())
                            }
                            _ => panic!("Must be market value data with currency"),
                        },
                    ),
                )

                placeholder=i18n_value_placeholder()
                clear_input=Some(clear_fields_read)
                on_enter=Some(
                    Box::new(move |_| {
                        on_accept();
                    }),
                )
            />
        }
        .into_view()
    };

    let value_header = move || {
        if is_rate_curve_data {
            i18n_rate_percent()
        } else {
            i18n_value()
        }
    };

    // ω <fn year_value_series_component>
    view! {
        <div class=SELF_CLASS>
            // α <plus-yvsc-view>

            <div style="display: grid; grid-template-columns: 0.1fr 0.4fr 0.6fr;">
                <div class=CssClasses::HeaderRight.to_string()></div>
                <div class=CssClasses::HeaderRight.to_string()>{i18n_year}</div>
                <div class=CssClasses::HeaderRight.to_string()>{value_header}</div>
                <For
                    each=move || 0..num_elements()
                    key=move |&i| nth_key(i)
                    children=move |i| {
                        let key = nth_key(i).unwrap();
                        let key_signal = key_signal(&key);
                        let year_value = move || {
                            let year_value_index = key_signal.get();
                            curve_read
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
                        view! {
                            <button on:click=delete_row>"🗑"</button>
                            {move || year_input(year_value())}
                            {move || {
                                if is_rate_curve_data {
                                    percent_display(year_value())
                                } else {
                                    value_display(year_value())
                                }
                            }}
                        }
                    }
                />

                <button disabled=move || !add_enabled_read.get() on:click=on_accept_evt>
                    "+"
                </button>
                <YearInput
                    updatable=Updatable::new(
                        None,
                        move |year| {
                            entry_complete_write.update(|entry_complete| entry_complete.0 = *year);
                            add_enabled_write
                                .update(|add_enabled| {
                                    tracing::info!("Updating live year to {year:?}");
                                    *add_enabled = entry_complete_read
                                        .with(|entry_complete| {
                                            entry_complete.0.is_some() && entry_complete.1.is_some()
                                        });
                                });
                        },
                    )

                    placeholder=i18n_year_placeholder()
                    clear_input=Some(clear_fields_read)
                    set_focus=Some(year_input_focus_read)
                    year_range=year_range
                    live_clamp=true
                />
                {if is_rate_curve_data { percent_input() } else { value_input() }}

            </div>
            <Show when=move || curve_read.with(|curve| curve.len() > 1) fallback=|| ()>
                <CollapsibleComponent
                    collapsed_header=i18n_show_rate_curve()
                    expanded_header=Some(i18n_hide_rate_curve())
                    is_expanded=false
                >
                    <div inner_html=move || {
                        updatable_read
                            .with(|updatable| {
                                RateCurveData {
                                    curve: &updatable.value,
                                }
                                    .plot()
                            })
                    }></div>
                </CollapsibleComponent>
            </Show>
        // ω <plus-yvsc-view>
        </div>
    }
}

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

/// Unit tests for `year_value_series_component`
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

// α <mod-def year_value_series_component>
// ω <mod-def year_value_series_component>
