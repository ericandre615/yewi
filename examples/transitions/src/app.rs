use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    KeyboardEvent,
    Callback,
};

use yewi::components::transition::CSSTransition;
use std::time::Duration;

pub struct App {}

pub enum AppMsg {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="app">
                <CSSTransition
                    duration=Duration::from_millis(1200)
                    name="some-list"
                >
                    <ul>
                        <li>{ "One" }</li>
                        <li>{ "Two" }</li>
                        <li>{ "Three" }</li>
                    </ul>
                </CSSTransition>
            </div>
        }
    }
}

