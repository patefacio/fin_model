//! Provide handlers for serving files
#![cfg(feature = "ssr")]

////////////////////////////////////////////////////////////////////////////////////
// --- module uses ---
////////////////////////////////////////////////////////////////////////////////////
use crate::AppComponent;
use axum::body::boxed;
use axum::body::Body;
use axum::body::BoxBody;
use axum::extract::State;
use axum::http::Request;
use axum::http::Response;
use axum::http::StatusCode;
use axum::http::Uri;
use axum::response::IntoResponse;
use axum::response::Response as AxumResponse;
use leptos::LeptosOptions;
use tower::ServiceExt;
use tower_http::services::ServeDir;

////////////////////////////////////////////////////////////////////////////////////
// --- functions ---
////////////////////////////////////////////////////////////////////////////////////
///   * **uri** - File to load
///   * **axum_state** - Leptos options stored in axum
///   * **req** - The request
///   * _return_ - The axum response which in success case should be the static file retrieved
pub async fn file_and_error_handler(
    uri: Uri,
    axum_state: State<LeptosOptions>,
    req: Request<Body>,
) -> AxumResponse {
    // α <fn file_and_error_handler>
    use leptos::view;
    let State(options) = axum_state;
    let root = options.site_root.clone();
    let res = get_static_file(uri.clone(), &root).await.unwrap();

    if res.status() == StatusCode::OK {
        tracing::warn!("Retrieved static file {uri:?} at root `{root}`");
        res.into_response()
    } else {
        tracing::warn!(
            "Could not retrieved static file {uri:?} at root `{root}` ... rendering app to stream"
        );
        let handler = leptos_axum::render_app_to_stream(
            options.to_owned(),
            move || view! { <AppComponent/> },
        );
        handler(req).await.into_response()
    }
    // ω <fn file_and_error_handler>
}

/// Get a static file/asset
///
///   * **uri** - File to load
///   * **root** - Path to root of server
///   * _return_ - The resulting asset as a response or the error as status code with message
pub async fn get_static_file(
    uri: Uri,
    root: &str,
) -> Result<Response<BoxBody>, (StatusCode, String)> {
    // α <fn get_static_file>
    let req = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();

    tracing::warn!("Getting static file {uri:?} at root `{root}`");
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
    // ω <fn get_static_file>
}

// α <mod-def file_serve>
// ω <mod-def file_serve>
