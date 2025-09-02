use crate::config;
use crate::models::{FolderInfo, TargetInfo};
use std::path::Path;
use tokio::sync::mpsc::UnboundedSender;
use walkdir::WalkDir;

pub fn find_target_dirs<P: AsRef<Path>>(
    dir: P,
    targets: Vec<TargetInfo>,
    tx: UnboundedSender<FolderInfo>,
) {
    let mut walker = WalkDir::new(dir).into_iter();

    while let Some(Ok(entry)) = walker.next() {
        let path = entry.path();

        if !entry.file_type().is_dir() || entry.path_is_symlink() {
            continue;
        }

        if let Some(name) = path.file_name().and_then(|n| n.to_str())
            && name.starts_with('.')
        {
            walker.skip_current_dir();
            continue;
        }

        if let Some(target) = targets.iter().find(|t| t.is_project_root(path)) {
            find_artifacts_in_project(path, target, &mut walker, &tx);
        }
    }
}

fn find_artifacts_in_project(
    project_root: &Path,
    target: &TargetInfo,
    walker: &mut walkdir::IntoIter,
    tx: &UnboundedSender<FolderInfo>,
) {
    let mut subwalker = WalkDir::new(project_root).into_iter();

    while let Some(Ok(entry)) = subwalker.next() {
        if let Some(name) = entry.file_name().to_str() {
            if target.artifacts.iter().any(|artifact| artifact == name) {
                let info = FolderInfo::new(entry.path().to_path_buf());
                let _ = tx.send(info);
                subwalker.skip_current_dir();
            }
        }
    }

    walker.skip_current_dir();
}

pub fn scan_current_dir(tx: UnboundedSender<FolderInfo>) {
    if let Ok(root) = std::env::current_dir() {
        let targets = get_targets();
        find_target_dirs(&root, targets, tx);
    }
}

fn get_targets() -> Vec<TargetInfo> {
    if let Ok(config) = config::load_config() {
        return config.targets;
    }
    Vec::new()
}
