use super::get_values;
use super::types;

pub fn units() -> Vec<types::Unit> {
    return vec![
        types::Unit {
            names: get_values("area_in2".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("area_ft2".to_string()),
                    text: "x/144".to_string(),
                    calc: |x| x / 144.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_m2".to_string()),
                    text: "x/(39.37^2)".to_string(),
                    calc: |x| x / (f32::powf(39.37, 2.0)),
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_ac".to_string()),
                    text: "x/6.27264e6".to_string(),
                    calc: |x| x / 6.27264e6,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_ha".to_string()),
                    text: "x / (39.37^2.0*1e4))".to_string(),
                    calc: |x| x / (f32::powf(39.37, 2.0) * 1e4),
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_km2".to_string()),
                    text: "x / (39.37^2.0*1e6))".to_string(),
                    calc: |x| x / (f32::powf(39.37, 2.0) * 1e6),
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_mi2".to_string()),
                    text: "x/(63360^2)".to_string(),
                    calc: |x| x / f32::powf(63360.0, 2.0),
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("area_ft2".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("area_in2".to_string()),
                    text: "x*144".to_string(),
                    calc: |x| x * 144.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_m2".to_string()),
                    text: "3.28084^2".to_string(),
                    calc: |x| x / f32::powf(3.28084, 2.0),
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_ac".to_string()),
                    text: "x/43560".to_string(),
                    calc: |x| x / 43560.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_ha".to_string()),
                    text: "x/3.28084^2*1e4".to_string(),
                    calc: |x| x / f32::powf(3.28084, 2.0) * 1e4,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_km2".to_string()),
                    text: "x/3.28084^2*1e6".to_string(),
                    calc: |x| x / f32::powf(3.28084, 2.0) * 1e6,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_mi2".to_string()),
                    text: "x/(2.788e7)".to_string(),
                    calc: |x| x / (2.788e7),
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("area_m2".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("area_in2".to_string()),
                    text: "x*39.37^2".to_string(),
                    calc: |x| x * f32::powf(39.37, 2.0),
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_ft2".to_string()),
                    text: "x*3.28084^2".to_string(),
                    calc: |x| x * f32::powf(3.28084, 2.0),
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_ac".to_string()),
                    text: "x/4047".to_string(),
                    calc: |x| x / 4047.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_ha".to_string()),
                    text: "x/1e4".to_string(),
                    calc: |x| x / 1e4,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_km2".to_string()),
                    text: "x/1e6".to_string(),
                    calc: |x| x / 1e6,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_mi2".to_string()),
                    text: "x/1609.344^2".to_string(),
                    calc: |x| x / f32::powf(1609.344, 2.0),
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("area_ac".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("area_in2".to_string()),
                    text: "x*6.27264e6".to_string(),
                    calc: |x| x * 6.27264e6,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_ft2".to_string()),
                    text: "x*43560".to_string(),
                    calc: |x| x * 43560.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_m2".to_string()),
                    text: "x*4046.86".to_string(),
                    calc: |x| x * 4046.86,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_ha".to_string()),
                    text: "x/2.471".to_string(),
                    calc: |x| x / 2.471,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_km2".to_string()),
                    text: "x/247.1".to_string(),
                    calc: |x| x / 247.1,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_mi2".to_string()),
                    text: "x/640".to_string(),
                    calc: |x| x / 640.0,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("area_ha".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("area_in2".to_string()),
                    text: "x*1.55e7".to_string(),
                    calc: |x| x * 1.55e7,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_ft2".to_string()),
                    text: "x*107639".to_string(),
                    calc: |x| x * 107639.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_m2".to_string()),
                    text: "x*10000".to_string(),
                    calc: |x| x * 10000.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_ac".to_string()),
                    text: "x*2.471".to_string(),
                    calc: |x| x * 2.471,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_km2".to_string()),
                    text: "x/100".to_string(),
                    calc: |x| x / 100.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_mi2".to_string()),
                    text: "x/258.999".to_string(),
                    calc: |x| x / 258.999,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("area_km2".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("area_in2".to_string()),
                    text: "x*39.37^2*1e6".to_string(),
                    calc: |x| x * f32::powf(39.37, 2.0) * 1e6,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_ft2".to_string()),
                    text: "x*3.28084^2*1e6".to_string(),
                    calc: |x| x * f32::powf(3.28084, 2.0) * 1e6,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_m2".to_string()),
                    text: "x*1e6".to_string(),
                    calc: |x| x * 1e6,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_ac".to_string()),
                    text: "x/4047*1e6".to_string(),
                    calc: |x| x / 4047.0 * 1e6,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_ha".to_string()),
                    text: "x*1e2".to_string(),
                    calc: |x| x * 1e2,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_mi2".to_string()),
                    text: "(x/5280^2)*1e6".to_string(),
                    calc: |x| (x / f32::powf(5280.0, 2.0)) * 1e6,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("area_mi2".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("area_in2".to_string()),
                    text: "x*63360^2".to_string(),
                    calc: |x| x * f32::powf(63360.0, 2.0),
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_ft2".to_string()),
                    text: "x*5280^2".to_string(),
                    calc: |x| x * f32::powf(5280.0, 2.0),
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_m2".to_string()),
                    text: "x*1609.344^2".to_string(),
                    calc: |x| x * f32::powf(1609.344, 2.0),
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_ac".to_string()),
                    text: "x*640".to_string(),
                    calc: |x| x * 640.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_ha".to_string()),
                    text: "x*258.999".to_string(),
                    calc: |x| x * 258.999,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("area_km2".to_string()),
                    text: "x*5280^2/1e6".to_string(),
                    calc: |x| x * f32::powf(5280.0, 2.0) / 1e6,
                    can_use_si: false,
                },
            ],
        },
    ];
}
