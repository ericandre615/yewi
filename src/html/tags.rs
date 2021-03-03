#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tag {
    Button,
    Paragraph,
    Anchor,
    Div,
    Span,
    Ul,
    Ol,
    Li,
    Header,
    Footer,
    Section,
    Aside,
    Nav,
    Main,
    Article,
    Details,
    Summary,
    Figure,
    FigCaption,
    HGroup,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    // Forms
    Form,
    Input,
    TextArea,
    FieldSet,
    Label,
    Select,
    SelectOption,
    Legend,
    // Formatting
    Mark,
    Small,
    Em,
    Strong,
    Br,
    B,
    I,
    Pre,
    Sub,
    Sup,
    Progress,
    Code,
    Cite,
    BlockQuote,
    Address,
    Abbr,
    Acronym,
    Menu,
    // Tables
    Table,
    Caption,
    Col,
    ColGroup,
    TBody,
    Td,
    TFoot,
    THead,
    Th,
    Tr,
    Script,
    NoScript,
    //Embed
    Canvas,
    Video,
    Audio,
    Frame,
    IFrame,
    FrameSet,
    Img,

    Empty,
}

impl Tag {
    pub fn as_html_str(&self) -> String {
        match *self {
            Tag::Button => String::from("button"),
            Tag::Paragraph => String::from("p"),
            Tag::Anchor => String::from("a"),
            Tag::Div => String::from("div"),
            Tag::Span => String::from("span"),
            Tag::Ul => String::from("ul"),
            Tag::Ol => String::from("ol"),
            Tag::Li => String::from("li"),
            Tag::Header => String::from("header"),
            Tag::Footer => String::from("footer"),
            Tag::Section => String::from("section"),
            Tag::Aside => String::from("aside"),
            Tag::Table => String::from("table"),
            Tag::Nav => String::from("nav"),
            Tag::Main => String::from("main"),
            Tag::Article => String::from("article"),
            Tag::Details => String::from("details"),
            Tag::Summary => String::from("summary"),
            Tag::Figure => String::from("figure"),
            Tag::FigCaption => String::from("figcaption"),
            Tag::HGroup => String::from("hgroup"),
            Tag::H1 => String::from("h1"),
            Tag::H2 => String::from("h2"),
            Tag::H3 => String::from("h3"),
            Tag::H4 => String::from("h4"),
            Tag::H5 => String::from("h5"),
            Tag::H6 => String::from("h6"),
            // Forms
            Tag::Form => String::from("form"),
            Tag::Input => String::from("input"),
            Tag::TextArea => String::from("textarea"),
            Tag::FieldSet => String::from("fieldset"),
            Tag::Label => String::from("label"),
            Tag::Select => String::from("select"),
            Tag::SelectOption => String::from("option"),
            Tag::Legend => String::from("legend"),
            // Formatting
            Tag::Mark => String::from("mark"),
            Tag::Small => String::from("small"),
            Tag::Em => String::from("em"),
            Tag::Strong => String::from("strong"),
            Tag::Br => String::from("br"),
            Tag::B => String::from("b"),
            Tag::I => String::from("i"),
            Tag::Pre => String::from("pre"),
            Tag::Sub => String::from("sub"),
            Tag::Sup => String::from("sup"),
            Tag::Progress => String::from("progress"),
            Tag::Code => String::from("code"),
            Tag::Cite => String::from("cite"),
            Tag::BlockQuote => String::from("blockquote"),
            Tag::Address => String::from("address"),
            Tag::Abbr => String::from("abbr"),
            Tag::Acronym => String::from("acronym"),
            Tag::Menu => String::from("menu"),
            // Tables
            Tag::Table => String::from("table"),
            Tag::Caption => String::from("caption"),
            Tag::Col => String::from("col"),
            Tag::ColGroup => String::from("colgroup"),
            Tag::TBody => String::from("tbody"),
            Tag::Td => String::from("td"),
            Tag::TFoot => String::from("tfoot"),
            Tag::THead => String::from("thead"),
            Tag::Th => String::from("th"),
            Tag::Tr => String::from("tr"),
            Tag::Script => String::from("script"),
            Tag::NoScript => String::from("noscript"),
            //Embed
            Tag::Canvas => String::from("canvas"),
            Tag::Video => String::from("video"),
            Tag::Audio => String::from("audio"),
            Tag::Frame => String::from("frame"),
            Tag::IFrame => String::from("iframe"),
            Tag::FrameSet => String::from("frameset"),
            Tag::Img => String::from("img"),

            Tag::Empty => String::from(""),
        }
    }

    pub fn str_as_tag(tag: &str) -> Tag {
        match tag {
            _ if tag == "div" => Tag::Div,
            _ if tag == "section" => Tag::Section,
            _ if tag == "header" => Tag::Header,
            _ if tag == "button" => Tag::Button,
            _ if tag == "p" => Tag::Paragraph,
            _ if tag == "a" => Tag::Anchor,
            _ if tag == "span" => Tag::Span,
            _ if tag == "ul" => Tag::Ul,
            _ if tag == "ol" => Tag::Ol,
            _ if tag == "li" => Tag::Li,
            _ if tag == "header" => Tag::Header,
            _ if tag == "footer" => Tag::Footer,
            _ if tag == "section" => Tag::Section,
            _ if tag == "aside" => Tag::Aside,
            _ if tag == "nav" => Tag::Nav,
            _ if tag == "main" => Tag::Main,
            _ if tag == "article" => Tag::Article,
            _ if tag == "details" => Tag::Details,
            _ if tag == "summary" => Tag::Summary,
            _ if tag == "figure" => Tag::Figure,
            _ if tag == "figcaption" => Tag::FigCaption,
            _ if tag == "hgroup" => Tag::HGroup,
            _ if tag == "h1" => Tag::H1,
            _ if tag == "h2" => Tag::H2,
            _ if tag == "h3" => Tag::H3,
            _ if tag == "h4" => Tag::H4,
            _ if tag == "h5" => Tag::H5,
            _ if tag == "h6" => Tag::H6,
            // Forms
            _ if tag == "form" => Tag::Form,
            _ if tag == "input" => Tag::Input,
            _ if tag == "textarea" => Tag::TextArea,
            _ if tag == "fieldset" => Tag::FieldSet,
            _ if tag == "label" => Tag::Label,
            _ if tag == "select" => Tag::Select,
            _ if tag == "option" => Tag::SelectOption,
            _ if tag == "legend" => Tag::Legend,
            // Formatting
            _ if tag == "mark" => Tag::Mark,
            _ if tag == "small" => Tag::Small,
            _ if tag == "em" => Tag::Em,
            _ if tag == "strong" => Tag::Strong,
            _ if tag == "br" => Tag::Br,
            _ if tag == "b" => Tag::B,
            _ if tag == "i" => Tag::I,
            _ if tag == "pre" => Tag::Pre,
            _ if tag == "sub" => Tag::Sub,
            _ if tag == "sup" => Tag::Sup,
            _ if tag == "progress" => Tag::Progress,
            _ if tag == "code" => Tag::Code,
            _ if tag == "cite" => Tag::Cite,
            _ if tag == "blockquote" => Tag::BlockQuote,
            _ if tag == "address" => Tag::Address,
            _ if tag == "abbr" => Tag::Abbr,
            _ if tag == "acronym" => Tag::Acronym,
            _ if tag == "menu" => Tag::Menu,
            // Tables
            _ if tag == "table" => Tag::Table,
            _ if tag == "caption" => Tag::Caption,
            _ if tag == "col" => Tag::Col,
            _ if tag == "colgroup" => Tag::ColGroup,
            _ if tag == "tbody" => Tag::TBody,
            _ if tag == "td" => Tag::Td,
            _ if tag == "tfoot" => Tag::TFoot,
            _ if tag == "thead" => Tag::THead,
            _ if tag == "th" => Tag::Th,
            _ if tag == "tr" => Tag::Tr,
            _ if tag == "script" => Tag::Script,
            _ if tag == "noscript" => Tag::NoScript,
            //Embed
            _ if tag == "canvas" => Tag::Canvas,
            _ if tag == "video" => Tag::Video,
            _ if tag == "audio" => Tag::Audio,
            _ if tag == "frame" => Tag::Frame,
            _ if tag == "iframe" => Tag::IFrame,
            _ if tag == "frameset" => Tag::FrameSet,
            _ if tag == "img" => Tag::Img,
            _ if tag.is_empty() => Tag::Empty,
            _ => Tag::Empty,
        }
    }

    pub fn is_element(&self, tag: &str) -> bool {
        if self.as_html_str() == tag.to_lowercase() {
            true
        } else {
            false
        }
    }

    pub fn is_tag(&self, tag: &Tag) -> bool {
        if self == tag {
            true
        } else {
            false
        }
    }

    pub fn as_tag(&self) -> &Self {
        self
    }
}

impl Default for Tag {
    fn default() -> Self { Tag::Div }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_html_str())
    }
}

impl From<&str> for Tag {
    fn from(tag: &str) -> Self {
        let tag = tag.to_lowercase();

        Tag::str_as_tag(&tag)
    }
}

impl From<String> for Tag {
    fn from(tag: String) -> Self {
        let tag = tag.to_lowercase();

        Tag::str_as_tag(&tag)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_tags() -> Vec<&'static str> {
        let tags = vec![
            "div", "button", "p", "a", "span",
            "ul", "ol", "li", "header", "footer",
            "section", "aside", "nav", "main", "article",
            "details", "summary", "figure", "figcaption", "hgroup",
            "h1", "h2", "h3", "h4", "h5", "h6", "form", "input",
            "textarea", "fieldset", "label", "select", "option", "legend",
            "mark", "small", "em", "strong", "br", "b", "i", "pre", "sub",
            "sup", "progress", "code", "cite", "blockquote", "address",
            "abbr", "acronym", "menu", "table", "caption", "col", "colgroup",
            "tbody", "td", "tfoot", "thead", "th", "tr", "script", "noscript",
            "canvas", "video", "audio", "frame", "iframe", "frameset", "img",
        ];

        tags
    }

    #[test]
    fn tag_as_html_str() {
        let section = Tag::Section;
        let div = Tag::Div;

        assert_eq!(section.as_html_str(), "section");
        assert_ne!(div.as_html_str(), "Div");
    }

    #[test]
    fn tag_default() {
        let default = Tag::default();

        assert_eq!(default, Tag::Div);
        assert_ne!(default, Tag::Section);
    }

    #[test]
    fn tag_works_in_format_str() {
        let section = Tag::Section;
        let expected_str = "Html Tag formatted as a <section></section>";
        let actual_str = format!("Html Tag formatted as a <{}></{}>", section, section);

        assert_eq!(expected_str, actual_str);
    }

    #[test]
    fn tag_is_element() {
        let ul = Tag::Ul;
        let is_ul = "UL";
        let is_not_ul = "Li";

        assert_eq!(ul.is_element(is_ul), true);
        assert_eq!(ul.is_element(is_not_ul), false);
    }

    #[test]
    fn tag_is_tag() {
        let nav = Tag::Nav;
        let is_nav = Tag::Nav;
        let is_not_nav = Tag::Main;

        assert_eq!(nav.is_tag(&is_nav), true);
        assert_eq!(nav.is_tag(&is_not_nav), false);
    }

    #[test]
    fn tag_from_str() {
        let section = Tag::from("SECTION");
        let none = Tag::from("header");

        assert_eq!(section, Tag::Section);
        assert_eq!(none, Tag::Header);
    }

    #[test]
    fn tag_empty() {
        let empty = Tag::Empty;
        let from_str_empty = Tag::from("notarealelement");
        let from_str_empty_empty = Tag::from("");

        assert_eq!(from_str_empty, empty);
        assert_eq!(from_str_empty_empty, empty);
    }

    #[test]
    fn tag_as_tag() {
        let section = Tag::from("section");

        println!("Section Tag as Tag {:?}", section.as_tag());
        assert_eq!(*section.as_tag(), Tag::Section);
    }

    #[test]
    fn tag_exhaustive() {
        let tags = get_tags();

        for t in tags {
            let tag = Tag::from(t);

            assert_eq!(tag.as_html_str(), t);
        }
    }
}

