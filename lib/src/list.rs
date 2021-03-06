use crate::todo::{Status, Task};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct List {
    pub name: String,
    pub icon: Option<String>,
    todos: Vec<Task>,
    status: Status,
}

impl Default for List {
    fn default() -> Self {
        List {
            name: "New list".to_string(),
            icon: None,
            todos: Vec::new(),
            status: Status::Open,
        }
    }
}

impl List {
    pub fn new(name: &str) -> Self {
        List {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn add_todo(mut self, todo: Task) -> Self {
        self.todos.push(todo);

        self
    }

    pub fn archive(mut self) -> Self {
        for task in self.todos.iter_mut() {
            task.archive();
        }
        self.status = Status::Archived;

        self
    }

    pub fn num_todos(self) -> usize {
        self.todos.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_list_values_are_correct() {
        let list = List::default();

        assert_eq!("New list", list.name);
        assert_eq!(0, list.todos.len());
    }

    #[test]
    fn new_uses_provided_string() {
        let list = List::new("New test list");

        assert_eq!("New test list", list.name);
    }

    #[test]
    fn add_todo_adds_todo() {
        let todo = Task::new("New test todo");
        let todo2 = Task::new("New test todo 2");

        let list = List::new("New test list")
            .add_todo(todo)
            .add_todo(todo2);

        assert_eq!(2, list.todos.len());
    }

    #[test]
    fn archive_sets_all_todos_to_archived() {
        let todo = Task::new("New test todo");
        let todo2 = Task::new("New test todo 2");

        let list = List::new("New test list")
            .add_todo(todo)
            .add_todo(todo2)
            .archive();

        assert_eq!(Status::Archived, list.status);

        for todo in list.todos {
            assert_eq!(Status::Archived, todo.check_status());
        }
    }

    #[test]
    fn num_todos_returns_correct_number() {
        let todo = Task::new("New test todo");
        let todo2 = Task::new("New test todo 2");

        let list = List::new("New test list")
            .add_todo(todo)
            .add_todo(todo2);

        assert_eq!(2, list.num_todos());
    }
}
