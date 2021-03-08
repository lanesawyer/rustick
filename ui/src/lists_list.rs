use rustick::{filter::Filter, list::List, todo::Tag};
use yew::prelude::*;
use serde::{Deserialize, Serialize};
use yew::Properties;

pub struct ListsList {
    props: ListsListProps,
}

#[derive(Properties, Clone, Serialize, Deserialize)]
pub struct ListsListProps {
    pub lists: Vec<List>,
    pub filters: Vec<Filter>,
    pub tags: Vec<Tag>,
}


impl Component for ListsList {
    type Message = ();
    type Properties = ListsListProps;
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            props
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let lists = html! {
            <>
                <h2>{ "Lists" }</h2>
                { for self.props.lists.iter().map(|l| html!{ <div>{"list"}</div>}) }
            </>
        };

        html! {
            <section>
                {lists}
                <h2>{ "Filters" }</h2>
                { for self.props.filters.iter().map(|f| html!{ <div>{"filter"}</div>}) }
                <h2>{ "Tags" }</h2>
                { for self.props.tags.iter().map(|t| html!{ <div>{"tag"}</div>}) }
            </section>
        }
    }
}
