//! Module for age_assumptions_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::AppContext;
use crate::Updatable;
use leptos::store_value;
use leptos::use_context;
use leptos::IntoAttribute;
use leptos::ReadSignal;
use leptos::SignalGet;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;
use plus_lookup::I18nAgeAssumptionsComponent;
use plus_modeled::AgeAssumptions;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Models age of various stages, eg retirement, death...
///
///   * **updatable** - The [AgeAssumptions] being edited.
///   * **is_hidden** - If true age assumptions will be hidden.
/// For dependents which are not modeled to their retirement/death.
///   * _return_ - View for age_assumptions_component
#[component]
pub fn AgeAssumptionsComponent(
    /// The [AgeAssumptions] being edited.
    updatable: Updatable<AgeAssumptions>,
    /// If true age assumptions will be hidden.
    /// For dependents which are not modeled to their retirement/death.
    is_hidden: ReadSignal<bool>,
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-aac";
    /// Maximum age supported by system
    pub const MAX_AGE: u32 = 120;
    let lang_selector = use_context::<AppContext>().unwrap().lang_selector;
    let i18n_retirement_age =
        move || I18nAgeAssumptionsComponent::RetirementAge(lang_selector.get()).to_string();
    let i18n_death_age =
        move || I18nAgeAssumptionsComponent::DeathAge(lang_selector.get()).to_string();
    // α <fn age_assumptions_component>

    use crate::IntegerInput;
    use crate::Updatable;
    use leptos::SignalGet;
    use plus_utils::SystemUnicodes;

    let retirement_age = updatable.value.retirement_age;
    let death_age = updatable.value.death_age;

    let stored_updatable = store_value(updatable);
    let signal_parent_update = move || {
        stored_updatable.update_value(|updatable| updatable.signal());
    };

    let retirement_age_updatable = Updatable::new(Some(retirement_age), move |retirement_age| {
        stored_updatable.update_value(|updatable| {
            updatable.value.retirement_age = retirement_age.unwrap();
        });
        signal_parent_update();
        log!("Retirement age updated to {retirement_age:?}");
    });

    let death_age_updatable = Updatable::new(Some(death_age), move |death_age| {
        stored_updatable.update_value(|updatable| {
            updatable.value.death_age = death_age.unwrap();
        });
        signal_parent_update();
        log!("Death age updated to {death_age:?}");
    });

    let style_hidden = move || {
        if is_hidden.get() {
            "display: none;"
        } else {
            ""
        }
    };

    // ω <fn age_assumptions_component>
    view! {
        <div class=SELF_CLASS style=style_hidden>
            // α <plus-aac-view>
            <div class="form">
                <hr/>
                <div class="form-row">
                    <label>
                        {format!(
                            "{} {}", i18n_retirement_age(), SystemUnicodes::BeachUmbrella
                            .as_unicode()
                        )}
                        <IntegerInput
                            updatable=retirement_age_updatable
                            range=Some(0..=MAX_AGE)
                            placeholder="🏖️"
                        />
                    </label>
                    <label>
                        {format!("{} {}", i18n_death_age(), SystemUnicodes::Tombstone.as_unicode())}
                        <IntegerInput
                            updatable=death_age_updatable
                            range=Some(0..=MAX_AGE)
                            placeholder="⚰️"
                        />
                    </label>
                </div>
            </div>
        // ω <plus-aac-view>
        </div>
    }
}

// α <mod-def age_assumptions_component>
// ω <mod-def age_assumptions_component>
