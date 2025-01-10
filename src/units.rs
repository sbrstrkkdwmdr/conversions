use crate::types::{self};
// use std::collections::HashMap;
use std::collections::BTreeMap;

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

pub fn get_unit_names() -> BTreeMap<String, Vec<String>> {
    let t: BTreeMap<String, Vec<String>> = BTreeMap::from([
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
                String::from("Inches"),
                String::from("inch"),
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
                String::from("Metres"),
                String::from("m"),
                String::from("metre"),
                String::from("meter"),
                String::from("meters"),
            ],
        ),
        (
            "dist_mi".to_string(),
            vec![
                String::from("Miles"),
                String::from("mi"),
                String::from("mile"),
            ],
        ),
        (
            "dist_au".to_string(),
            vec![
                String::from("Astronomical Units"),
                String::from("au"),
                String::from("astronomical unit"),
            ],
        ),
        (
            "dist_ly".to_string(),
            vec![
                String::from("Light Years"),
                String::from("ly"),
                String::from("Light Year"),
            ],
        ),
        (
            "dist_pc".to_string(),
            vec![
                String::from("Parsecs"),
                String::from("pc"),
                String::from("Parsec"),
            ],
        ),
        (
            "time_s".to_string(),
            vec![
                String::from("Seconds"),
                String::from("s"),
                String::from("Second"),
                String::from("sec"),
            ],
        ),
        (
            "time_min".to_string(),
            vec![
                String::from("Minutes"),
                String::from("min"),
                String::from("Minute"),
            ],
        ),
        (
            "time_hr".to_string(),
            vec![
                String::from("Hours"),
                String::from("h"),
                String::from("Hour"),
                String::from("hr"),
            ],
        ),
        (
            "time_d".to_string(),
            vec![String::from("Day"), String::from("Days"), String::from("d")],
        ),
        (
            "time_wk".to_string(),
            vec![
                String::from("Week"),
                String::from("Weeks"),
                String::from("wk"),
                String::from("sennight"),
            ],
        ),
        (
            "time_fn".to_string(),
            vec![
                String::from("Fortnight"),
                String::from("Fortnights"),
                String::from("fn"),
            ],
        ),
        (
            "time_mth".to_string(),
            vec![
                String::from("Month"),
                String::from("Months"),
                String::from("mth"),
                String::from("mon"),
            ],
        ),
        (
            "time_qua".to_string(),
            vec![
                String::from("Quarantine"),
                String::from("Quarantines"),
                String::from("quarantina"),
                String::from("quarentine"),
            ],
        ),
        (
            "time_yr".to_string(),
            vec![
                String::from("Year"),
                String::from("Years"),
                String::from("yr"),
            ],
        ),
        (
            "time_dec".to_string(),
            vec![
                String::from("Decades"),
                String::from("Decade"),
                String::from("dec"),
            ],
        ),
        (
            "time_cen".to_string(),
            vec![
                String::from("Centuries"),
                String::from("Century"),
                String::from("cent"),
            ],
        ),
        (
            "time_mil".to_string(),
            vec![String::from("Millennia"), String::from("Millennium")],
        ),
        ("time_ma".to_string(), vec![String::from("Megaannum")]),
        (
            "time_eon".to_string(),
            vec![String::from("Eons"), String::from("Eon")],
        ),
        (
            "vol_tsp".to_string(),
            vec![
                String::from("Teaspoons"),
                String::from("Teaspoon"),
                String::from("tsp"),
            ],
        ),
        (
            "vol_tbp".to_string(),
            vec![
                String::from("Tablespoons"),
                String::from("Tablespoon"),
                String::from("tbp"),
            ],
        ),
        (
            "vol_floz".to_string(),
            vec![
                String::from("Fluid Ounces"),
                String::from("Fluid Ounce"),
                String::from("floz"),
            ],
        ),
        (
            "vol_c".to_string(),
            vec![String::from("Cups"), String::from("Cup"), String::from("c")],
        ),
        (
            "vol_pt".to_string(),
            vec![
                String::from("Pints"),
                String::from("Pint"),
                String::from("pt"),
            ],
        ),
        (
            "vol_l".to_string(),
            vec![
                String::from("Litres"),
                String::from("Liters"),
                String::from("Litre"),
                String::from("Liter"),
                String::from("L"),
            ],
        ),
        (
            "vol_gal".to_string(),
            vec![
                String::from("Gallons"),
                String::from("Gallon"),
                String::from("gal"),
            ],
        ),
        (
            "vol_m3".to_string(),
            vec![
                String::from("Cubic Metres"),
                String::from("Cubic Metre"),
                String::from("m³"),
                String::from("m3"),
                String::from("m^3"),
            ],
        ),
        (
            "mass_g".to_string(),
            vec![
                String::from("Grams"),
                String::from("Gram"),
                String::from("g"),
            ],
        ),
        (
            "mass_oz".to_string(),
            vec![
                String::from("Ounces"),
                String::from("Ounce"),
                String::from("oz"),
            ],
        ),
        (
            "mass_lb".to_string(),
            vec![
                String::from("Pounds"),
                String::from("Pound"),
                String::from("lb"),
            ],
        ),
        (
            "mass_st".to_string(),
            vec![
                String::from("Stone"),
                String::from("Stones"),
                String::from("st"),
            ],
        ),
        (
            "mass_t".to_string(),
            vec![
                String::from("US Tons"),
                String::from("US Ton"),
                String::from("t"),
                String::from("Ton"),
            ],
        ),
        (
            "mass_mt".to_string(),
            vec![
                String::from("Metric Tonnes"),
                String::from("Metric Tonne"),
                String::from("mt"),
                String::from("Tonne"),
            ],
        ),
        (
            "pres_pa".to_string(),
            vec![
                String::from("Pascals"),
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
                String::from("mmHg"),
                String::from("millimetre of Mercury"),
                String::from("millimetres of Mercury"),
                String::from("Torr"),
                String::from("millimeter of Mercury"),
                String::from("millimeters of Mercury"),
            ],
        ),
        (
            "pres_psi".to_string(),
            vec![String::from("psi"), String::from("Pounds per square inch")],
        ),
        ("pres_bar".to_string(), vec![String::from("Bar")]),
        (
            "pres_atm".to_string(),
            vec![
                String::from("Standard Atmospheres"),
                String::from("Standard Atmosphere"),
                String::from("atm"),
                String::from("Atmosphere"),
                String::from("std atm"),
            ],
        ),
        (
            "nrg_ev".to_string(),
            vec![
                String::from("Electron Volts"),
                String::from("Electron Volt"),
                String::from("eV"),
                String::from("Electronvolt"),
            ],
        ),
        (
            "nrg_j".to_string(),
            vec![
                String::from("Joules"),
                String::from("Joule"),
                String::from("j"),
            ],
        ),
        (
            "nrg_cal".to_string(),
            vec![
                String::from("Calories"),
                String::from("Calorie"),
                String::from("cal"),
            ],
        ),
        (
            "nrg_btu".to_string(),
            vec![
                String::from("British Thermal Units"),
                String::from("British Thermal Unit"),
                String::from("btu"),
            ],
        ),
        (
            "nrg_wh".to_string(),
            vec![
                String::from("Watt Hours"),
                String::from("Watt Hour"),
                String::from("wH"),
            ],
        ),
        (
            "pow_w".to_string(),
            vec![
                String::from("Watts"),
                String::from("Watt"),
                String::from("w"),
            ],
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
                String::from("Square inches"),
                String::from("Square inch"),
                String::from("in²"),
                String::from("in2"),
                String::from("sqin"),
            ],
        ),
        (
            "area_ft2".to_string(),
            vec![
                String::from("Square feet"),
                String::from("Square foot"),
                String::from("ft²"),
                String::from("ft2"),
                String::from("sqft"),
            ],
        ),
        (
            "area_m2".to_string(),
            vec![
                String::from("Square metres"),
                String::from("Square metre"),
                String::from("m²"),
                String::from("m2"),
                String::from("sqm"),
            ],
        ),
        (
            "area_ac".to_string(),
            vec![
                String::from("Acres"),
                String::from("Acre"),
                String::from("ac"),
            ],
        ),
        (
            "area_ha".to_string(),
            vec![
                String::from("Hectares"),
                String::from("Hectare"),
                String::from("Ha"),
            ],
        ),
        (
            "area_km2".to_string(),
            vec![
                String::from("Square kilometres"),
                String::from("Square kilometre"),
                String::from("km²"),
                String::from("km2"),
                String::from("sqkm"),
            ],
        ),
        (
            "area_mi2".to_string(),
            vec![
                String::from("Square miles"),
                String::from("Square mile"),
                String::from("mi²"),
                String::from("mi2"),
                String::from("sqmi"),
            ],
        ),
        (
            "angle_grad".to_string(),
            vec![
                String::from("Gradians"),
                String::from("Gradian"),
                String::from("grad"),
            ],
        ),
        (
            "angle_deg".to_string(),
            vec![
                String::from("Degrees"),
                String::from("Degree"),
                String::from("deg"),
            ],
        ),
        (
            "angle_rad".to_string(),
            vec![
                String::from("Radians"),
                String::from("Radian"),
                String::from("rad"),
            ],
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
                String::from("Knots"),
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
        for name in &names {
            if &name.to_lowercase() == &key.to_lowercase() {
                // let x: &String = &names[0];
                unit_list = get_unit_via_list(&key);
            }
        }
    }
    for unit in unit_list {
        // print!("{}", "finding conversions for: ");
        // println!("{}", unit.names[1]);
        let test = get_calculation(&second_key, &unit);
        if test.names[0] != "TEMPLATE".to_string() {
            return (unit, test);
        }
    }

    return (types::template_unit(), types::template_conversion());
}

fn get_unit_second(key: &String, units: Vec<types::Unit>) -> types::Unit {
    for unit in units {
        for name in &unit.names {
            if &name.to_lowercase() == &key.to_lowercase() {
                return unit;
            }
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

    if angle.names[0] != bar {
        foo.push(angle);
    }
    if area.names[0] != bar {
        foo.push(area);
    }
    if distance.names[0] != bar {
        foo.push(distance);
    }
    if energy.names[0] != bar {
        foo.push(energy);
    }
    if mass.names[0] != bar {
        foo.push(mass);
    }
    if power.names[0] != bar {
        foo.push(power);
    }
    if pressure.names[0] != bar {
        foo.push(pressure);
    }
    if speed.names[0] != bar {
        foo.push(speed);
    }
    if temperature.names[0] != bar {
        foo.push(temperature);
    }
    if time.names[0] != bar {
        foo.push(time);
    }
    if volume.names[0] != bar {
        foo.push(volume);
    }
    return foo;
}

fn get_calculation(key: &String, unit: &types::Unit) -> types::Conversion {
    let bar = &unit.conversions;
    for conversion in bar {
        for name in &conversion.names {
            if &name.to_lowercase() == &key.to_lowercase() {
                // print!("{}", "found: ");
                // println!("{}", conversion.names[0]);
                return conversion.clone();
            }
        }
    }
    return types::template_conversion();
}

pub fn valid_key(key: &String) -> bool {
    let foo = get_unit_names().values().cloned().collect::<Vec<_>>();
    for bar in foo {
        for haystack in bar {
            if &haystack.to_lowercase() == &key.to_lowercase() {
                return true;
            }
        }
    }
    return false;
}

pub fn key_can_use_si(key: &String) -> bool {
    let units = get_unit_via_list(key);
    return units[0].can_use_si;
    // return true;
}
