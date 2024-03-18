use std::convert::From;
use std::{env, thread, time::Duration};  // TODO remove this once we implement the real yaml config
use iced::widget::pane_grid;
use crate::apps;
use crate::apps::main_window::MainWindowState;

#[derive(Debug, Clone)]
pub struct State {
    pub repository_path: Option<String>,
    pub recent_repository_paths: Vec<String>,
    pub main_window: Option<apps::main_window::MainWindowState>,
}

impl State {
    pub async fn load() -> Self {  // TODO we can probably rename it to "from(yaml_file_path)"
        let paths = vec![
            String::from(env::var("PWD").unwrap()), // TODO temporary hard-coded list of repos
        ];
        thread::sleep(Duration::from_millis(2000));
        Self {
            repository_path: None,
            recent_repository_paths: paths,
            main_window: None,
        }
    }
}

impl Into<apps::main_window::MainWindowState> for State {
    fn into(self) -> apps::main_window::MainWindowState {
        match self.main_window {
            Some(app_state) => app_state,
            None => MainWindowState::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    NavigateTo(String, State),
    SelectRepository(State),
    MainWindow(apps::main_window::message::Message),
}

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
