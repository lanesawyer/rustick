use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod components;
mod todo_ui;

use components::{Header, Footer};
use todo_ui::TodoUi;

struct Model {
    link: ComponentLink<Self>,
    state: State,
}

struct State {
    tasks: Vec<TodoUi>
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            state: State {
                tasks: Vec::new()
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => print!("todo")
        }

        let window: web_sys::Window = web_sys::window().expect("window not available");
        window.alert_with_message("hello from wasm!").expect("alert failed");

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
                <TodoUi completed=true />
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
