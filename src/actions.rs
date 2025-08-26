use uuid::Uuid;

use crate::{
    models::{FolderInfo, Removed},
    ui::app::AppMode,
};

#[derive(Clone)]
pub enum AppAction {
    Render,
    Quit,
    Tick,
    KeyUp,
    KeyDown,
    KeyEnter,
    KeyCharLowerM,
    KeyCharLowerP,
    KeyCharLowerS,
    SwitchMode(AppMode),
    ArtifactNewRow(FolderInfo),
    ArtifactUpdateRowRemoveStatus { id: Uuid, removed: Removed },
}
