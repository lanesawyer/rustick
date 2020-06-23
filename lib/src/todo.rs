use uuid::Uuid;

pub struct Task {
    id: Uuid,
    task_name: String,
    status: Status,
    priority: Priority,
    tags: Vec<Tag>,
    comments: Vec<Comment>
}

impl Default for Task {
    fn default() -> Self {
        Task {
            id: Uuid::new_v4(),
            task_name: "New todo".to_string(),
            status: Status::Open,
            priority: Priority::Wont,
            tags: Default::default(),
            comments: Default::default()
        }
    }
}

impl Task {
    pub fn new(task: &str) -> Task {
        Task {
            task_name: task.to_string(),
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Status {
    Open,
    Complete,
    Archived,
}

#[derive(Copy, Clone)]
pub enum Priority {
    Must,
    Should,
    Could,
    Wont,
}

pub struct Tag {
    text: String
}

pub struct Comment {
    text: String
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_todo_values_are_correct() {
        let todo = Task::default();

        assert_eq!("New todo", todo.task_name);
        assert_eq!(Status::Open, todo.status);
    }

    #[test]
    fn new_todo_has_task() {
        let todo = Task::new("New test todo");

        assert_eq!("New test todo", todo.task_name);
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
