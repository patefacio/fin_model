//! Module for ccd_years_and_date leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use leptos::component;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::WriteSignal;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Show year and date examples
///
///   * **show_update** - Function to display state updates
///   * _return_ - View for ccd_years_and_date
#[component]
pub fn CcdYearsAndDate(
    /// Function to display state updates
    show_update: WriteSignal<String>,
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-cyad; ccd-section";
    let component_id = crate::component_id!("`CcdYearsAndDate`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn ccd_years_and_date>

    use crate::DateInput;
    use crate::Updatable;
    use crate::YearCurrencyValueInput;
    use crate::YearInput;
    use crate::YearRangeInput;
    use crate::YearValueInput;
    use leptos::SignalSet;
    use plus_modeled::Currency;
    use plus_modeled::YearCurrencyValue;
    use plus_modeled::YearRange;

    // ω <fn ccd_years_and_date>
    view! {
        <div class=SELF_CLASS>
            // α <plus-cyad-view>

            <div class="title">"Years and Dates"</div>
            <div class="ccd-time">

                <div style="padding: 1em;">
                    <h4>"Year Input"</h4>
                    <p inner_html="Year Input - Supports range and provides a <em>live</em> clamp type
                    functionality. With <em>live clamp</em> true, if the user enters a year with the proper number
                    of digits it will be within range. This is achieved by modifying user input on the fly to
                    stay within range. As this may be disorienting it is optional.
                    "></p>
                    <div>
                        <h5>"With Clamp, RangeInclusive(1900 to 2300)"</h5>
                        <YearInput
                            updatable=Updatable::new(
                                None,
                                move |y| { show_update.set(format!("Year updated -> {y:?}")) },
                            )

                            placeholder="year"
                            live_clamp=true
                            year_range=YearRange {
                                start: 1900,
                                end: 2300,
                            }
                        />

                    </div>
                    <div>
                        <h5>"Without Clamp, RangeInclusive(1900 to 2300)"</h5>
                        <YearInput
                            updatable=Updatable::new(
                                None,
                                move |y| { show_update.set(format!("Year updated -> {y:?}")) },
                            )

                            placeholder="year"
                            year_range=YearRange {
                                start: 1900,
                                end: 2300,
                            }
                        />

                    </div>
                    <div>
                        <h4>
                            "Without Clamp, RangeInclusive(2020 to 2030) With Initial Valid Year"
                        </h4>
                        <YearInput
                            updatable=Updatable::new(
                                Some(2030),
                                move |y| { show_update.set(format!("Year updated -> {y:?}")) },
                            )

                            placeholder="year"
                            year_range=YearRange {
                                start: 2020,
                                end: 2030,
                            }
                        />

                    </div>

                    <div>
                        <h4>"Year Range Input"</h4>
                        <YearRangeInput updatable=Updatable::new(
                            None,
                            move |yr| {
                                show_update.set(format!("Year Range updated -> {yr:?}"));
                            },
                        )/>
                    </div>
                    <div>
                        <h4>"Year Value Input"</h4>
                        <YearValueInput updatable=Updatable::new(
                            None,
                            move |yv| {
                                show_update.set(format!("Year Value updated -> {yv:?}"));
                            },
                        )/>
                    </div>

                    <div>
                        <h4>"Year Currency Value Input Without Values"</h4>
                        <YearCurrencyValueInput updatable=Updatable::new(
                            None,
                            move |ycv| show_update.set(format!("YearCurrencyValue set to {ycv:?}")),
                        )/>
                    </div>
                    <div>
                        <h4>"Year Currency Value Input With Values"</h4>
                        <YearCurrencyValueInput updatable=Updatable::new(
                            Some(YearCurrencyValue {
                                year: 1998,
                                currency: Currency::Jpy as i32,
                                value: 25.55,
                            }),
                            move |ycv| {
                                show_update.set(format!("YearCurrencyValue set to {ycv:?}"))
                            },
                        )/>
                    </div>

                </div>
                <div style="padding: 1em;">
                    <h4>"Date Input (Range (1990 -> 2070))"</h4>
                    <p inner_html="
                    <p>
                    A date input component with following features:
                    </p>
                    <ul>
                    <li>Any non numeric character (e.g. space or '/' advances from month or day field)</li>
                    <li>Tab from month to day to year and shift-tab for reverse</li>
                    <li>Year range <strong>with clamp</strong> is supported</li>
                    <li>Signals on complete/valid input</li>
                    <li>Class `invalid` if input is not valid</li>
                    </ul>
                    "></p>
                    <DateInput
                        updatable=Updatable::new(
                            None,
                            move |n| {
                                show_update.set(format!("Date updated -> {n:?}"));
                            },
                        )

                        year_range=Some(YearRange {
                            start: 1990,
                            end: 2070,
                        })
                    />

                </div>
            </div>

        // ω <plus-cyad-view>
        </div>
    }
}

// α <mod-def ccd_years_and_date>
// ω <mod-def ccd_years_and_date>
