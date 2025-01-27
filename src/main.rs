use std::env;
mod types;
// use types::Unit;
mod units;
// use types;

fn main() {
    let args: Vec<_> = env::args().collect();
    let parsed = parse_args(args);
    if parsed.3.len() > 0 {
        println!("{}", parsed.3);
        return;
    }

    let key1 = parsed.0;
    let key2 = parsed.1;
    let value_string = parsed.2;

    let value: f32 = value_string.trim().parse().unwrap();

    println!("{}", "");
    println!("{}", "");

    if key1 == key2 {
        println!("{}", "===");
        println!("{}", "ERROR");
        println!("{}", "---");
        println!("{}", "Cannot convert a unit into itself!");
    } else {
        let foo = convert(key1, key2, value);
        println!("{}", "===");
        println!("{}", foo.2);
        println!("{}", foo.1);
        println!("{}", foo.3);
    }
}

/**
 * succeeded, value, title, equation
 */
fn convert(key_one: String, key_two: String, input: f32) -> (bool, f32, String, String) {
    let k1: (String, f32, types::Prefix) = remove_si(key_one);
    let k2: (String, f32, types::Prefix) = remove_si(key_two);

    if !units::valid_key(&k1.0) {
        return (
            false,
            0.0,
            "Invalid Keys".to_string(),
            "first unit is invalid".to_string(),
        );
    }
    if !units::valid_key(&k2.0) {
        return (
            false,
            0.0,
            "Invalid Keys".to_string(),
            "second unit is invalid".to_string(),
        );
    }

    let foo = units::get_unit(k1.0, k2.0);
    let mut bar = (foo.1.calc)(input);
    let mut f1: String = foo.0.names[0].to_string();
    let mut f2: String = foo.1.names[0].to_string();
    let mut equation = "(".to_owned() + &foo.1.text + &")";
    if foo.0.can_use_si && k1.2.name.len() > 0 {
        bar *= k1.1;
        f1 = k1.2.name + &f1.to_lowercase();
        equation += &(" * ".to_owned() + &k1.1.to_string());
    }
    if foo.1.can_use_si && k2.2.name.len() > 0 {
        bar /= k2.1;
        f2 = k2.2.name + &f2.to_lowercase();
        equation += &(" / ".to_owned() + &k2.1.to_string());
    }
    let mut title = f1.to_string();
    title.push_str(" -> ");
    title.push_str(&f2);
    if foo.0.names[0] == "TEMPLATE" {
        return (
            false,
            bar,
            "Invalid Conversion".to_string(),
            "Could not find conversion for k1".to_string(),
        );
    }
    if foo.1.names[0] == "TEMPLATE" {
        return (
            false,
            bar,
            "Invalid Conversion".to_string(),
            "Could not find conversion for k2".to_string(),
        );
    }
    return (true, bar, title.to_string(), equation);
}

fn remove_si(key: String) -> (String, f32, types::Prefix) {
    // key.replace("k".to_string(), "");
    let prefixes: Vec<types::Prefix> = vec![
        types::Prefix {
            prefix: "Q".to_string(),
            name: "Quetta".to_string(),
            value: 1e27,
        },
        types::Prefix {
            prefix: "R".to_string(),
            name: "Ronna".to_string(),
            value: 1e27,
        },
        types::Prefix {
            prefix: "Y".to_string(),
            name: "Yotta".to_string(),
            value: 1e24,
        },
        types::Prefix {
            prefix: "Z".to_string(),
            name: "Zetta".to_string(),
            value: 1e21,
        },
        types::Prefix {
            prefix: "E".to_string(),
            name: "Exa".to_string(),
            value: 1e18,
        },
        types::Prefix {
            prefix: "P".to_string(),
            name: "Peta".to_string(),
            value: 1e15,
        },
        types::Prefix {
            prefix: "T".to_string(),
            name: "Tera".to_string(),
            value: 1e12,
        },
        types::Prefix {
            prefix: "G".to_string(),
            name: "Giga".to_string(),
            value: 1e9,
        },
        types::Prefix {
            prefix: "M".to_string(),
            name: "Mega".to_string(),
            value: 1e6,
        },
        types::Prefix {
            prefix: "k".to_string(),
            name: "Kilo".to_string(),
            value: 1e3,
        },
        types::Prefix {
            prefix: "h".to_string(),
            name: "Hecto".to_string(),
            value: 1e2,
        },
        types::Prefix {
            prefix: "da".to_string(),
            name: "Deca".to_string(),
            value: 1e1,
        },
        types::Prefix {
            prefix: "d".to_string(),
            name: "Deci".to_string(),
            value: 1e-1,
        },
        types::Prefix {
            prefix: "c".to_string(),
            name: "Centi".to_string(),
            value: 1e-2,
        },
        types::Prefix {
            prefix: "m".to_string(),
            name: "Milli".to_string(),
            value: 1e-3,
        },
        types::Prefix {
            prefix: "µ".to_string(),
            name: "Micro".to_string(),
            value: 1e-6,
        },
        types::Prefix {
            prefix: "n".to_string(),
            name: "Nano".to_string(),
            value: 1e-9,
        },
        types::Prefix {
            prefix: "p".to_string(),
            name: "Pico".to_string(),
            value: 1e-12,
        },
        types::Prefix {
            prefix: "f".to_string(),
            name: "Femto".to_string(),
            value: 1e-15,
        },
        types::Prefix {
            prefix: "a".to_string(),
            name: "Atto".to_string(),
            value: 1e-18,
        },
        types::Prefix {
            prefix: "z".to_string(),
            name: "Zepto".to_string(),
            value: 1e-21,
        },
        types::Prefix {
            prefix: "y".to_string(),
            name: "Yocto".to_string(),
            value: 1e-24,
        },
        types::Prefix {
            prefix: "r".to_string(),
            name: "Ronto".to_string(),
            value: 1e27,
        },
        types::Prefix {
            prefix: "q".to_string(),
            name: "Quecto".to_string(),
            value: 1e27,
        },
    ];

    // remove prefix then check its still valid
    // eg. km -> m works but not kelvin -> elvin
    for prefix in prefixes {
        // let bar = &prefix.prefix;
        if key.starts_with(&prefix.prefix) {
            let foo: String = key.replace(&prefix.prefix, "");
            if units::valid_key(&foo) && units::key_can_use_si(&foo) {
                return (foo, prefix.value, prefix);
            }
        } else if key.starts_with(&prefix.name) {
            let foo: String = key.replace(&prefix.name, "");
            if units::valid_key(&foo) && units::key_can_use_si(&foo) {
                return (foo, prefix.value, prefix);
            }
        }
    }
    return (
        key,
        1.0,
        types::Prefix {
            prefix: "".to_string(),
            name: "".to_string(),
            value: 1.0,
        },
    );
}

fn parse_args(args: Vec<String>) -> (String, String, String, String) {
    if args.len() <= 1 {
        return ("".to_owned(),"".to_owned(),"".to_owned(),"Error - missing required args!\nPlease use the following format:\n`main.exe -i [input unit] -o [output unit] -v`\nEg. main.exe -i metre -o foot -v 1.5`\n`Arguments can be in any order".to_owned());
    }
    if has_arg("--v".to_owned(), (&args).to_owned()) || has_arg("--version".to_owned(), (&args).to_owned()) || has_arg("-version".to_owned(), (&args).to_owned()) {
        return (
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "Current version\n1.0.0".to_owned(),
        );
    }
    if has_arg("-list".to_owned(), (&args).to_owned()) {
        let mut t = "List of units".to_owned();
        let units = units::get_unit_names();

        let mut curcat = "".to_owned();
        for unit in units {
            let category = (&unit.0.split("_").collect::<Vec<&str>>()[0]).to_string();
            let text = &unit.1[0];
            if &curcat != &category {
                t.push_str("\n\n---");
                t.push_str(get_category(&category));
                t.push_str("---\n");
            }
            curcat = category;
            t.push_str(text);
            t.push_str(", ");
        }
        return ("".to_owned(), "".to_owned(), "".to_owned(), t);
    }
    // get input
    // -i, -input
    let input = get_arg("-i", (&args).to_owned());
    //get output
    // -o, -output
    let output = get_arg("-o", (&args).to_owned());

    //get value
    // -v, -value, -n, -number
    let val = get_arg("-v", (&args).to_owned());
    return (input, output, val, "".to_owned());
}

fn get_arg(key: &str, args: Vec<String>) -> String {
    let mut i = 0;
    for arg in &args {
        if arg == &key {
            if (&args[i + 1]).starts_with("\"") {
                let temp = &args.join(" ");
                let haystack = temp.split('"');
                for needle in haystack {
                    if needle.starts_with(&args[i + 1]) {
                        return needle.to_string();
                    }
                }
            } else {
                return (&args[i + 1]).to_string();
            }
        }
        i += 1;
    }
    return "".to_string();
}

fn has_arg(key: String, args: Vec<String>) -> bool {
    for arg in &args {
        if arg == &key {
            return true;
        }
    }
    return false;
}

fn get_category(key: &str) -> &str {
    match key {
        "angle" => {
            return "Angle";
        }
        "area" => {
            return "Area";
        }
        "dist" => {
            return "Distance";
        }
        "mass" => {
            return "Mass";
        }
        "nrg" => {
            return "Energy";
        }
        "pow" => {
            return "Power";
        }
        "pres" => {
            return "Pressure";
        }
        "speed" => {
            return "Speed";
        }
        "temp" => {
            return "Temperature";
        }
        "time" => {
            return "Time";
        }
        "vol" => {
            return "Volume";
        }
        _ => {
            return key;
        }
    }
}
