use yew::prelude::{
    Component,
    ComponentLink,
    Children,
    Html,
    html,
    Properties,
    ShouldRender,
};

use crate::utils::generate_unique_id;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct LabelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub label_for: String,
    #[prop_or(generate_unique_id())]
    pub id: String,
    #[prop_or_default]
    pub class: String,
}

#[derive(Debug)]
pub struct Label {
    props: LabelProps,
}

impl Component for Label {
    type Message = ();
    type Properties = LabelProps;

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
        let children = self.props.children.clone();
        let id = self.props.id.clone();
        let classes = self.props.class.clone();
        let label_for = self.props.label_for.clone();

        html! {
            <label for=label_for id=id class=format!("yewi-label {}", classes)>
                { children }
            </label>
        }
    }
}
