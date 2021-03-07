use serde::{Deserialize, Serialize};
use yew::{
    format::Json,
    prelude::*,
    services::{storage::Area, StorageService},
};

mod components;
mod project_list;
mod todo_item;

use components::{Footer, Header};
use project_list::ProjectList;
use rustick::todo::Task;
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
    new_task: String,
}

enum Msg {
    AddTask,
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
                new_task: String::from(""),
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddTask => print!("todo"),
        }

        let window: web_sys::Window = web_sys::window().expect("window not available");
        window
            .alert_with_message("hello from wasm!")
            .expect("alert failed");

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
        html! {
            <>
                <Header />
                <ProjectList />
                <main>
                    <h1>{ "Todos" }</h1>
                    <ul class="todo-list">
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
                        <button onclick=self.link.callback(|_| Msg::AddTask)>{ "Add task" }</button>
                    </div>
                </main>
                <Footer />
            </>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

pub fn main() {
    App::<Model>::new().mount_to_body();
}
