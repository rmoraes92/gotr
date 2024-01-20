use iced::{
    widget::{container, column, text, pick_list},
    Length,
    Element,
};

use crate::{messages::app::Message, states::app::State};

pub fn view<'a>(s: &'a State) -> Element<'a, Message> {
    container(
        column![
            text("Recent Repos"),
            // combo_box(
            //     &s.cbox_recent_repo_paths_state,
            //     "Select a repo",
            //     Some(&s.get_last_open_repo_path()),
            //     Message::RepositorySelected,
            // ),
            pick_list(
                s.repo_file_paths.clone(),
                Some(s.get_last_open_repo_path()),
                Message::RepositorySelected,
            ).placeholder("last opened repos")
        ],
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .into()
}