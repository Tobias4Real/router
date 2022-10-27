pub mod graph;
pub mod args;

use std::env;
use std::process::exit;
use args::Args;

fn main() {
    env::set_var("RUST_LOG", "info");

    let string_args: Vec<String> = env::args().collect();
    let args = Args::parse(string_args).map_err(|err| {
        println!("{}", err);
        exit(-1);
    }).unwrap();

    println!("Command: {}", args.cmd);

    if let Some(lat) = args.lat {
        println!("Latitude: {}", lat);
    }

    println!("Hello, world!");
}

