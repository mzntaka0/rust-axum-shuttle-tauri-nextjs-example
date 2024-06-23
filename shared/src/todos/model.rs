use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

// TODO: split models and dtos. remove ToSchema
// consider to include validate
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, FromRow, ToSchema)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate, ToSchema)]
pub struct CreateTodo {
    #[validate(length(min = 1, max = 100, message = "Can not be empty and over text length"))]
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate, ToSchema)]
pub struct UpdateTodo {
    #[validate(length(min = 1, max = 100, message = "Can not be empty and over text length"))]
    pub text: Option<String>,
    pub completed: Option<bool>,
}
