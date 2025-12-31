use crate::messages::{print_error, print_success};
use crate::settings::Settings;
use crate::verbose_log;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

// ==========================================
// Core Processor
// ==========================================

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

        // Mode logic
        if let Some(mode) = settings.mode {
            set_permissions(path, mode, settings)?;
        }

        // Init logic (Separated, but called here for flow)
        run_initializers(path, settings);

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

    // Mode logic for file
    if let Some(mode) = settings.mode {
        set_permissions(path, mode, settings)?;
    }

    Ok(())
}

// ==========================================
// Helper: Permissions (Cross-Platform Safe)
// ==========================================

fn set_permissions(path: &Path, mode: u32, settings: &Settings) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = fs::metadata(path)?;
        let mut perms = metadata.permissions();
        perms.set_mode(mode);
        fs::set_permissions(path, perms)?;
        verbose_log!(settings, "Set permissions to {:o}", mode);
    }

    #[cfg(not(unix))]
    {
        verbose_log!(
            settings,
            "Skipping permissions (Windows/Non-Unix doesn't support chmod via mode)"
        );
    }

    Ok(())
}

// ==========================================
// Helper: Initializers (Git, Cargo, etc.)
// ==========================================

fn run_initializers(path: &Path, settings: &Settings) {
    if !path.is_dir() {
        // Safety check
        if settings.git || settings.npm || settings.cargo || settings.go {
            verbose_log!(
                settings,
                "Skipping init: '{}' is not a directory.",
                path.display()
            );
            print_error(&format!(
                "Cannot initialize project: '{}' is a file.",
                path.display()
            ));
        }
        return;
    }

    if settings.git {
        let _ = run_git(path, settings);
    }
    if settings.cargo {
        let _ = run_cargo(path, settings);
    }
    if settings.npm {
        let _ = run_npm(path, settings);
    }
    if settings.go {
        let _ = run_go(path, settings);
    }
}

fn run_git(path: &Path, settings: &Settings) -> io::Result<()> {
    run_command("git", &["init"], path, settings, "Git initialized")
}

fn run_cargo(path: &Path, settings: &Settings) -> io::Result<()> {
    run_command("cargo", &["init"], path, settings, "Cargo initialized")
}

fn run_npm(path: &Path, settings: &Settings) -> io::Result<()> {
    run_command("npm", &["init", "-y"], path, settings, "NPM initialized")
}

fn run_go(path: &Path, settings: &Settings) -> io::Result<()> {
    let dir_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("my-project");
    run_command(
        "go",
        &["mod", "init", dir_name],
        path,
        settings,
        "Go module initialized",
    )
}

fn run_command(
    cmd: &str,
    args: &[&str],
    dir: &Path,
    settings: &Settings,
    success_msg: &str,
) -> io::Result<()> {
    verbose_log!(settings, "Running: {} {:?}", cmd, args);

    // .output() waits for the command to finish automatically
    let output = Command::new(cmd).args(args).current_dir(dir).output();

    match output {
        Ok(out) => {
            if out.status.success() {
                print_success(success_msg);
                if settings.verbose && !out.stdout.is_empty() {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    println!("   > {}", stdout.trim().replace('\n', "\n   > "));
                }
            } else {
                let stderr = String::from_utf8_lossy(&out.stderr);
                print_error(&format!("{} failed: {}", cmd, stderr.trim()));
            }
            Ok(())
        }
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                print_error(&format!("Command '{}' not found in PATH", cmd));
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}
