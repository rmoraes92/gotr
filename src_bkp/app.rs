// use std::path::Path;

// use git2::{IndexAddOption, IndexMatchedPath};
use iced::{executor::Default, widget::pane_grid, Application, Command, Theme};

use crate::{git2_ext::ExtRepo, messages::{app::Message, pane_grid::PaneGridEvent}, states::{app::State, custom_gridpane::CustomGridPane}, views::{
        // self,
        app_loading, main_window_logs_and_commits, manage_repository, select_repo, staging_details
    }
};


pub enum App {
    Loading,
    Loaded(State),
    ShowCommitSummary(State),
    ShowHEADSummary(State),
}

impl Application for App {
    type Executor = Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        // TODO wtf is flags for?
        (
            App::Loading,
            Command::batch(vec![
                Command::perform(State::load(), Message::StateLoaded)
            ]),
            // TODO Is it possible to NOT pass a Command?
            // Command::none() // TODO Yes it is
        )
    }

    fn title(&self) -> String {
        format!("Git On The Rocks")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match self {
            Self::Loading => {
                match message {
                    Self::Message::StateLoaded(state) => {
                        *self = App::Loaded(state);
                    },
                    _ => (),
                }
                Command::none()
            },
            Self::Loaded(state) => {
                match message {
                    Self::Message::RepositorySelected(path) => {
                        state.selected_repo_path = Some(path);
                        state.selected_commit = None;
                        *self = App::ShowHEADSummary(state.clone());
                    },
                    _ => (),
                }
                Command::none()
            },
            Self::ShowCommitSummary(state) => {
                match message {
                    Self::Message::CommitSelected(oid) => {
                        state.selected_commit = Some(oid);
                        *self = App::ShowCommitSummary(state.clone());
                    },
                    Self::Message::ShowHEADSummary => {
                        state.selected_commit = None;
                        *self = App::ShowHEADSummary(state.clone());
                    },
                    Self::Message::PaneGridResized(pane_grid::ResizeEvent { split, ratio }) => {
                        state.main_window_panegrid_state.as_mut().unwrap().resize(&split, ratio);
                    },
                    Self::Message::PaneGrid(PaneGridEvent::Resized(pane_grid::ResizeEvent { split, ratio })) => {
                        state.main_window_panegrid_state.as_mut().unwrap().resize(&split, ratio);
                    }
                    _ => (),
                }
                Command::none()
            },
            Self::ShowHEADSummary(state) => {
                // 1st view after we select a repository
                match message {
                    Self::Message::CommitSelected(oid) => {
                        println!("Message::CommitSelected");
                        state.selected_commit = Some(oid);
                        *self = App::ShowCommitSummary(state.clone());
                    },
                    Self::Message::StageFile(file_path) => {
                        state.selected_commit = None;
                        let repo = state.get_repo();
                        repo.stage_file(file_path);
                        *self = App::ShowHEADSummary(state.clone());
                    },
                    Self::Message::UnstageFile(file_path) => {
                        state.selected_commit = None;
                        let repo = state.get_repo();
                        repo.unstage_file(file_path);
                        *self = App::ShowHEADSummary(state.clone());
                    },
                    _ => (),
                }
                Command::none()
            },
            // _ => Command::none(),
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        match self {
            App::Loading => {
                app_loading::view()
            },
            App::Loaded(state) => {
                select_repo::view(state)
            },
            App::ShowCommitSummary(state) => {
                // manage_repository::view(state)
                main_window_logs_and_commits::view(state)
            },
            App::ShowHEADSummary(state) => {
                println!("App::ShowHEADSummary");
                staging_details::view(state)
            },
        }
    }
}
