use super::get_values;
use super::types;

pub fn units() -> Vec<types::Unit> {
    return vec![
        types::Unit {
            names: get_values("vol_tsp".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("vol_tbp".to_string()),
                    text: "x/3".to_string(),
                    calc: |x| x / 3.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_floz".to_string()),
                    text: "x/6".to_string(),
                    calc: |x| x / 6.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_c".to_string()),
                    text: "x/48.692".to_string(),
                    calc: |x| x / 48.692,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_pt".to_string()),
                    text: "x/96".to_string(),
                    calc: |x| x / 96.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_l".to_string()),
                    text: "x/202.884".to_string(),
                    calc: |x| x / 202.884,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("vol_gal".to_string()),
                    text: "x/768".to_string(),
                    calc: |x| x / 768.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_m3".to_string()),
                    text: "x/202884.2".to_string(),
                    calc: |x| x / 202884.2,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("vol_tbp".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("vol_tsp".to_string()),
                    text: "x*3".to_string(),
                    calc: |x| x * 3.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_floz".to_string()),
                    text: "x/2".to_string(),
                    calc: |x| x / 2.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_c".to_string()),
                    text: "x/16.231".to_string(),
                    calc: |x| x / 16.231,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_pt".to_string()),
                    text: "x/32".to_string(),
                    calc: |x| x / 32.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_l".to_string()),
                    text: "x/67.628".to_string(),
                    calc: |x| x / 67.628,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("vol_gal".to_string()),
                    text: "x/256".to_string(),
                    calc: |x| x / 256.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_m3".to_string()),
                    text: "x/67628.05".to_string(),
                    calc: |x| x / 67628.05,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("vol_floz".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("vol_tsp".to_string()),
                    text: "x*6".to_string(),
                    calc: |x| x * 6.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_tbp".to_string()),
                    text: "x*2".to_string(),
                    calc: |x| x * 2.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_c".to_string()),
                    text: "x/8.115".to_string(),
                    calc: |x| x / 8.115,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_pt".to_string()),
                    text: "x/33.814".to_string(),
                    calc: |x| x / 33.814,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_l".to_string()),
                    text: "x/16".to_string(),
                    calc: |x| x / 16.0,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("vol_gal".to_string()),
                    text: "x/128".to_string(),
                    calc: |x| x / 128.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_m3".to_string()),
                    text: "x/33814".to_string(),
                    calc: |x| x / 33814.0,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("vol_c".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("vol_tsp".to_string()),
                    text: "x*48.692".to_string(),
                    calc: |x| x * 48.692,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_tbp".to_string()),
                    text: "x*16.231".to_string(),
                    calc: |x| x * 16.231,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_floz".to_string()),
                    text: "x*8.115".to_string(),
                    calc: |x| x * 8.115,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_pt".to_string()),
                    text: "x/1.972".to_string(),
                    calc: |x| x / 1.972,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_l".to_string()),
                    text: "x/4.167".to_string(),
                    calc: |x| x / 4.167,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("vol_gal".to_string()),
                    text: "x/15.772".to_string(),
                    calc: |x| x / 15.772,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_m3".to_string()),
                    text: "x/4167".to_string(),
                    calc: |x| x / 4167.0,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("vol_pt".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("vol_tsp".to_string()),
                    text: "x*96".to_string(),
                    calc: |x| x * 96.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_tbp".to_string()),
                    text: "x*32".to_string(),
                    calc: |x| x * 32.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_floz".to_string()),
                    text: "x*16".to_string(),
                    calc: |x| x * 16.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_c".to_string()),
                    text: "x*1.972".to_string(),
                    calc: |x| x * 1.972,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_l".to_string()),
                    text: "x/2.113".to_string(),
                    calc: |x| x / 2.113,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("vol_gal".to_string()),
                    text: "x/8".to_string(),
                    calc: |x| x / 8.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_m3".to_string()),
                    text: "x/33814".to_string(),
                    calc: |x| x / 33814.0,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("vol_l".to_string()),
            can_use_si: true,
            conversions: vec![
                types::Conversion {
                    names: get_values("vol_tsp".to_string()),
                    text: "x*202.884".to_string(),
                    calc: |x| x * 202.884,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_tbp".to_string()),
                    text: "x*67.628".to_string(),
                    calc: |x| x * 67.628,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_floz".to_string()),
                    text: "x*33.814".to_string(),
                    calc: |x| x * 33.814,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_c".to_string()),
                    text: "x*33.814/8.115".to_string(),
                    calc: |x| x * 33.814 / 8.115,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_pt".to_string()),
                    text: "x*33.814/16".to_string(),
                    calc: |x| x * 33.814 / 16.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_gal".to_string()),
                    text: "x*33.814/128".to_string(),
                    calc: |x| x * 33.814 / 128.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_m3".to_string()),
                    text: "x/1e3".to_string(),
                    calc: |x| x / 1e3,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("vol_gal".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("vol_tsp".to_string()),
                    text: "x*768".to_string(),
                    calc: |x| x * 768.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_tbp".to_string()),
                    text: "x*256".to_string(),
                    calc: |x| x * 256.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_floz".to_string()),
                    text: "x*128".to_string(),
                    calc: |x| x * 128.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_c".to_string()),
                    text: "x*15.7725".to_string(),
                    calc: |x| x * 15.7725,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_pt".to_string()),
                    text: "x*8".to_string(),
                    calc: |x| x * 8.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_l".to_string()),
                    text: "x*3.785".to_string(),
                    calc: |x| x * 3.785,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("vol_m3".to_string()),
                    text: "x/264.2".to_string(),
                    calc: |x| x / 264.2,
                    can_use_si: false,
                },
            ],
        },
        types::Unit {
            names: get_values("vol_m3".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    names: get_values("vol_tsp".to_string()),
                    text: "x*202884".to_string(),
                    calc: |x| x * 202884.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_tbp".to_string()),
                    text: "x*67628".to_string(),
                    calc: |x| x * 67628.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_floz".to_string()),
                    text: "x*33814".to_string(),
                    calc: |x| x * 33814.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_c".to_string()),
                    text: "x*33814/8.115".to_string(),
                    calc: |x| x * 33814.0 / 8.115,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_pt".to_string()),
                    text: "x*33814/16".to_string(),
                    calc: |x| x * 33814.0 / 16.0,
                    can_use_si: false,
                },
                types::Conversion {
                    names: get_values("vol_l".to_string()),
                    text: "x*1000".to_string(),
                    calc: |x| x * 1000.0,
                    can_use_si: true,
                },
                types::Conversion {
                    names: get_values("vol_gal".to_string()),
                    text: "x*33814/128".to_string(),
                    calc: |x| x * 33814.0 / 128.0,
                    can_use_si: false,
                },
            ],
        },
    ];
}
