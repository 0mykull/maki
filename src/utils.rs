use crate::args::Settings;
use crate::init;
use crate::messages::{print_error, print_success};
use crate::verbose_log;
use std::os::unix::fs::PermissionsExt;
use std::{fs, io, path::Path};

pub fn process_paths(settings: &Settings) {
    for path_str in &settings.paths {
        if let Err(e) = create_entry(path_str, settings) {
            print_error(&format!("Failed to create {}: {}", path_str, e));
        }
    }
}

fn create_entry(user_path: &str, settings: &Settings) -> io::Result<()> {
    let path = Path::new(user_path);
    verbose_log!(settings, "Processing: {:?}", path);

    // --- Directory Logic ---
    if user_path.ends_with('/') {
        if !path.exists() {
            fs::create_dir_all(path)?;
            print_success(&format!("Directory created: {}", user_path));
        }

        // Handle --mode for Directory
        if let Some(mode) = settings.mode {
            set_permissions(path, mode, settings)?;
        }

        init::run_initializers(path, settings);
        return Ok(());
    }

    // --- File Logic ---
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
        && !parent.exists()
    {
        fs::create_dir_all(parent)?;
    }

    if !path.exists() {
        fs::File::create(path)?;
        print_success(&format!("File created: {}", user_path));
    } else {
        verbose_log!(settings, "File exists, touching timestamp.");
        fs::File::create(path)?;
    }

    // Handle --mode for File
    if let Some(mode) = settings.mode {
        set_permissions(path, mode, settings)?;
    }

    Ok(())
}

fn set_permissions(path: &Path, mode: u32, settings: &Settings) -> io::Result<()> {
    let metadata = fs::metadata(path)?;
    let mut perms = metadata.permissions();
    perms.set_mode(mode);
    fs::set_permissions(path, perms)?;
    verbose_log!(settings, "Set permissions to {:o}", mode);
    Ok(())
}
