use crate::messages::{help_message, usage_message, version_message};
use std::process;

pub struct Settings {
    pub verbose: bool,
    pub git: bool,
    pub npm: bool,
    pub cargo: bool,
    pub go: bool,
    // executable removed
    pub mode: Option<u32>,
    pub paths: Vec<String>,
}

impl Settings {
    pub fn parse() -> Self {
        let mut settings = Settings {
            verbose: false,
            git: false,
            npm: false,
            cargo: false,
            go: false,
            // executable removed
            mode: None,
            paths: Vec::new(),
        };

        let args: Vec<String> = std::env::args().skip(1).collect();
        let mut iter = args.into_iter();

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                // --- Meta ---
                "-h" | "--help" => {
                    help_message();
                    process::exit(0);
                }
                "-V" | "--version" => {
                    version_message();
                    process::exit(0);
                }
                "-v" | "--verbose" => settings.verbose = true,

                // --- Init Flags ---
                "-g" | "--git" => settings.git = true,
                "-n" | "--npm" => settings.npm = true,
                "-c" | "--cargo" => settings.cargo = true,
                "--go" | "--golang" => settings.go = true,

                // --- Value Flags ---
                "-m" | "--mode" => {
                    if let Some(val) = iter.next() {
                        match u32::from_str_radix(&val, 8) {
                            Ok(m) => settings.mode = Some(m),
                            Err(_) => {
                                eprintln!("Error: Invalid mode '{}'. Use octal (e.g. 755)", val);
                                process::exit(1);
                            }
                        }
                    } else {
                        eprintln!("Error: --mode requires a value (e.g. --mode 755)");
                        process::exit(1);
                    }
                }

                // --- Paths ---
                _ => {
                    if arg.starts_with('-') {
                        eprintln!("Unknown flag: {}", arg);
                        process::exit(1);
                    } else {
                        settings.paths.push(arg);
                    }
                }
            }
        }

        if settings.paths.is_empty() {
            eprintln!("Error: No path provided.");
            usage_message();
            process::exit(1);
        }

        settings
    }
}
