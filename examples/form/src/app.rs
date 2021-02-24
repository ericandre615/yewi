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
    FormState,
    Label,
    inputs::{
        TextInput,
        Checkbox,
        Radio,
        RadioGroup,
    }
};

pub struct App {
    link: ComponentLink<Self>,
}

pub enum AppMsg {
    FormSubmit(FormState),
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("Starting Form3 Example");
        Self {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMsg::FormSubmit(form_data) => {
                ConsoleService::info(&format!("FormDataState {:?}", form_data));
            }
        }

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
                    handle_submit=self.link.callback(AppMsg::FormSubmit)
                >
                    <Label label_for="my-input-id">{ "Do it" }</Label>
                    <TextInput id="my-input-id" name="my-input" placeholder="Enter Text" />
                    <TextInput id="label-input" name="label-input" placeholder="Labeled Input" label="Label My Input" />

                    <Label label_for="my-checkbox-id">{ "Check it" }</Label>
                    <Checkbox id="my-checkbox-id" name="my-checkbox" value="CheckingIt" />
                    <Checkbox id="withed-label" name="better-checkbox" value="CheckItWithLabels" label="Label Check" />
                    <RadioGroup name="gender">
                        <Radio id="mail" value="male" name="other-radio-omitted" />
                        <Radio id="femail" value="female" label="female" />
                    </RadioGroup>
                    <button type="submit">{ "Submit" }</button>
                </Form>
            </div>
        }
    }
}

