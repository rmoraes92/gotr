
use std::str::from_utf8;

use git2::{Repository, Oid, DiffLineType};
use gotr::git2_ext::*;

#[derive(Debug, Clone)]
struct EasyFileDiff {
    header: String,
    old_lines: Vec<(u32, String)>,
    new_lines: Vec<(u32, String)>,
}

impl EasyFileDiff {
    fn default() -> Self {
        Self {
            header: String::from(""),
            old_lines: vec![],
            new_lines: vec![],
        }
    }
}

fn main() {
    let repo = Repository::open("/home/ghost/Projects/iced").unwrap();
    let commit = repo.find_commit(Oid::from_str("c4ba657de86d7606587dad5124f435141258f570").unwrap()).unwrap();
    // let parent_commit = commit.get_immediate_parent().unwrap();
    // let _diff = repo.diff_commit_to_commit(&parent_commit,&commit).unwrap();
    let diff2 = repo.diff_commit_to_immediate_parent(&commit).unwrap();

    let mut ediff_vec: Vec<EasyFileDiff> = vec![];  // TODO this needs to be a vector
    let mut ediff_buffer: EasyFileDiff = EasyFileDiff::default();

    let _ = diff2.print(git2::DiffFormat::Patch, |d, h, l| {
        let x = match l.origin_value() {
            DiffLineType::Context => "Context",
            DiffLineType::Addition => "Addition",
            DiffLineType::Deletion => "Deletion",
            DiffLineType::ContextEOFNL => "ContextEOFNL",
            DiffLineType::AddEOFNL => "AddEOFNL",
            DiffLineType::DeleteEOFNL => "DeleteEOFNL",
            DiffLineType::FileHeader =>"FileHeader",
            DiffLineType::HunkHeader => "HunkHeader",
            DiffLineType::Binary => "Binary",
        };
        println!(">>> processing [{}]", x);

        match l.origin_value() {
            DiffLineType::Context => {
                let ctx = from_utf8(l.content()).unwrap().to_string();
                let no = l.new_lineno().unwrap();
                ediff_buffer.old_lines.push((no, ctx.clone()));
                ediff_buffer.new_lines.push((no, ctx.clone()));
            },
            DiffLineType::Addition => {
                let ctx = from_utf8(l.content()).unwrap().to_string();
                match l.new_lineno() {
                    Some(no) => ediff_buffer.new_lines.push((no, ctx.clone())),
                    None => (),
                };
                match l.old_lineno() {
                    Some(no) => ediff_buffer.old_lines.push((no, ctx.clone())),
                    None => (),
                };
            },
            DiffLineType::Deletion => {
                let ctx = from_utf8(l.content()).unwrap().to_string();
                match l.new_lineno() {
                    Some(no) => ediff_buffer.new_lines.push((no, ctx.clone())),
                    None => (),
                };
                match l.old_lineno() {
                    Some(no) => ediff_buffer.old_lines.push((no, ctx.clone())),
                    None => (),
                };
            },
            DiffLineType::ContextEOFNL => todo!(),
            DiffLineType::AddEOFNL => todo!(),
            DiffLineType::DeleteEOFNL => todo!(),
            DiffLineType::FileHeader => {
                if ediff_buffer.header.len() > 0 {
                    ediff_vec.push(ediff_buffer.clone());
                    ediff_buffer = EasyFileDiff::default();
                }
                ediff_buffer.header = from_utf8(l.content()).unwrap().to_string()
            },
            DiffLineType::HunkHeader => {
                // if ediff_buffer.header.len() > 0 {
                //     ediff_vec.push(ediff_buffer.clone());
                //     ediff_buffer = EasyDiff::default();
                // }
                // ediff_buffer.header = from_utf8(l.content()).unwrap().to_string()
            },
            DiffLineType::Binary => todo!(),
        };
        true
    });
    dbg!(&ediff_vec);
}
