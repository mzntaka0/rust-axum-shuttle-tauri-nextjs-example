use todos::repository::TodoRepositoryTrait;

// TODO: should be depended on only service..?
#[derive(Clone)]
pub struct TodoDependency<TR>
where
    TR: TodoRepositoryTrait,
{
    pub todo_repository: TR,
}
