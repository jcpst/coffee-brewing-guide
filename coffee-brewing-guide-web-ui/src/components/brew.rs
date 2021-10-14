use coffee_brewing_guide::Brew;
use yew::prelude::*;
pub enum Msg {}

#[derive(Properties, Clone)]
pub struct Props {
    pub brew: Brew,
}

pub struct BrewTable {
    props: Props,
}

impl Component for BrewTable {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <table class="w3-table-all w3-half">
                <tr>
                    <th>{"Water"}</th>
                    <th>{"Beans"}</th>
                </tr>
                <tr>
                    <td>{self.props.brew.water}</td>
                    <td>{self.props.brew.beans}</td>
                </tr>
            </table>
        }
    }
}
