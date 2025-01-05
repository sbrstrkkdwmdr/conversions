pub(crate) struct Unit {
    pub name: String,
    pub names: Vec<String>,
    pub can_use_si: bool, // whether or not it can use SI prefixes eg. kilo-, centi-
    pub conversions: Vec<Conversion>,
}

pub struct Conversion {
    pub name: String,
    pub names: Vec<String>,
    pub text: String, // a text formatted way of calculating eg. x*2.5
    pub calc: fn(f32) -> f32,
}
