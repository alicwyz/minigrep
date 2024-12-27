
use std::env;
//std::env::args returns an iterator of the cli arguments
//collect -> vector
//(requires Unicode)

use std::process;

use minigrep::Config;
//pub keyword!!

fn main() {

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprint!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for \"{}\"", config.query); //Print to outpur stream
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) { //Match run(config) to Err(e)
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

