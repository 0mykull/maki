mod messages;
mod settings;
mod utils;

use settings::Settings;

fn main() {
    let settings = Settings::parse();
    utils::process_paths(&settings);
}
