use crate::errors::Result;
use std::{fs, path::Path, process::Command};

pub fn last_modified<P: AsRef<Path>>(path: P) -> Option<u64> {
    fs::metadata(path)
        .ok()?
        .modified()
        .ok()?
        .elapsed()
        .ok()
        .map(|d| d.as_secs())
}

#[cfg(unix)]
pub fn calculate_dir_size<P: AsRef<Path>>(dir: P) -> u64 {
    let cmd = Command::new("du").arg("-sk").arg(dir.as_ref()).output();

    let output = if let Ok(output) = cmd {
        output
    } else {
        return 0;
    };

    if !output.status.success() {
        return 0;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<u64>().ok())
        .map(|kb| kb * 1024)
        .unwrap_or(0)
}

#[cfg(not(unix))]
pub fn calculate_dir_size<P: AsRef<Path>>(dir: P) -> u64 {
    use rayon::prelude::*;
    use std::fs;
    use walkdir::WalkDir;
    WalkDir::new(dir)
        .into_iter()
        .par_bridge()
        .filter_map(Result::ok)
        .filter(|entry| !entry.path_is_symlink())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| fs::metadata(entry.path()).map(|m| m.len()).unwrap_or(0))
        .sum()
}

pub fn remove_path(path: &Path) -> Result<()> {
    // if path.is_file() {
    //     fs::remove_file(path)?;
    // } else if path.is_dir() {
    //     fs::remove_dir_all(path)?;
    // }
    std::thread::sleep(std::time::Duration::from_secs(2));
    log::info!("{}", path.display().to_string());
    Ok(())
}
