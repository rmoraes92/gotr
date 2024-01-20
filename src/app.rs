use iced::{Application, Theme, executor::Default, Command};

use crate::{states::app::State, messages::app::Message,
    views::{
        app_loading,
        app_loaded,
        manage_repository
    }
};


pub enum App {
    Loading,
    Loaded(State),
    RepositorySelected(State),
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
                        *self = App::RepositorySelected(state.clone());
                    },
                    _ => (),
                }
                Command::none()
            },
            Self::RepositorySelected(state) => {
                match message {
                    Self::Message::CommitSelected(oid) => {
                        state.selected_commit = Some(oid);
                        *self = App::RepositorySelected(state.clone());
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
            App::RepositorySelected(state) => {
                manage_repository::view(state)
            }
        }
    }
}