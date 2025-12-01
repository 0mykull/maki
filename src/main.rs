mod messages;

use messages::*;

use std::{fs, io, path::Path, process};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        usage_message();
        process::exit(0);
    }
    if args.iter().any(|flag| flag == "--help") {
        help_message();
        process::exit(0);
    }
    if args.iter().any(|flag| flag == "--version") {
        version_message();
        process::exit(0);
    }
    if args.len() == 2
        && let Err(e) = create(&args[1])
    {
        eprintln!("Error creating: {e}");
    }
}

fn create(user_path: &str) -> io::Result<()> {
    // path is string, ends with '/' create dir, else create file
    // constraints: dir alr exists,
    let path = Path::new(user_path);

    if user_path.ends_with('/') {
        fs::create_dir_all(path)?;
        println!("Directory {} created successfully.", path.display());
    } else {
        let parent_dirs = path.parent().unwrap();
        fs::create_dir_all(parent_dirs)?;
        fs::File::create(path)?;
    }
    Ok(())
}
