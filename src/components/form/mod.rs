mod form;
mod label;
pub mod inputs;
mod input_type;
mod labeled_input;

pub use form::{Form, FormMethod, FormState};
pub use label::{Label};
pub use input_type::{InputType};
use labeled_input::LabeledInput;
