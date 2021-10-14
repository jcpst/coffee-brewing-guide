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
            <div class="w3-container">

                <Title>{ "Coffee Brewing Guide" }</Title>

                { self.view_brew_technique_buttons() }

                <div class="w3-panel w3-light-blue w3-round-large">
                    <div class="w3-row-padding">
                        <div class="w3-half">
                            <label>{"Cups"}</label>
                            <input class="w3-input" type="range" min="1" max="12"
                                value={self.cups.to_string()}
                                oninput=self.link.callback(move |val: InputData| {
                                    Msg::GetCups(val.value.parse::<u16>().unwrap())
                                })
                            />
                            <span>{self.cups}</span>
                        </div>
                        <div class="w3-half">
                            <label>{"Amount per Cup"}</label>
                            <input class="w3-input" type="number"
                                value={self.amount_per_cup.to_string()}
                                oninput=self.link.callback(move |val: InputData| {
                                    Msg::GetAmountPerCup(val.value.parse::<u16>().unwrap())
                                })
                            />
                        </div>
                    </div>
                </div>

                <BrewTable brew=self.brew></BrewTable>
                
            </div>
        }
    }
}

impl Model {
    fn view_brew_technique_buttons(&self) -> Html {
        html! {
            <div>
                {self.brew_button(BrewTechnique::PourOver)}
                {self.brew_button(BrewTechnique::AeroPress)}
                {self.brew_button(BrewTechnique::FilteredIceCoffee)}
            </div>
        }
    }

    fn brew_button(&self, technique: BrewTechnique) -> Html {
        let mut classes = String::from("w3-bar-item w3-button");

        if self.brew_technique == technique {
            classes.push_str(" w3-white w3-border w3-large")
        } else {
            classes.push_str(" w3-black")
        }

        html! {
            <button class={classes} onclick=self.link.callback(move |_|
                Msg::ToggleTechnique(technique)
            )>
                {technique.to_string()}
            </button>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
