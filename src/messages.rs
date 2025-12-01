// const YELLOW: &str = "\x1b[1;33m";
// const RED: &str = "\x1b[1;31m";
// const GREEN: &str = "\x1b[1;32m";
// const RESET: &str = "\x1b[0m";

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

pub fn version_message() {
    println!("{NAME}: {VERSION}")
}

pub fn usage_message() {
    println!("Usage: {NAME} newdir/newfile [options]...")
}

pub fn help_message() {
    let help_message = format!(
        "
Description:
    {NAME}: A modern replacement for touch and mkdir commands.
Usage:
    Create New File: {NAME} newfile
    Create New Directory: {NAME} newdir/
Example:
    {NAME} foo.txt bar.txt - Creates foo.txt and bar.txt
    {NAME} foo/bar.txt - Creates bar.txt in foo directory
    {NAME} foo/foobar.txt bar.txt baz/ - Creates foobar.txt in foo directory, 
    bar.txt in current directory and baz directory   
"
    );
    println!("{}", help_message);
}
