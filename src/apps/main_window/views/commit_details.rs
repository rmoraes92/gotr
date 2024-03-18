use iced;
use git2;
use crate::git;
use crate::git::MyDiffFile;
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
    let my_diff = git::MyDiff::from_diff(&diff);

    // https://github.com/iced-rs/iced/issues/2293

    let children: Vec<iced::Element<'a, globals::Message>> = my_diff.entries.into_iter().map(|my_diffentry|{
        diff_header_view(my_diffentry)
    }).collect();

    return iced::widget::column(children).into();

    //for entries in MyDiff::from_diff(&diff).entries {
    //    println!("consuming delta");
    //}

    // iced::widget::text("Dude At Work!").into()
}

pub fn diff_header_view<'a>(entry: git::MyDiffFile) -> iced::Element<'a, globals::Message> {
    let hunks: Vec<iced::Element<'a, globals::Message>> = entry.hunks.into_iter().map(|hunk| {
        diff_hunk_view(hunk)
    }).collect();
    iced::widget::column![
        iced::widget::row![
            iced::widget::text(entry.status),
            iced::widget::text(entry.from.unwrap_or(String::from("<no_old_file>"))),
            iced::widget::text(entry.to.unwrap_or(String::from("<no_new_file>"))),
        ],
        iced::widget::row(hunks),
    ].into()
}

pub fn diff_hunk_view<'a>(hunk: git::MyDiffFileHunk) -> iced::Element<'a, globals::Message> {
    iced::widget::row![
        iced::widget::text(hunk.truncate_old_lines()).font(iced::Font::MONOSPACE),
        iced::widget::text(hunk.truncate_new_lines()).font(iced::Font::MONOSPACE),
    ].into()
}
