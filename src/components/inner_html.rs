use yew::{
    Component,
    ComponentLink,
    Properties,
    ShouldRender,
    Html,
    html,
};

use yew::virtual_dom::VNode;
use web_sys::Node;

use crate::html::Tag;

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub element: Tag,
    #[prop_or(String::from("<!-- no html -->"))]
    pub html: String,
    #[prop_or(String::new())]
    pub classes: String,
}

#[derive(Debug)]
pub struct InnerHtml {
    props: Props,
}

impl Component for InnerHtml {
    type Message = ();
    type Properties = Props;

    fn create(props: Props, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
        }

        true
    }

    fn view(&self) -> Html {
        self.render_html()
    }
}

impl InnerHtml {
    fn render_html(&self) -> Html {
        let content = &self.props.html;
        let classes = &self.props.classes;
        let tag = &self.props.element.as_html_str();
        let element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element(tag)
            .unwrap();
        element.class_list().add_1(classes).unwrap();
        element.set_inner_html(content);

        let node = Node::from(element);
        let vnode = VNode::VRef(node);

        vnode
    }
}
