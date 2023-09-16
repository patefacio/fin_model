//! Module for person_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::AppContext;
use leptos::use_context;
use leptos::IntoAttribute;
use leptos::SignalGet;
use leptos::StoredValue;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;
use plus_lookup::I18nPersonComponent;
use plus_modeled::Person;
use plus_modeled::PersonType;
use std::collections::HashMap;
use std::collections::HashSet;

////////////////////////////////////////////////////////////////////////////////////
// --- structs ---
////////////////////////////////////////////////////////////////////////////////////
/// Enough information about all people to prevent collisions when saving one.
/// The constraints include:
/// - Each must have different name.
/// - Only one can be `PrimaryOwner`
#[derive(Debug, Clone)]
pub struct PersonSharedContext {
    /// The names of all persons
    pub names: HashSet<String>,
    /// Map of type to person count
    pub owner_types: HashMap<PersonType, u32>,
}

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models a person's details
///
///   * **person_stored_value** - The [Person] being edited.
/// The shared context is the list of names already taken by collection
/// of people and the current count of all person types.
///   * _return_ - View for person_component
#[component]
pub fn PersonComponent(
    /// The [Person] being edited.
    /// The shared context is the list of names already taken by collection
    /// of people and the current count of all person types.
    person_stored_value: StoredValue<Person>,
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-pc";
    let lang_selector = use_context::<AppContext>().unwrap().lang_selector;
    let i18n_name = move || I18nPersonComponent::Name(lang_selector.get()).to_string();
    let i18n_birth_year = move || I18nPersonComponent::BirthYear(lang_selector.get()).to_string();
    let i18n_role = move || I18nPersonComponent::Role(lang_selector.get()).to_string();
    let i18n_retirement_age =
        move || I18nPersonComponent::RetirementAge(lang_selector.get()).to_string();
    // α <fn person_component>

    use crate::AgeAssumptionsComponent;
    use crate::AppContext;
    use crate::EnumSelect;
    use crate::Updatable;
    use crate::YearInput;
    use leptos::create_node_ref;
    use leptos::create_signal;
    use leptos::use_context;
    use leptos::IntoAttribute;
    use leptos::Show;
    use leptos::SignalGet;
    use leptos::SignalSet;
    use leptos_dom::html::Input;
    use plus_lookup::i18n_enum_display::I18nEnums;
    use plus_modeled::AgeAssumptions;
    use plus_modeled::LangSelector;
    use plus_modeled::PersonType;
    use plus_modeled::DEFAULT_DEATH_AGE;
    use plus_modeled::DEFAULT_RETIREMENT_AGE;

    let lang_selector = use_context::<AppContext>().unwrap().lang_selector;

    leptos::on_cleanup(move || log!("CLEANING UP HOLDING"));

    let person = person_stored_value.with_value(|person| person.clone());
    let node_ref = create_node_ref::<Input>();
    let birth_year = person.birth_year;
    let name = person.name.clone();

    let age_assumptions = person
        .age_assumptions
        .clone()
        .unwrap_or_else(|| AgeAssumptions {
            retirement_age: DEFAULT_RETIREMENT_AGE,
            death_age: DEFAULT_DEATH_AGE,
        });

    let person_type = PersonType::from_i32(person.person_type).unwrap();

    let (is_dependent, set_is_dependent) = create_signal(person_type == PersonType::Dependent);

    let birth_year_updatable = Updatable::new(birth_year, move |&birth_year| {
        person_stored_value.update_value(|person| {
            person.birth_year = birth_year;
            log!("Birth year updated to {birth_year:?}");
        })
    });

    let age_assumptions_updatable = Updatable::new(age_assumptions, move |&age_assumptions| {
        person_stored_value.update_value(|person| {
            person.age_assumptions = Some(age_assumptions);
            log!("Age assumptions updated to {age_assumptions:?}");
        });
    });

    let person_type_updatable = Updatable::new(person_type, move |&new_person_type| {
        person_stored_value.update_value(|person| {
            person.person_type = new_person_type as i32;
            set_is_dependent.set(new_person_type == PersonType::Dependent);
            log!("Person type updated to {new_person_type:?}");
        });
    });

    // ω <fn person_component>
    view! {
        <div class=SELF_CLASS>
            // α <plus-pc-view>
            <fieldset>
                <legend>"Person"</legend>
                <div class="form">
                    <div class="form-row">
                        <label>
                            {i18n_name}
                            <input
                                id="name"
                                value=name
                                node_ref=node_ref
                                on:input=move |_| {
                                    if let Some(node_ref) = node_ref.get() {
                                        person_stored_value
                                            .update_value(move |person| {
                                                person.name = node_ref.value();
                                            });
                                    }
                                }
                            />

                        </label>
                        <label>
                            {i18n_role} <div style="display: inline-block">
                                <EnumSelect
                                    updatable=person_type_updatable
                                    column_count=1
                                    label=Some(
                                        Box::new(move |e| {
                                            I18nEnums::PersonType(lang_selector.get(), e).to_string()
                                        }),
                                    )
                                />

                            </div>
                        </label>
                    </div>
                    <div class="form-row">
                        <label>{i18n_birth_year} <YearInput updatable=birth_year_updatable/></label>
                    </div>
                    <AgeAssumptionsComponent
                        updatable=age_assumptions_updatable
                        is_hidden=is_dependent
                    />
                </div>
                </fieldset>
            // ω <plus-pc-view>
        </div>
    }
}

// α <mod-def person_component>
// ω <mod-def person_component>
