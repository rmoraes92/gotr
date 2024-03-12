use iced;
use git2;
use crate::git;
use crate::globals;
use crate::apps::main_window::state::MainWindowState;

pub fn view<'a>(state: MainWindowState) -> iced::Element<'a, globals::Message> {
    let commit_oid: String = match state.commit_details_oid {
        Some(s) => s,
        None => {
            return iced::widget::text("no commit selected yet!").into();
        }
    };
    let repo_path: &String = state.repository_path.as_ref().unwrap();
    let repo: git2::Repository = git::open(repo_path);
    let commit: git2::Commit = git::get_commit(
        &repo, &commit_oid);
    let parent_commit = match git::get_parent_commit(&commit) {
        Some(c) => c,
        None => {
            return iced::widget::text("detailing first push is not implemented yet").into();
        }
    };
    let diff = git::get_diff(&repo, &parent_commit, &commit).unwrap();

    for delta in diff.deltas() {
        println!("consuming delta");
    }

    iced::widget::text("Dude At Work!").into()
}
