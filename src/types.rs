pub(crate) struct Unit {
    pub names: Vec<String>,
    pub can_use_si: bool, // whether or not it can use SI prefixes eg. kilo-, centi-
    pub conversions: Vec<Conversion>,
}

#[derive(Clone)]
pub struct Conversion {
    pub names: Vec<String>,
    pub text: String, // a text formatted way of calculating eg. x*2.5
    pub calc: fn(f32) -> f32,
    pub can_use_si: bool,
}

pub fn template_unit() -> Unit {
    return Unit {
        names: vec!["TEMPLATE".to_string()],
        can_use_si: false,
        conversions: vec![template_conversion()],
    };
}

pub fn template_conversion() -> Conversion {
    return Conversion {
        names: vec!["TEMPLATE".to_string()],
        text: "x".to_string(),
        calc: |x| x,
        can_use_si: false,
    };
}

pub struct Prefix {
    pub prefix: String,
    pub name: String,
    pub value: f32,
}
