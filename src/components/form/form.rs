use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    InputData,
    Callback,
    Children,
};
use web_sys::{FocusEvent, Element, HtmlFormElement, FormData, HtmlInputElement};
use wasm_bindgen::{JsValue, JsCast};
use yew::services::{ConsoleService};

use std::convert::TryFrom;

type SubmitEvent = FocusEvent;

#[derive(Debug, Clone, PartialEq)]
pub enum FormMethod {
    Post,
    Get,
    Dialog,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormValue {
    Num(f32),
    Str(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FormItem {
    id: String,
    name: String,
    value: FormValue,
    touched: bool,
    valid: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FormState {
    inputs: Vec<FormItem>
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct FormProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(String::new())]
    pub id: String,
    #[prop_or(String::new())]
    pub name: String,
    #[prop_or(String::new())]
    pub action: String,
    #[prop_or(FormMethod::Get)]
    pub method: FormMethod,
    #[prop_or(String::new())]
    pub target: String,
    #[prop_or(false)]
    pub novalidate: bool,
    #[prop_or(String::new())]
    pub class: String,
    #[prop_or(Callback::noop())]
    pub handle_change: Callback<String>,
    #[prop_or(Callback::noop())]
    pub handle_submit: Callback<SubmitEvent>, // SubmitDAta?
}

pub struct Form {
    link: ComponentLink<Self>,
    props: FormProps,
}

pub enum FormMsg {
    UpdateValue(InputData),
    Submit(SubmitEvent),
}

impl Component for Form {
    type Message = FormMsg;
    type Properties = FormProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            FormMsg::UpdateValue(change) => {
                //self.value = change.value.clone();
                //self.props.handle_change.emit(self.value.clone());
            },
            FormMsg::Submit(event) => {
                let form = event.target().unwrap().unchecked_into::<HtmlFormElement>();
                let form_data = FormData::new_with_form(&form);

                for i in 0..form.elements().length() {
                    let item = form.elements().item(i).unwrap();
                    let value = self.get_value(item.clone());
                    ConsoleService::info(&format!("INPUT {:?}", value));
                }

                self.props.handle_submit.emit(event);
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
        let id = self.props.id.clone();
        let classes = self.props.class.clone();

        html! {
            <form
                id=id
                type="text"
                method=self.get_method()
                class=format!("yewi-form {}", classes)
                onsubmit=self.link.callback(|e: SubmitEvent| {
                    e.prevent_default();
                    FormMsg::Submit(e)
                })
            >
                { children }
            </form>
        }
    }
}

impl Form {
    fn get_method(&self) -> String {
        // I think method string is case-sensitive (MDN)
        let method = match self.props.method {
            FormMethod::Get => "get",
            FormMethod::Post => "post",
            FormMethod::Dialog => "dialog",
        };

        method.to_string()
    }

    fn get_value(&self, element: Element) -> String {
        if let Ok(input) = element.dyn_into::<HtmlInputElement>() {
            return input.value();
        }

        "".into()
    }
}
