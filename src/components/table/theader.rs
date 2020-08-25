use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    Children,
    Callback,
};
use web_sys::{MouseEvent, HtmlElement};
use wasm_bindgen::{JsCast};

use crate::components::table::columns::{ColumnFilter, Sort};
use crate::components::form::inputs::{TextInput};

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct THeaderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(String::new())]
    pub class: String,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub field: String,
    #[prop_or(ColumnFilter::Empty)]
    pub filter: ColumnFilter,
    #[prop_or(false)]
    pub sortable: bool,
    #[prop_or(Callback::noop())]
    pub handle_filter: Callback<(String, String)>,
    #[prop_or(Callback::noop())]
    pub handle_sort: Callback<(Sort, String)>,

}

pub struct THeader {
    link: ComponentLink<Self>,
    props: THeaderProps,
}

pub enum Msg {
    FilterChangeText(String),
    SortChange(MouseEvent),
}

impl Component for THeader {
    type Message = Msg;
    type Properties = THeaderProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FilterChangeText(input) => {
                let field = self.props.field.clone();
                self.props.handle_filter.emit((input, field));
            },
            Msg::SortChange(event) => {
                let target = event.target().unwrap().unchecked_into::<HtmlElement>();
                let field = self.props.field.clone();
                let sort_up = "yewi-sort-up";
                let sort_down = "yewi-sort-down";
                let sort_direction = match target.class_name() {
                    _s if _s == sort_up => Sort::Asc,
                    _s if _s == sort_down => Sort::Dsc,
                    _ => Sort::NoSort,
                };

                self.props.handle_sort.emit((sort_direction, field));
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
        let classes = self.props.class.clone();
        let label = self.props.label.clone();
        let children = self.props.children.clone();

        html! {
            <th class=format!("yewi-theader {}", classes) data-label=label>
                { children }
                { self.render_sort() }
                { self.render_filter() }
            </th>
        }
    }
}

impl THeader {
    fn render_filter(&self) -> Html {
        match &self.props.filter {
            ColumnFilter::Text => html! {
                <TextInput
                    class="yewi-filter"
                    handle_change=self.link.callback(Msg::FilterChangeText)
                />
            },
            ColumnFilter::Empty => html! { <></> },
        }
    }

    fn render_sort(&self) -> Html {
        if self.props.sortable {
            html! {
                <span class="yewi-table-sort" onclick=self.link.callback(Msg::SortChange)>
                    <span class="yewi-sort-up">{ '\u{25b2}' }</span>
                    <span class="yewi-sort-down">{ '\u{25bc}' }</span>
                </span>
            }
        } else {
            html! { <></> }
        }
    }
}
