use yew::{Html};
use serde_json;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColumnFilter {
    Text,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sort {
    Asc,
    Dsc,
    NoSort,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Accessor {
    pub callback: fn(serde_json::Value) -> Html,
}

impl Accessor {
    pub fn callback(callback: fn(serde_json::Value) -> Html) -> Self {
        Self {
            callback,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CSSWidth {
    Px(f32),
    Em(f32),
    Cm(f32),
    Mm(f32),
    In(f32),
    Pt(f32),
    Pc(f32),
    Percent(f32),
    Rem(f32),
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    pub header: Option<String>,
    pub field: String,
    pub sortable: bool,
    pub filter: ColumnFilter,
    pub accessor: Option<Accessor>,
    pub header_key: Option<String>,
    pub width: CSSWidth,
}

impl Default for Column {
    fn default() -> Column {
        Column {
            header: None,
            field: String::from(""),
            sortable: false,
            filter: ColumnFilter::Empty,
            accessor: None,
            header_key: None,
            width: CSSWidth::Empty,
        }
    }
}

pub type Columns = Vec<Column>;
