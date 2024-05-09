//use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRequest, Path, Request, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::de::DeserializeOwned;
use utoipa;
use validator::Validate;

// TODO: move this to shared
use todos::model::{CreateTodo, UpdateTodo};
use todos::repository::{TodoRepositoryForDb, TodoRepositoryTrait};
use todos::service::{TodoService, TodoServiceTrait};

use super::dependency::TodoDependency;

#[derive(Debug)]
pub struct ValidatedJson<T>(T);

// TODO: move to more general place
#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|rejection| {
                let message = format!("Json parse error: [{}]", rejection);
                (StatusCode::BAD_REQUEST, message)
            })?;
        value.validate().map_err(|rejection| {
            let message = format!("Validation error: [{}]", rejection).replace('\n', ", ");
            (StatusCode::BAD_REQUEST, message)
        })?;
        Ok(ValidatedJson(value))
    }
}

#[utoipa::path(
    post,
    path = "/todos",
    request_body = CreateTodo,
    responses(
        (status = CREATED, description = "Created Todo successfully", body = Todo),
        (status = NOT_FOUND, description = "Todo couldn't be created")
    )
)]
pub async fn create<T>(
    // TODO: Refactor generics
    State(state): State<TodoDependency<TodoService<TodoRepositoryForDb>, TodoRepositoryForDb>>,
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
) -> Result<impl IntoResponse, StatusCode>
where
    T: TodoRepositoryTrait,
{
    let todo = state
        .todo_service
        .create(payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::CREATED, Json(todo)))
}

#[utoipa::path(
    get,
    path = "/todos/{id}",
    responses(
        (status = 200, description = "Todo found", body = Todo),
        (status = NOT_FOUND, description = "Todo not found")
    ),
    params(
        ("id" = i32, Path, description = "todo id"),
    )
)]
pub async fn find<T: TodoRepositoryTrait>(
    Path(id): Path<i32>,
    State(state): State<TodoDependency<TodoService<TodoRepositoryForDb>, TodoRepositoryForDb>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = state
        .todo_service
        .find(id)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::OK, Json(todo)))
}

#[utoipa::path(
    get,
    path = "/todos",
    responses(
        (status = 200, description = "Todos found", body = Vec<Todo>),
        (status = NOT_FOUND, description = "Todos not found")
    )
)]
pub async fn find_all<T: TodoRepositoryTrait>(
    State(state): State<TodoDependency<TodoService<TodoRepositoryForDb>, TodoRepositoryForDb>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = state.todo_service.find_all().await.unwrap();
    Ok((StatusCode::OK, Json(todo)))
}

#[utoipa::path(
    patch,
    path = "/todos/{id}",
    request_body = UpdateTodo,
    responses(
        (status = 200, description = "todo successfully updated", body = Todo),
        (status = NOT_FOUND, description = "todo not updated")
    ),
    params(
        ("id" = i32, Path, description = "todo id"),
    )
)]
pub async fn update<T: TodoRepositoryTrait>(
    State(state): State<TodoDependency<TodoService<TodoRepositoryForDb>, TodoRepositoryForDb>>,
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = state
        .todo_service
        .update(id, payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::OK, Json(todo)))
}

#[utoipa::path(
    delete,
    path = "/todos/{id}",
    responses(
        (status = 200, description = "todo successfully deleted", body = Todo),
        (status = NO_CONTENT, description = "todo not deleted"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    params(
        ("id" = i32, Path, description = "todo id"),
    )
)]
pub async fn delete<T: TodoRepositoryTrait>(
    Path(id): Path<i32>,
    State(state): State<TodoDependency<TodoService<TodoRepositoryForDb>, TodoRepositoryForDb>>,
) -> StatusCode {
    state
        .todo_service
        .delete(id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}
