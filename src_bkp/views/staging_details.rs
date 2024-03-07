// use std::path::Path;

use git2::{
    // Branch, Delta, DiffDelta, 
    Repository, StatusOptions};
use iced::{font::Weight, widget::{button, column, container, horizontal_space, row, scrollable, text, vertical_space}, Element, Font};
use crate::views::logs_column;
use crate::{git2_ext::{ExtDiff, ExtRepo}, messages::app::Message, states::app::State};

pub const USE_PANE_GRID: bool = true;

pub fn view<'a>(s: &'a State) -> Element<'a, Message> {
    let repo = Repository::open(s.selected_repo_path.clone().unwrap()).unwrap();
    let diff = repo.get_diff_head();
    let ediff_vec = diff.get_easy_vec();
    let mut font_bold = Font::default();
    font_bold.weight = Weight::Bold;

    let mut opts = StatusOptions::new();
    opts.include_untracked(false);
    let _statuses = repo.statuses(Some(&mut opts)).unwrap();

    // let x: Vec<String> = statuses.into_iter()
    //     .filter(|se: &git2::StatusEntry<'_>| se.status() != git2::Status::CURRENT)
    //     .map(|se: git2::StatusEntry<'_>| {
    //         match se.head_to_index() {
    //             Some(ddelta) => {
    //                 return ddelta.new_file().path().unwrap().display().to_string()
    //             },
    //             None => String::from("???"),
    //         }
    //     }).collect();

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
                                if true {  // TODO this is
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
