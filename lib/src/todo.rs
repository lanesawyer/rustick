use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    status: Status,
    pub priority: Priority,
    pub tags: Vec<Tag>,
    comments: Vec<Comment>,
    pub list_id: u32,
    pub due_date: DateTime<Utc>,
    created_date: DateTime<Utc>,
    modified_date: DateTime<Utc>,
}

impl Default for Task {
    fn default() -> Self {
        Task {
            id: 0, // Uuid::new_v4(), - using a simpler value so I can get rid of some compiler errors
            description: "New todo".to_string(),
            status: Status::Open,
            priority: Priority::P4,
            tags: Default::default(),
            comments: Default::default(),
            list_id: Default::default(),
            due_date: Utc::now(),
            created_date: Utc::now(),
            modified_date: Utc::now(),
        }
    }
}

impl Task {
    pub fn new(task: &str) -> Task {
        Task {
            description: task.to_string(),
            tags: vec![
                Tag {
                    text: "heyo".to_string(),
                    color: "#AABBCC".to_string(),
                },
                Tag {
                    text: "there".to_string(),
                    color: "#DDEEFF".to_string(),
                }
            ],
            ..Default::default()
        }
    }

    pub fn open(&mut self) {
        self.status = Status::Open;
    }

    pub fn complete(&mut self) {
        self.status = Status::Complete;
    }

    pub fn archive(&mut self) {
        self.status = Status::Archived;
    }

    pub fn check_status(self) -> Status {
        self.status
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Open,
    Complete,
    Archived,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Priority {
    P1,
    P2,
    P3,
    P4,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tag {
    pub text: String,
    pub color: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Comment {
    text: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_todo_values_are_correct() {
        let todo = Task::default();

        assert_eq!("New todo", todo.description);
        assert_eq!(Status::Open, todo.status);
    }

    #[test]
    fn new_todo_has_task() {
        let todo = Task::new("New test todo");

        assert_eq!("New test todo", todo.description);
    }

    #[test]
    fn open_todo_is_opened() {
        let mut todo = Task::new("Open task");
        todo.open();

        assert_eq!(Status::Open, todo.status);
    }

    #[test]
    fn completed_todo_is_completed() {
        let mut todo = Task::new("Completed task");
        todo.complete();

        assert_eq!(Status::Complete, todo.status);
    }

    #[test]
    fn archived_todo_is_archived() {
        let mut todo = Task::new("Archived task");
        todo.archive();

        assert_eq!(Status::Archived, todo.status);
    }
}
