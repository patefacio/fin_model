//! Module for error_display_component leptos function/component

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use cfg_if::cfg_if;
use http::status::StatusCode;
use leptos::component;
use leptos::view;
use leptos::IntoView;
use leptos::RwSignal;
use leptos_dom::Errors;
use thiserror::Error;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
/// A component to display errors served by error boundaries
///
///   * **outside_errors** - TODO Document Param(outside_errors)
///   * **errors** - TODO Document Param(errors)
///   * _return_ - View for error_display_component
#[component]
pub fn ErrorDisplayComponent(
    /// TODO Document Param(outside_errors)
    #[prop(optional)]
    outside_errors: Option<Errors>,
    /// TODO Document Param(errors)
    #[prop(optional)]
    errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let component_id = crate::component_id!("`ErrorDisplayComponent`");
    #[cfg(debug_assertions)]
    crate::log_component!(crate::COMPONENT_LOG_LEVEL, component_id);
    // α <fn error_display_component>

    use leptos::create_rw_signal;
    use leptos::For;
    use leptos::SignalGetUntracked;

    let errors = match outside_errors {
        Some(e) => create_rw_signal(e),
        None => match errors {
            Some(e) => e,
            None => panic!("No Errors found and we expected errors!"),
        },
    };
    // Get Errors from Signal
    let errors = errors.get_untracked();

    // Downcast lets us take a type that implements `std::error::Error`
    let errors: Vec<AppError> = errors
        .into_iter()
        .filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned())
        .collect();
    println!("Errors: {errors:#?}");

    // Only the response code for the first error is actually sent from the server
    // this may be customized by the specific application
    cfg_if! { if #[cfg(feature="ssr")] {
        use leptos::use_context;
        let response = use_context::<ResponseOptions>();
        if let Some(response) = response {
            response.set_status(errors[0].status_code());
        }
    }}

    view! {
        <h1>{if errors.len() > 1 { "Errors" } else { "Error" }}</h1>
        <For
            // a function that returns the items we're iterating over; a signal is fine
            each=move || { errors.clone().into_iter().enumerate() }
            // a unique key for each item as a reference
            key=|(index, _error)| *index
            // renders each item to a view
            children=move |error| {
                let error_string = error.1.to_string();
                let error_code = error.1.status_code();
                view! {
                    <h2>{error_code.to_string()}</h2>
                    <p>"Error: " {error_string}</p>
                }
            }
        />
    }

    // ω <fn error_display_component>
}

// α <mod-def error_display_component>

#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;

/// AppError Todo
#[derive(Clone, Debug, Error)]
pub enum AppError {
    /// Not found TODO
    #[error("Not Found")]
    NotFound,
}

impl AppError {
    /// Get status_code TODO
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

// ω <mod-def error_display_component>
