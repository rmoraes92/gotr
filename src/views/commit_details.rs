use git2::Repository;
use iced::{Font, Length};
use iced::font::Weight;
use iced::{Element, widget::text};
use iced::widget::{column, row, scrollable, space, vertical_space};

use crate::git2_ext::{ExtRepo, ExtCommit, ExtDiff};
use crate::{states::app::State, messages::app::Message};

pub fn view<'a>(s: &'a State) -> Element<'a, Message> {
    let repo = Repository::open(s.selected_repo_path.clone().unwrap()).unwrap();
    let commit = repo.find_commit(s.selected_commit.unwrap()).unwrap();
    let parent_commit = commit.get_immediate_parent().unwrap();
    let diff = repo.diff_commit_to_commit(&parent_commit,&commit).unwrap();
    let easy_file_diff = diff.get_easy_vec();

    // Widgets
    // -------
    let mut font_bold = Font::default();
    font_bold.weight = Weight::Bold;
    column![
        row![
            text("Commit Hash: ").font(font_bold),
            text(format!("{}", commit.id())),
        ],
        row![
            text("Author: ").font(font_bold),
            text(format!("{}", commit.author())),
        ],
        row![
            text("Stats: ").font(font_bold),
            text(format!("{} files changed.", diff.deltas().map(|diffdelta| diffdelta.nfiles()).sum::<u16>())),
        ],
        row![
            text(format!("\n{}\n", commit.message().unwrap())).font(Font::MONOSPACE),
        ],
        scrollable(
            column(
                easy_file_diff.into_iter().map(|efd| {
                    column![
                        text(efd.header.clone()).font(font_bold),
                        row![
                            text(efd.truncate_all_old_lines()).font(Font::MONOSPACE),
                            text(efd.truncate_all_new_lines()).font(Font::MONOSPACE),
                        ],
                        vertical_space(32),
                    ].into()
                }).collect()
            )
        )
    ].into()
}