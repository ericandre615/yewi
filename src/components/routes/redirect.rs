use yew::{
    Component,
    ComponentLink,
    Properties,
    ShouldRender,
    Html,
    html,
};

use yew::services::{ConsoleService};

use yew_router::{
    router::Router,
    route::Route,
    agent::{RouteAgentDispatcher, RouteRequest},
};

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct RedirectProps {
    pub to: String,
}

#[derive(Debug)]
pub struct Redirect {
    props: RedirectProps,
    link: ComponentLink<Self>,
    router: RouteAgentDispatcher<()>,
}

pub enum RedirectMsg {
    Redirecting,
}

impl Component for Redirect {
    type Message = RedirectMsg;
    type Properties = RedirectProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgentDispatcher::new();

        Self {
            props,
            router,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            RedirectMsg::Redirecting => {
                let route = Route::from(self.props.to.clone());
                self.router.send(RouteRequest::ChangeRoute(route));
            },
        }

        false
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
        self.link.send_message(RedirectMsg::Redirecting);
        html! { <></> }
    }
}
