use coffee_brewing_guide::{Brew, BrewTechnique};
use components::{BrewTable, Title};
use yew::prelude::*;

mod components;

enum Msg {
    ToggleTechnique(BrewTechnique),
    GetCups(u16),
    GetAmountPerCup(u16),
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    cups: u16,
    amount_per_cup: u16,
    brew_technique: BrewTechnique,
    brew: Brew,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            cups: 1,
            amount_per_cup: 250,
            brew_technique: BrewTechnique::PourOver,
            brew: Brew::new(250., 15.),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleTechnique(technique) => {
                self.brew_technique = technique;
            }
            Msg::GetCups(cups) => {
                self.cups = cups;
            }
            Msg::GetAmountPerCup(amount) => {
                self.amount_per_cup = amount;
            }
        }

        self.brew = self
            .brew_technique
            .get_guide(self.cups, self.amount_per_cup);

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
            <div>
                <Title>{ "Coffee Calculator" }</Title>
                <div class="">
                    
                    <BrewTable brew=self.brew></BrewTable> 
                    <div class="w3-theme-l4">
                        <div class="w3-cell-row">
                            <div class="w3-cell w3-padding-small">
                                {self.selector()}
                            </div>
                        </div>
                        <div class="w3-cell-row w3-padding">
                            <div class="w3-cell">
                                {self.cups()}
                            </div>
                            <div class="w3-cell">
                                {self.amount_per_cup()}
                            </div>
                        </div>
                    </div>
                    <div class="w3-panel">
                    </div>
                </div>
                <footer class="w3-container w3-bottom w3-theme w3-margin-top">
                    <p>
                        <a href="https://github.com/jcpst/coffee-brewing-guide">
                            {"source code"}
                        </a>
                    </p>
                </footer>
            </div>
        }
    }
}

impl Model {
    fn selector(&self) -> Html {
        html! {
            <select class="w3-input w3-theme-l4 w3-border-0"
                onchange=self.link.callback(|event| match event {
                    ChangeData::Select(elem) => {
                        let value = elem.selected_index();
                        Msg::ToggleTechnique(match value {
                            0 => BrewTechnique::PourOver,
                            1 => BrewTechnique::AeroPress,
                            2 => BrewTechnique::FilteredIceCoffee,
                            _ => unreachable!()
                        })
                    }
                    _ => unreachable!()
                })>
                <option>{BrewTechnique::PourOver.to_string()}</option>
                <option>{BrewTechnique::AeroPress.to_string()}</option>
                <option>{BrewTechnique::FilteredIceCoffee.to_string()}</option>
            </select>
        }
    }

    fn cups(&self) -> Html {
        html! {
            <>
                <label class="">{"Cups"}</label>
                <input class="w3-input" type="range" min="1" max="12"
                    style="width:12em"
                    value={self.cups.to_string()}
                    oninput=self.link.callback(move |val: InputData| {
                        Msg::GetCups(val.value.parse::<u16>().unwrap())
                    })
                />
                <span class="">{self.cups}</span>
            </>
        }
    }

     fn amount_per_cup(&self) -> Html {
        html! {
            <>
                <label>{"Amount per Cup"}</label>
                <input class="w3-input" type="range" inputmode="numeric"
                    min="100" max="400" step="50" style="width:7em;"
                    value={self.amount_per_cup.to_string()}
                    oninput=self.link.callback(move |val: InputData| {
                        Msg::GetAmountPerCup(val.value.parse::<u16>().unwrap())
                    })
                />
                <span>{self.amount_per_cup}</span>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
