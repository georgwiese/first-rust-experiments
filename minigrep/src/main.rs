use std::env;
use minigrep::Config;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing config: {:?}", err);
        process::exit(1)
    });
    if let Err(e) = minigrep::run(&config) {
        eprintln!("Error running program: {:?}", e);
        process::exit(2)
    }
}
