//use std::sync::Arc;

use axum::async_trait;

// TODO: move this to shared
use super::model::{CreateTodo, Todo, UpdateTodo};
use super::repository::TodoRepositoryTrait;

#[derive(Debug, Clone)]
pub struct TodoService<TR>
where
    TR: TodoRepositoryTrait,
{
    todo_repository: TR,
}

#[async_trait]
pub trait TodoServiceTrait
where
    Self: Clone + std::marker::Send + std::marker::Sync + 'static,
{
    async fn create(&self, payload: CreateTodo) -> Result<Todo, &str>;
    async fn find(&self, id: i32) -> Result<Todo, &str>;
    async fn find_all(&self) -> Result<Vec<Todo>, &str>;
    async fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo, &str>;
    async fn delete(&self, id: i32) -> Result<&str, &str>;
}

impl<TR> TodoService<TR>
where
    TR: TodoRepositoryTrait,
{
    pub fn new(todo_repository: TR) -> Self {
        Self { todo_repository }
    }
}

#[async_trait]
impl<TR> TodoServiceTrait for TodoService<TR>
where
    TR: TodoRepositoryTrait,
{
    async fn create(&self, payload: CreateTodo) -> Result<Todo, &str> {
        let todo = self
            .todo_repository
            .create(payload)
            .await
            .or(Err("couldn't create a todo"))?;

        Ok(todo)
    }

    async fn find(&self, id: i32) -> Result<Todo, &str> {
        let todo = self
            .todo_repository
            .find(id)
            .await
            .or(Err("couldn't find the todo"))?;
        Ok(todo)
    }

    async fn find_all(&self) -> Result<Vec<Todo>, &str> {
        let todo = self.todo_repository.all().await.unwrap();
        Ok(todo)
    }

    async fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo, &str> {
        let todo = self
            .todo_repository
            .update(id, payload)
            .await
            .or(Err("couldn't update the todo"))?;
        Ok(todo)
    }

    async fn delete(&self, id: i32) -> Result<&str, &str> {
        Ok(self
            .todo_repository
            .delete(id)
            .await
            .map(|_| "todo was not found")
            .unwrap_or("error"))
    }
}
