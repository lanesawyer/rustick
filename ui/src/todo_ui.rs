use rustick::todo::{Status, Task};
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew::Properties;

pub struct TodoUi {
    link: ComponentLink<Self>,
    props: TodoUiProps,
    task: Task,
    editing: bool,
    new_description: String,
}

impl TodoUi {
    fn view_entry_edit_input(&self, id: u32) -> Html {
        if self.editing {
            html! {
                <input class="edit"
                    type="text"
                    value=&self.new_description
                    oninput=self.link.callback(|e: InputData| TodoUiMsg::UpdateEdit(e.value))
                    onblur=self.link.callback(move |_| TodoUiMsg::Edit(id))
                    onkeypress=self.link.callback(move |e: KeyboardEvent| {
                        if e.key() == "Enter" { TodoUiMsg::Edit(id) } else { TodoUiMsg::Ignore }
                    }) />
            }
        } else {
            html! { <input type="hidden" /> }
        }
    }
}

pub enum TodoUiMsg {
    Add,
    Edit(u32),
    Update(String),
    UpdateEdit(String),
    Remove(usize),
    //SetFilter(Filter),
    ToggleAll,
    ToggleEdit(u32),
    Toggle(u32),
    ClearCompleted,
    Ignore,
}

#[derive(Properties, Clone, Serialize, Deserialize)]
pub struct TodoUiProps {
    pub task: Task,
}

impl Component for TodoUi {
    type Message = TodoUiMsg;
    type Properties = TodoUiProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            task: Task::new("gah"),
            editing: false,
            new_description: "hm".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TodoUiMsg::Add => {}
            TodoUiMsg::Edit(e) => {}
            TodoUiMsg::Update(_) => {}
            TodoUiMsg::UpdateEdit(_) => {}
            TodoUiMsg::Remove(_) => {}
            TodoUiMsg::ToggleAll => {}
            TodoUiMsg::ToggleEdit(_) => {}
            TodoUiMsg::Toggle(_) => {}
            TodoUiMsg::ClearCompleted => {}
            TodoUiMsg::Ignore => {}
        }
        //self.storage.store(KEY, Json(&self.state.entries));
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let classes = match self.props.task.clone().check_status() {
            Status::Complete => vec!["complete".to_string()],
            _ => Vec::new(),
        };

        let checked = match self.props.task.clone().check_status() {
            Status::Complete => true,
            _ => false,
        };

        let id = self.props.task.id;

        html! {
            <li>
                <input
                    type="checkbox"
                    class=classes
                    checked=checked />

                <label
                    for=id
                    onclick=self.link.callback(move |_| TodoUiMsg::ToggleEdit(id)) >
                    { &self.props.task.description }
                </label>
                                    // <input class="new-todo"
                    //     placeholder="What's next?"
                    //     value=&self.value
                    //     oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                    //     onkeypress=self.link.callback(|e: KeyboardEvent| {
                    //         if e.key() == "Enter" { Msg::Add } else { Msg::Nope }
                    //     }) />
                { self.view_entry_edit_input(self.task.id) }
            </li>
        }
    }
}
