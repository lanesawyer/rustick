use serde::{Deserialize, Serialize};
use yew::{
    format::Json,
    prelude::*,
    services::{storage::Area, StorageService},
};

mod components;
mod lists_list;
mod todo_item;

use components::header::Header;
use lists_list::ListsList;
use rustick::{filter::Filter, list::List, todo::{Tag, Task}};
use todo_item::TodoItem;

const KEY: &str = "rustic.local";

struct Model {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
}

impl Model {
    fn view_task(&self, t: &Task) -> Html {
        html! {
            <TodoItem task=t />
        }
    }
}

#[derive(Serialize, Deserialize)]
struct State {
    tasks: Vec<Task>,
    lists: Vec<List>,
    filters: Vec<Filter>,
    tags: Vec<Tag>,
    new_task: String,
}

enum Msg {
    AddTask(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");

        Self {
            link,
            storage,
            state: State {
                tasks: vec![Task::new("wooo"), Task::new("eeee")],
                filters: vec![Filter{}],
                lists: vec![],
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
                new_task: String::from(""),
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddTask(name) => self.state.tasks.push(Task::new(&name)),
        }

        self.storage.store(KEY, Json(&self.state.tasks));
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {

        let this = html! { 
            <ListsList 
                lists={&self.state.lists}
                filters={&self.state.filters}
                tags={&self.state.tags} 
            />
        };

        html! {
            <>
                <Header />
                {this}
                <main>
                    <h2>{ "Todos" }</h2>
                    <ul class="todoList">
                        // TODO: Group by date, Task needs due_date: DateTime<UTC> field to work
                        { for self.state.tasks.iter().map(|t| self.view_task(&t)) }
                    </ul>
                    <div>
                        <input
                            type="text"
                            value=&self.state.new_task
                        />
                        // <input
                        //     value=&self.state.newTask
                        //     oninput=self.link.callback(|e: InputData| Msg::UpdateNewTask(e.value))
                        // />
                        <button onclick=self.link.callback(|_| Msg::AddTask("hey".to_string()))>{ "Add task" }</button>
                    </div>
                </main>
            </>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

pub fn main() {
    App::<Model>::new().mount_to_body();
}
