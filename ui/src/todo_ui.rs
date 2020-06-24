use yew::prelude::*;
use yew::Properties;

#[derive(Clone)]
pub struct TodoUi {
    link: ComponentLink<Self>,
    props: TodoUiProps,
    editing: bool,
    new_description: String
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

#[derive(Properties, Clone, PartialEq)]
pub struct TodoUiProps {
    #[prop_or(false)]
    pub completed: bool,
    #[prop_or("New task".to_string())]
    title: String
}

impl Component for TodoUi {
    type Message = TodoUiMsg;
    type Properties = TodoUiProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            editing: false,
            new_description: "".to_string()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TodoUiMsg::Add => {}
            TodoUiMsg::Edit(_) => {}
            TodoUiMsg::Update(_) => {}
            TodoUiMsg::UpdateEdit(_) => {}
            TodoUiMsg::Remove(_) => {}
            TodoUiMsg::ToggleAll => {}
            TodoUiMsg::ToggleEdit(_) => {}
            TodoUiMsg::Toggle(_) => {}
            TodoUiMsg::ClearCompleted => {}
            TodoUiMsg::Ignore => {}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <label>
                    <input 
                        type="checkbox" 
                        class=if self.props.completed { vec!["complete".to_string()] } else { Vec::new() }
                        checked=self.props.completed />
                        { &self.props.title }
                </label>
                                    // <input class="new-todo"
                    //     placeholder="What's next?"
                    //     value=&self.value
                    //     oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                    //     onkeypress=self.link.callback(|e: KeyboardEvent| {
                    //         if e.key() == "Enter" { Msg::Add } else { Msg::Nope }
                    //     }) />
                { self.view_entry_edit_input(12) }
            </div>
        }
    }
}