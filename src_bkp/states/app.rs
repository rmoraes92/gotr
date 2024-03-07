use std::env;
use git2::{Oid, Repository};
use iced::widget::{combo_box::State as ComboBoxState, pane_grid};

use super::custom_gridpane::{CustomGridPane, PANE_LEFT, PANE_RIGHT};

#[derive(Debug, Clone)]
pub struct State {
    pub repo_file_paths: Vec<String>,
    pub cbox_recent_repo_paths_state: ComboBoxState<String>,
    pub selected_repo_path: Option<String>,
    pub selected_commit: Option<Oid>,
    pub main_window_panegrid_state: Option<pane_grid::State<CustomGridPane>>,
    pub main_window_panegrid_pane_count: usize,
}

impl State {
    pub async fn load() -> Self {
        let paths = vec![
            String::from(env::var("PWD").unwrap()), // TODO temporary hard-coded list of repos
        ];
        let (mut main_window_panegrid_state, _pane) = pane_grid::State::new(
            CustomGridPane::new(PANE_LEFT),
        );
        let axis = pane_grid::Axis::Vertical;
        main_window_panegrid_state.split(axis, &_pane, CustomGridPane::new(PANE_RIGHT));

        Self {
            repo_file_paths: paths.clone(),
            cbox_recent_repo_paths_state: ComboBoxState::new(paths),
            selected_repo_path: None,
            selected_commit: None,
            main_window_panegrid_state: Some(main_window_panegrid_state),
            main_window_panegrid_pane_count: 1,
        }
    }
    pub fn get_last_open_repo_path(&self) -> String {
        self.repo_file_paths.clone().into_iter().nth(0).unwrap()
    }
    pub fn get_repo(&self) -> Repository {
        Repository::open(
            self.selected_repo_path.clone().unwrap()).unwrap()
    }
}