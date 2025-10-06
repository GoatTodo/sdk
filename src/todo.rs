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

pub struct TodoRevision {
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
        revision_date: Option<Timestamp>,
    ) -> Result<Self, String> {
        let revision_timestamp = match revision_date {
            Some(t) => t,
            None => match Timestamp::now() {
                Ok(t) => t,
                Err(e) => {
                    let mut error_message = String::from(
                        "Error: Couldn't create a Todo due to a Timestamp error...\n  ",
                    );
                    error_message.push_str(e.as_str());

                    return Err(error_message);
                }
            },
        };

        let first_revision = TodoRevision {
            title: Some(title.clone()),
            description: description.clone(),
            completed: Some(completed),
            due_date,
            tags: tags.clone(),
            revision_date: revision_timestamp,
        };

        let revisions = vec![first_revision];

        Ok(Self {
            title,
            description,
            completed,
            due_date,
            tags: tags.unwrap_or_default(),
            revision_date: revision_timestamp,
            revisions,
        })
    }

    /// Adds a revision to a todo. This is how a user might "edit" the todo.
    pub fn add_revision(&mut self, revision: TodoRevision) -> Result<(), ()> {
        if self.revision_date.get() >= revision.revision_date.get() {
            return Err(());
        }

        let revision_title = if let Some(revision_title) = revision.title {
            if self.title != revision_title {
                self.title = revision_title.clone();
                Some(revision_title)
            } else {
                None
            }
        } else {
            None
        };

        let revision_description = if let Some(revision_description) = revision.description {
            if let Some(current_description) = &self.description {
                if *current_description != revision_description {
                    self.description = Some(revision_description.clone());
                    Some(revision_description)
                } else {
                    None
                }
            } else {
                self.description = Some(revision_description.clone());
                Some(revision_description)
            }
        } else {
            None
        };
        let revision_completed = if let Some(revision_completed) = revision.completed {
            if self.completed != revision_completed {
                self.completed = revision_completed;
                Some(revision_completed)
            } else {
                None
            }
        } else {
            None
        };

        let revision_due_date = if let Some(revision_due_date) = revision.due_date {
            if let Some(current_due_date) = self.due_date {
                if current_due_date.get() != revision_due_date.get() {
                    self.due_date = Some(revision_due_date);
                    Some(revision_due_date)
                } else {
                    None
                }
            } else {
                self.due_date = Some(revision_due_date);
                Some(revision_due_date)
            }
        } else {
            None
        };

        // assume that the current tags are sorted, but the revision_tags are not
        let revision_tags = if let Some(mut revision_tags) = revision.tags {
            if revision_tags.is_empty() {
                None
            } else if self.tags.is_empty() || self.tags != revision_tags {
                revision_tags.sort();
                self.tags = revision_tags.clone();
                Some(revision_tags)
            } else {
                None
            }
        } else {
            None
        };

        self.revisions.push(TodoRevision::new(
            revision_title,
            revision_description,
            revision_completed,
            revision_due_date,
            revision_tags,
            revision.revision_date,
        ));
        Ok(())
    }
}

impl TodoRevision {
    fn new(
        title: Option<String>,
        description: Option<String>,
        completed: Option<bool>,
        due_date: Option<Timestamp>,
        tags: Option<Vec<String>>,
        revision_date: Timestamp,
    ) -> Self {
        Self {
            title,
            description,
            completed,
            due_date,
            tags,
            revision_date,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

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
            None,
        )
        .unwrap();

        assert_eq!(title, new_todo.title);
        assert_eq!(description, new_todo.description);
        assert_eq!(completed, new_todo.completed);
        assert!(new_todo.due_date.is_none());
        assert_eq!(tags, new_todo.tags);
        assert!(new_todo.revision_date.get() > UNIX_EPOCH);
        assert_eq!(1, new_todo.revisions.len());
    }

    #[test]
    fn test_new_with_explicit_timestamp() {
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
            Some(Timestamp::now().unwrap()),
        )
        .unwrap();

        assert_eq!(title, new_todo.title);
        assert_eq!(description, new_todo.description);
        assert_eq!(completed, new_todo.completed);
        assert!(new_todo.due_date.is_none());
        assert_eq!(tags, new_todo.tags);
        assert!(new_todo.revision_date.get() > UNIX_EPOCH);
        assert_eq!(1, new_todo.revisions.len());
    }

    #[test]
    fn test_new_revision_title() {
        let title = String::from("🦮 Walk the dog");
        let revision_title = String::from("🦮 🦮 Walk MORE dogs");
        let description = Some(String::from("Be sure to walk the dog before it rains!"));
        let completed = false;
        let due_date = None;
        let tags = vec![String::from("my-list"), String::from("dog-related")];

        let mut new_todo = Todo::new(
            title.clone(),
            description.clone(),
            completed,
            due_date,
            Some(tags.clone()),
            None,
        )
        .unwrap();

        thread::sleep(Duration::from_secs(1));
        let now = Timestamp::now().unwrap();

        let new_revision =
            TodoRevision::new(Some(revision_title.clone()), None, None, None, None, now);

        assert!(new_todo.add_revision(new_revision).is_ok());
        assert_eq!(revision_title, new_todo.title);
        assert_eq!(description, new_todo.description);
        assert_eq!(completed, new_todo.completed);
        assert!(new_todo.due_date.is_none());
        assert_eq!(tags, new_todo.tags);
        assert_eq!(2, new_todo.revisions.len());
    }

    #[test]
    fn test_new_revision_description() {
        let title = String::from("🦮 Walk the dog");
        let description = Some(String::from("Be sure to walk the dog before it rains!"));
        let revision_description = Some(String::from("Walk the dog before it snows! 🌨️"));
        let completed = false;
        let due_date = None;
        let tags = vec![String::from("my-list"), String::from("dog-related")];

        let mut new_todo = Todo::new(
            title.clone(),
            description.clone(),
            completed,
            due_date,
            Some(tags.clone()),
            None,
        )
        .unwrap();

        thread::sleep(Duration::from_secs(1));
        let now = Timestamp::now().unwrap();

        let new_revision =
            TodoRevision::new(None, revision_description.clone(), None, None, None, now);

        assert!(new_todo.add_revision(new_revision).is_ok());
        assert_eq!(title, new_todo.title);
        assert_eq!(revision_description.unwrap(), new_todo.description.unwrap());
        assert_eq!(completed, new_todo.completed);
        assert!(new_todo.due_date.is_none());
        assert_eq!(tags, new_todo.tags);
        assert_eq!(2, new_todo.revisions.len());
    }
}
