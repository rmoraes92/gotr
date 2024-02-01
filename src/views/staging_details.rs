use std::path::Path;

use git2::{Branch, Delta, DiffDelta, Repository, StatusOptions};
use iced::{font::Weight, widget::{button, column, container, horizontal_space, row, scrollable, text, vertical_space}, Element, Font};
use crate::views::logs_column;
use crate::{git2_ext::{ExtDiff, ExtRepo}, messages::app::Message, states::app::State};

pub fn view<'a>(s: &'a State) -> Element<'a, Message> {
    let repo = Repository::open(s.selected_repo_path.clone().unwrap()).unwrap();
    let diff = repo.get_diff_head();
    let ediff_vec = diff.get_easy_vec();
    let mut font_bold = Font::default();
    font_bold.weight = Weight::Bold;

    let mut opts = StatusOptions::new();
    opts.include_untracked(false);
    let statuses = repo.statuses(Some(&mut opts));

    let staged_deltas: Vec<DiffDelta> = statuses.unwrap().iter()
        .filter(|se| se.status() != git2::Status::CURRENT)
        .map(|se| se.head_to_index().unwrap()).collect();

    row![
        logs_column::view(s),
        scrollable(
            column(
                ediff_vec.into_iter().map(|ediff| {
                    let file_path = ediff.new_file_path.unwrap();
                    // let status = repo.status_file(&Path::new(&file_path)).unwrap();
                    // println!("rendering diff = {:?}", ediff.header.clone());
                    // println!("file status = {:?}", &status);
                    container(
                        column![
                            row![
                                if ediff.is_staged(&staged_deltas) {  // TODO this is
                                                                                 // not working as
                                                                                 // intended
                                    button(text("stage")).on_press(Message::StageFile(file_path))
                                } else {
                                    button(text("unstage")).on_press(Message::UnstageFile(file_path))
                                },
                                horizontal_space(16),
                                text(ediff.header.clone()).font(font_bold),
                            ],
                            vertical_space(32),
                        ]
                    )
                    .into()
                }).collect()
            )
        )
    ].into()
}

/*
git diff HEAD.

git_object *obj = NULL;
int error = git_revparse_single(&obj, repo, "HEAD^{tree}");

git_tree *tree = NULL;
error = git_tree_lookup(&tree, repo, git_object_id(obj));

git_diff *diff = NULL;
error = git_diff_tree_to_workdir_with_index(&diff, repo, tree, NULL);

*/
