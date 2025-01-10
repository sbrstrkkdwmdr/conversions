use super::get_values;
use super::types;

pub fn units() -> Vec<types::Unit> {
    return vec![
        types::Unit {
            names: get_values("pres_pa".to_string()),
            can_use_si: true,
            conversions: vec![
                types::Conversion {
                    names: get_values("pres_mmHg".to_string()),
                    text: "x/133.322".to_string(),
                    calc: |x| x / 133.322,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pres_psi".to_string()),
                    text: "x/6894.76".to_string(),
                    calc: |x| x / 6894.76,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pres_bar".to_string()),
                    text: "x/100000".to_string(),
                    calc: |x| x / 100000.0,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("pres_atm".to_string()),
                    text: "x/101325".to_string(),
                    calc: |x| x / 101325.0,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("pres_mmHg".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("pres_pa".to_string()),
                    text: "x*133.322".to_string(),
                    calc: |x| x * 133.322,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("pres_psi".to_string()),
                    text: "x/51.7149".to_string(),
                    calc: |x| x / 51.7149,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pres_bar".to_string()),
                    text: "x/750.062".to_string(),
                    calc: |x| x / 750.062,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("pres_atm".to_string()),
                    text: "x/760".to_string(),
                    calc: |x| x / 760.0,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("pres_psi".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("pres_pa".to_string()),
                    text: "x*6894.76".to_string(),
                    calc: |x| x * 6894.76,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("pres_mmHg".to_string()),
                    text: "x*51.7149".to_string(),
                    calc: |x| x * 51.7149,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pres_bar".to_string()),
                    text: "x/14.504".to_string(),
                    calc: |x| x / 14.504,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("pres_atm".to_string()),
                    text: "x/14.696".to_string(),
                    calc: |x| x / 14.696,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("pres_bar".to_string()),
            can_use_si: true,
            conversions: vec![
                types::Conversion {
                    names: get_values("pres_pa".to_string()),
                    text: "x*100000".to_string(),
                    calc: |x| x * 100000.0,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("pres_mmHg".to_string()),
                    text: "x*750.062".to_string(),
                    calc: |x| x * 750.062,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pres_psi".to_string()),
                    text: "x*14.504".to_string(),
                    calc: |x| x * 14.504,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pres_atm".to_string()),
                    text: "x/1.013".to_string(),
                    calc: |x| x / 1.013,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("pres_atm".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("pres_pa".to_string()),
                    text: "x*101325".to_string(),
                    calc: |x| x * 101325.0,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("pres_mmHg".to_string()),
                    text: "x*760".to_string(),
                    calc: |x| x * 760.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pres_psi".to_string()),
                    text: "x*14.696".to_string(),
                    calc: |x| x * 14.696,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pres_bar".to_string()),
                    text: "x*1.013".to_string(),
                    calc: |x| x * 1.013,
                    can_use_si: true,
                },
            ],
        },
    ];
}
