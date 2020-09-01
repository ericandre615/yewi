use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
};
use serde::{Serialize, Deserialize};
use serde_json;
use serde_json::json;

use yewi::components::table::{Table};
use yewi::components::table::columns::{
    Column,
    Columns,
    CSSWidth,
    ColumnFilter,
    Sort,
    Accessor,
};

#[derive(Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct UserData {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub age: u32,
}

pub struct App {
    link: ComponentLink<Self>,
    columns: Columns,
    data: serde_json::Value,
    initial_data: serde_json::Value,
}

pub enum AppMsg {
    FilterData((String, String)),
    SortData((Sort, String)),
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let columns: Columns = vec![
            Column {
                header: Some(String::from("Name")),
                field: String::from("name"),
                sortable: false,
                filter: ColumnFilter::Text,
                accessor: Some(Accessor::callback(|row| {
                    let data: UserData = serde_json::from_value(row).unwrap();
                    html! {
                        <span>{ data.first_name }{ " " }{ data.last_name }</span>
                    }
                })),
                header_key: None,
                width: CSSWidth::Percent(40.0),
            },
            Column {
                header: Some(String::from("Age")),
                field: String::from("age"),
                sortable: true,
                ..Default::default()
            },
            Column {
                field: String::from("extra"),
                header_key: None,
                filter: ColumnFilter::Empty,
                width: CSSWidth::Px(200.0),
                ..Default::default()
            },
            Column {
                field: String::from(""),
                accessor: Some(Accessor::callback(|row| {
                    let data: UserData = serde_json::from_value(row).unwrap();
                    html! { <button id=data.id>{ "Remove" }</button> }
                })),
                ..Default::default()
            },
        ];
        let data = json!([
            {
                "id": 11111,
                "first_name": "john",
                "last_name": "strombolini",
                "age": 45,
                "not_in": "nodata",
            },
            {
                "id": 22222,
                "first_name": "duder",
                "last_name": "Mc Duderson",
                "age": 22,
                "extra": "not on struct",
            },
            {
                "id": 33333,
                "first_name": "jethro",
                "last_name": "McCheeseburgers",
                "age": 19,
            },
            {
                "id": 44444,
                "first_name": "Germy",
                "last_name": "Pivens",
                "age": 68,
            }
        ]);

        Self {
            link,
            columns,
            data: data.clone(),
            initial_data: data.clone(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMsg::FilterData(filter) => {
                let (value, field) = filter;
                let data: Vec<UserData> = serde_json::from_value(self.data.clone()).unwrap();
                let name_field = String::from("name");
                let filtered_data: Vec<UserData> = data.into_iter().filter(|item| {
                    match &field {
                        f if f == &name_field => {
                            let name = format!("{} {}", item.first_name, item.last_name);
                            if f.is_empty() { return true; }
                            name.contains(&value)
                        },
                        _ => true,
                    }
                }).collect();

                if field == name_field && value.is_empty() {
                    self.data = self.initial_data.clone();
                    return true;
                }

                self.data = serde_json::to_value(filtered_data).unwrap();
            },
            AppMsg::SortData(sort) => {
                let (direction, field) = sort;
                let age_field = "age";
                let mut data: Vec<UserData> = serde_json::from_value(self.data.clone()).unwrap();

                data.sort_by(|a, b| {
                    match &field {
                        f if f == age_field => {
                            match direction {
                                Sort::Asc => a.age.cmp(&b.age),
                                Sort::Dsc => b.age.cmp(&a.age),
                                Sort::NoSort => a.cmp(&b),
                            }
                        }
                        _ => a.cmp(&b)
                    }
                });

                self.data = serde_json::to_value(data).unwrap();
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let columns = self.columns.clone();
        let data = self.data.clone();

        html! {
            <div id="app">
                <Table
                    columns=columns
                    data=data
                    onfilter=self.link.callback(AppMsg::FilterData)
                    onsort=self.link.callback(AppMsg::SortData)
                />
            </div>
        }
    }
}
