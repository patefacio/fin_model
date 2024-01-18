//! Module for year_value_input leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::component;
use leptos::view;
#[allow(unused_imports)]
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::MaybeSignal;
use plus_modeled::YearValue;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a _year_ combined with a _value_.
///
///   * **updatable** - The [YearValue] being edited
///   * **align_left** - If set, numeric text aligned to left.
///   * **year_placeholder** - Placeholder shown if entry is empty.
///   * **value_placeholder** - Placeholder shown if entry is empty.
///   * _return_ - View for year_value_input
#[component]
pub fn YearValueInput(
    /// The [YearValue] being edited
    updatable: Updatable<Option<YearValue>>,
    /// If set, numeric text aligned to left.
    #[prop(default = false)]
    align_left: bool,
    /// Placeholder shown if entry is empty.
    #[prop(default=MaybeSignal::Static(String::from("year")), into)]
    year_placeholder: MaybeSignal<String>,
    /// Placeholder shown if entry is empty.
    #[prop(default=MaybeSignal::Static(String::from("value")), into)]
    value_placeholder: MaybeSignal<String>,
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-yvi";
    let component_id = crate::component_id!("`YearValueInput`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
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

    let updatable_stored_value = store_value(YearValueData {
        year,
        value,
        updatable,
    });

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
        updatable_stored_value.update_value(|year_value_data| {
            year_value_data.year = *new_year;
            signal_pair(year_value_data);
        })
    });

    let value_updatable = Updatable::new(value, move |new_value| {
        updatable_stored_value.update_value(|year_value_data| {
            year_value_data.value = *new_value;
            signal_pair(year_value_data);
        })
    });

    // ω <fn year_value_input>
    view! {
        <div class=SELF_CLASS>
            // α <plus-yvi-view>

            <YearInput
                input_class=Some("yvi-year".to_string())
                updatable=year_updatable
                placeholder=year_placeholder
                align_left=align_left
            />
            <NumericInput
                input_class=Some("yvi-value".to_string())
                placeholder=value_placeholder
                updatable=value_updatable
                non_negative=true
                modification=Some(Modification::Prefix("$".into()))
                align_left=align_left
            />

        // ω <plus-yvi-view>
        </div>
    }
}

// α <mod-def year_value_input>
// ω <mod-def year_value_input>
