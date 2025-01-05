pub(crate) struct Unit {
    pub name: String,
    pub names: Vec<String>,
    pub can_use_si: bool, // whether or not it can use SI prefixes eg. kilo-, centi-
    pub conversions: Vec<Conversion>,
}

#[derive(Clone)]
pub struct Conversion {
    pub name: String,
    pub names: Vec<String>,
    pub text: String, // a text formatted way of calculating eg. x*2.5
    pub calc: fn(f32) -> f32,
}


pub fn template_unit() -> Unit {
    return Unit {
        name: "TEMPLATE".to_string(),
        names: vec!["TEMPLATE".to_string()],
        can_use_si: false,
        conversions: vec![template_conversion()],
    };
}

pub fn template_conversion() -> Conversion {
    return Conversion {
        name: "TEMPLATE".to_string(),
        names: vec!["TEMPLATE".to_string()],
        text: "x".to_string(),
        calc: |x| x,
    };
}