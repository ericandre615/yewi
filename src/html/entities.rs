#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Entity {
    Copyright,
    Reg,
    Trade,
    Commat,
    Star,
    Starf,
    Sung,
    Flat,
    Natural,
    Sharp,
    Check,
    Cross,
    Excl,
    Num,
    Amp,
    Ast,
    Quest,
    Hat,
    Lbrace,
    Rbrace,
    Tilde,
    Circ,
    Nbsp,
    Ensp,
    Emsp,
    Iexcl,
    Micro,
    Para,
    Iquest,
    Hyphen,
    Ndash,
    Mdash,
    Horbar,
    Bull,
    Caret,
    Quot,
    Apos,
    Laquo,
    Raquo,
    Lsaquo,
    Rsaquo,
    Plus,
    Minus,
    Times,
    Divide,
    Equals,
    Ne,
    Plusmn,
    Not,
    Lt,
    Gt,
    Deg,
    Radic,
    Infin,
    Alpha,
    Beta,
    Gamma,
    Delta,
    Epsilon,
    Zeta,
    Eta,
    Theta,
    Iota,
    Kappa,
    Lambda,
    Mu,
    Nu,
    Xi,
    Omicron,
    Pi,
    Phi,
    Omega,

    Empty,
}

impl Entity {
    pub fn as_unicode_str(&self) -> &str {
        match *self {
            Entity::Copyright => "\u{00A9}",
            Entity::Reg => "\u{00AE}",
            Entity::Trade => "\u{2122}",
            Entity::Commat => "\u{0040}",
            Entity::Star => "\u{2606}",
            Entity::Starf => "\u{2605}",
            Entity::Sung => "\u{266A}",
            Entity::Flat => "\u{266D}",
            Entity::Natural => "\u{266E}",
            Entity::Sharp => "\u{266F}",
            Entity::Check => "\u{2713}",
            Entity::Cross => "\u{2717}",
            Entity::Excl => "\u{0021}",
            Entity::Num => "\u{0023}",
            Entity::Amp => "\u{0026}",
            Entity::Ast => "\u{002A}",
            Entity::Quest => "\u{003F}",
            Entity::Hat => "\u{005E}",
            Entity::Lbrace => "\u{007B}",
            Entity::Rbrace => "\u{007D}",
            Entity::Tilde => "\u{007E}",
            Entity::Circ => "\u{02C6}",
            Entity::Nbsp => "\u{00A0}",
            Entity::Ensp => "\u{2002}",
            Entity::Emsp => "\u{2003}",
            Entity::Iexcl => "\u{00A1}",
            Entity::Micro => "\u{00B5}",
            Entity::Para => "\u{00B6}",
            Entity::Iquest => "\u{00BF}",
            Entity::Hyphen => "\u{2010}",
            Entity::Ndash => "\u{2013}",
            Entity::Mdash => "\u{2014}",
            Entity::Horbar => "\u{2015}",
            Entity::Bull => "\u{2022}",
            Entity::Caret => "\u{2041}",
            Entity::Quot => "\u{0022}",
            Entity::Apos => "\u{0027}",
            Entity::Laquo => "\u{00AB}",
            Entity::Raquo => "\u{00BB}",
            Entity::Lsaquo => "\u{2039}",
            Entity::Rsaquo => "\u{203A}",
            Entity::Plus => "\u{002B}",
            Entity::Minus => "\u{2212}",
            Entity::Times => "\u{00D7}",
            Entity::Divide => "\u{00F7}",
            Entity::Equals => "\u{003D}",
            Entity::Ne => "\u{2260}",
            Entity::Plusmn => "\u{00B1}",
            Entity::Not => "\u{00AC}",
            Entity::Lt => "\u{003C}",
            Entity::Gt => "\u{003E}",
            Entity::Deg => "\u{00B0}",
            Entity::Radic => "\u{221A}",
            Entity::Infin => "\u{221E}",
            Entity::Alpha => "\u{03B1}",
            Entity::Beta => "\u{03B2}",
            Entity::Gamma => "\u{03B3}",
            Entity::Delta => "\u{03B4}",
            Entity::Epsilon => "\u{03B5}",
            Entity::Zeta => "\u{03B6}",
            Entity::Eta => "\u{03B7}",
            Entity::Theta => "\u{03B8}",
            Entity::Iota => "\u{03B9}",
            Entity::Kappa => "\u{03BA}",
            Entity::Lambda => "\u{03BB}",
            Entity::Mu => "\u{03BC}",
            Entity::Nu => "\u{03BD}",
            Entity::Xi => "\u{03BE}",
            Entity::Omicron => "\u{03BF}",
            Entity::Pi => "\u{03C0}",
            Entity::Phi => "\u{03C6}",
            Entity::Omega => "\u{03C9}",

            Entity::Empty => "",
        }
    }

    pub fn as_uni_str(&self) -> &str {
        self.as_unicode_str()
    }

    pub fn str_as_enum(entity: &str) -> Entity {
        match entity {
            _ if entity == "\u{00A9}" => Entity::Copyright,
            _ if entity == "\u{00AE}" => Entity::Reg,
            _ if entity == "\u{2122}" => Entity::Trade,
            _ if entity == "\u{0040}" => Entity::Commat,
            _ if entity == "\u{2606}" => Entity::Star,
            _ if entity == "\u{2605}" => Entity::Starf,
            _ if entity == "\u{266A}" => Entity::Sung,
            _ if entity == "\u{266D}" => Entity::Flat,
            _ if entity == "\u{266E}" => Entity::Natural,
            _ if entity == "\u{266F}" => Entity::Sharp,
            _ if entity == "\u{2713}" => Entity::Check,
            _ if entity == "\u{2717}" => Entity::Cross,
            _ if entity == "\u{0021}" => Entity::Excl,
            _ if entity == "\u{0023}" => Entity::Num,
            _ if entity == "\u{0026}" => Entity::Amp,
            _ if entity == "\u{002A}" => Entity::Ast,
            _ if entity == "\u{003F}" => Entity::Quest,
            _ if entity == "\u{005E}" => Entity::Hat,
            _ if entity == "\u{007B}" => Entity::Lbrace,
            _ if entity == "\u{007D}" => Entity::Rbrace,
            _ if entity == "\u{007E}" => Entity::Tilde,
            _ if entity == "\u{02C6}" => Entity::Circ,
            _ if entity == "\u{00A0}" => Entity::Nbsp,
            _ if entity == "\u{2002}" => Entity::Ensp,
            _ if entity == "\u{2003}" => Entity::Emsp,
            _ if entity == "\u{00A1}" => Entity::Iexcl,
            _ if entity == "\u{00B5}" => Entity::Micro,
            _ if entity == "\u{00B6}" => Entity::Para,
            _ if entity == "\u{00BF}" => Entity::Iquest,
            _ if entity == "\u{2010}" => Entity::Hyphen,
            _ if entity == "\u{2013}" => Entity::Ndash,
            _ if entity == "\u{2014}" => Entity::Mdash,
            _ if entity == "\u{2015}" => Entity::Horbar,
            _ if entity == "\u{2022}" => Entity::Bull,
            _ if entity == "\u{2041}" => Entity::Caret,
            _ if entity == "\u{0022}" => Entity::Quot,
            _ if entity == "\u{0027}" => Entity::Apos,
            _ if entity == "\u{00AB}" => Entity::Laquo,
            _ if entity == "\u{00BB}" => Entity::Raquo,
            _ if entity == "\u{2039}" => Entity::Lsaquo,
            _ if entity == "\u{203A}" => Entity::Rsaquo,
            _ if entity == "\u{002B}" => Entity::Plus,
            _ if entity == "\u{2212}" => Entity::Minus,
            _ if entity == "\u{00D7}" => Entity::Times,
            _ if entity == "\u{00F7}" => Entity::Divide,
            _ if entity == "\u{003D}" => Entity::Equals,
            _ if entity == "\u{2260}" => Entity::Ne,
            _ if entity == "\u{00B1}" => Entity::Plusmn,
            _ if entity == "\u{00AC}" => Entity::Not,
            _ if entity == "\u{003C}" => Entity::Lt,
            _ if entity == "\u{003E}" => Entity::Gt,
            _ if entity == "\u{00B0}" => Entity::Deg,
            _ if entity == "\u{221A}" => Entity::Radic,
            _ if entity == "\u{221E}" => Entity::Infin,
            _ if entity == "\u{03B1}" => Entity::Alpha,
            _ if entity == "\u{03B2}" => Entity::Beta,
            _ if entity == "\u{03B3}" => Entity::Gamma,
            _ if entity == "\u{03B4}" => Entity::Delta,
            _ if entity == "\u{03B5}" => Entity::Epsilon,
            _ if entity == "\u{03B6}" => Entity::Zeta,
            _ if entity == "\u{03B7}" => Entity::Eta,
            _ if entity == "\u{03B8}" => Entity::Theta,
            _ if entity == "\u{03B9}" => Entity::Iota,
            _ if entity == "\u{03BA}" => Entity::Kappa,
            _ if entity == "\u{03BB}" => Entity::Lambda,
            _ if entity == "\u{03BC}" => Entity::Mu,
            _ if entity == "\u{03BD}" => Entity::Nu,
            _ if entity == "\u{03BE}" => Entity::Xi,
            _ if entity == "\u{03BF}" => Entity::Omicron,
            _ if entity == "\u{03C0}" => Entity::Pi,
            _ if entity == "\u{03C6}" => Entity::Phi,
            _ if entity == "\u{03C9}" => Entity::Omega,

            _ if entity.is_empty() => Entity::Empty,
            _ => Entity::Empty,
        }
    }
}

impl Default for Entity {
    fn default() -> Self { Entity::Empty }
}

impl From<&str> for Entity {
    fn from(entity: &str) -> Self {
        Entity::str_as_enum(&entity)
    }
}

impl From<String> for Entity {
    fn from(entity: String) -> Self {
        Entity::str_as_enum(&entity)
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_unicode_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_entities() -> Vec<(Entity, &'static str)> {
        let entities = vec![
            (Entity::Copyright, "©"),
            (Entity::Reg, "®"),
            (Entity::Trade, "™"),
            (Entity::Commat, "@"),
            (Entity::Star, "☆"),
            (Entity::Starf, "★"),
            (Entity::Sung, "♪"),
            (Entity::Flat, "♭"),
            (Entity::Natural, "♮"),
            (Entity::Sharp, "♯"),
            (Entity::Check, "✓"),
            (Entity::Cross, "✗"),
            (Entity::Excl, "!"),
            (Entity::Num, "#"),
            (Entity::Amp, "&"),
            (Entity::Ast, "*"),
            (Entity::Quest, "?"),
            (Entity::Hat, "^"),
            (Entity::Lbrace, "{"),
            (Entity::Rbrace, "}"),
            (Entity::Tilde, "~"),
            (Entity::Circ, "ˆ"),
            (Entity::Nbsp, " "), // the spaces are invisible, but different
            (Entity::Ensp, " "),
            (Entity::Emsp, " "),
            (Entity::Iexcl, "¡"),
            (Entity::Micro, "µ"),
            (Entity::Para, "¶"),
            (Entity::Iquest, "¿"),
            (Entity::Hyphen, "‐"),
            (Entity::Ndash, "–"),
            (Entity::Mdash, "—"),
            (Entity::Horbar, "―"),
            (Entity::Bull, "•"),
            (Entity::Caret, "⁁"),
            (Entity::Quot, "\""),
            (Entity::Apos, "'"),
            (Entity::Laquo, "«"),
            (Entity::Raquo, "»"),
            (Entity::Lsaquo, "‹"),
            (Entity::Rsaquo, "›"),
            (Entity::Plus, "+"),
            (Entity::Minus, "−"),
            (Entity::Times, "×"),
            (Entity::Divide, "÷"),
            (Entity::Equals, "="),
            (Entity::Ne, "≠"),
            (Entity::Plusmn, "±"),
            (Entity::Not, "¬"),
            (Entity::Lt, "<"),
            (Entity::Gt, ">"),
            (Entity::Deg, "°"),
            (Entity::Radic, "√"),
            (Entity::Infin, "∞"),
            (Entity::Alpha, "α"),
            (Entity::Beta, "β"),
            (Entity::Gamma, "γ"),
            (Entity::Delta, "δ"),
            (Entity::Epsilon, "ε"),
            (Entity::Zeta, "ζ"),
            (Entity::Eta, "η"),
            (Entity::Theta, "θ"),
            (Entity::Iota, "ι"),
            (Entity::Kappa, "κ"),
            (Entity::Lambda, "λ"),
            (Entity::Mu, "μ"),
            (Entity::Nu, "ν"),
            (Entity::Xi, "ξ"),
            (Entity::Omicron, "ο"),
            (Entity::Pi, "π"),
            (Entity::Phi, "φ"),
            (Entity::Omega, "ω"),

            (Entity::Empty, ""),
        ];

        entities
    }

    #[test]
    fn defaults_to_empty() {
        let default = Entity::default();
        let expected = Entity::Empty;

        assert_eq!(expected, default);
    }

    #[test]
    fn unrecognized_from_is_empty() {
        let actual = Entity::from("nothing");
        let expected = Entity::Empty;

        assert_eq!(expected, actual);
    }

    #[test]
    fn implements_from() {
        let actual_from_code = Entity::from("\u{00A9}");
        let actual_from_literal = Entity::from("©");
        let expected = Entity::Copyright;

        assert_eq!(expected, actual_from_code);
        assert_eq!(expected, actual_from_literal);
    }

    #[test]
    fn entity_as_unicode_str() {
        let copyright = Entity::Copyright;

        assert_eq!(copyright.as_unicode_str(), "\u{00A9}");
        assert_eq!(copyright.as_uni_str(), "\u{00A9}");
    }

    #[test]
    fn entity_works_in_format_str() {
        let trade = Entity::Trade;
        let expected_str = "Entity works in format str ™";
        let actual_str = format!("Entity works in format str {}", trade);

        assert_eq!(expected_str, actual_str);
    }

    #[test]
    fn entities_exhaustive() {
        let pairs = get_entities();

        for p in pairs {
            let (entity, unicode) = p;

            assert_eq!(entity.as_unicode_str(), unicode);
        }
    }

    #[test]
    fn entity_from_str_exhaustive() {
        let pairs = get_entities();

        for p in pairs {
            let (entity, unicode_str) = p;
            let from_str = Entity::from(unicode_str);

            assert_eq!(from_str, entity);
        }
    }
}

