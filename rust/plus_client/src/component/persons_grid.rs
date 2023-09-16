//! Module for persons_grid leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::AppContext;
use crate::CollectionGrid;
use crate::CollectionGridEditType;
use crate::PersonSharedContext;
use crate::Updatable;
use leptos::use_context;
use leptos::IntoAttribute;
use leptos::SignalGet;
use leptos::StoredValue;
use leptos::{component, view, IntoView};
#[allow(unused_imports)]
use leptos_dom::log;
use leptos_dom::View;
use plus_lookup::I18nPersonsGrid;
use plus_modeled::Person;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// Display and edit support for list of persons.
///
///   * **persons_updatable** - Persons to edit
///   * **shared_context_updatable** - The shared context
///   * _return_ - View for persons_grid
#[component]
pub fn PersonsGrid(
    /// Persons to edit
    persons_updatable: Updatable<Vec<Person>>,
    /// The shared context
    shared_context_updatable: Updatable<PersonSharedContext>,
) -> impl IntoView {
    pub const SELF_CLASS: &str = "plus-pg";
    let lang_selector = use_context::<AppContext>().unwrap().lang_selector;
    let i18n_people = move || I18nPersonsGrid::People(lang_selector.get()).to_string();
    let i18n_grid_help = move || I18nPersonsGrid::GridHelp(lang_selector.get()).to_string();
    // α <fn persons_grid>

    use crate::CollectionGridComponent;

    // ω <fn persons_grid>
    view! {
        <div class=SELF_CLASS>
            // α <plus-pg-view>
            <div class="grid-label">{i18n_people}</div>
            <p inner_html=i18n_grid_help></p>
            <CollectionGridComponent
                rows_updatable=persons_updatable
                shared_context_updatable=shared_context_updatable
            />
        // ω <plus-pg-view>
        </div>
    }
}

////////////////////////////////////////////////////////////////////////////////////
// --- trait impls ---
////////////////////////////////////////////////////////////////////////////////////
impl CollectionGrid for Person {
    type SharedContext = PersonSharedContext;
    /// Get the display fields for the element.
    ///
    ///   * _return_ - The fields as elements
    fn get_fields(&self) -> Vec<View> {
        // α <fn CollectionGrid::get_fields for Person>

        use plus_lookup::I18nEnums;
        use plus_modeled::PersonType;
        use plus_modeled::DEFAULT_RETIREMENT_AGE;

        let lang_selector = use_context::<AppContext>().unwrap().lang_selector;

        let person_type = I18nEnums::PersonType(
            lang_selector.get(),
            &PersonType::from_i32(self.person_type).unwrap_or_default(),
        )
        .to_string();

        vec![
            view! { <div class="cgc-header-cell">{self.name.clone()}</div> }.into_view(),
            view! { <div class="cgc-header-cell">{person_type}</div> }.into_view(),
            view! { <div class="cgc-header-cell">{self.age_assumptions.map(|aa| aa.retirement_age.to_string()).unwrap_or(DEFAULT_RETIREMENT_AGE.to_string())}</div> }.into_view(),
        ]
        // ω <fn CollectionGrid::get_fields for Person>
    }

    /// Get the header for the rows.
    ///
    ///   * _return_ - The header
    fn get_header() -> Vec<String> {
        // α <fn CollectionGrid::get_header for Person>

        use leptos::SignalGetUntracked;
        
        let lang_selector = use_context::<AppContext>().unwrap().lang_selector.get_untracked();

        vec![
            I18nPersonsGrid::Name(lang_selector).to_string(),
            I18nPersonsGrid::Role(lang_selector).to_string(),
            I18nPersonsGrid::RetirementAge(lang_selector).to_string(),
        ]
        // ω <fn CollectionGrid::get_header for Person>
    }

    /// Get the text for `Add New Item`.
    ///
    ///   * _return_ - The add item label
    fn get_add_item_label() -> String {
        // α <fn CollectionGrid::get_add_item_label for Person>

        use leptos::SignalGetUntracked;

        let lang_selector = use_context::<AppContext>().unwrap().lang_selector.get_untracked();
        I18nPersonsGrid::NewPerson(lang_selector).to_string()
        // ω <fn CollectionGrid::get_add_item_label for Person>
    }

    /// Get key that uniquely identifies the element.
    ///
    ///   * _return_ - The key for the element
    fn get_key(&self) -> String {
        // α <fn CollectionGrid::get_key for Person>
        self.name.clone()
        // ω <fn CollectionGrid::get_key for Person>
    }

    /// Create new element to edit
    ///
    ///   * _return_ - New element
    fn new() -> Self {
        // α <fn CollectionGrid::new for Person>
        Person::default()
        // ω <fn CollectionGrid::new for Person>
    }

    /// Create a view to edit the row
    ///
    ///   * **edit_type** - Type of edit
    ///   * **row_stored_value** - Row to edit.
    ///   * **shared_context_stored_value** - Updatable containing the shared context.
    ///   * _return_ - The edit view
    fn edit_row(
        edit_type: CollectionGridEditType,
        row_stored_value: StoredValue<Self>,
        shared_context_stored_value: StoredValue<Self::SharedContext>,
    ) -> View {
        // α <fn CollectionGrid::edit_row for Person>

        use crate::PersonComponent;
        view! { <PersonComponent person_stored_value=row_stored_value/> }
        // ω <fn CollectionGrid::edit_row for Person>
    }

    /// Return true if row edit satisfies any shared context constraints
    ///
    ///   * **edited_row** - The edited row
    ///   * **shared_context** - The current shared context
    ///   * _return_ - An error message if not acceptable change, None otherwise
    fn accept_row_edit(
        edited_row: &Self,
        shared_context: &mut Self::SharedContext,
    ) -> Option<String> {
        // α <fn CollectionGrid::accept_row_edit for Person>
        todo!("Implement `accept_row_edit`")
        // ω <fn CollectionGrid::accept_row_edit for Person>
    }
}

// α <mod-def persons_grid>
// ω <mod-def persons_grid>
