#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputType {
    Button,
    Form,
    Input,
    TextArea,
    FieldSet,
    Label,
    Select,
    SelectOption,
    Legend,

    Checkbox,
    Radio,
    Text,
    Password,
    Email,
    Tel,
    Search,
    Url,
    Number,
    Range,
    Color,
    File,
    Image,
    Reset,
    Submit,

    Date,
    DatetimeLocal,
    Month,
    Time,
    Week,

    Hidden,
    Empty,
}

impl InputType {
    pub fn as_html_str(&self) -> String {
        match *self {
            InputType::Button => String::from("button"),
            InputType::Form => String::from("form"),
            InputType::Input => String::from("input"),
            InputType::TextArea => String::from("textarea"),
            InputType::FieldSet => String::from("fieldset"),
            InputType::Label => String::from("label"),
            InputType::Select => String::from("select"),
            InputType::SelectOption => String::from("option"),
            InputType::Legend => String::from("legend"),
            InputType::Checkbox => String::from("checkbox"),
            InputType::Radio => String::from("radio"),
            InputType::Text => String::from("text"),
            InputType::Password => String::from("password"),
            InputType::Email => String::from("email"),
            InputType::Tel => String::from("tel"),
            InputType::Search => String::from("search"),
            InputType::Url => String::from("url"),
            InputType::Number => String::from("number"),
            InputType::Range => String::from("range"),
            InputType::Color => String::from("color"),
            InputType::File => String::from("file"),
            InputType::Image => String::from("image"),
            InputType::Reset => String::from("reset"),
            InputType::Submit => String::from("submit"),
            InputType::Date => String::from("date"),
            InputType::DatetimeLocal => String::from("datetime-local"),
            InputType::Month => String::from("month"),
            InputType::Time => String::from("time"),
            InputType::Week => String::from("week"),

            InputType::Hidden => String::from("hidden"),
            InputType::Empty => String::from(""),
        }
    }

    pub fn str_as_input_type(tag: &str) -> InputType {
        match tag {
            _ if tag == "button" => InputType::Button,
            _ if tag == "form" => InputType::Form,
            _ if tag == "input" => InputType::Input,
            _ if tag == "textarea" => InputType::TextArea,
            _ if tag == "fieldset" => InputType::FieldSet,
            _ if tag == "label" => InputType::Label,
            _ if tag == "select" => InputType::Select,
            _ if tag == "option" => InputType::SelectOption,
            _ if tag == "legend" => InputType::Legend,
            _ if tag == "checkbox" => InputType::Checkbox,
            _ if tag == "radio" => InputType::Radio,
            _ if tag == "text" => InputType::Text,
            _ if tag == "password" => InputType::Password,
            _ if tag == "email" => InputType::Email,
            _ if tag == "tel" => InputType::Tel,
            _ if tag == "search" => InputType::Search,
            _ if tag == "url" => InputType::Url,
            _ if tag == "number" => InputType::Number,
            _ if tag == "range" => InputType::Range,
            _ if tag == "color" => InputType::Color,
            _ if tag == "file" => InputType::File,
            _ if tag == "image" => InputType::Image,
            _ if tag == "reset" => InputType::Reset,
            _ if tag == "submit" => InputType::Submit,
            _ if tag == "date" => InputType::Date,
            _ if tag == "datetime-local" => InputType::DatetimeLocal,
            _ if tag == "month" => InputType::Month,
            _ if tag == "time" => InputType::Time,
            _ if tag == "week" => InputType::Week,

            _ if tag == "hidden" => InputType::Hidden,
            _ if tag.is_empty() => InputType::Empty,
            _ => InputType::Empty,
        }
    }

    pub fn is_element(&self, tag: &str) -> bool {
        if self.as_html_str() == tag.to_lowercase() {
            true
        } else {
            false
        }
    }

    pub fn is_input_type(&self, tag: &InputType) -> bool {
        if self == tag {
            true
        } else {
            false
        }
    }

    pub fn as_input_type(&self) -> &Self {
        self
    }
}

impl Default for InputType {
    fn default() -> Self { InputType::Empty }
}

impl From<&str> for InputType {
    fn from(tag: &str) -> Self {
        let tag = tag.to_lowercase();

        InputType::str_as_input_type(&tag)
    }
}

impl From<String> for InputType {
    fn from(tag: String) -> Self {
        let tag = tag.to_lowercase();

        InputType::str_as_input_type(&tag)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_tags() -> Vec<&'static str> {
        let tags = vec![
            "button", "form", "input", "textarea", "fieldset", "label",
            "select", "option", "legend", "checkbox", "radio", "text",
            "password", "email", "tel", "search", "url", "number", "range",
            "color", "file", "image", "reset", "submit", "date", "datetime-local",
            "month", "time", "week", "hidden",
        ];

        tags
    }

    #[test]
    fn tag_as_html_str() {
        let textarea = InputType::TextArea;
        let radio = InputType::Radio;

        assert_eq!(textarea.as_html_str(), "textarea");
        assert_ne!(radio.as_html_str(), "Radio");
    }

    #[test]
    fn tag_default() {
        let default = InputType::default();

        assert_eq!(default, InputType::Empty);
        assert_ne!(default, InputType::Input);
    }

    #[test]
    fn tag_is_element() {
        let checkbox = InputType::Checkbox;
        let is_checkbox = "CHECKBOX";
        let is_not_checkbox = "RADIO";

        assert_eq!(checkbox.is_element(is_checkbox), true);
        assert_eq!(checkbox.is_element(is_not_checkbox), false);
    }

    #[test]
    fn tag_is_tag() {
        let button = InputType::Button;
        let is_button = InputType::Button;
        let is_not_button = InputType::Checkbox;

        assert_eq!(button.is_input_type(&is_button), true);
        assert_eq!(button.is_input_type(&is_not_button), false);
    }

    #[test]
    fn tag_from_str() {
        let checkbox = InputType::from("CHECKBOX");
        let radio = InputType::from("radio");

        assert_eq!(checkbox, InputType::Checkbox);
        assert_eq!(radio, InputType::Radio);
    }

    #[test]
    fn tag_empty() {
        let empty = InputType::Empty;
        let from_str_empty = InputType::from("notarealelement");
        let from_str_empty_empty = InputType::from("");

        assert_eq!(from_str_empty, empty);
        assert_eq!(from_str_empty_empty, empty);
    }

    #[test]
    fn tag_as_tag() {
        let checkbox = InputType::from("checkbox");

        println!("Checkbox InputType as InputType {:?}", checkbox.as_input_type());
        assert_eq!(*checkbox.as_input_type(), InputType::Checkbox);
    }

    #[test]
    fn tag_exhaustive() {
        let tags = get_tags();

        for t in tags {
            let tag = InputType::from(t);

            assert_eq!(tag.as_html_str(), t);
        }
    }
}

