use yew::prelude::*;

pub struct ProjectList {}

impl Component for ProjectList {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <section>
                <h2>{ "Projects" }</h2>
            </section>
        }
    }
}
