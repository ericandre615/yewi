use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    Children,
};

// the js version had the concept of colSpan and label. Look into how they were used
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct TDataProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(String::new())]
    pub class: String,
    #[prop_or(String::new())]
    pub label: String,
}

pub struct TData {
    props: TDataProps,
}

pub enum Msg {}

impl Component for TData {
    type Message = Msg;
    type Properties = TDataProps;

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
        let label = self.props.label.clone();
        let children = self.props.children.clone();

        html! {
            <td class=format!("yewi-tdata {}", classes) data-label={ label }>
                { children }
            </td>
        }
    }
}
