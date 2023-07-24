use io_project::Config;
use std::env;

use std::process;

fn main() {
    // let args: Vec<String> = env::args().collect();
    //dbg!(args);

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        // prints to stderr
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = io_project::run(config) {
        // prints to stderr
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
