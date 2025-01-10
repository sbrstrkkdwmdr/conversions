use super::get_values;
use super::types;

pub fn units() -> Vec<types::Unit> {
    return vec![
        types::Unit {
            names: get_values("pow_erg".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("pow_w".to_string()),
                    text: "x/1e7".to_string(),
                    calc: |x| x / 1e7,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("pow_dbm".to_string()),
                    text: "x/1e7*30".to_string(),
                    calc: |x| x / 1e7 * 30.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_ftlbsec".to_string()),
                    text: "x/1.3558179483e7".to_string(),
                    calc: |x| x / 1.3558179483e7,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_calsec".to_string()),
                    text: "x*(6/25e7)".to_string(),
                    calc: |x| x * (6.0 / 25e7),
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("pow_horse".to_string()),
                    text: "x/745.7e7".to_string(),
                    calc: |x| x / 745.7e7,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_btusec".to_string()),
                    text: "x/1.0550558526e10".to_string(),
                    calc: |x| x / 1.0550558526e10,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("pow_w".to_string()),
            can_use_si: true,
            conversions: vec![
                types::Conversion {
                    names: get_values("pow_erg".to_string()),
                    text: "x*1e7".to_string(),
                    calc: |x| x * 1e7,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_dbm".to_string()),
                    text: "x*30".to_string(),
                    calc: |x| x * 30.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_ftlbsec".to_string()),
                    text: "x/1.3558179483".to_string(),
                    calc: |x| x / 1.3558179483,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_calsec".to_string()),
                    text: "x*(6/25)".to_string(),
                    calc: |x| x * (6.0 / 25.0),
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("pow_horse".to_string()),
                    text: "x/745.7".to_string(),
                    calc: |x| x / 745.7,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_btusec".to_string()),
                    text: "x/1055.0558526".to_string(),
                    calc: |x| x / 1055.0558526,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("pow_dbm".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("pow_erg".to_string()),
                    text: "x*1e7/30".to_string(),
                    calc: |x| x * 1e7 / 30.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_w".to_string()),
                    text: "x/30".to_string(),
                    calc: |x| x / 30.0,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("pow_ftlbsec".to_string()),
                    text: "x/(1.3558179483*30)".to_string(),
                    calc: |x| x / (1.3558179483 * 30.0),
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_calsec".to_string()),
                    text: "x*(6/(25*30))".to_string(),
                    calc: |x| x * (6.0 / (25.0 * 30.0)),
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("pow_horse".to_string()),
                    text: "x/(745.7*30)".to_string(),
                    calc: |x| x / (745.7 * 30.0),
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_btusec".to_string()),
                    text: "x/(1055.0558526*30)".to_string(),
                    calc: |x| x / (1055.0558526 * 30.0),
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("pow_ftlbsec".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("pow_erg".to_string()),
                    text: "x*1.3558179483e7".to_string(),
                    calc: |x| x * 1.3558179483e7,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_w".to_string()),
                    text: "x*1.3558179483".to_string(),
                    calc: |x| x * 1.3558179483,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("pow_dbm".to_string()),
                    text: "x*30*1.3558179483".to_string(),
                    calc: |x| x * 30.0 * 1.3558179483,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_calsec".to_string()),
                    text: "x*(6/25*1.3558179483)".to_string(),
                    calc: |x| x * (6.0 / 25.0 * 1.3558179483),
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("pow_horse".to_string()),
                    text: "x*1.3558179483/745.7".to_string(),
                    calc: |x| x * 1.3558179483 / 745.7,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_btusec".to_string()),
                    text: "x*1.3558179483/1055.0558526".to_string(),
                    calc: |x| x * 1.3558179483 / 1055.0558526,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("pow_calsec".to_string()),
            can_use_si: true,
            conversions: vec![
                types::Conversion {
                    names: get_values("pow_erg".to_string()),
                    text: "(x*1e7)/(6/25)".to_string(),
                    calc: |x| (x * 1e7) / (6.0 / 25.0),
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_w".to_string()),
                    text: "x/(6/25)".to_string(),
                    calc: |x| x / (6.0 / 25.0),
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("pow_dbm".to_string()),
                    text: "(x*30)/(6/25)".to_string(),
                    calc: |x| (x * 30.0) / (6.0 / 25.0),
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_ftlbsec".to_string()),
                    text: "(x/1.3558179483)/(6/25)".to_string(),
                    calc: |x| (x / 1.3558179483) / (6.0 / 25.0),
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_horse".to_string()),
                    text: "(x/745.7)*(6/25)".to_string(),
                    calc: |x| (x / 745.7) * (6.0 / 25.0),
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_btusec".to_string()),
                    text: "(x/1055.0558526)*(6/25)".to_string(),
                    calc: |x| (x / 1055.0558526) * (6.0 / 25.0),
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("pow_horse".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("pow_erg".to_string()),
                    text: "x*745.7e7".to_string(),
                    calc: |x| x * 745.7e7,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_w".to_string()),
                    text: "x*745.7".to_string(),
                    calc: |x| x * 745.7,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("pow_dbm".to_string()),
                    text: "x*30*745.7".to_string(),
                    calc: |x| x * 30.0 * 745.7,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_ftlbsec".to_string()),
                    text: "(x*745.7)/1.3558179483".to_string(),
                    calc: |x| (x * 745.7) / 1.3558179483,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_calsec".to_string()),
                    text: "x*745.7(6/25)".to_string(),
                    calc: |x| x * 745.7 * (6.0 / 25.0),
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("pow_btusec".to_string()),
                    text: "x*745.7/1055.055852".to_string(),
                    calc: |x| x * 745.7 / 1055.055852,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("pow_btusec".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("pow_erg".to_string()),
                    text: "x*1055.0558526e7".to_string(),
                    calc: |x| x * 1055.0558526e7,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_w".to_string()),
                    text: "x*1055.0558526".to_string(),
                    calc: |x| x * 1055.0558526,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("pow_dbm".to_string()),
                    text: "x*30*1055.0558526".to_string(),
                    calc: |x| x * 30.0 * 1055.0558526,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_ftlbsec".to_string()),
                    text: "x*1055.0558526/1.3558179483".to_string(),
                    calc: |x| x * 1055.0558526 / 1.3558179483,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("pow_calsec".to_string()),
                    text: "x*1055.0558526*(6/25)".to_string(),
                    calc: |x| x * 1055.0558526 * (6.0 / 25.0),
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("pow_horse".to_string()),
                    text: "x*1055.0558526/745.7".to_string(),
                    calc: |x| x * 1055.0558526 / 745.7,
                    can_use_si: false,
                },
            ],
        },
    ];
}
