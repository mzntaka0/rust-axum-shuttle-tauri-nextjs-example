use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use todos::repository::TodoRepositoryForDb;
use todos::service::TodoService;

use super::controller;
use super::dependency::TodoDependency;

// TODO: change pool type for any db. is it better way to input pool for routes? injesting state is
// more better..?
// injesting state itself is more flexible for changing repository impl
// TODO: probably specifying TodoRepositoryForDb is not correct in terms of more general coding
pub fn routes(pool: PgPool) -> Router {
    let todo_repository = TodoRepositoryForDb::new(pool);
    let dependency = TodoDependency {
        // TODO: replace w/ Arc
        todo_service: TodoService::new(todo_repository.clone()),
        todo_repository,
    };
    Router::new()
        .nest(
            "/todos",
            Router::new()
                .route(
                    "/",
                    post(controller::create::<TodoRepositoryForDb>)
                        .get(controller::find_all::<TodoRepositoryForDb>),
                )
                .route(
                    "/:id",
                    get(controller::find::<TodoRepositoryForDb>)
                        .delete(controller::delete::<TodoRepositoryForDb>)
                        .patch(controller::update::<TodoRepositoryForDb>),
                ),
        )
        .with_state(dependency)
}
