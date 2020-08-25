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
pub struct TRowProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(String::new())]
    pub class: String,
}

pub struct TRow {
    props: TRowProps,
}

pub enum Msg {}

impl Component for TRow {
    type Message = Msg;
    type Properties = TRowProps;

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
        //colSpan?
        // TRowLinker? which would need to be a RouteAnchor
        html! {
            <tr class=format!("yewi-trow {}", classes)>
                { children }
            </tr>
        }
    }
}
