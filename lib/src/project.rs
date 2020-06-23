use crate::todo::{Status, Task};

pub struct Project {
    name: String,
    todos: Vec<Task>,
    status: Status,
}

impl Default for Project {
    fn default() -> Self {
        Project {
            name: "New project".to_string(),
            todos: Vec::new(),
            status: Status::Open,
        }
    }
}

impl Project {
    pub fn new(name: &str) -> Self {
        Project {
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

    pub fn get_archived(self) -> Vec<Task> {
        self.todos
            .iter_mut()
            .filter(|todo| todo.check_status() == Status::Archived)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_project_values_are_correct() {
        let project = Project::default();

        assert_eq!("New project", project.name);
        assert_eq!(0, project.todos.len());
    }

    #[test]
    fn new_uses_provided_string() {
        let project = Project::new("New test project");

        assert_eq!("New test project", project.name);
    }

    #[test]
    fn add_todo_adds_todo() {
        let todo = Task::new("New test todo");
        let todo2 = Task::new("New test todo 2");

        let project = Project::new("New test project")
            .add_todo(todo)
            .add_todo(todo2);

        assert_eq!(2, project.todos.len());
    }

    #[test]
    fn archive_sets_all_todos_to_archived() {
        let todo = Task::new("New test todo");
        let todo2 = Task::new("New test todo 2");

        let project = Project::new("New test project")
            .add_todo(todo)
            .add_todo(todo2)
            .archive();

        assert_eq!(Status::Archived, project.status);

        for todo in project.todos {
            assert_eq!(Status::Archived, todo.check_status());
        }
    }

    #[test]
    fn num_todos_returns_correct_number() {
        let todo = Task::new("New test todo");
        let todo2 = Task::new("New test todo 2");

        let project = Project::new("New test project")
            .add_todo(todo)
            .add_todo(todo2);

        assert_eq!(2, project.num_todos());
    }

    #[test]
    fn search_returns_correct_todos() {
        let todo = Task::new("New test todo");
        let mut todo2 = Task::new("New test todo 2");
        todo2.archive();

        let archived_todos = Project::new("New test project")
            .add_todo(todo)
            .add_todo(todo2)
            .get_archived();

        assert_eq!(1, archived_todos.len());
    }
}
