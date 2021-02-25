use yew::{
    Component,
    ComponentLink,
    Properties,
    Children,
    Callback,
    ShouldRender,
    Html,
    html,
    events::MouseEvent,
};

use crate::utils::generate_unique_id;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ButtonType {
    Button,
    Submit,
    Reset,
}

impl ButtonType {
    pub fn type_as_str(&self) -> String {
        match *self {
            ButtonType::Button => String::from("button"),
            ButtonType::Submit => String::from("submit"),
            ButtonType::Reset => String::from("reset"),
        }
    }

    pub fn str_as_enum(tag: &str) -> ButtonType {
        match tag {
            _ if tag == "button" => ButtonType::Button,
            _ if tag == "submit" => ButtonType::Submit,
            _ if tag == "reset" => ButtonType::Reset,
            _ => ButtonType::Button,
        }
    }
}

impl Default for ButtonType {
    fn default() -> Self {
        Self::Button
    }
}

impl From<&str> for ButtonType {
    fn from(tag: &str) -> Self {
        let tag = tag.to_lowercase();

        ButtonType::str_as_enum(&tag)
    }
}

impl From<String> for ButtonType {
    fn from(tag: String) -> Self {
        let tag = tag.to_lowercase();

        ButtonType::str_as_enum(&tag)
    }
}


#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(generate_unique_id())]
    pub id: String,
    #[prop_or_default]
    pub class: String,
    #[prop_or(Callback::noop())]
    pub handle_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub button_type: ButtonType,
}

pub enum ButtonMsg {
    Clicked(MouseEvent),
}

pub struct Button {
    link: ComponentLink<Self>,
    props: ButtonProps,
}

impl Component for Button {
    type Message = ButtonMsg;
    type Properties = ButtonProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ButtonMsg::Clicked(event) => {
                self.props.handle_click.emit(event);
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
        let classes = self.props.class.clone();
        let children = self.props.children.clone();
        let button_type = self.props.button_type.type_as_str().clone();

        html! {
            <button
                id=id
                type=button_type
                class=format!("yewi-button {}", classes)
                onclick=self.link.callback(|e| ButtonMsg::Clicked(e))
            >
                { children }
            </button>
        }
    }
}
