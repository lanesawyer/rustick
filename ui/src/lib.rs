use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::{
    format::Json,
    prelude::*,
    services::{storage::Area, StorageService},
};

mod components;
mod todo_ui;

use components::{Footer, Header};
use rustick::todo::Task;
use todo_ui::TodoUi;

const KEY: &str = "rustic.local";

struct Model {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
}

impl Model {
    fn view_task(&self, t: &Task) -> Html {
        html! {
            <TodoUi task=t />
        }
    }
}

#[derive(Serialize, Deserialize)]
struct State {
    tasks: Vec<Task>,
}

enum Msg {
    AddOne,
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
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => print!("todo"),
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
                <h1>{ "Todos" }</h1>
                <ul class="todo-list">
                    { for self.state.tasks.iter().map(|t| self.view_task(&t)) }
                </ul>
                <div>
                    <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                </div>
                <Footer />
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
