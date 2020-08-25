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
use serde_json;

use crate::components::table::columns::*;
use crate::components::table::thead::*;
use crate::components::table::theader::*;
use crate::components::table::tbody::*;
use crate::components::table::tdata::*;
use crate::components::table::trow::*;
use crate::components::table::tfoot::*;

type TableData = serde_json::Value;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct TableProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(String::new())]
    pub class: String,
    #[prop_or_default]
    pub columns: Columns,
    #[prop_or_default]
    pub data: TableData,
    #[prop_or(Callback::noop())]
    pub onfilter: Callback<(String, String)>,
    #[prop_or(Callback::noop())]
    pub onsort: Callback<(Sort, String)>
}

pub struct Table {
    link: ComponentLink<Self>,
    props: TableProps,
}

pub enum Msg {
    FilterChange((String, String)),
    SortChange((Sort, String)),
}

impl Component for Table {
    type Message = Msg;
    type Properties = TableProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FilterChange(change) => {
                self.props.onfilter.emit(change);
            },
            Msg::SortChange(change) => {
                self.props.onsort.emit(change);
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
        let children = self.props.children.clone();

        html! {
            <table class=format!("yewi-table {}", classes)>
               { self.render_header() }
               { self.render_rows()   }
               { self.render_footer() }
            </table>
        }
    }
}

impl Table {
    fn render_footer(&self) -> Html {
        if !self.props.children.is_empty() {
            html! {
                <TFoot>
                    <TRow>
                        <TData>{ self.props.children.clone() }</TData>
                    </TRow>
                </TFoot>
            }
        } else {
            html! { <></> }
        }
    }

    fn render_header(&self) -> Html {
        html! {
            <THead>
                <TRow>{
                    for self.props.columns.iter().map(|column| {
                        let header = match &column.header {
                            Some(s) => s.clone(),
                            None => String::from(""),
                        };
                        let field = column.field.clone();
                        let label = create_label(&column.header);
                        let sortable = column.sortable;
                        html! {
                            <THeader
                                label=label
                                field=field
                                sortable=sortable
                                filter=column.filter
                                handle_filter=self.link.callback(Msg::FilterChange)
                                handle_sort=self.link.callback(Msg::SortChange)
                            >
                                { header }
                            </THeader>
                        }
                    })
                }</TRow>
            </THead>
        }
    }

    fn render_rows(&self) -> Html {
        let empty = Vec::new();
        let data = match self.props.data.as_array() {
            Some(arr) => arr,
            None => &empty,
        };

        if !data.is_empty() {
            html! {
                <TBody>{
                    for data.iter().map(|row| {
                        html! {
                            <TRow>{
                                for self.props.columns.iter().map(|column| {
                                    let label = create_label(&column.header);
                                    let display = match &column.accessor {
                                        Some(accessor) => html! {<TData label=label>{ (accessor.callback)(row.clone()) }</TData> },
                                        None => html! { <TData label=label>{ row[column.field.clone()].clone() } </TData> },
                                    };

                                    display
                                })
                            }</TRow>
                        }
                    })
                }</TBody>
            }
        } else {
            html!{ <></> }
        }
    }
}

fn create_label(str: &Option<String>) -> String {
    let re_str = match str {
        Some(s) => s.as_str(),
        None => "",
    };
    re_str.to_lowercase().replace(" ", "-")
}
