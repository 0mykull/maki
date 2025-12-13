use crate::args::Settings;
use crate::messages::{print_error, print_success};
use crate::verbose_log;
use std::io;
use std::path::Path;
use std::process::Command;

pub fn run_initializers(path: &Path, settings: &Settings) {
    if !path.is_dir() {
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
