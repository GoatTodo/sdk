#![allow(dead_code)]

use crate::timestamp::Timestamp;

pub struct Todo {
    title: String,
    description: Option<String>,
    completed: bool,
    due_date: Option<Timestamp>,
    tags: Vec<String>,
    revision_date: Timestamp,
    revisions: Vec<TodoRevision>,
}

struct TodoRevision {
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
    due_date: Option<Timestamp>,
    tags: Option<Vec<String>>,
    revision_date: Timestamp,
}

impl Todo {
    /// Creates a new Todo.
    pub fn new(
        title: String,
        description: Option<String>,
        completed: bool,
        due_date: Option<Timestamp>,
        tags: Option<Vec<String>>,
    ) -> Result<Self, String> {
        let current_timestamp = match Timestamp::now() {
            Ok(t) => t,
            Err(e) => {
                let mut error_message =
                    String::from("Error: Couldn't create a Todo due to a Timestamp error...\n  ");
                error_message.push_str(e.as_str());

                return Err(error_message);
            }
        };

        let first_revision = TodoRevision {
            title: Some(title.clone()),
            description: description.clone(),
            completed: Some(completed),
            due_date: due_date.clone(),
            tags: tags.clone(),
            revision_date: current_timestamp,
        };

        let revisions = vec![first_revision];

        Ok(Self {
            title,
            description,
            completed: completed,
            due_date,
            tags: tags.unwrap_or_default(),
            revision_date: current_timestamp,
            revisions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        const UNIX_EPOCH: u64 = 0;

        let title = String::from("🦮 Walk the dog");
        let description = Some(String::from("Be sure to walk the dog before it rains!"));
        let completed = false;
        let due_date = None;
        let tags = vec![String::from("my-list"), String::from("dog-related")];

        let new_todo = Todo::new(
            title.clone(),
            description.clone(),
            completed,
            due_date,
            Some(tags.clone()),
        )
        .unwrap();

        assert_eq!(title, new_todo.title);
        assert_eq!(description, new_todo.description);
        assert_eq!(completed, new_todo.completed);
        assert_eq!(true, new_todo.due_date.is_none());
        assert_eq!(tags, new_todo.tags);
        assert_eq!(
            true,
            new_todo.revision_date.get_current_timestamp() > UNIX_EPOCH
        );
        assert_eq!(1, new_todo.revisions.len());
    }
}
