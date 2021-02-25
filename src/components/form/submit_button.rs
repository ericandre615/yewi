use yew::{
    Component,
    ComponentLink,
    Properties,
    ShouldRender,
    Callback,
    Children,
    Html,
    html,
    events::MouseEvent,
};

use crate::utils::generate_unique_id;
use crate::components::ui::{Button, ButtonType};

#[derive(Properties, Clone, PartialEq)]
pub struct SubmitButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(generate_unique_id())]
    pub id: String,
    #[prop_or_default]
    pub class: String,
    #[prop_or(Callback::noop())]
    pub handle_click: Callback<MouseEvent>,
}

pub enum SubmitButtonMsg {
    Clicked(MouseEvent),
}

pub struct SubmitButton {
    link: ComponentLink<Self>,
    props: SubmitButtonProps,
}

impl Component for SubmitButton {
    type Message = SubmitButtonMsg;
    type Properties = SubmitButtonProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            SubmitButtonMsg::Clicked(event) => {
                self.props.handle_click.emit(event);
            }
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
        let classes = self.props.class.clone();
        let children = self.props.children.clone();

        html! {
            <Button
                id=id
                button_type=ButtonType::Submit
                class=format!("yewi-submit-button {}", classes)
                handle_click=self.link.callback(SubmitButtonMsg::Clicked)
            >
                { children }
            </Button>
        }
    }
}
