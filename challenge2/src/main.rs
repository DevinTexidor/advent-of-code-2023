use std::env;
use std::process;

use challenge1::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem Parsing Args: {err}");
        process::exit(1);
    });

    if let Err(e) = challenge1::run(config) {
        println!("App Run Error: {e}");
        process::exit(1);
    };
}
