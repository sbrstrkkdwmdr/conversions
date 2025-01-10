use super::get_values;
use super::types;

pub fn units() -> Vec<types::Unit> {
    return vec![
        types::Unit {
            
            names: get_values("nrg_ev".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    
                    names: get_values("nrg_j".to_string()),
                    text: "x/6.242e18".to_string(),
                    calc: |x| x / 6.242e18,
                    can_use_si: true,
                },
                types::Conversion {
                    
                    names: get_values("nrg_cal".to_string()),
                    text: "x/2.611e19".to_string(),
                    calc: |x| x / 2.611e19,
                    can_use_si: true,
                },
                types::Conversion {
                    
                    names: get_values("nrg_btu".to_string()),
                    text: "x/6.585e21".to_string(),
                    calc: |x| x / 6.585e21,
                    can_use_si: false,
                },
                types::Conversion {
                    
                    names: get_values("nrg_wh".to_string()),
                    text: "x/2.247e22".to_string(),
                    calc: |x| x / 2.247e22,
                    can_use_si: true,
                },
            ],
        },
        types::Unit {
            
            names: get_values("nrg_j".to_string()),
            can_use_si: true,
            conversions: vec![
                types::Conversion {
                    
                    names: get_values("nrg_ev".to_string()),
                    text: "x*6.242e18".to_string(),
                    calc: |x| x * 6.242e18,
                    can_use_si: false,
                },
                types::Conversion {
                    
                    names: get_values("nrg_cal".to_string()),
                    text: "x/4.184".to_string(),
                    calc: |x| x / 4.184,
                    can_use_si: true,
                },
                types::Conversion {
                    
                    names: get_values("nrg_btu".to_string()),
                    text: "x/1055.06".to_string(),
                    calc: |x| x / 1055.06,
                    can_use_si: false,
                },
                types::Conversion {
                    
                    names: get_values("nrg_wh".to_string()),
                    text: "x/3600".to_string(),
                    calc: |x| x / 3600.0,
                    can_use_si: true,
                },
            ],
        },
        types::Unit {
            
            names: get_values("nrg_cal".to_string()),
            can_use_si: true,
            conversions: vec![
                types::Conversion {
                    
                    names: get_values("nrg_ev".to_string()),
                    text: "x/2.611e19".to_string(),
                    calc: |x| x / 2.611e19,
                    can_use_si: false,
                },
                types::Conversion {
                    
                    names: get_values("nrg_j".to_string()),
                    text: "x*4.184".to_string(),
                    calc: |x| x * 4.184,
                    can_use_si: true,
                },
                types::Conversion {
                    
                    names: get_values("nrg_btu".to_string()),
                    text: "x/252.164".to_string(),
                    calc: |x| x / 252.164,
                    can_use_si: false,
                },
                types::Conversion {
                    
                    names: get_values("nrg_wh".to_string()),
                    text: "x/860.421".to_string(),
                    calc: |x| x / 860.421,
                    can_use_si: true,
                },
            ],
        },
        types::Unit {
            
            names: get_values("nrg_btu".to_string()),
            can_use_si: false,
            conversions: vec![
                types::Conversion {
                    
                    names: get_values("nrg_ev".to_string()),
                    text: "x*6.585e21".to_string(),
                    calc: |x| x * 6.585e21,
                    can_use_si: false,
                },
                types::Conversion {
                    
                    names: get_values("nrg_j".to_string()),
                    text: "x*1055.06".to_string(),
                    calc: |x| x * 1055.06,
                    can_use_si: true,
                },
                types::Conversion {
                    
                    names: get_values("nrg_cal".to_string()),
                    text: "x*252.164".to_string(),
                    calc: |x| x * 252.164,
                    can_use_si: true,
                },
                types::Conversion {
                    
                    names: get_values("nrg_wh".to_string()),
                    text: "x/3.41214".to_string(),
                    calc: |x| x / 3.41214,
                    can_use_si: true,
                },
            ],
        },
        types::Unit {
            
            names: get_values("nrg_wh".to_string()),
            can_use_si: true,
            conversions: vec![
                types::Conversion {
                    
                    names: get_values("nrg_ev".to_string()),
                    text: "x*2.247e22".to_string(),
                    calc: |x| x * 2.247e22,
                    can_use_si: false,
                },
                types::Conversion {
                    
                    names: get_values("nrg_j".to_string()),
                    text: "x*3600".to_string(),
                    calc: |x| x * 3600.0,
                    can_use_si: true,
                },
                types::Conversion {
                    
                    names: get_values("nrg_cal".to_string()),
                    text: "x*860.421".to_string(),
                    calc: |x| x * 860.421,
                    can_use_si: true,
                },
                types::Conversion {
                    
                    names: get_values("nrg_btu".to_string()),
                    text: "x*3.41214".to_string(),
                    calc: |x| x * 3.41214,
                    can_use_si: false,
                },
            ],
        },
    ];
}
