use std::env;
use git2::{Oid, Repository};
use iced::widget::combo_box::State as ComboBoxState;

#[derive(Debug, Clone)]
pub struct State {
    pub repo_file_paths: Vec<String>,
    pub cbox_recent_repo_paths_state: ComboBoxState<String>,
    pub selected_repo_path: Option<String>,
    pub selected_commit: Option<Oid>,
}

impl State {
    pub async fn load() -> Self {
        let paths = vec![
            String::from(env::var("PWD").unwrap()), // TODO temporary hard-coded list of repos
        ];
        Self {
            repo_file_paths: paths.clone(),
            cbox_recent_repo_paths_state: ComboBoxState::new(paths),
            selected_repo_path: None,
            selected_commit: None,
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