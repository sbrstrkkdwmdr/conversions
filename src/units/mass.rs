use super::get_values;
use super::types;

pub fn units() -> Vec<types::Unit> {
    return vec![
        types::Unit {
            names: get_values("mass_g".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("mass_oz".to_string()),
                    text: "x/28.35".to_string(),
                    calc: |x| x / 28.35,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("mass_lb".to_string()),
                    text: "x/453.592".to_string(),
                    calc: |x| x / 453.592,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("mass_st".to_string()),
                    text: "x/6350.29".to_string(),
                    calc: |x| x / 6350.29,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("mass_t".to_string()),
                    text: "x/907185".to_string(),
                    calc: |x| x / 907185.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("mass_mt".to_string()),
                    text: "x/1e6".to_string(),
                    calc: |x| x / 1e6,
                    can_use_si: true,
                },
            ],
        },
        types::Unit {
            names: get_values("mass_oz".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("mass_g".to_string()),
                    text: "x*28.35".to_string(),
                    calc: |x| x * 28.35,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("mass_lb".to_string()),
                    text: "x/16".to_string(),
                    calc: |x| x / 16.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("mass_st".to_string()),
                    text: "x/224".to_string(),
                    calc: |x| x / 224.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("mass_t".to_string()),
                    text: "x/32000".to_string(),
                    calc: |x| x / 32000.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("mass_mt".to_string()),
                    text: "x/35274".to_string(),
                    calc: |x| x / 35274.0,
                    can_use_si: true,
                },
            ],
        },
        types::Unit {
            names: get_values("mass_lb".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("mass_g".to_string()),
                    text: "x*453.592".to_string(),
                    calc: |x| x * 453.592,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("mass_oz".to_string()),
                    text: "x*16".to_string(),
                    calc: |x| x * 16.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("mass_st".to_string()),
                    text: "x/14".to_string(),
                    calc: |x| x / 14.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("mass_t".to_string()),
                    text: "x/2000".to_string(),
                    calc: |x| x / 2000.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("mass_mt".to_string()),
                    text: "x/2205".to_string(),
                    calc: |x| x / 2205.0,
                    can_use_si: true,
                },
            ],
        },
        types::Unit {
            names: get_values("mass_st".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("mass_g".to_string()),
                    text: "x*6350.29".to_string(),
                    calc: |x| x * 6350.29,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("mass_oz".to_string()),
                    text: "x*224".to_string(),
                    calc: |x| x * 224.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("mass_lb".to_string()),
                    text: "x*14".to_string(),
                    calc: |x| x * 14.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("mass_t".to_string()),
                    text: "x*0.007".to_string(),
                    calc: |x| x * 0.007,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("mass_mt".to_string()),
                    text: "x/157.473".to_string(),
                    calc: |x| x / 157.473,
                    can_use_si: true,
                },
            ],
        },
        types::Unit {
            names: get_values("mass_t".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("mass_g".to_string()),
                    text: "x*907185".to_string(),
                    calc: |x| x * 907185.0,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("mass_oz".to_string()),
                    text: "x*32000".to_string(),
                    calc: |x| x * 32000.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("mass_lb".to_string()),
                    text: "x*2000".to_string(),
                    calc: |x| x * 2000.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("mass_st".to_string()),
                    text: "x/0.007".to_string(),
                    calc: |x| x / 0.007,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("mass_mt".to_string()),
                    text: "x/1.102".to_string(),
                    calc: |x| x / 1.102,
                    can_use_si: true,
                },
            ],
        },
        types::Unit {
            names: get_values("mass_mt".to_string()),
            can_use_si: true,
            conversions: vec![
                types::Conversion {
                    names: get_values("mass_g".to_string()),
                    text: "x*1e6".to_string(),
                    calc: |x| x * 1e6,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("mass_oz".to_string()),
                    text: "x*35274".to_string(),
                    calc: |x| x * 35274.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("mass_lb".to_string()),
                    text: "x*2204.62".to_string(),
                    calc: |x| x * 2204.62,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("mass_st".to_string()),
                    text: "x*157.473".to_string(),
                    calc: |x| x * 157.473,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("mass_t".to_string()),
                    text: "x*1.102".to_string(),
                    calc: |x| x * 1.102,
                    can_use_si: false,
                },
            ],
        },
    ];
}
