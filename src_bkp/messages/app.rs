use git2::Oid;
use iced::widget::pane_grid;

// use anyhow::Result as AnyResult;
use crate::states::app::State;
use super::pane_grid::PaneGridEvent;

#[derive(Debug, Clone)]
pub enum Message {
    StateLoaded(State),
    RepositorySelected(String),
    CommitSelected(Oid),
    ShowHEADSummary,
    StageFile(String),
    UnstageFile(String),
    // PaneGrid Events
    PaneGrid(PaneGridEvent),
    PaneGridClicked(pane_grid::Pane),
    PaneGridDragged(pane_grid::DragEvent),
    PaneGridResized(pane_grid::ResizeEvent),
    PaneGridRestore,
    PaneGridMaximize(pane_grid::Pane),
    PaneGridClose(pane_grid::Pane),
    PaneGridSplit(pane_grid::Axis, pane_grid::Pane),
}
