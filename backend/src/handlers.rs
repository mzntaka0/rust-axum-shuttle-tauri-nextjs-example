use axum::{
    async_trait,
    extract::{Extension, FromRequest, Path, Request},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use utoipa;
use validator::Validate;

use crate::repositories::{CreateTodo, TodoRepository, UpdateTodo};

#[derive(Debug)]
pub struct ValidatedJson<T>(T);

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
    responses(
        (status = CREATED, description = "Created Todo found successfully", body = Todo),
        (status = NOT_FOUND, description = "Todo couldn't be created")
    )
)]
pub async fn create_todo<T>(
    Extension(repository): Extension<Arc<T>>,
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
) -> Result<impl IntoResponse, StatusCode>
where
    T: TodoRepository,
{
    let todo = repository
        .create(payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    Ok((StatusCode::CREATED, Json(todo)))
}

#[utoipa::path(
    get,
    path = "/todos/{id}",
    responses(
        (status = 200, description = "Todo was found", body = Todo),
        (status = NOT_FOUND, description = "Todo wasn't found")
    ),
    params(
        ("id" = i32, Path, description = "todo id"),
    )
)]
pub async fn find_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.find(id).await.or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::OK, Json(todo)))
}

#[utoipa::path(
    get,
    path = "/todos",
    responses(
        (status = 200, description = "Todos were found", body = Todo),
        (status = NOT_FOUND, description = "Todos couldn't be found")
    )
)]
pub async fn all_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.all().await.unwrap();
    Ok((StatusCode::OK, Json(todo)))
}

#[utoipa::path(
    patch,
    path = "/todos/{id}",
    responses(
        (status = 200, description = "todo successfully updated", body = Todo),
        (status = NOT_FOUND, description = "todo couldn't be updated")
    ),
    params(
        ("id" = i32, Path, description = "todo id"),
    )
)]
pub async fn update_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
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
        (status = NO_CONTENT, description = "todo couldn't be deleted"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    params(
        ("id" = i32, Path, description = "todo id"),
    )
)]
pub async fn delete_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode {
    repository
        .delete(id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}
