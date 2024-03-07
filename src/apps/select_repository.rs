use crate::apps;
use crate::globals;
use iced;

pub const APP_NAME: &str = "select_repository";

pub fn update(
    _state: &globals::State,
    _message: globals::Message,
) -> (Option<globals::State>, iced::Command<globals::Message>) {
    (None, iced::Command::none())
}

pub fn view<'a>(state: &'a globals::State) -> iced::Element<'a, globals::Message> {
    let recent_repos: Vec<String> = state.recent_repository_paths.clone();
    let first_option: String = recent_repos.clone().into_iter().nth(0).unwrap();
    iced::widget::container(iced::widget::column![
        iced::widget::text("Recent Repositories"),
        iced::widget::pick_list(
            recent_repos,
            Some(first_option),
            |path| {
                let mut new_state = state.clone();
                new_state.repository_path = Some(path.clone());
                new_state.main_window = Some(apps::main_window::MainWindowState::new(path));
                globals::Message::NavigateTo(apps::main_window::APP_NAME.to_string(), new_state)
            }
        )
        .placeholder("/some/repo")
    ])
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .center_x()
    .center_y()
    .into()
}
