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
    fn entity_as_unicode_str() {
        let copyright = Entity::Copyright;

        assert_eq!(copyright.as_unicode_str(), "\u{00A9}");
        assert_eq!(copyright.as_uni_str(), "\u{00A9}");
    }

    #[test]
    fn entities_exhaustive() {
        let pairs = get_entities();

        for p in pairs {
            let (entity, unicode) = p;

            assert_eq!(entity.as_unicode_str(), unicode);
        }
    }
}

