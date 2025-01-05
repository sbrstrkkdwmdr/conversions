mod types;
// use types::Unit;
mod units;
// use types;

fn main() {
    println!("Hello, world!");

    let foo = convert("test".to_string(), "test2".to_string(), 1.0);
}

fn convert(keyOne: String, keyTwo: String, input: f32) -> (bool, f32, String, String) {
    let k1 = remove_si(keyOne);
    let k2 = remove_si(keyTwo);

    if !units::valid_key(&k1.0) {
        return (false, 0.0, "K1 INVALID".to_string());
    }
    if !units::valid_key(&k2.0) {
        return (false, 0.0, "K2 INVALID".to_string());
    }

    let foo = units::get_unit(k1.0, k2.0);
    return (true, (foo.1.calc)(input), foo.1.text);
}

fn remove_si(key: String) -> (String, f32) {
    let mut foo: &String = &key;
    // key.replace("k".to_string(), "");
    let prefixes: Vec<types::Prefix> = vec![
        types::Prefix {
            prefix: "Q".to_string(),
            name: "quetta".to_string(),
            value: 1e27,
        },
        types::Prefix {
            prefix: "R".to_string(),
            name: "ronna".to_string(),
            value: 1e27,
        },
        types::Prefix {
            prefix: "Y".to_string(),
            name: "yotta".to_string(),
            value: 1e24,
        },
        types::Prefix {
            prefix: "Z".to_string(),
            name: "zetta".to_string(),
            value: 1e21,
        },
        types::Prefix {
            prefix: "E".to_string(),
            name: "exa".to_string(),
            value: 1e18,
        },
        types::Prefix {
            prefix: "P".to_string(),
            name: "peta".to_string(),
            value: 1e15,
        },
        types::Prefix {
            prefix: "T".to_string(),
            name: "tera".to_string(),
            value: 1e12,
        },
        types::Prefix {
            prefix: "G".to_string(),
            name: "giga".to_string(),
            value: 1e9,
        },
        types::Prefix {
            prefix: "M".to_string(),
            name: "mega".to_string(),
            value: 1e6,
        },
        types::Prefix {
            prefix: "k".to_string(),
            name: "kilo".to_string(),
            value: 1e3,
        },
        types::Prefix {
            prefix: "h".to_string(),
            name: "hecto".to_string(),
            value: 1e2,
        },
        types::Prefix {
            prefix: "da".to_string(),
            name: "deca".to_string(),
            value: 1e1,
        },
        types::Prefix {
            prefix: "d".to_string(),
            name: "deci".to_string(),
            value: 1e-1,
        },
        types::Prefix {
            prefix: "c".to_string(),
            name: "centi".to_string(),
            value: 1e-2,
        },
        types::Prefix {
            prefix: "m".to_string(),
            name: "milli".to_string(),
            value: 1e-3,
        },
        types::Prefix {
            prefix: "µ".to_string(),
            name: "micro".to_string(),
            value: 1e-6,
        },
        types::Prefix {
            prefix: "n".to_string(),
            name: "nano".to_string(),
            value: 1e-9,
        },
        types::Prefix {
            prefix: "p".to_string(),
            name: "pico".to_string(),
            value: 1e-12,
        },
        types::Prefix {
            prefix: "f".to_string(),
            name: "femto".to_string(),
            value: 1e-15,
        },
        types::Prefix {
            prefix: "a".to_string(),
            name: "atto".to_string(),
            value: 1e-18,
        },
        types::Prefix {
            prefix: "z".to_string(),
            name: "zepto".to_string(),
            value: 1e-21,
        },
        types::Prefix {
            prefix: "y".to_string(),
            name: "yocto".to_string(),
            value: 1e-24,
        },
        types::Prefix {
            prefix: "r".to_string(),
            name: "ronto".to_string(),
            value: 1e27,
        },
        types::Prefix {
            prefix: "q".to_string(),
            name: "quecto".to_string(),
            value: 1e27,
        },
    ];

    // remove prefix then check its still valid
    // eg. km -> m works but not kelvin -> elvin
    for prefix in prefixes {
        // let bar = &prefix.prefix;
        if key.starts_with(&prefix.prefix) {
            let foo: String = key.replace(&prefix.prefix, "");
            if units::valid_key(&foo) {
                return (foo, prefix.value);
            }
        } else if key.starts_with(&prefix.name) {
            let foo: String = key.replace(&prefix.name, "");
            if units::valid_key(&foo) {
                return (foo, prefix.value);
            }
        }
    }
    return (key, 1.0);
}
