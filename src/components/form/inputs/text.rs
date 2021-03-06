use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    InputData,
    Callback,
};

#[derive(Properties, Clone, Debug)]
pub struct TextInputProps {
    #[prop_or(String::new())]
    pub id: String,
    #[prop_or(String::new())]
    pub name: String,
    #[prop_or(String::new())]
    pub initial_value: String,
    #[prop_or(String::new())]
    pub value: String,
    #[prop_or(String::new())]
    pub placeholder: String,
    #[prop_or(String::new())]
    pub class: String,
    #[prop_or(Callback::noop())]
    pub handle_change: Callback<String>,
}

pub struct TextInput {
    link: ComponentLink<Self>,
    props: TextInputProps,
    value: String,
}

pub enum Msg {
    UpdateValue(InputData),
}


impl Component for TextInput {
    type Message = Msg;
    type Properties = TextInputProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let value = if !props.value.is_empty() {
            props.value.clone()
        } else {
            props.initial_value.clone()
        };

        Self {
            link,
            props,
            value,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateValue(change) => {
                self.value = change.value.clone();
                self.props.handle_change.emit(self.value.clone());
            },
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.value != props.value {
            self.props.value = props.value.clone();
            self.value = props.value;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let id = self.props.id.clone();
        let name = self.props.name.clone();
        let placeholder = self.props.placeholder.clone();
        let classes = self.props.class.clone();
        let value = self.value.clone();

        html! {
            <div>
                <input
                    type="text"
                    id=id
                    name=name
                    class=format!("yewi-text-input {}", classes)
                    oninput=self.link.callback(|v: InputData| Msg::UpdateValue(v))
                    value={ value }
                    placeholder={ placeholder }
                />
            </div>
        }
    }
}
