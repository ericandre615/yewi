mod form;
mod label;
pub mod inputs;
mod input_type;
mod labeled_input;
mod submit_button;

pub use form::{Form, FormMethod, FormState};
pub use label::{Label};
pub use input_type::{InputType};
pub use submit_button::SubmitButton;

use labeled_input::LabeledInput;
