use crate::timestamp::Timestamp;

pub struct Todo {
    completed: bool,
    title: String,
    description: String,
    due_date: Timestamp,
    tags: Vec<String>,
    last_revision_date: Timestamp,
    revisions: Vec<TodoRevision>,
}

struct TodoRevision {
    completed: Option<bool>,
    title: Option<String>,
    description: Option<String>,
    due_date: Option<Timestamp>,
    tags: Option<Vec<String>>,
    revision_date: Timestamp,
}
