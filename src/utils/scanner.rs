use crate::models::FolderInfo;
use std::path::Path;
use tokio::sync::mpsc::UnboundedSender;

const TARGETS: &[&str] = &[
    "target",       // Rust
    "node_modules", // Node.js
    ".venv",        // Python
    "__pycache__",  // Python
    "build",        // Flutter
    "dist",         // Java
];

pub fn find_target_dirs<P: AsRef<Path>>(dir: P, targets: &[&str], tx: UnboundedSender<FolderInfo>) {
    use walkdir::WalkDir;
    let mut walker = WalkDir::new(dir).into_iter();
    while let Some(Ok(entry)) = walker.next() {
        if let Some(name) = entry.path().file_name()
            && entry.file_type().is_dir()
            && targets.iter().any(|t| *t == name)
        {
            let info = FolderInfo::new(entry.path().to_path_buf());
            let _ = tx.send(info);
            walker.skip_current_dir();
            continue;
        }
    }
}

pub fn scan_current_dir(tx: UnboundedSender<FolderInfo>) {
    let root = std::env::current_dir().expect("could not get current_dir");
    find_target_dirs(&root, TARGETS, tx);
}
