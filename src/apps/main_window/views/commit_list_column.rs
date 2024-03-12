use crate::git;
use crate::globals;
use crate::apps::main_window::message::Message as MainWindowMsg;
use crate::apps::main_window::state::MainWindowState;

pub fn view<'a>(state: MainWindowState) -> iced::Element<'a, globals::Message> {
    let repo_path: &String = state.repository_path.as_ref().unwrap();
    let repo: git2::Repository = git::open(repo_path);
    let branch: git2::Branch = git::get_head_branch(&repo).unwrap();
    let commits: Vec<git2::Commit> = git::get_commits(&repo, &branch).unwrap();

    let col = iced::widget::column(
        commits.into_iter().map(|commit| {
            iced::widget::column![
                iced::widget::row![
                    iced::widget::button(iced::widget::text("details")).on_press(
                        globals::Message::MainWindow(
                            MainWindowMsg::CommitSelected(commit.id())
                        )
                    ),
                    iced::widget::text(
                        format!(
                            "{} {}",
                            commit.message().unwrap(),
                            commit.author().to_string(),
                            // commit.get_short_id(),
                        )
                    ),
                    // horizontal_space(8),
                ],
                // vertical_space(8),
                // horizontal_rule(1),
            ].into()
        })
    );
    let ret = iced::widget::scrollable(col); // TODO how do I set this to horizontal ?

    ret.into()
}
