use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    ChangeData,
    MouseEvent,
    Callback,
};

#[derive(Properties, Clone, PartialEq)]
pub struct RadioGroupProps {
    #[prop_or(String::new()]
    pub id: String,
    pub name: String,
    #[prop_or(String::new())]
    pub class: String,
    #[prop_or(false)]
    pub checked: bool, // in this case if we use checked it needs to be the Radio button or value not a bool
    #[prop_or(Callback::noop())]
    pub handle_change: Callback<bool>,
}

pub struct RadioGroup {
    link: ComponentLink<Self>,
    props: RadioGroupProps,
    checked: bool,
}

pub enum Msg {
    ToggleChecked,
}


impl Component for RadioGroup {
    type Message = Msg;
    type Properties = RadioGroupProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let checked = props.checked;
        Self {
            link,
            props,
            checked,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleChecked => {
                self.checked = !self.checked;
                self.props.handle_change.emit(self.checked);
            },
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
        let children = self.props.children.clone();
        let id = self.props.id;
        let name = self.props.name.clone();
        let classes = self.props.class.clone();

        // might need ChildrenWithProps? Need to pass name to all children
        html! {
            <div id=id class=format!("yewi-radio-group {}", classes)>
                {
                    for children.iter().map(|mut item| {
                        item.name = name;
                        item
                    })
                }
            //<input
            //    id=id
            //    name=name
            //    type="radio"
            //    class=format!("yewi-radio {}", classes)
            //    onclick=self.link.callback(|_| Msg::ToggleChecked)
            //    checked={ self.checked }
            //    value=value
            ///>
            </div>
        }
    }
}
