use std::cmp::min;

use git2::Repository;

use iced::{
    Element,
    Length,
    widget::{*, column},
};

use crate::git2_ext::{ExtRepo, ExtCommit};
use crate::{messages::app::Message, states::app::State, views::commit_details};

pub fn view<'a>(s: &'a State) -> Element<'a, Message> {
    let repo = Repository::open(s.selected_repo_path.clone().unwrap()).unwrap();
    let branch = repo.get_head_branch().unwrap();
    let commits = repo.get_commits(&branch).unwrap();
    let commits = commits[..min(commits.len(), 10)].to_vec(); // TODO find a way to paginate this!
    container(
        row![
            scrollable(
                column(
                    commits.into_iter().map(|commit| {
                        row![
                            text(
                                format!(
                                    "{} {} {}",
                                    commit.message().unwrap(),
                                    commit.author().to_string(),
                                    commit.get_short_id(),
                                )
                            ),
                            button(
                                text("details")
                            ).on_press(
                                Message::CommitSelected(commit.id())
                            ),
                        ].into()
                    }).collect()
                )
            ),
            column![
                match s.selected_commit {
                    Some(_) => commit_details::view(s),
                    _ => text("no commit selected").into(),
                }
            ],
        ],
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .into()
}