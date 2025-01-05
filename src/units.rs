use crate::types::{self};
use std::collections::HashMap;

pub mod angle;
pub mod area;
pub mod distance;
pub mod energy;
pub mod mass;
pub mod power;
pub mod pressure;
pub mod speed;
pub mod temperature;
pub mod time;
pub mod volume;

fn get_unit_names() -> HashMap<String, Vec<String>> {
    let t: HashMap<String, Vec<String>> = HashMap::from([
        (
            "arbitrary".to_string(),
            vec![
                String::from("Arbitrary units"),
                String::from("idk"),
                String::from("wtf"),
                String::from("???"),
                String::from("?"),
            ],
        ),
        (
            "temp_c".to_string(),
            vec![
                String::from("Celsius"),
                String::from("℃"),
                String::from("°C"),
                String::from("Celcius"),
                String::from("Centigrade"),
                String::from("C"),
            ],
        ),
        (
            "temp_f".to_string(),
            vec![
                String::from("Fahrenheit"),
                String::from("℉"),
                String::from("°F"),
                String::from("F"),
            ],
        ),
        (
            "temp_k".to_string(),
            vec![
                String::from("Kelvin"),
                String::from("°K"),
                String::from("K"),
            ],
        ),
        (
            "dist_in".to_string(),
            vec![
                String::from("Inch"),
                String::from("in"),
                String::from("'"),
                String::from("`"),
            ],
        ),
        (
            "dist_ft".to_string(),
            vec![
                String::from("Feet"),
                String::from("ft"),
                String::from("foot"),
                String::from("\""),
                String::from("''"),
                String::from("``"),
            ],
        ),
        (
            "dist_m".to_string(),
            vec![
                String::from("Metre"),
                String::from("m"),
                String::from("Meter"),
            ],
        ),
        (
            "dist_footfield".to_string(),
            vec![
                String::from("Football Field"),
                String::from("ff"),
                String::from("footballfield"),
            ],
        ),
        (
            "dist_mi".to_string(),
            vec![String::from("Mile"), String::from("mi")],
        ),
        (
            "dist_au".to_string(),
            vec![String::from("Astronomical Unit"), String::from("au")],
        ),
        (
            "dist_ly".to_string(),
            vec![String::from("Light Year"), String::from("ly")],
        ),
        (
            "dist_pc".to_string(),
            vec![String::from("Parsec"), String::from("pc")],
        ),
        (
            "time_s".to_string(),
            vec![
                String::from("Second"),
                String::from("s"),
                String::from("sec"),
            ],
        ),
        (
            "time_min".to_string(),
            vec![String::from("Minute"), String::from("min")],
        ),
        (
            "time_hr".to_string(),
            vec![String::from("Hour"), String::from("h"), String::from("hr")],
        ),
        (
            "time_d".to_string(),
            vec![String::from("Day"), String::from("d")],
        ),
        (
            "time_wk".to_string(),
            vec![
                String::from("Week"),
                String::from("wk"),
                String::from("sennight"),
            ],
        ),
        (
            "time_fn".to_string(),
            vec![String::from("Fortnight"), String::from("fn")],
        ),
        (
            "time_mth".to_string(),
            vec![
                String::from("Month"),
                String::from("mth"),
                String::from("mon"),
            ],
        ),
        (
            "time_qua".to_string(),
            vec![
                String::from("Quarantine"),
                String::from("quarantina"),
                String::from("quarentine"),
            ],
        ),
        (
            "time_yr".to_string(),
            vec![String::from("Year"), String::from("yr")],
        ),
        ("time_dec".to_string(), vec![String::from("Decade")]),
        (
            "time_cen".to_string(),
            vec![String::from("Century"), String::from("cent")],
        ),
        (
            "time_mil".to_string(),
            vec![String::from("Millennium"), String::from("Millennia")],
        ),
        ("time_ma".to_string(), vec![String::from("Megaannum")]),
        ("time_eon".to_string(), vec![String::from("Eon")]),
        (
            "vol_tsp".to_string(),
            vec![String::from("Teaspoon"), String::from("tsp")],
        ),
        (
            "vol_tbp".to_string(),
            vec![String::from("Tablespoon"), String::from("tbp")],
        ),
        (
            "vol_floz".to_string(),
            vec![String::from("Fluid Ounce"), String::from("floz")],
        ),
        (
            "vol_c".to_string(),
            vec![String::from("Cup"), String::from("c")],
        ),
        (
            "vol_pt".to_string(),
            vec![String::from("Pint"), String::from("pt")],
        ),
        (
            "vol_l".to_string(),
            vec![
                String::from("Litre"),
                String::from("Liter"),
                String::from("L"),
            ],
        ),
        (
            "vol_gal".to_string(),
            vec![String::from("Galloon"), String::from("gal")],
        ),
        (
            "vol_m3".to_string(),
            vec![
                String::from("Cubic Metre"),
                String::from("m³"),
                String::from("m3"),
                String::from("m^3"),
            ],
        ),
        (
            "mass_g".to_string(),
            vec![String::from("Gram"), String::from("g")],
        ),
        (
            "mass_oz".to_string(),
            vec![String::from("Ounce"), String::from("oz")],
        ),
        (
            "mass_lb".to_string(),
            vec![String::from("Pound"), String::from("lb")],
        ),
        (
            "mass_st".to_string(),
            vec![String::from("Stone"), String::from("st")],
        ),
        (
            "mass_t".to_string(),
            vec![
                String::from("US Ton"),
                String::from("t"),
                String::from("Ton"),
            ],
        ),
        (
            "mass_mt".to_string(),
            vec![
                String::from("Metric Tonne"),
                String::from("mt"),
                String::from("Tonne"),
            ],
        ),
        (
            "pres_pa".to_string(),
            vec![
                String::from("Pascal"),
                String::from("Pa"),
                String::from("N m² ⁻¹"),
                String::from("N/m^2"),
                String::from("N/m"),
                String::from("Nm"),
            ],
        ),
        (
            "pres_mmHg".to_string(),
            vec![
                String::from("millimetre of Mercury"),
                String::from("mmHg"),
                String::from("Torr"),
                String::from("millimeter of Mercury"),
            ],
        ),
        (
            "pres_psi".to_string(),
            vec![String::from("Pounds per square inch"), String::from("psi")],
        ),
        ("pres_bar".to_string(), vec![String::from("Bar")]),
        (
            "pres_atm".to_string(),
            vec![
                String::from("Standard Atmosphere"),
                String::from("atm"),
                String::from("Atmosphere"),
                String::from("std atm"),
            ],
        ),
        (
            "nrg_ev".to_string(),
            vec![
                String::from("Electron Volt"),
                String::from("eV"),
                String::from("Electronvolt"),
            ],
        ),
        (
            "nrg_j".to_string(),
            vec![String::from("Joule"), String::from("j")],
        ),
        (
            "nrg_cal".to_string(),
            vec![String::from("Calorie"), String::from("cal")],
        ),
        (
            "nrg_btu".to_string(),
            vec![String::from("British Thermal Unit"), String::from("btu")],
        ),
        (
            "nrg_wh".to_string(),
            vec![String::from("Watt Hour"), String::from("wH")],
        ),
        (
            "pow_w".to_string(),
            vec![String::from("Watt"), String::from("w")],
        ),
        (
            "pow_horse".to_string(),
            vec![String::from("Horse Power"), String::from("hp")],
        ),
        (
            "pow_erg".to_string(),
            vec![
                String::from("Ergs"),
                String::from("erg s⁻¹"),
                String::from("erg/s"),
            ],
        ),
        (
            "pow_ftlbsec".to_string(),
            vec![
                String::from("Foot-pounds per second"),
                String::from("ft lb s⁻¹"),
                String::from("ftlb/s"),
                String::from("ft lb s"),
                String::from("ftlbs"),
                String::from("ftlbsec"),
                String::from("ft lb sec"),
            ],
        ),
        (
            "pow_dbm".to_string(),
            vec![
                String::from("Decibel-milliwatts"),
                String::from("dBm"),
                String::from("dbmw"),
            ],
        ),
        (
            "pow_btusec".to_string(),
            vec![
                String::from("BTU per second"),
                String::from("btu s⁻¹"),
                String::from("btus"),
                String::from("btusec"),
            ],
        ),
        (
            "pow_calsec".to_string(),
            vec![
                String::from("Calories per second"),
                String::from("cal s⁻¹"),
                String::from("cals"),
                String::from("calsec"),
            ],
        ),
        (
            "area_in2".to_string(),
            vec![
                String::from("Square inch"),
                String::from("in²"),
                String::from("in2"),
                String::from("sqin"),
            ],
        ),
        (
            "area_ft2".to_string(),
            vec![
                String::from("Square foot"),
                String::from("ft²"),
                String::from("ft2"),
                String::from("sqft"),
            ],
        ),
        (
            "area_m2".to_string(),
            vec![
                String::from("Square metre"),
                String::from("m²"),
                String::from("m2"),
                String::from("sqm"),
            ],
        ),
        (
            "area_ac".to_string(),
            vec![String::from("Acre"), String::from("ac")],
        ),
        (
            "area_ha".to_string(),
            vec![String::from("Hectare"), String::from("Ha")],
        ),
        (
            "area_km2".to_string(),
            vec![
                String::from("Square kilometre"),
                String::from("km²"),
                String::from("km2"),
                String::from("sqkm"),
            ],
        ),
        (
            "area_mi2".to_string(),
            vec![
                String::from("Square mile"),
                String::from("mi²"),
                String::from("mi2"),
                String::from("sqmi"),
            ],
        ),
        (
            "angle_grad".to_string(),
            vec![String::from("Gradian"), String::from("grad")],
        ),
        (
            "angle_deg".to_string(),
            vec![String::from("Degree"), String::from("deg")],
        ),
        (
            "angle_rad".to_string(),
            vec![String::from("Radian"), String::from("rad")],
        ),
        (
            "speed_kmh".to_string(),
            vec![
                String::from("Kilometres per hour"),
                String::from("km h⁻¹"),
                String::from("kmh"),
                String::from("kph"),
                String::from("km/h"),
            ],
        ),
        (
            "speed_mph".to_string(),
            vec![
                String::from("Miles per hour"),
                String::from("mi h⁻¹"),
                String::from("mph"),
                String::from("mih"),
                String::from("mi/h"),
                String::from("m/h"),
            ],
        ),
        (
            "speed_kt".to_string(),
            vec![
                String::from("Knot"),
                String::from("kt"),
                String::from("nmi h⁻¹"),
                String::from("nmih"),
                String::from("nmh"),
                String::from("Nautical miles per hour"),
            ],
        ),
        (
            "speed_ms".to_string(),
            vec![
                String::from("Metres per second"),
                String::from("m s⁻¹"),
                String::from("ms"),
                String::from("mps"),
                String::from("m/s"),
            ],
        ),
        (
            "speed_c".to_string(),
            vec![
                String::from("Light"),
                String::from("c"),
                String::from("lightspeed"),
            ],
        ),
    ]);
    return t;
}

fn get_values(key: String) -> Vec<String> {
    if let Some(values) = get_unit_names().get(&key) {
        return values.to_vec();
    } else {
        return Vec::new();
    }
}

pub fn get_unit(key: String, second_key: String) -> (types::Unit, types::Conversion) {
    let unit_names: Vec<Vec<String>> = get_unit_names().values().cloned().collect::<Vec<_>>();
    let mut unit_list: Vec<types::Unit> = vec![types::template_unit()];
    for names in unit_names {
        if names.contains(&key) {
            let x = &names[0];
            unit_list = get_unit_via_list(&key);
        }
    }
    for unit in unit_list {
        let test = get_calculation(&second_key, &unit);
        if test.name != "TEMPLATE".to_string() {
            return (unit, test);
        }
    }

    return (types::template_unit(), types::template_conversion());
}

fn get_unit_second(key: &String, units: Vec<types::Unit>) -> types::Unit {
    for unit in units {
        if unit.names.contains(&key) {
            return unit;
        }
    }
    return types::template_unit();
}

fn get_unit_via_list(key: &String) -> Vec<types::Unit> {
    let mut foo: Vec<types::Unit> = Vec::new();
    let bar = "TEMPLATE".to_string();
    let angle = get_unit_second(key, angle::units());
    let area = get_unit_second(key, area::units());
    let distance = get_unit_second(key, distance::units());
    let energy = get_unit_second(key, energy::units());
    let mass = get_unit_second(key, mass::units());
    let power = get_unit_second(key, power::units());
    let pressure = get_unit_second(key, pressure::units());
    let speed = get_unit_second(key, speed::units());
    let temperature = get_unit_second(key, temperature::units());
    let time = get_unit_second(key, time::units());
    let volume = get_unit_second(key, volume::units());

    if angle.name != bar {
        foo.push(angle);
    }
    if area.name != bar {
        foo.push(area);
    }
    if distance.name != bar {
        foo.push(distance);
    }
    if energy.name != bar {
        foo.push(energy);
    }
    if mass.name != bar {
        foo.push(mass);
    }
    if power.name != bar {
        foo.push(power);
    }
    if pressure.name != bar {
        foo.push(pressure);
    }
    if speed.name != bar {
        foo.push(speed);
    }
    if temperature.name != bar {
        foo.push(temperature);
    }
    if time.name != bar {
        foo.push(time);
    }
    if volume.name != bar {
        foo.push(volume);
    }
    return foo;
}

fn get_calculation(key: &String, unit: &types::Unit) -> types::Conversion {
    let mut bar = &unit.conversions;
    for conversion in bar {
        if conversion.names.contains(&key) {
            return conversion.clone();
        }
    }
    return types::template_conversion();
}
