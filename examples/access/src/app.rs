use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    KeyboardEvent,
    Callback,
};

use yewi::components::access::Access;

pub struct App {}

pub enum AppMsg {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="app">
               <Access access=get_some_user_policy("Role:CannotSeeSomething")>
                <p>{ "I should not be rendered" }</p>
               </Access>
               <Access access=get_some_user_policy("Role:CanSeeSomething")>
                <p>{ "I should be rendered" }</p>
               </Access>
            </div>
        }
    }
}

fn get_some_user_policy(permission: &str) -> bool {
    // NOTE: this is just a mock example of what you might use
    // say if you hit an api/db/something to determine if a user has permission
    // for a particular
    match permission {
        p if p == "Role:CanSeeSomething" => true,
        p if p == "Role:CannotSeeSomething" => false,
        _ => false
    }

}
