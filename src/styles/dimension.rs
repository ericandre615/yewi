use std::ops::{Div, Sub, Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CSSDimension {
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

impl CSSDimension {
    pub fn as_css_str(&self) -> String {
        let val = match &self {
            CSSDimension::Px(v) => format!("{}px", v),
            CSSDimension::Em(v) => format!("{}em", v),
            CSSDimension::Cm(v) => format!("{}cm", v),
            CSSDimension::Mm(v) => format!("{}mm", v),
            CSSDimension::In(v) => format!("{}in", v),
            CSSDimension::Pt(v) => format!("{}pt", v),
            CSSDimension::Pc(v) => format!("{}pc", v),
            CSSDimension::Percent(v) => format!("{}%", v),
            CSSDimension::Rem(v) => format!("{}rem", v),
            CSSDimension::Empty => format!("{}", 0.0),
        };

        val
    }

    pub fn as_value(&self) -> f32 {
        let val = match *self {
            CSSDimension::Px(v) => v,
            CSSDimension::Em(v) => v,
            CSSDimension::Cm(v) => v,
            CSSDimension::Mm(v) => v,
            CSSDimension::In(v) => v,
            CSSDimension::Pt(v) => v,
            CSSDimension::Pc(v) => v,
            CSSDimension::Percent(v) => v,
            CSSDimension::Rem(v) => v,
            CSSDimension::Empty => 0.0,
        };

        val
    }

    fn match_output(&self, value: f32) -> CSSDimension {
        match &self {
            CSSDimension::Px(_) => CSSDimension::Px(value),
            CSSDimension::Em(_) => CSSDimension::Em(value),
            CSSDimension::Cm(_) => CSSDimension::Cm(value),
            CSSDimension::Mm(_) => CSSDimension::Mm(value),
            CSSDimension::In(_) => CSSDimension::In(value),
            CSSDimension::Pt(_) => CSSDimension::Pt(value),
            CSSDimension::Pc(_) => CSSDimension::Pc(value),
            CSSDimension::Percent(_) => CSSDimension::Percent(value),
            CSSDimension::Rem(_) => CSSDimension::Rem(value),
            CSSDimension::Empty => CSSDimension::Empty,
        }
    }
}

impl Default for CSSDimension {
    fn default() -> CSSDimension {
        CSSDimension::Empty
    }
}

impl std::fmt::Display for CSSDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_css_str())
    }
}

impl Div for CSSDimension {
    type Output = CSSDimension;

    fn div(self, other: CSSDimension) -> Self::Output {
        let value = self.as_value() / other.as_value();
        self.match_output(value)
    }
}

impl Div<f32> for CSSDimension {
    type Output = CSSDimension;

    fn div(self, other: f32) -> Self::Output {
        let value = self.as_value() / other;
        self.match_output(value)
    }
}

impl Sub for CSSDimension {
    type Output = CSSDimension;

    fn sub(self, other: CSSDimension) -> Self::Output {
        let value = self.as_value() - other.as_value();
        self.match_output(value)
    }
}

impl Sub<f32> for CSSDimension {
    type Output = CSSDimension;

    fn sub(self, other: f32) -> Self::Output {
        let value = self.as_value() - other;
        self.match_output(value)
    }
}

impl Add for CSSDimension {
    type Output = CSSDimension;

    fn add(self, other: CSSDimension) -> Self::Output {
        let value = self.as_value() + other.as_value();
        self.match_output(value)
    }
}

impl Add<f32> for CSSDimension {
    type Output = CSSDimension;

    fn add(self, other: f32) -> Self::Output {
        let value = self.as_value() + other;
        self.match_output(value)
    }
}

impl Mul for CSSDimension {
    type Output = CSSDimension;

    fn mul(self, other: CSSDimension) -> Self::Output {
        let value = self.as_value() * other.as_value();
        self.match_output(value)
    }
}

impl Mul<f32> for CSSDimension {
    type Output = CSSDimension;

    fn mul(self, other: f32) -> Self::Output {
        let value = self.as_value() * other;
        self.match_output(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_css_dimensions() -> Vec<(CSSDimension, f32, &'static str)> {
        vec![
            (CSSDimension::Px(50.0), 50_f32, "px"),
            (CSSDimension::Em(50.1), 50.1, "em"),
            (CSSDimension::Cm(50.2), 50.2, "cm"),
            (CSSDimension::Mm(50.3), 50.3, "mm"),
            (CSSDimension::In(50.4), 50.4, "in"),
            (CSSDimension::Pt(50.5), 50.5, "pt"),
            (CSSDimension::Pc(50.6), 50.6, "pc"),
            (CSSDimension::Percent(50.7), 50.7, "%"),
            (CSSDimension::Rem(50.8), 50.8, "rem"),
            (CSSDimension::Empty, 0.0, ""),
        ]
    }

    #[test]
    fn css_default() {
        let default = CSSDimension::default();
        let empty = CSSDimension::Empty;
        let expected_value = 0.0;
        let expected_str = "0";

        assert_eq!(empty, default);
        assert_eq!(expected_value, empty.as_value());
        assert_eq!(expected_str, empty.as_css_str());
    }

    #[test]
    fn css_maths() {
        let px = CSSDimension::Px(200.0);
        let px2 = CSSDimension::Px(50.0);
        let em = CSSDimension::Em(20.0);

        assert_eq!((px - px2), CSSDimension::Px(150.0));
        assert_eq!(px / 2.0, CSSDimension::Px(100.0));
        assert_eq!(px * px2, CSSDimension::Px(10000.0));
        assert_eq!(px + px2, CSSDimension::Px(250.0));
        assert_eq!(px + em, CSSDimension::Px(220.0));
        assert_eq!(em + px, CSSDimension::Em(220.0));
    }

    #[test]
    fn exhaustive_css_as_str() {
        //let px = CSSDimension::Px(20.0);
        //let percent = CSSDimension::Percent(20.0);
        //let em = CSSDimension::Em(20.0);
        //let cm = CSSDimension::Cm(20.0);
        //let mm = CSSDimension::Mm(20.0);
        //let inch = CSSDimension::In(20.0);
        //let pt = CSSDimension::Pt(20.0);
        //let pc = CSSDimension::Pc(20.0);
        //let rem = CSSDimension::Rem(20.0);

        //assert_eq!(px.as_css_str(), "20.0px");
        //assert_eq!(percent.as_css_str(), "20.0%");
        //assert_eq!(em.as_css_str(), "20.0em");
        //assert_eq!(cm.as_css_str(), "20.0cm");
        //assert_eq!(mm.as_css_str(), "20.0mm");
        //assert_eq!(inch.as_css_str(), "20.0in");
        //assert_eq!(pt.as_css_str(), "20.0pt");
        //assert_eq!(pc.as_css_str(), "20.0pc");
        //assert_eq!(rem.as_css_str(), "20.0rem");
        let pairs = get_css_dimensions();

        for p in pairs {
            let (dimension, dim_value, dim_str) = p;
            let actual_str = format!("{}{}", dim_value, dim_str);

            assert_eq!(dimension.as_css_str(), actual_str);
        }
    }

    #[test]
    fn css_dimensions_works_in_format_str() {
        let expected_str_px = "width: 50.1px";
        let expected_str_percent = "width: 40.25%";
        let actual_px = format!("width: {}", CSSDimension::Px(50.10));
        let actual_percent = format!("width: {}", CSSDimension::Percent(40.25));

        assert_eq!(expected_str_px, actual_px);
        assert_eq!(expected_str_percent, actual_percent);
    }

    #[test]
    fn css_dimensions_removes_single_zero_as_str() {
        let expected_str = "width: 50px";
        let actual_str = format!("width: {}", CSSDimension::Px(50.000));

        assert_eq!(expected_str, actual_str);
    }

    #[test]
    fn css_as_value() {
        let px = CSSDimension::Px(20.0);
        let percent = CSSDimension::Percent(40.0);

        assert_eq!(px.as_value(), 20.0);
        assert_eq!(percent.as_value(), 40.0);
    }
}

