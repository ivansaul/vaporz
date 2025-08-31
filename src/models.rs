use crate::utils::{
    fs::{calculate_dir_size, last_modified},
    humanize,
};
use std::{
    path::PathBuf,
    sync::{Arc, OnceLock},
};
use uuid::Uuid;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum ProcessStatus {
    #[default]
    Pending,
    Completed,
    InProgress,
    Failed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FolderInfo {
    pub id: Uuid,
    pub path: PathBuf,
    pub removal_status: ProcessStatus,
    size: Arc<OnceLock<u64>>,
    last_modified: Arc<OnceLock<u64>>,
}

impl FolderInfo {
    pub fn new(path: PathBuf) -> Self {
        let info = Self {
            id: Uuid::new_v4(),
            path: path,
            size: Arc::new(OnceLock::new()),
            last_modified: Arc::new(OnceLock::new()),
            removal_status: ProcessStatus::default(),
        };
        info.bg();
        info
    }

    pub fn path_string(&self) -> String {
        self.path.display().to_string()
    }

    pub fn size(&self) -> Option<u64> {
        self.size.get().copied()
    }

    pub fn last_modified(&self) -> Option<u64> {
        self.last_modified.get().copied()
    }

    fn bg(&self) {
        let (path, cell) = (self.path.clone(), self.size.clone());
        tokio::task::spawn_blocking(move || {
            let size = calculate_dir_size(path);
            let _ = cell.set(size);
        });
        let (path, cell) = (self.path.clone(), self.last_modified.clone());
        tokio::task::spawn_blocking(move || {
            let elapsed = last_modified(path).unwrap_or_default();
            let _ = cell.set(elapsed);
        });
    }
}

impl FolderInfo {
    pub fn human_size(&self) -> Option<String> {
        Some(humanize::format_size(self.size()?))
    }

    pub fn human_last_modified(&self) -> Option<String> {
        Some(humanize::format_last_modified(self.last_modified()?))
    }
}
