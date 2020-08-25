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
pub struct TFootProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(String::new())]
    pub class: String,
}

pub struct TFoot {
    props: TFootProps,
}

pub enum Msg {}

impl Component for TFoot {
    type Message = Msg;
    type Properties = TFootProps;

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
        let classes = self.props.class.clone();
        let children = self.props.children.clone();

        html! {
            <tfoot class=format!("yewi-tfoot {}", classes)>
                { children }
            </tfoot>
        }
    }
}
