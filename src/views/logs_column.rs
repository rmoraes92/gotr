use std::cmp::min;

use git2::Repository;
use iced::widget::pane_grid::{self, Pane};
use iced::widget::rule::FillMode;
use iced::widget::text::LineHeight;
use iced::Element;
use iced::widget::{button, column, horizontal_rule, horizontal_space, row, text, vertical_space, PaneGrid};
use crate::git2_ext::{ExtCommit, ExtRepo};
use crate::{messages::app::Message, states::app::State};


pub fn view<'a>(s: &'a State) -> Element<'a, Message> {
    let repo = Repository::open(s.selected_repo_path.clone().unwrap()).unwrap();
    let branch = repo.get_head_branch().unwrap();
    let commits = repo.get_commits(&branch).unwrap();
    let commits = commits[..min(commits.len(), 10)].to_vec(); // TODO find a way to paginate this!
    let unstaged_files = -1;

    // TODO convert this to PaneGrid at some point

    column![
        column![
            row![
                button(text("details")).on_press(Message::ShowHEADSummary),
                horizontal_space(8),
                column![
                    text(format!("{} unstaged files", unstaged_files)),
                    text("Commit Changes"),
                ],
            ],
            vertical_space(8),
        ],
        column(
            commits.into_iter().map(|commit| {
                column![
                    row![
                        button(text("details")).on_press(
                            Message::CommitSelected(commit.id())
                        ),
                        horizontal_space(8),
                        text(
                            format!(
                                "{} {} {}",
                                commit.message().unwrap(),
                                commit.author().to_string(),
                                commit.get_short_id(),
                            )
                        ),
                    ],
                    vertical_space(8),
                    // horizontal_rule(1),
                ].into()
            }).collect()
        ),
    ].into()
}