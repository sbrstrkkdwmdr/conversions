use super::get_values;
use super::types;

pub fn units() -> Vec<types::Unit> {
    return vec![
        types::Unit {
            name: "PLACEHOLDER".to_string(),
            names: get_values("PLACEHOLDER".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    name: "PLACEHOLDER".to_string(),
                    names: get_values("PLACEHOLDER".to_string()),
                    text: "PLACEHOLDER".to_string(),
                    calc: |x| placeholder,
                    can_use_si: false,
                },
            ],
        },
    ];
}