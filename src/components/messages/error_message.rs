use yew::{
    Component,
    ComponentLink,
    Properties,
    Html,
    html,
    ShouldRender,
    Callback,
};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct ErrorProps {
    #[prop_or(None)]
    pub error: Option<String>,
    #[prop_or(false)]
    pub dismissible: bool,
    #[prop_or_default]
    pub class: String,
    #[prop_or(Callback::noop())]
    pub handle_dismiss: Callback<()>,
}

#[derive(Debug, Clone)]
pub struct ErrorMessage {
    props: ErrorProps,
    link: ComponentLink<Self>,
    visible: bool,
}

pub enum Msg {
    Dismiss,
}

impl Component for ErrorMessage {
    type Message = Msg;
    type Properties = ErrorProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let is_visible = match props.error {
            Some(_) => true,
            None => false,
        };

        Self {
            props,
            link,
            visible: is_visible,
        }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::Dismiss => {
                self.visible = false;
                self.props.handle_dismiss.emit(());
            },
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            let is_visible = match &props.error {
                Some(_) => true,
                None => false,
            };

            if self.props.error != props.error {
                self.visible = is_visible;
            }

            self.props = props;

            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let classes = self.props.class.clone();
        let error = match &self.props.error {
            Some(err) => format!("{}", err),
            None => format!(""),
        };

        if self.visible {
            html! {
                <div class=format!("error {}", classes)>
                    <p class="error-message">
                        { format!("{}", error) }
                    </p>
                    { self.render_dismiss() }
                </div>
            }
        } else {
            html! { <></> }
        }
    }
}

impl ErrorMessage {
    fn render_dismiss(&self) -> Html {
        if self.props.dismissible {
            html! {
                <button
                    type="button"
                    class="error-dismiss btn"
                    onclick=self.link.callback(|_| Msg::Dismiss)
                >
                    { "X" }
                </button>
            }
        } else {
            html! { <></> }
        }
    }
}
