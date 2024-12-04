use axum::extract::Path;
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{routing::get, Router};
use tower_service::Service;
use worker::*;

fn router() -> Router {
    Router::new()
        .route("/", get(greeting))
        .route("/*username", get(get_widget))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}

pub async fn greeting() -> &'static str {
    let response: serde_json::Value = octocrab::instance()
        .graphql(&serde_json::json!({ "query": "{ viewer { login }}" }))
        .await.expect("REASON");
    dbg!(response);

    "Hello there! Oxwazz here 😁"
}

const ARROW_SVG: &str = r#"<svg data-v-e8d572f6="" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" aria-hidden="true" role="img" class="icon flex-shrink-0 h-6 w-6" width="1em" height="1em" viewBox="0 0 24 24"><path fill="currentColor" d="M4 11v2h12l-5.5 5.5l1.42 1.42L19.84 12l-7.92-7.92L10.5 5.5L16 11z"></path></svg>"#;

pub async fn get_widget(Path(username): Path<String>) -> Response {
    (
        StatusCode::OK,
        [("content-type", HeaderValue::from_static("image/svg+xml"))],
        ARROW_SVG,
    )
        .into_response()
}
