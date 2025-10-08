use std::{env, fs, path::Path};
use std::process;

/// The main entry point for the maki CLI tool.
/// It reads the first command-line argument and determines whether to create a
/// file or a directory based on the presence of a trailing slash '/'.
fn main() {
    // 1. Collect command-line arguments.
    let args: Vec<String> = env::args().collect();

    // 2. Check for the correct number of arguments. We expect the program name + one target path.
    if args.len() != 2 {
        eprintln!("
  \x1b[1;36m<maki>\x1b[0m - Minimal File/Directory Creator

  \x1b[1;33mUSAGE:\x1b[0m maki <path>

  \x1b[1;32mEXAMPLES:\x1b[0m
  \u{25b6} \x1b[36mmaki components/Button.rs\x1b[0m  (Creates a file)
  \u{25b6} \x1b[36mmaki lib/data/\x1b[0m               (Creates a directory, including parents)
");
        process::exit(1);
    }

    // The target path is the second element in the arguments vector.
    let target_input = &args[1];

    // Determine if the input path is requesting a directory.
    let is_dir_request = target_input.ends_with('/');

    // Prepare the actual path string for file system operations.
    let target_path_str: String;

    if is_dir_request {
        // For directories, trim the trailing slash, as the `create_dir_all` function
        // expects the path to be the directory name itself.
        target_path_str = target_input.trim_end_matches('/').to_string();
    } else {
        // For files, use the path exactly as provided.
        target_path_str = target_input.clone();
    }

    // Basic validation to prevent creating items with empty names.
    if target_path_str.is_empty() {
        eprintln!("\x1b[1;31mError:\x1b[0m The resulting file or directory name is empty.");
        process::exit(1);
    }

    let target_path = Path::new(&target_path_str);

    if is_dir_request {
        // --- Directory Creation ---
        println!("\x1b[1;34mDIR:\x1b[0m Creating directory structure: '{}'", target_path_str);
        // Use create_dir_all to create the directory and any necessary parent directories.
        match fs::create_dir_all(target_path) {
            Ok(_) => println!("\x1b[1;32mSUCCESS:\x1b[0m Directory '{}' created.", target_path_str),
            Err(e) => {
                eprintln!("\x1b[1;31mERROR:\x1b[0m Failed to create directory '{}': {}", target_path_str, e);
                process::exit(1);
            }
        }
    } else {
        // --- File Creation ---
        println!("\x1b[1;34mFILE:\x1b[0m Creating file: '{}'", target_path_str);
        // fs::File::create will create the file and truncate it if it already exists.
        match fs::File::create(target_path) {
            Ok(_) => println!("\x1b[1;32mSUCCESS:\x1b[0m File '{}' created.", target_path_str),
            Err(e) => {
                eprintln!("\x1b[1;31mERROR:\x1b[0m Failed to create file '{}': {}", target_path_str, e);
                process::exit(1);
            }
        }
    }
}
