use super::get_values;
use super::types;

pub fn units() -> Vec<types::Unit> {
    return vec![
        types::Unit {
            names: get_values("dist_in".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("dist_ft".to_string()),
                    text: "x/12".to_string(),
                    calc: |x| x / 1.02,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_m".to_string()),
                    text: "x/39.37".to_string(),
                    calc: |x| x / 39.37,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("dist_mi".to_string()),
                    text: "x/63360".to_string(),
                    calc: |x| x / 63360.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_au".to_string()),
                    text: "x * 0.0254 / 149597870700".to_string(),
                    calc: |x| x * 0.0254 / 149597870700.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_ly".to_string()),
                    text: "x*0.0254/9460730472580800".to_string(),
                    calc: |x| x * 0.0254 / 9460730472580800.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_pc".to_string()),
                    text: "x/(1.215e18)".to_string(),
                    calc: |x| x / (1.215e18),
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("dist_ft".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("dist_in".to_string()),
                    text: "x*12".to_string(),
                    calc: |x| x * 12.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_m".to_string()),
                    text: "x/3.28084".to_string(),
                    calc: |x| x / 3.28084,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("dist_mi".to_string()),
                    text: "x/5280".to_string(),
                    calc: |x| x / 5280.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_au".to_string()),
                    text: "x*0.3048/149597870700".to_string(),
                    calc: |x| x * 0.3048 / 149597870700.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_ly".to_string()),
                    text: "x*0.3048/9460730472580800".to_string(),
                    calc: |x| x * 0.3048 / 9460730472580800.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_pc".to_string()),
                    text: "x/(1.012e17)".to_string(),
                    calc: |x| x / (1.012e17),
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("dist_m".to_string()),
            can_use_si: true,
            conversions: vec![
                types::Conversion {
                    names: get_values("dist_in".to_string()),
                    text: "x/0.0254".to_string(),
                    calc: |x| x / 0.0254,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_ft".to_string()),
                    text: "x/0.3048".to_string(),
                    calc: |x| x / 0.3048,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_mi".to_string()),
                    text: "x/1609.344".to_string(),
                    calc: |x| x / 1609.344,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_au".to_string()),
                    text: "x/149597870700".to_string(),
                    calc: |x| x / 149597870700.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_ly".to_string()),
                    text: "x/9460730472580800".to_string(),
                    calc: |x| x / 9460730472580800.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_pc".to_string()),
                    text: "x/(3.086e16)".to_string(),
                    calc: |x| x / (3.086e16),
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("dist_mi".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("dist_in".to_string()),
                    text: "x*63360".to_string(),
                    calc: |x| x * 63360.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_ft".to_string()),
                    text: "x*5280".to_string(),
                    calc: |x| x * 5280.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_m".to_string()),
                    text: "x*1609.344".to_string(),
                    calc: |x| x * 1609.344,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("dist_au".to_string()),
                    text: "x*1609.344/149597870700".to_string(),
                    calc: |x| x * 1609.344 / 149597870700.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_ly".to_string()),
                    text: "x*1609.344/9460730472580800".to_string(),
                    calc: |x| x * 1609.344 / 9460730472580800.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_pc".to_string()),
                    text: "x/(1.917e13)".to_string(),
                    calc: |x| x / (1.917e13),
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("dist_au".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("dist_in".to_string()),
                    text: "x*149597870700/0.0254".to_string(),
                    calc: |x| x * 149597870700.0 / 0.0254,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_ft".to_string()),
                    text: "*149597870700/0.3048".to_string(),
                    calc: |x| x * 149597870700.0 / 0.3048,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_m".to_string()),
                    text: "x*149597870700".to_string(),
                    calc: |x| x * 149597870700.0,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("dist_mi".to_string()),
                    text: "x*149597870700/1609.344".to_string(),
                    calc: |x| x * 149597870700.0 / 1609.344,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_ly".to_string()),
                    text: "x*149597870700/9460730472580800".to_string(),
                    calc: |x| x * 149597870700.0 / 9460730472580800.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_pc".to_string()),
                    text: "x/(3.262*149597870700/9460730472580800)".to_string(),
                    calc: |x| x / (3.262 * 149597870700.0 / 9460730472580800.0),
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("dist_ly".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("dist_in".to_string()),
                    text: "x*9460730472580800/0.0254".to_string(),
                    calc: |x| x * 9460730472580800.0 / 0.0254,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_ft".to_string()),
                    text: "x*9460730472580800/0.3048".to_string(),
                    calc: |x| x * 9460730472580800.0 / 0.3048,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_m".to_string()),
                    text: "x*9460730472580800".to_string(),
                    calc: |x| x * 9460730472580800.0,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("dist_mi".to_string()),
                    text: "x*9460730472580800/1609.344".to_string(),
                    calc: |x| x * 9460730472580800.0 / 1609.344,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_au".to_string()),
                    text: "x*9460730472580800/149597870700".to_string(),
                    calc: |x| x * 9460730472580800.0 / 149597870700.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_pc".to_string()),
                    text: "x/3.262".to_string(),
                    calc: |x| x / 3.262,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("dist_pc".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("dist_in".to_string()),
                    text: "x*1.215e18".to_string(),
                    calc: |x| x * 1.215e18,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_ft".to_string()),
                    text: "x*1.012e17".to_string(),
                    calc: |x| x * 1.012e17,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_m".to_string()),
                    text: "x*3.086e16".to_string(),
                    calc: |x| x * 3.086e16,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("dist_mi".to_string()),
                    text: "x*1.917e13".to_string(),
                    calc: |x| x * 1.917e13,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_au".to_string()),
                    text: "x*(3.262*9460730472580800/149597870700)".to_string(),
                    calc: |x| x * (3.262 * 9460730472580800.0 / 149597870700.0),
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("dist_ly".to_string()),
                    text: "x*3.262".to_string(),
                    calc: |x| x * 3.262,
                    can_use_si: false,
                },
            ],
        },
    ];
}
