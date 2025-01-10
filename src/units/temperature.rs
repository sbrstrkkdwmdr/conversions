use super::get_values;
use super::types;

pub fn units() -> Vec<types::Unit> {
    return vec![
        types::Unit {
            names: get_values("temp_c".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("temp_f".to_string()),
                    text: "x*9/5+32".to_string(),
                    calc: |x| x * 9.0 / 5.0 + 32.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("temp_k".to_string()),
                    text: "x+273.15".to_string(),
                    calc: |x| x + 273.15,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("temp_f".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("temp_c".to_string()),
                    text: "((x)-32)*5/9".to_string(),
                    calc: |x| (x - 32.0) * 5.0 / 9.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("temp_k".to_string()),
                    text: "((x)-32)*5/9 + 273.15".to_string(),
                    calc: |x| (x - 32.0) * 5.0 / 9.0 + 273.15,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("temp_k".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("temp_c".to_string()),
                    text: "x-273.15".to_string(),
                    calc: |x| x - 273.15,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("temp_f".to_string()),
                    text: "(x-273.15)*9/5+32".to_string(),
                    calc: |x| (x - 273.15) * 9.0 / 5.0 + 32.0,
                    can_use_si: false,
                },
            ],
        },
    ];
}
