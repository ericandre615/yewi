use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    ChangeData,
    MouseEvent,
    Callback,
};

use crate::utils::generate_unique_id;
use crate::components::form::LabeledInput;

#[derive(Properties, Clone, PartialEq)]
pub struct RadioProps {
    #[prop_or(generate_unique_id())]
    pub id: String,
    #[prop_or(String::new())]
    pub name: String,
    pub value: String,
    #[prop_or(String::new())]
    pub placeholder: String,
    #[prop_or(String::new())]
    pub label: String,
    #[prop_or(String::new())]
    pub class: String,
    #[prop_or(false)]
    pub checked: bool,
    #[prop_or(Callback::noop())]
    pub handle_change: Callback<bool>,
}

pub struct Radio {
    link: ComponentLink<Self>,
    props: RadioProps,
    checked: bool,
}

pub enum Msg {
    ToggleChecked,
}


impl Component for Radio {
    type Message = Msg;
    type Properties = RadioProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let checked = props.checked;
        Self {
            link,
            props,
            checked,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleChecked => {
                self.checked = !self.checked;
                self.props.handle_change.emit(self.checked);
            },
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let id = self.props.id.clone();
        let name = self.props.name.clone();
        let value = self.props.value.clone();
        let label_id = self.props.id.clone();
        let label = self.props.label.clone();
        let classes = self.props.class.clone();

        html! {
            <LabeledInput label=label id=label_id>
                <input
                    id=id
                    name=name
                    type="radio"
                    class=format!("yewi-radio {}", classes)
                    onclick=self.link.callback(|_| Msg::ToggleChecked)
                    checked={ self.checked }
                    value=value
                />
            </LabeledInput>
        }
    }
}

