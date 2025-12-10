mod flags;
mod messages;

use flags::*;
use messages::*;

use std::{fs, io, path::Path, process};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut flags: Vec<String> = Vec::new();
    let mut verbose: bool = false;

    if args.len() < 2 {
        usage_message();
        process::exit(0);
    }
    if args.iter().any(|flag| flag == "--help" || flag == "-H") {
        help_message();
        process::exit(0);
    }
    if args.iter().any(|flag| flag == "--version") {
        version_message();
        process::exit(0);
    }

    if args.len() > 1
        && let Err(e) = create(&args[1])
    {
        eprintln!("Error creating: {e}");
    }
}

fn create(user_path: &str) -> io::Result<()> {
    // path is string, ends with '/' create dir, else create file
    // constraints: dir alr exists,
    // add features: create multiple files or dirs
    let path = Path::new(user_path);

    // create directory
    if user_path.ends_with('/') {
        fs::create_dir_all(path)?;
        println!("Directory {} created successfully.", path.display());
    }
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    fs::File::create(path)?;
    println!("File {} created successfully", path.display());
    // create file
    // else {
    //     let parent_dirs = path.parent().unwrap();
    //     fs::create_dir_all(parent_dirs)?;
    //     fs::File::create(path)?;
    //     println!("File {} created successfully.", path.display());
    // }
    Ok(())
}
