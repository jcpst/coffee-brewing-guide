use yew::prelude::*;

pub enum Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

pub struct Title {
    props: Props,
}

impl Component for Title {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _msg: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <header class="w3-card w3-bar w3-theme">
                <h1 class="w3-bar-item">{ for self.props.children.iter() }</h1>
            </header>
        }
    }
}
