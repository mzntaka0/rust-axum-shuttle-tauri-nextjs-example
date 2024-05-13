use axum::{http::HeaderValue, routing::get, Json, Router};
use hyper::header::CONTENT_TYPE;
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::domains;
use todos::model::{CreateTodo, Todo, UpdateTodo};

#[utoipa::path(
    get,
    path = "/docs/openapi.json",
    responses(
        (status = 200, description = "JSON file", body = ())
    )
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

// NOTE: wanna split into each domains and merge here
#[derive(OpenApi)]
#[openapi(
    paths(
        domains::todos::controller::find_all,
        domains::todos::controller::create,
        domains::todos::controller::delete,
        domains::todos::controller::find,
        domains::todos::controller::update
    ),
    components(schemas(Todo, CreateTodo, UpdateTodo))
)]
struct ApiDoc;

#[derive(Clone)]
pub struct AppState {
    pub postgres: PgPool,
}

pub fn create_app(pool: PgPool, secrets: shuttle_runtime::SecretStore) -> Router {
    let doc = ApiDoc::openapi().to_pretty_json().unwrap();
    std::fs::write("openapi.json", doc.to_string()).unwrap_or(());
    Router::new()
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
        .route("/openapi.json", get(openapi))
        .route("/", get(root))
        .merge(domains::todos::route::routes(pool))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_origin(
                    secrets
                        .get("REMOTE")
                        .unwrap()
                        .parse::<HeaderValue>()
                        .unwrap(),
                )
                .allow_origin(
                    "https://rust-todo-two.vercel.app"
                        .parse::<HeaderValue>()
                        .unwrap(),
                )
                .allow_methods(Any)
                .allow_headers(vec![CONTENT_TYPE]),
        )
}

async fn root() -> &'static str {
    "Hello, world!"
}
