//! Module for person_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::Updatable;
use leptos::{component, view, IntoView, Scope};
#[allow(unused_imports)]
use leptos_dom::console_log;
use plus_modeled::Person;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a person's details
///
///   * **cx** - Context
///   * **updatable** - The [Person] being edited
///   * _return_ - View for person_component
#[component]
pub fn PersonComponent(
    /// Context
    cx: Scope,
    /// The [Person] being edited
    updatable: Updatable<Option<Person>>,
) -> impl IntoView {
    // α <fn person_component>

    use crate::EnumSelect;
    use crate::NumericInput;
    use crate::YearInput;
    use crate::Updatable;
    use plus_modeled::PersonType;

    view! { cx,
        <fieldset>
            <div>"Name"</div>
            <div>
                <div>"Role"</div>
                <EnumSelect
                    updatable=Updatable::new(
                        updatable
                            .value
                            .as_ref()
                            .map(|person| PersonType::from_i32(person.person_type).unwrap_or_default())
                            .unwrap_or_default(),
                        |person_type| {
                            console_log(&format!("Person type updated to {person_type:?}"));
                        },
                    )
                    column_count=1
                />
            </div>
            <div>
                <div>
                    <div>"Birth Year"</div>
                    <YearInput updatable=Updatable::new(
                        updatable.value.as_ref().map(|person| person.birth_year),
                        |birth_year| { console_log(&format!("Birth year updated to {birth_year:?}")) },
                    )/>
                </div>
            </div>
        </fieldset>
    }

    // ω <fn person_component>
}

// α <mod-def person_component>
// ω <mod-def person_component>
