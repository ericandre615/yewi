use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    Children,
};

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct AccessProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(false)]
    pub access: bool,
}

pub struct Access {
    props: AccessProps,
}

impl Component for Access {
    type Message = ();
    type Properties = AccessProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
        let access = self.props.access;
        let children = self.props.children.clone();

        if access {
            html! {{ for children.iter().map(|c| c) }}
        } else {
            html! { <></> }
        }
    }
}

