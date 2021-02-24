use yew::{
    Component,
    ComponentLink,
    Properties,
    Children,
    ShouldRender,
    Html,
    html,
};

use crate::utils::generate_unique_id;

#[derive(Properties, Clone, PartialEq)]
pub struct LabeledInputProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(String::new())]
    pub label: String,
    #[prop_or(generate_unique_id())]
    pub id: String,
}

pub(crate) struct LabeledInput {
    props: LabeledInputProps,
}

impl Component for LabeledInput {
    type Message = ();
    type Properties = LabeledInputProps;

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
        let label = self.props.label.clone();

        match label.is_empty() {
            true => self.render_children(),
            false => self.render_with_label(),
        }
    }
}

impl LabeledInput {
    fn render_with_label(&self) -> Html {
        let label = self.props.label.clone();
        let id = self.props.id.clone();
        let children = self.props.children.clone();

        html! {
            <label for=id class="yewi-label">
                { children }
                <span class="yewi-label-text">{ label }</span>
            </label>
        }
    }

    fn render_children(&self) -> Html {
        let children = self.props.children.clone();

        html! {
            <>
                { children }
            </>
        }
    }
}
