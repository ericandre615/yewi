use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    Children,
    NodeRef,
    Callback,
};
use yew::services::{TimeoutService, Task};
use std::time::Duration;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct TransitionProps {
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
    #[prop_or(Callback::noop())]
    pub on_appear: Callback<()>,
    #[prop_or(Callback::noop())]
    pub on_appearing: Callback<()>,
    #[prop_or(Callback::noop())]
    pub on_appeared: Callback<()>,
    #[prop_or(Callback::noop())]
    pub on_enter: Callback<()>,
    #[prop_or(Callback::noop())]
    pub on_entering: Callback<()>,
    #[prop_or(Callback::noop())]
    pub on_entered: Callback<()>,
    #[prop_or(Callback::noop())]
    pub on_exit: Callback<()>,
    #[prop_or(Callback::noop())]
    pub on_exiting: Callback<()>,
    #[prop_or(Callback::noop())]
    pub on_exited: Callback<()>,
}

pub struct Transition {
    props: TransitionProps,
    node_ref: NodeRef,
    task_appearing: Option<Box<dyn Task>>,
    callback_appearing: Callback<()>,
    task_appeared: Option<Box<dyn Task>>,
    callback_appeared: Callback<()>,
    task_entering: Option<Box<dyn Task>>,
    callback_entering: Callback<()>,
    task_entered: Option<Box<dyn Task>>,
    callback_entered: Callback<()>,
    has_entered: bool,
    task_exiting: Option<Box<dyn Task>>,
    callback_exiting: Callback<()>,
    task_exited: Option<Box<dyn Task>>,
    callback_exited: Callback<()>,
    next_tick_duration: Duration,
}

pub enum Msg {
    TestUpdate,
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

impl Component for Transition {
    type Message = Msg;
    type Properties = TransitionProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            node_ref: NodeRef::default(),
            task_appearing: None,
            callback_appearing: link.callback(|_| Msg::Appearing),
            task_appeared: None,
            callback_appeared: link.callback(|_| Msg::Appeared),
            task_entering: None,
            callback_entering: link.callback(|_| Msg::Entering),
            task_entered: None,
            callback_entered: link.callback(|_| Msg::Entered),
            has_entered: false,
            task_exiting: None,
            callback_exiting: link.callback(|_| Msg::Exiting),
            task_exited: None,
            callback_exited: link.callback(|_| Msg::Exited),
            next_tick_duration: Duration::from_millis(1),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Appearing => {
                self.props.on_appearing.emit(());
                self.task_appearing = None;
            },
            Msg::Appeared => {
                self.props.on_appeared.emit(());
                self.task_appeared = None;
            },
            Msg::Entering => {
                self.props.on_entering.emit(());
                self.task_entering = None;
            },
            Msg::Entered => {
                self.props.on_entered.emit(());
                self.task_entered = None;
                self.has_entered = true;
            },
            Msg::Exiting => {
                self.props.on_exiting.emit(());
                self.task_exiting = None;
            },
            Msg::Exited => {
                self.props.on_exited.emit(());
                self.task_exited = None;
            },
            _ => {},
        }

        false // true == potential infinite loop
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        //if self.props != props {
        //    self.props = props;
        //    true
        //} else {
        //    false
        //}
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render && self.props.appear.is_some() {
            self.props.on_appear.emit(());
            let appear = match self.props.appear {
                Some(duration) => duration,
                None => self.props.duration,
            };

            {
                let handle = TimeoutService::spawn(
                    appear,//Duration::from_millis(self.props.appear),
                    self.callback_appeared.clone(),
                );
                self.task_appeared = Some(Box::new(handle));
            }
            {
                let handle = TimeoutService::spawn(
                    self.next_tick_duration,
                    self.callback_appearing.clone(),
                );
                self.task_appearing = Some(Box::new(handle));
            }
        }

        if !self.has_entered {
            self.props.on_enter.emit(());
            let enter = match self.props.enter {
                Some(duration) => duration,
                None => self.props.duration,
            };

            {
                let handle = TimeoutService::spawn(
                    enter,//Duration::from_millis(duration),
                    self.callback_entered.clone(),
                );
                self.task_entered = Some(Box::new(handle));
            }
            {
                let handle = TimeoutService::spawn(
                    self.next_tick_duration,
                    self.callback_entering.clone(),
                );
                self.task_entering = Some(Box::new(handle));
            }
        }
    }

    fn destroy(&mut self) {
        self.props.on_exit.emit(());
        // timeout for duration then self.exited();
        // not sure how this will work
        // if yew will call destroy before it's removed from dom?
        // after? TODO: look into this
        let exit = match self.props.exit {
            Some(duration) => duration,
            None => self.props.duration,
        };

        // TODO: this won't work as it goes out of scope/unmounted before timer
        // need to have block timeout?
        {
            let handle = TimeoutService::spawn(
                exit,//Duration::from_millis(exit),
                self.callback_exited.clone(),
            );
            self.task_exited = Some(Box::new(handle));
        }
        {
            let handle = TimeoutService::spawn(
                self.next_tick_duration,
                self.callback_exiting.clone(),
            );
            self.task_exiting = Some(Box::new(handle));
        }
        // fake a blocking timeout?
        //for n in 1..5000 { ConsoleService::info(&format!("time {}", n)); }
        // as I thought thread::sleep doesn't work in wasm..
        //sleep(Duration::from_millis(4200));
    }

    fn view(&self) -> Html {
        let classes = self.props.class.clone();
        let children = self.props.children.clone();
        let transition_class = format!("{}-transition", self.props.name.to_string());

        html! {
            <div
                ref=self.node_ref.clone()
                class=format!("transition {} {}", transition_class, classes)
            >
                { children }
            </div>
        }
    }
}

