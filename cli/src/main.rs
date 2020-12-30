use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::process;

pub struct Config {
    pub src: String,
    pub dst: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let src = match args.next() {
            Some(a) => a,
            None => return Err("Must specify src path"),
        };

        let dst = match args.next() {
            Some(a) => a,
            None => return Err("Must specify dst path"),
        };

        Ok(Config { src, dst })
    }
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    let items = pvs::run(&config.src).unwrap_or_else(|err| {
        eprintln!("Error parsing file: {}", err);
        process::exit(1);
    });

    let dst = File::create(config.dst).unwrap_or_else(|err| {
        eprintln!("Error creating destination file: {}", err);
        process::exit(1);
    });

    serde_json::to_writer(BufWriter::new(dst), &items).unwrap_or_else(|err| {
        eprintln!("Error serializing to JSON: {}", err);
        process::exit(1);
    });
}
