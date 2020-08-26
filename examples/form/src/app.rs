use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    KeyboardEvent,
    Callback,
};
use yew::services::ConsoleService;

use yewi::components::form::{
    Form,
    FormMethod,
    Label,
    inputs::{
        TextInput,
        Checkbox,
    }
};

pub struct App {
    link: ComponentLink<Self>,
}

pub enum AppMsg {
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("Starting Form3 Example");
        Self {
            link,
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
                <Form
                    method=FormMethod::Post
                >
                    <Label label_for="my-input-id">{ "Do it" }</Label>
                    <TextInput id="my-input-id" name="my-input" placeholder="Enter Text" />
                    <Label label_for="my-checkbox-id">{ "Check it" }</Label>
                    <Checkbox id="my-checkbox-id" name="my-checkbox" value="CheckingIt" />
                    <button type="submit">{ "Submit" }</button>
                </Form>
            </div>
        }
    }
}

