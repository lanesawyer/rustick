use rustick::todo::{Status, Task};
use serde::{Deserialize, Serialize};
use yew::{prelude::*, services::ConsoleService, virtual_dom::VNode};
use yew::Properties;

pub struct TodoItem {
    link: ComponentLink<Self>,
    props: TodoUiProps,
    task: Task,
    editing: bool,
    new_description: String,
}

impl TodoItem {
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
            html! { <img src="icons/ellypsis-vertical.svg" /> }
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

impl Component for TodoItem {
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
            TodoUiMsg::Edit(_e) => {}
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
        let task = &self.props.task;
        
        let classes = match task.clone().check_status() {
            Status::Complete => vec!["complete".to_string()],
            _ => Vec::new(),
        };


        let checked = match task.clone().check_status() {
            Status::Complete => true,
            Status::Open | Status::Archived => false,
        };

        let id = task.id;
        let priority = match task.priority {
            Must => ("1", "Must"),
            Should => ("2", "Should"),
            Could => ("3", "Could"),
            Wont => ("4", "Wont"),
        };
        let priority_class = format!("todoItem-priority{}", priority.0);

        let tags = task.tags.iter().map(|t| -> Html { 
            html!{ 
                <span class="tag" style={format!("background-color: {}", &t.color)}>{&t.text}</span> 
            }
        }).collect::<VNode>();
        
        ConsoleService::info(&format!("task: {:?}", task));

        let details = html! {
            <span class="todoItem-details">
                <span class="todoItem-detailsContainer">
                    <img src="icons/calendar-event.svg" />
                    <span>{"Due Date"}</span>
                    <img src="icons/flag.svg" />
                    <span class={priority_class}>{ priority.1 }</span>
                    <img src="icons/tag.svg" />
                    { tags }
                </span>
                <span class="todoItem-project">{&task.project_id}</span>
            </span>
        };
        
        html! {
            <li class="todoItem">
                <span class="todoItem-todo">
                    <span>
                        <input
                            type="checkbox"
                            class=classes
                            checked=checked />
                        <label
                            for=id
                            onclick=self.link.callback(move |_| TodoUiMsg::ToggleEdit(id)) >
                            { &task.description }
                        </label>
                    </span>
                    { self.view_entry_edit_input(self.task.id) }
                </span>
                { details }
                // <input class="new-odo"
                //     placeholder="What's next?"
                //     value=&self.value
                //     oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                //     onkeypress=self.link.callback(|e: KeyboardEvent| {
                //         if e.key() == "Enter" { Msg::Add } else { Msg::Nope }
                //     }) />
            </li>
        }
    }
}