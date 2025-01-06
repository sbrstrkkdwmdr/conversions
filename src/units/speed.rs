use super::get_values;
use super::types;

pub fn units() -> Vec<types::Unit> {
    return vec![
        types::Unit {
            name: "Kilometres per hour".to_string(),
            names: get_values("speed_kmh".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    name: "Miles per hour".to_string(),
                    names: get_values("speed_mph".to_string()),
                    text: "x/1.609".to_string(),
                    calc: |x| x / 1.609,
                    can_use_si: false,
                },
                types::Conversion {
                    name: "Knot".to_string(),
                    names: get_values("speed_kt".to_string()),
                    text: "x/1.852".to_string(),
                    calc: |x| x / 1.852,
                    can_use_si: false,
                },
                types::Conversion {
                    name: "Metres per second".to_string(),
                    names: get_values("speed_ms".to_string()),
                    text: "x/3.6".to_string(),
                    calc: |x| x / 3.6,
                    can_use_si: true,
                },
                types::Conversion {
                    name: "Light".to_string(),
                    names: get_values("speed_c".to_string()),
                    text: "x/1079252848.8".to_string(),
                    calc: |x| x / 1079252848.8,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            name: "Miles per hour".to_string(),
            names: get_values("speed_mph".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    name: "Kilometres per hour".to_string(),
                    names: get_values("speed_kmh".to_string()),
                    text: "x*1.609".to_string(),
                    calc: |x| x * 1.609,
                    can_use_si: false,
                },
                types::Conversion {
                    name: "Knot".to_string(),
                    names: get_values("speed_kt".to_string()),
                    text: "x/1.151".to_string(),
                    calc: |x| x / 1.151,
                    can_use_si: false,
                },
                types::Conversion {
                    name: "Metres per second".to_string(),
                    names: get_values("speed_ms".to_string()),
                    text: "x/2.237".to_string(),
                    calc: |x| x / 2.237,
                    can_use_si: true,
                },
                types::Conversion {
                    name: "Light".to_string(),
                    names: get_values("speed_c".to_string()),
                    text: "x/670635728.546".to_string(),
                    calc: |x| x / 670635728.546,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            name: "Knot".to_string(),
            names: get_values("speed_kt".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    name: "Kilometres per hour".to_string(),
                    names: get_values("speed_kmh".to_string()),
                    text: "x*1.852".to_string(),
                    calc: |x| x * 1.852,
                    can_use_si: false,
                },
                types::Conversion {
                    name: "Miles per hour".to_string(),
                    names: get_values("speed_mph".to_string()),
                    text: "x*1.151".to_string(),
                    calc: |x| x * 1.151,
                    can_use_si: false,
                },
                types::Conversion {
                    name: "Metres per second".to_string(),
                    names: get_values("speed_ms".to_string()),
                    text: "x/1.944".to_string(),
                    calc: |x| x / 1.944,
                    can_use_si: true,
                },
                types::Conversion {
                    name: "Light".to_string(),
                    names: get_values("speed_c".to_string()),
                    text: "x/582796538.352".to_string(),
                    calc: |x| x / 582796538.352,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            name: "Metres per second".to_string(),
            names: get_values("speed_ms".to_string()),
            can_use_si: true,
            conversions: vec![
                types::Conversion {
                    name: "Kilometres per hour".to_string(),
                    names: get_values("speed_kmh".to_string()),
                    text: "x*3.6*299792458".to_string(),
                    calc: |x| x * 3.6 * 299792458.0,
                    can_use_si: false,
                },
                types::Conversion {
                    name: "Miles per hour".to_string(),
                    names: get_values("speed_mph".to_string()),
                    text: "x*2.237".to_string(),
                    calc: |x| x * 2.237,
                    can_use_si: false,
                },
                types::Conversion {
                    name: "Knot".to_string(),
                    names: get_values("speed_kt".to_string()),
                    text: "x*1.944".to_string(),
                    calc: |x| x * 1.944,
                    can_use_si: false,
                },
                types::Conversion {
                    name: "Light".to_string(),
                    names: get_values("speed_c".to_string()),
                    text: "x/299792458".to_string(),
                    calc: |x| x / 299792458.0,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            name: "Light".to_string(),
            names: get_values("speed_c".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    name: "Kilometres per hour".to_string(),
                    names: get_values("speed_kmh".to_string()),
                    text: "x*3.6*299792458".to_string(),
                    calc: |x| x * 3.6 * 299792458.0,
                    can_use_si: false,
                },
                types::Conversion {
                    name: "Miles per hour".to_string(),
                    names: get_values("speed_mph".to_string()),
                    text: "x*2.237*299792458".to_string(),
                    calc: |x| x * 2.237 * 299792458.0,
                    can_use_si: false,
                },
                types::Conversion {
                    name: "Knot".to_string(),
                    names: get_values("speed_kt".to_string()),
                    text: "x*1.944*299792458".to_string(),
                    calc: |x| x * 1.944 * 299792458.0,
                    can_use_si: false,
                },
                types::Conversion {
                    name: "Metres per second".to_string(),
                    names: get_values("speed_ms".to_string()),
                    text: "x*299792458".to_string(),
                    calc: |x| x * 299792458.0,
                    can_use_si: true,
                },
            ],
        },
    ];
}
