//! Module for expandable_rate_component leptos function/component

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
/// Starts as a single rate entry. User has option to expand entry to
/// a full rate curve. If as single rate, assumes the year is MIN_DATE.
/// MIN_DATE is a system constant that represents the earliest date we deal with.
/// If expanded user enters list of YearValue entries. The expand button
/// turns into a collapse button when expanded. If user tries to collapse without
/// any entries it simply collapses. If they collapse with 1 entry it collapses
/// with that one entry and keeps the date they've entered. If they collapse with
/// more than one entry the component throws up a popup dialog warning that
/// all entries except the first will be discarded.
///
///   * **cx** - Context
///   * **updatable** - The [RateCurve] being edited
///   * **year_range** - Range of valid years.
///   * _return_ - View for expandable_rate_component
#[component]
pub fn ExpandableRateComponent(
    /// Context
    cx: Scope,
    /// The [RateCurve] being edited
    updatable: Updatable<RateCurve>,
    /// Range of valid years.
    #[prop(default=YearRange{ start: 1900, end: 2400 })]
    year_range: YearRange,
) -> impl IntoView {
    // α <fn expandable_rate_component>
    use crate::PercentInput;
    use crate::RateCurveComponent;
    use leptos::create_rw_signal;
    use leptos::store_value;
    use leptos::Show;
    use leptos::SignalGet;
    use leptos::SignalUpdate;
    use leptos::SignalWith;
    use plus_utils::plus_constants::MIN_DATE;

    let is_expanded = create_rw_signal(cx, false);

    let stored_single_value = store_value(
        cx,
        updatable
            .value
            .curve
            .first()
            .map(|year_value| year_value.value),
    );

    let stored_updatable = store_value(cx, updatable);

    let single_value_updatable = move || {
        Updatable::new(stored_single_value.get_value(), move |value| {
            let new_first_year_value = YearValue {
                year: MIN_DATE,
                value: value.unwrap(),
            };
            log!("Percent is updating => {new_first_year_value:?}");
            stored_updatable
                .update_value(|updatable| updatable.value.curve = vec![new_first_year_value]);
            log!(
                "Latest curve is => {:?}",
                stored_updatable.with_value(|u| u.value.clone())
            );
        })
    };

    let show_single_rate = move || {
        is_expanded.track();
        let new_single_rate = stored_updatable
            .with_value(|updatable| updatable.value.curve.last().map(|yv| yv.value));
        stored_single_value.update_value(|value| *value = new_single_rate);
        view! { cx,
            <PercentInput placeholder=Some("value".to_string()) updatable=single_value_updatable()/>
        }
        .into_view(cx)
    };

    let show_rate_curve = move || {
        is_expanded.track();
        view! { cx,
            <RateCurveComponent updatable=Updatable::new(
                stored_updatable.with_value(|updatable| updatable.value.clone()),
                move |new_rc| {
                    log!("Rate Curve -> {new_rc:?}");
                    {
                        stored_updatable
                            .update_value(move |updatable| {
                                updatable
                                    .update_and_then_signal(move |rc| {
                                        let new_rc = new_rc.clone();
                                        *rc = new_rc;
                                    })
                            })
                    }
                },
            )/>
        }
        .into_view(cx)
    };

    view! { cx,
        <div>
            <span
                data-text="Collapsing removes entered data. Are you sure you want to close?"
                class="tooltip left"
            >
                <button
                    class="collapse-button"
                    on:click=move |_| {
                        is_expanded
                            .update(|value| {
                                *value = !*value;
                            })
                    }

                    style="margin-left: 0.2rem;"
                >
                    {move || { if is_expanded.get() { "...-" } else { "...+" } }}
                </button>
            </span>
            <Show when=move || { !is_expanded.get() } fallback=|_| ()>

                {move || show_single_rate()}

            </Show>

        </div>
        <Show when=move || { is_expanded.get() } fallback=|_| ()>

            {move || { show_rate_curve() }}

        </Show>
    }
    // ω <fn expandable_rate_component>
}

// α <mod-def expandable_rate_component>
// ω <mod-def expandable_rate_component>