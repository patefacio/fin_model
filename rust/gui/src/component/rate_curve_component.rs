//! Module for rate_curve_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::RateCurve;

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
///   * _return_ - View for rate_curve_component
#[component]
pub fn RateCurveComponent(
    /// Context
    cx: Scope,
    /// The [RateCurve] being edited
    updatable: Updatable<RateCurve>,
) -> impl IntoView {
    // α <fn rate_curve_component>
    use crate::Modification;
    use crate::NumericInput;
    use crate::PercentInput;
    use crate::Updatable;
    use crate::Year;
    use crate::YearInput;
    use crate::YearRangeInput;
    use leptos::create_signal;
    use leptos::For;
    use leptos::*;
    use plus_modeled::YearValue;
    use std::collections::BTreeMap;

    console_log("Constructing RateCurveComponent!!");

    fn make_ordered_points(rate_curve: &RateCurve) -> BTreeMap<u32, f64> {
        // TODO: Use collect here
        let mut ordered_points = BTreeMap::new();
        rate_curve.curve.iter().for_each(|yv| {
            ordered_points.insert(yv.year, yv.value);
        });

        ordered_points
    }

    let (ordered_points, set_ordered_points) =
        create_signal(cx, make_ordered_points(&updatable.value));
    let (rate_curve, set_rate_curve) = create_signal(cx, updatable.value);

    view! { cx,
        <div>
            <div style="display: inline-flex;">
                <div></div>
                <div></div>
                <div>
                    <h6>"Year"</h6>
                </div>
                <div>
                    <h6>"Rate"</h6>
                </div>
            </div>
        </div>
        <For
            each=move || { ordered_points.get() }
            key=|entry| { entry.0 }
            view=move |cx, element| {
                let (year, value) = element;
                let update_year_value = move |new_year_value: YearValue| {
                    let year = new_year_value.year;
                    set_ordered_points
                        .update(|ordered_points| {
                            if let Some(value) = ordered_points.get_mut(&year) {
                                console_log(&format!("Changing value at {year} to {value}"));
                                *value = new_year_value.value;
                            } else {
                                console_log(
                                    &format!(
                                        "Inserting value at {year} with {value} -> {ordered_points:?}"
                                    ),
                                );
                                ordered_points.insert(year, new_year_value.value);
                            }
                        });
                };
                view! { cx,
                    <div>
                        <div style="display: inline-flex;">
                            <button>"🗑"</button>
                            <button>"✎"</button>
                            <YearInput
                                updatable=Updatable::new(
                                    Some(year),
                                    move |year| {
                                        console_log(&format!("Year is updating -> {year:?} and {element:?}"));
                                        set_ordered_points
                                            .update(|ordered_points| {
                                                ordered_points.remove(&element.0);
                                            });
                                        if let Some(year) = year.clone() {
                                            let new_year_value = YearValue {
                                                year,
                                                value: element.1,
                                            };
                                            update_year_value(new_year_value);
                                        }
                                    },
                                )
                                placeholder=Some("year".to_string())
                            />
                            <PercentInput
                                updatable=Updatable::new(
                                    Some(value),
                                    move |p| {
                                        console_log(&format!("Percent is updating for index {p:?}"));
                                        console_log(&format!("Percent is updating for index {}", element.clone().0));
                                    },
                                )
                                placeholder=Some("rate".to_string())
                            />
                        </div>
                    </div>
                }
            }
        />
        <div>
            <div style="display: inline-flex;">
                <div></div>
                <button>"+"</button>
                <YearInput
                    updatable=Updatable::new(None, |y| { console_log("Year is updating") })
                    placeholder=Some("year".to_string())
                />
                <PercentInput
                    updatable=Updatable::new(None, |p| { console_log(&format!("Percent is updating => {p:?}")) })
                    placeholder=Some("rate".to_string())
                />
            </div>
        </div>
        <div>"Rate Curve TODO"</div>
    }

    // ω <fn rate_curve_component>
}

// α <mod-def rate_curve_component>
// ω <mod-def rate_curve_component>
