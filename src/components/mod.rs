#[allow(clippy::module_inception)]

mod inner_html;
pub mod transition;
pub mod access;
pub mod form;
pub mod table;
pub mod messages;
pub mod ui;

pub use inner_html::InnerHtml;

