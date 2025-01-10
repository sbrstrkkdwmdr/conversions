use std::f32::consts::PI;

use super::get_values;
use super::types;

pub fn units() -> Vec<types::Unit> {
    return vec![
        types::Unit {
            names: get_values("angle_grad".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("angle_deg".to_string()),
                    text: "x*0.9".to_string(),
                    calc: |x| x * 0.9,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("angle_rad".to_string()),
                    text: "(x*π)/200".to_string(),
                    calc: |x| (x * PI) / 200.0,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("angle_deg".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("angle_grad".to_string()),
                    text: "x/0.9".to_string(),
                    calc: |x| x / 0.9,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("angle_rad".to_string()),
                    text: "(x*π)/180".to_string(),
                    calc: |x| (x * PI) / 180.0,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("angle_rad".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("angle_grad".to_string()),
                    text: "(x*200)/π".to_string(),
                    calc: |x| (x * 200.0) / PI,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("angle_deg".to_string()),
                    text: "(x*180)/π".to_string(),
                    calc: |x| (x * 180.0) / PI,
                    can_use_si: false,
                },
            ],
        },
    ];
}
