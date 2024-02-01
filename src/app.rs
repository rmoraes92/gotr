use std::path::Path;

use git2::{IndexAddOption, IndexMatchedPath};
use iced::{Application, Theme, executor::Default, Command};

use crate::{git2_ext::ExtRepo, messages::app::Message, states::app::State, views::{
        app_loaded, app_loading, manage_repository, staging_details
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
            ]), // TODO Is it possible to NOT pass a Command?
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
                    _ => (),
                }
                Command::none()
            },
            Self::ShowHEADSummary(state) => {
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

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        match self {
            App::Loading => {
                app_loading::view()
            },
            App::Loaded(state) => {
                app_loaded::view(state)
            },
            App::ShowCommitSummary(state) => {
                manage_repository::view(state)
            },
            App::ShowHEADSummary(state) => {
                println!("App::ShowHEADSummary");
                staging_details::view(state)
            },
        }
    }
}
