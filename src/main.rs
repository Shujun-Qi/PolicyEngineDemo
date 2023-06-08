use std::env;
mod intoto;
mod tuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let sourcetype = &args[1];
    println!("Enforcing policy for {}", sourcetype);
    let dirname = &args[2];
    println!("Reading file from {}", dirname);
    match sourcetype.as_str() {
        "intoto" => intoto::enforce_intoto(&dirname),
        "tuf" => tuf::enforce_tuf(&dirname),
        _ => println!("Unknown Type"),
    }
}