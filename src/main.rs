use std::process;
use std::env;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config  = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem with arguments : {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Error : {}", e);

        process::exit(2);
    }
}


