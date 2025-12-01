use std::{
    env,
    // fs::{self, File},
    // path::PathBuf,
    // process::exit,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        println!("{NAME}: Feature coming soon");
    } else {
        help();
        version();
    }
}

fn check_flag() {
    todo!();
}

fn version() {
    println!("{NAME}: {VERSION}")
}

fn help() {
    println!("{NAME}: A modern replacement for touch and mkdir commands.");
    println!("Options:\n -V, --version: Show version")
}
