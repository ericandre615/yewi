use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    KeyboardEvent,
    Callback,
};

use yew::services::ConsoleService;
use yewi::components::messages::ErrorMessage;

pub struct App {
    link: ComponentLink<Self>,
    no_dismiss_err: Option<String>,
    dismissible_err: Option<String>,
    manual_dismiss_err: Option<String>,
    counter: u32
}

pub enum AppMsg {
    DismissError,
    HandleDismiss,
    AddError,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            no_dismiss_err: Some(String::from("Can not dismiss this Error")),
            dismissible_err: Some(String::from("Can dimiss this Error")),
            manual_dismiss_err: Some(String::from("Can manually dismiss this Error")),
            counter: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMsg::HandleDismiss => {
                // clear error
                ConsoleService::info("Error Message Dismissedi manually");
                // self.dimissible_err = None;
                // if this state is updated to anything, including a new error
                // then the ErrorMessage Component Should show the message again automatically
            },
            AppMsg::DismissError => {
                // clear error
                ConsoleService::info("Error Message Dismissed by Component");
                self.manual_dismiss_err = None;
            },
            AppMsg::AddError => {
                self.counter += 1;
                self.dismissible_err = Some(format!("Additional Error {}", self.counter));
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let no_dismiss_err = self.no_dismiss_err.clone();
        let dismissible_err = self.dismissible_err.clone();
        let manual_dismiss_err = self.manual_dismiss_err.clone();

        html! {
            <div id="app">
                <ErrorMessage
                    class="no-dimiss"
                    error=no_dismiss_err
                />
                <ErrorMessage
                    class="dismiss"
                    error=dismissible_err
                    dismissible=true
                    handle_dismiss=self.link.callback(|_| AppMsg::HandleDismiss)
                />
                <ErrorMessage
                    class="manual-dimiss"
                    error=manual_dismiss_err
                />
                <button type="button"
                    onclick=self.link.callback(|_| AppMsg::DismissError)
                >
                    { "Manually Dimiss" }
                </button>
                <button type="button"
                    onclick=self.link.callback(|_| AppMsg::AddError)
                >
                    { "Add New Error" }
                </button>
            </div>
        }
    }
}

