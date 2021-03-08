use yew::prelude::*;

pub struct Header {}

impl Component for Header {
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
            <header>
                <span>
                    <img src="icons/hamburger.svg" />
                    { "Rustick" }
                </span>
                <span>
                    <img src="icons/search.svg" />
                </span>
            </header>
        }
    }
}
