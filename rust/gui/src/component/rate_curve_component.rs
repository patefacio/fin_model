//! Module for rate_curve_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
#[allow(unused_imports)]
use leptos::log;
use leptos::SignalWithUntracked;
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
    use leptos::log;
    use leptos::For;
    use leptos::IntoAttribute;
    use leptos::SignalGet;
    use leptos::SignalUpdate;
    use leptos::SignalWith;
    use plus_modeled::YearValue;

    log!("Creating RateCurveComponent -> {cx:?}");

    let mut updatable = updatable;

    /// Sort entries by year and remove any duplicate years
    fn clean_curve(points: &mut Vec<YearValue>) {
        // First sort the data by year to ensure points are ordered
        points.sort_by(|a, b| a.year.cmp(&b.year));
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

    let (updatable, set_updatable) = create_signal(cx, updatable);
    let (curve, set_curve) = create_signal(cx, {
        let curve = updatable.with(|updatable| updatable.value.curve.clone());
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

    let on_accept = move || {
        set_curve.update(move |curve| {
            entry_complete.with(|entry_complete| {
                log!("Accessing entry complete {entry_complete:?}");
                curve.push(YearValue {
                    year: entry_complete.0.unwrap(),
                    value: entry_complete.1.unwrap(),
                });
                clean_curve(curve);
                set_clear_fields.update(|_| {});
                set_year_input_focus.update(|_| {});
            });
        });
        signal_parent_update()
    };
    let on_accept_evt = move |_| on_accept();

    let on_accept_enter: Option<Box<dyn FnMut(String)>> = Some(Box::new(move |_| {
        on_accept();
    }));

    view! { cx,
        <div class="rate-curve-data">
            <div style="display: grid; grid-template-columns: 0.1fr 0.4fr 0.6fr;">
                <div class="header"></div>
                <div class="header">"Year"</div>
                <div class="header">"Rate(%)"</div>
                <For
                    each=curve
                    key=|year_value| { year_value.year }
                    view=move |cx, year_value| {
                        let (disabled, _set_disabled) = create_signal(cx, true);
                        let remove_me = move |_event| {
                            set_curve
                                .update(|curve| {
                                    if let Some(found_index)
                                        = curve
                                            .iter()
                                            .position(|elm_year_value| {
                                                elm_year_value.year == year_value.year
                                            })
                                    {
                                        curve.remove(found_index);
                                    }
                                });
                            signal_parent_update();
                        };
                        view! { cx,
                            <button on:click=remove_me>"🗑"</button>
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
                            <PercentInput
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
                            />
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
                    on_enter=on_accept_enter
                />
            </div>
        </div>
        <div inner_html=move || { updatable.with(|updatable| updatable.value.plot()) }></div>
    }

    // ω <fn rate_curve_component>
}

// α <mod-def rate_curve_component>
// ω <mod-def rate_curve_component>
