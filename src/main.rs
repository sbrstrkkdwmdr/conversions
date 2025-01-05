mod types;
// use types::Unit;
mod units;
// use types;

fn main() {
    println!("Hello, world!");

    let foo = units::get_unit("test".to_string(), "test2".to_string());
}
