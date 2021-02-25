use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    InputData,
    Callback,
};

use crate::utils::generate_unique_id;
use crate::components::form::LabeledInput;

#[derive(Properties, Clone, Debug)]
pub struct PasswordInputProps {
    #[prop_or(generate_unique_id())]
    pub id: String,
    #[prop_or(String::new())]
    pub name: String,
    #[prop_or(String::new())]
    pub initial_value: String,
    #[prop_or(String::new())]
    pub value: String,
    #[prop_or(String::new())]
    pub placeholder: String,
    #[prop_or(String::new())]
    pub label: String,
    #[prop_or(String::new())]
    pub class: String,
    #[prop_or(Callback::noop())]
    pub handle_change: Callback<String>,
}

pub struct PasswordInput {
    link: ComponentLink<Self>,
    props: PasswordInputProps,
    value: String,
}

pub enum Msg {
    UpdateValue(InputData),
}


impl Component for PasswordInput {
    type Message = Msg;
    type Properties = PasswordInputProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let value = if !props.value.is_empty() {
            props.value.clone()
        } else {
            props.initial_value.clone()
        };

        Self {
            link,
            props,
            value,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateValue(change) => {
                self.value = change.value.clone();
                self.props.handle_change.emit(self.value.clone());
            },
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.value != props.value {
            self.props.value = props.value.clone();
            self.value = props.value;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let id = self.props.id.clone();
        let name = self.props.name.clone();
        let placeholder = self.props.placeholder.clone();
        let classes = self.props.class.clone();
        let label_id = self.props.id.clone();
        let label = self.props.label.clone();
        let value = self.value.clone();

        html! {
            <LabeledInput label=label id=label_id>
                <input
                    type="password"
                    id=id
                    name=name
                    class=format!("yewi-text-input yewi-password-input {}", classes)
                    oninput=self.link.callback(|v: InputData| Msg::UpdateValue(v))
                    value={ value }
                    placeholder={ placeholder }
                />
            </LabeledInput>
        }
    }
}
