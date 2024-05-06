mod domains;

use std::{env, sync::Arc};

use axum::{extract::Extension, http::HeaderValue, routing::get, Json, Router};
use dotenv::dotenv;
use hyper::header::CONTENT_TYPE;
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use todos::repository::{CreateTodo, Todo, TodoRepositoryForDb, UpdateTodo};

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

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] _secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    dotenv::dotenv().ok();

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migrations failed :(");

    // logging
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    dotenv().ok();

    let app = create_app(pool);

    Ok(app.into())
}

fn create_app(pool: PgPool) -> Router {
    let repository = TodoRepositoryForDb::new(pool.clone());
    let doc = ApiDoc::openapi().to_pretty_json().unwrap();
    std::fs::write("openapi.json", doc.to_string()).unwrap_or(());
    Router::new()
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
        .route("/openapi.json", get(openapi))
        .route("/", get(root))
        .merge(domains::todos::route::routes(pool))
        .layer(Extension(Arc::new(repository)))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
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

//#[cfg(test)]
//mod test {
//    use super::*;
//    use crate::repositories::{test_utils::TodoRepositoryForMemory, CreateTodo, Todo};
//    use axum::{
//        body::{to_bytes, Body},
//        http::{header, Method, Request, StatusCode},
//        response::Response,
//    };
//    use tower::ServiceExt;
//
//    fn build_todo_req_with_json(path: &str, method: Method, json_body: String) -> Request<Body> {
//        Request::builder()
//            .uri(path)
//            .method(method)
//            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
//            .body(Body::from(json_body))
//            .unwrap()
//    }
//
//    fn build_todo_req_with_empty(method: Method, path: &str) -> Request<Body> {
//        Request::builder()
//            .uri(path)
//            .method(method)
//            .body(Body::empty())
//            .unwrap()
//    }
//
//    async fn res_to_todo(res: Response) -> Todo {
//        let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
//        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
//        let todo: Todo = serde_json::from_str(&body)
//            .expect(&format!("cannot convert Todo instance. Body: {}", body));
//        todo
//    }
//
//    #[tokio::test]
//    async fn should_return_hello_world() {
//        let repository = TodoRepositoryForMemory::new();
//        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
//
//        let res = create_app(repository).oneshot(req).await.unwrap();
//
//        let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
//
//        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
//
//        assert_eq!(body, "Hello, world!");
//    }
//
//    #[tokio::test]
//    async fn should_create_todo() {
//        let expected = Todo::new(1, "should_return_created_todo".to_string());
//
//        let repository = TodoRepositoryForMemory::new();
//        let req = build_todo_req_with_json(
//            "/todos",
//            Method::POST,
//            r#"{ "text": "should_return_created_todo" }"#.to_string(),
//        );
//
//        let res = create_app(repository).oneshot(req).await.unwrap();
//
//        let todo = res_to_todo(res).await;
//        assert_eq!(expected, todo);
//    }
//
//    #[tokio::test]
//    async fn should_find_todo() {
//        let expected = Todo::new(1, "should_find_todo".to_string());
//
//        let repository = TodoRepositoryForMemory::new();
//
//        repository
//            .create(CreateTodo::new("should_find_todo".to_string()))
//            .await
//            .expect("failed to create a todo");
//
//        let req = build_todo_req_with_empty(Method::GET, "/todos/1");
//
//        let res = create_app(repository).oneshot(req).await.unwrap();
//
//        let todo = res_to_todo(res).await;
//        assert_eq!(expected, todo);
//    }
//
//    #[tokio::test]
//    async fn should_get_all_todos() {
//        let expected = Todo::new(1, "should_get_all_todos".to_string());
//
//        let repository = TodoRepositoryForMemory::new();
//        repository
//            .create(CreateTodo::new("should_get_all_todos".to_string()))
//            .await
//            .expect("failed to create a todo");
//
//        let req = build_todo_req_with_empty(Method::GET, "/todos");
//
//        let res = create_app(repository).oneshot(req).await.unwrap();
//
//        let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
//
//        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
//
//        let todo: Vec<Todo> = serde_json::from_str(&body)
//            .expect(&format!("cannot convert Todo instance. body: {}", body));
//
//        assert_eq!(vec![expected], todo);
//    }
//
//    #[tokio::test]
//    async fn should_update_todo() {
//        let expected = Todo::new(1, "should_update_todo".to_string());
//
//        let repository = TodoRepositoryForMemory::new();
//
//        repository
//            .create(CreateTodo::new("before_update_todo".to_string()))
//            .await
//            .expect("failed to create a todo");
//
//        let req = build_todo_req_with_json(
//            "/todos/1",
//            Method::PATCH,
//            r#"{
//            "id": 1,
//            "text": "should_update_todo",
//            "completed": false
//            }"#
//            .to_string(),
//        );
//
//        let res = create_app(repository).oneshot(req).await.unwrap();
//        let todo = res_to_todo(res).await;
//        assert_eq!(expected, todo);
//    }
//
//    #[tokio::test]
//    async fn should_delete_todo() {
//        let repository = TodoRepositoryForMemory::new();
//
//        repository
//            .create(CreateTodo::new("should_delete_todo".to_string()))
//            .await
//            .expect("failed to create a todo");
//        let req = build_todo_req_with_empty(Method::DELETE, "/todos/1");
//        let res = create_app(repository).oneshot(req).await.unwrap();
//        assert_eq!(StatusCode::NO_CONTENT, res.status());
//    }
//}
