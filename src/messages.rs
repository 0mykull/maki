// const YELLOW: &str = "\x1b[1;33m";
// // const RED: &str = "\x1b[1;31m";
// // const GREEN: &str = "\x1b[1;32m";
// const RESET: &str = "\x1b[0m";
//
// const VERSION: &str = env!("CARGO_PKG_VERSION");
// const NAME: &str = env!("CARGO_PKG_NAME");
//
// pub fn version_message() {
//     println!("{YELLOW}{NAME}: {VERSION}{RESET}")
// }
//
// pub fn usage_message() {
//     println!(
//         "{YELLOW}Usage: {NAME} parent_dir/child_dir/ parent_dir/child_file [Options]...{RESET}"
//     )
// }
//
// pub fn help_message() {
//     let help_message = format!(
//         "
// Description:
//     {NAME}: A modern replacement for touch and mkdir commands.
// Usage:
//     Create New File: {NAME} newfile
//     Create New Directory: {NAME} newdir/
// Example:
//     {NAME} foo.txt bar.txt - Creates foo.txt and bar.txt
//     {NAME} foo/bar.txt - Creates bar.txt in foo directory
//     {NAME} foo/foobar.txt bar.txt baz/ - Creates foobar.txt in foo directory,
//     bar.txt in current directory and baz directory
// "
//     );
//     println!("{YELLOW}{}{RESET}", help_message);
// }

// A global macro for verbose logging
#[macro_export]
macro_rules! verbose_log {
    ($settings:expr, $($arg:tt)*) => {
        if $settings.verbose {
            eprintln!("[VERBOSE] {}", format!($($arg)*));
        }
    }
}

pub fn usage_message() {
    eprintln!("Usage: maki [FLAGS] <PATH>...");
}

pub fn help_message() {
    eprintln!("maki - Modern touch/mkdir replacement");
    eprintln!("Usage: maki [FLAGS] <PATH>...");
    eprintln!("\nFlags:");
    eprintln!("  -h, --help       Show help");
    eprintln!("  -V, --version    Show version");
    eprintln!("  -v, --verbose    Enable verbose output");
    eprintln!("  -m, --mode <OCT> Set permissions (e.g. 755)");
    eprintln!("\nProject Init (Directories only):");
    eprintln!("  -g, --git        Initialize git repo");
    eprintln!("  -n, --npm        Initialize npm project");
    eprintln!("  -c, --cargo      Initialize cargo project");
    eprintln!("      --go         Initialize go module");
}

pub fn version_message() {
    eprintln!("maki 0.1.0");
}

pub fn print_success(msg: &str) {
    eprintln!("✅ {}", msg);
}

pub fn print_error(msg: &str) {
    eprintln!("❌ Error: {}", msg);
}
