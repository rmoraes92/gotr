use iced::widget::pane_grid;

#[derive(Debug, Clone)]
pub enum PaneGridEvent {
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    Restore,
    Maximize(pane_grid::Pane),
    Close(pane_grid::Pane),
    Split(pane_grid::Axis, pane_grid::Pane),
}