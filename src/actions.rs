use crate::{models::FolderInfo, ui::app::AppMode};

#[derive(Clone, PartialEq, Eq)]
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
    ArtifactsSortByPath,
    ArtifactsSortBySize,
    ArtifactsSortByLastMod,
    ArtifactsRemoveRow,
    ArtifactsInsertRow(FolderInfo),
    ArtifactsSelectNextRow,
    ArtifactsSelectPreviousRow,
    Error(String),
}
