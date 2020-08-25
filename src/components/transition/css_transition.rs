use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    Children,
    NodeRef,
};

use web_sys::Element;
use std::time::Duration;

use crate::components::transition::Transition;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct CSSTransitionProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(String::new())]
    pub class: String,
    pub name: String,
    #[prop_or(Duration::from_millis(0))]
    pub duration: Duration,
    #[prop_or_default]
    pub appear: Option<Duration>,
    #[prop_or_default]
    pub enter: Option<Duration>,
    #[prop_or_default]
    pub exit: Option<Duration>,
}

pub struct CSSTransition {
    link: ComponentLink<Self>,
    props: CSSTransitionProps,
    node_ref: NodeRef, // Maybe Transition component passes it's node ref? Maybe doesn't matter
}

pub enum Msg {
    Appear,
    Appearing,
    Appeared,
    Exit,
    Exiting,
    Exited,
    Enter,
    Entering,
    Entered,
}

impl Component for CSSTransition {
    type Message = Msg;
    type Properties = CSSTransitionProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Appear => {
                self.appear();
            },
            Msg::Appearing => {
                self.appearing();
            },
            Msg::Appeared => {
                self.appeared();
            },
            Msg::Entering => {
                self.entering();
            },
            Msg::Enter => {
                self.enter();
            },
            Msg::Entered => {
                self.entered();
            },
            Msg::Exiting => {
                self.exiting();
            },
            Msg::Exit => {
                self.exit();
            },
            Msg::Exited => {
                self.exited();
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
        let classes = self.props.class.clone();
        let name = self.props.name.clone();
        let duration = self.props.duration;
        let appear = self.props.appear;
        let enter = self.props.enter;
        let exit = self.props.exit;
        let children = self.props.children.clone();

        html! {
            <Transition
                ref=self.node_ref.clone()
                class=classes
                name=name
                duration=duration
                appear=appear
                enter=enter
                exit=exit
                on_appearing=self.link.callback(|_| Msg::Appearing)
                on_appear=self.link.callback(|_| Msg::Appear)
                on_appeared=self.link.callback(|_| Msg::Appeared)
                on_entering=self.link.callback(|_| Msg::Entering)
                on_enter=self.link.callback(|_| Msg::Enter)
                on_entered=self.link.callback(|_| Msg::Entered)
                on_exiting=self.link.callback(|_| Msg::Exiting)
                on_exit=self.link.callback(|_| Msg::Exit)
                on_exited=self.link.callback(|_| Msg::Exited)
            >
                { children }
            </Transition>
        }
    }
}

impl CSSTransition {
    fn apply_transition(&self, classname: &str) {
        let element = self.node_ref.cast::<Element>().unwrap();
        element.class_list().add_1(&format!("{}-{}", self.props.name, classname)).unwrap();
    }

    fn remove_transition(&self, classname: &str) {
        let element = self.node_ref.cast::<Element>().unwrap();
        element.class_list().remove_1(&format!("{}-{}", self.props.name, classname)).unwrap();
    }

    fn appearing(&self) {
        self.remove_transition(&"appear");
        self.apply_transition(&"appearing");
    }

    fn appear(&self) {
        self.apply_transition(&"appear");
    }

    fn appeared(&self) {
        self.remove_transition(&"appearing");
        self.apply_transition(&"appeared");
    }

    fn exiting(&self) {
        self.remove_transition(&"exit");
        self.apply_transition(&"exiting");
    }

    fn exit(&self) {
        self.apply_transition(&"exit");
    }

    fn exited(&self) {
        self.remove_transition(&"exiting");
        self.apply_transition(&"exited");
    }

    fn entering(&self) {
        self.remove_transition(&"enter");
        self.apply_transition(&"entering");
    }

    fn enter(&self) {
        self.apply_transition(&"enter");
    }

    fn entered(&self) {
        self.remove_transition(&"entering");
        self.apply_transition(&"entered");
    }
}
