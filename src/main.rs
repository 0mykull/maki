mod args;
mod init;
mod messages;
mod utils;

use args::Settings;

fn main() {
    let settings = Settings::parse();
    utils::process_paths(&settings);
}
