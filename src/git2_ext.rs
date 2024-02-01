use std::{path::Path, str::from_utf8};

use git2::{
    Branch, BranchType, Commit, Delta, Diff, DiffDelta, DiffLineType, IndexMatchedPath, Object, Repository as Repo, Revwalk, Sort
};
use anyhow::{bail, Result};

pub trait ExtRepo {
    fn get_diff_head(&self) -> Diff;
    fn get_head_branch(&self) -> Result<Branch>;
    fn get_commits(&self, branch: &Branch) -> Result<Vec<Commit>>;
    fn diff_commit_to_commit(&self, old_commit: &Commit, new_commit: &Commit) -> Option<Diff>;
    fn diff_commit_to_immediate_parent(&self, new_commit: &Commit) -> Option<Diff>;
    fn stage_file(&self, file_path: String);
    fn unstage_file(&self, file_path: String);
}

impl ExtRepo for Repo {
    fn get_diff_head(&self) -> Diff {
        let rev_path = "HEAD^{tree}";
        let obj = self.revparse_single(&rev_path).unwrap();
        let tree = self.find_tree(obj.id()).unwrap();
        let diff = self.diff_tree_to_workdir_with_index(Some(&tree), None).unwrap();
        return diff;
    }
    fn stage_file(&self, file_path: String) {
        // git_index_update_all
        let mut idx = self.index().unwrap();
        let matcher: &mut IndexMatchedPath = &mut |path: &Path, _matched_spec: &[u8]| -> i32 {
            println!("trying to add: {}", path.display());
            let status = self.status_file(path).unwrap();
            dbg!(status);
            if status.contains(git2::Status::WT_MODIFIED) || status.contains(git2::Status::WT_NEW) {
                println!("add '{}'", path.display());
                return 0;
            } else {
                return 1;
            };
        };
        // idx.update_all(vec![file_path], Some(matcher)).unwrap();
        idx.add_all(vec![file_path], git2::IndexAddOption::DEFAULT, Some(matcher)).unwrap();
        idx.write().unwrap();
    }
    fn unstage_file(&self, file_path: String) {
        let mut idx = self.index().unwrap();
        let matcher: &mut IndexMatchedPath = &mut |path: &Path, _matched_spec: &[u8]| -> i32 {
            println!("trying to unstage: {}", path.display());
            let status = self.status_file(path).unwrap();
            dbg!(status);
            if status.contains(git2::Status::WT_MODIFIED) || status.contains(git2::Status::WT_NEW) {
                println!("unstage '{}'", path.display());
                return 0;
            } else {
                return 1;
            };
        };
        // idx.update_all(vec![file_path], Some(matcher)).unwrap();
        idx.remove_all(vec![file_path], Some(matcher)).unwrap();
        idx.write().unwrap();
    }
    fn get_head_branch(&self) -> Result<Branch> {
        let branches = self.branches(Some(BranchType::Local))?;
        for item in branches {
            let (branch, _btype) = match item {
                Ok(t) => t,
                _ => continue,
            };
            if branch.is_head() {
                return Ok(branch);
            }
        }
        bail!("no branch selected to head");
    }
    fn get_commits(&self, branch: &Branch) -> Result<Vec<Commit>> {
        let name = branch.name()?.unwrap();
        let rev_path = format!("heads/{}", name);
        let obj: Object<'_> = self.revparse_single(&rev_path)?;
        let mut walker: Revwalk<'_> = self.revwalk().unwrap();
        walker.set_sorting(Sort::TOPOLOGICAL)?;
        walker.push(obj.id())?;
        let mut ret: Vec<Commit> = vec![];
        for (_, item) in walker.into_iter().enumerate() {
            let oid = item?;
            let commit = self.find_commit(oid)?;
            ret.push(commit);
        }
        return Ok(ret);
    }
    fn diff_commit_to_commit(&self, old_commit: &Commit, new_commit: &Commit) -> Option<Diff> {
        let old_tree = old_commit.tree().unwrap();
        let new_tree = new_commit.tree().unwrap();
        self.diff_tree_to_tree(
            Some(&old_tree),
            Some(&new_tree),
            None,
        ).ok()
    }
    fn diff_commit_to_immediate_parent(&self, new_commit: &Commit) -> Option<Diff> {
        let old_tree = new_commit.parent(0).unwrap().tree().unwrap();
        let new_tree = new_commit.tree().unwrap();
        self.diff_tree_to_tree(
            Some(&old_tree),
            Some(&new_tree),
            None,
        ).ok()
    }
}

pub trait ExtCommit {
    fn get_short_id(&self) -> String;
    fn get_immediate_parent(&self) -> Option<Commit>;
}

impl ExtCommit for Commit<'_> {
    fn get_short_id(&self) -> String {
        self.id().to_string()[0..8].to_string()
    }
    fn get_immediate_parent(&self) -> Option<Commit> {
        self.parent(0).ok()
    }
}

#[derive(Debug, Clone)]
pub struct EasyFileDiff {
    pub header: String,
    pub old_file_path: Option<String>,
    pub new_file_path: Option<String>,
    pub old_lines: Vec<(u32, String, String)>,
    pub new_lines: Vec<(u32, String, String)>,
    pub status: Option<Delta>,
}

impl EasyFileDiff {
    pub fn default() -> Self {
        Self {
            header: String::from(""),
            old_file_path: None,
            new_file_path: None,
            old_lines: vec![],
            new_lines: vec![],
            status: None,
        }
    }
    pub fn is_staged(&self, staged_deltas: &Vec<DiffDelta>) -> bool {
        for d_delta in staged_deltas {
            if self.new_file_path.unwrap() == d_delta.new_file().path().unwrap().display().to_string() {
                return true;
            }
        }
        return false;
    }
    pub fn truncate_all_new_lines(&self) -> String {
        let mut ret = String::from("");
        for (n, m, t) in &self.new_lines {
            ret.push_str(format!("{} {} {}",
                n,
                m,
                t,
            ).as_str());
        }
        return ret.clone();
    }
    pub fn truncate_all_old_lines(&self) -> String {
        let mut ret = String::from("");
        for (n, m, t) in &self.old_lines {
            ret.push_str(format!("{} {} {}",
                n,
                m,
                t,
            ).as_str());
        }
        return ret.clone();
    }
}

pub trait ExtDiff {
    fn get_easy_vec(&self) -> Vec<EasyFileDiff>;
}

impl ExtDiff for Diff<'_> {
    fn get_easy_vec(&self) -> Vec<EasyFileDiff> {
        // TODO this looks atrocious
        let mut ediff_vec: Vec<EasyFileDiff> = vec![];  // TODO this needs to be a vector
        let mut ediff_buffer: EasyFileDiff = EasyFileDiff::default();
    
        let _ = self.print(git2::DiffFormat::Patch, |d_delta, d_hunk, d_line| {
            //dbg!(&d_delta);
            //dbg!(&d_hunk);
            //dbg!(&d_line);
            //println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
            match d_line.origin_value() {
                DiffLineType::Context => {
                    // println!("> Context");
                    let ctx = from_utf8(d_line.content()).unwrap().to_string();
                    let no = d_line.new_lineno().unwrap();
                    ediff_buffer.old_lines.push((no, "".to_string(), ctx.clone()));
                    ediff_buffer.new_lines.push((no, "".to_string(), ctx.clone()));
                },
                DiffLineType::Addition => {
                    // println!("> Addition");
                    let ctx = from_utf8(d_line.content()).unwrap().to_string();
                    match d_line.new_lineno() {
                        Some(no) => ediff_buffer.new_lines.push((no, "+".to_string(), ctx.clone())),
                        None => (), // println!("no addition on new file"),
                    };
                    match d_line.old_lineno() {
                        Some(no) => ediff_buffer.old_lines.push((no, "+".to_string(), ctx.clone())),
                        None => (), // println!("no addition on old file"),
                    };
                },
                DiffLineType::Deletion => {
                    // println!("> Deletion");
                    let ctx = from_utf8(d_line.content()).unwrap().to_string();
                    match d_line.new_lineno() {
                        Some(no) => ediff_buffer.new_lines.push((no, "-".to_string(), ctx.clone())),
                        None => (), // println!("no deletion on new file"),
                    };
                    match d_line.old_lineno() {
                        Some(no) => ediff_buffer.old_lines.push((no, "-".to_string(), ctx.clone())),
                        None => (), // println!("no deletion on old file"),
                    };
                },
                DiffLineType::ContextEOFNL => {
                    // println!("ContextEOFNL - no idea what to do with this");
                },
                DiffLineType::AddEOFNL => {
                    // println!("> AddEOFNL");
                    // TODO this does NOT generate any entries. We might as well skip it all time?
                    let ctx = from_utf8(d_line.content()).unwrap().to_string();
                    match d_line.new_lineno() {
                        Some(no) => ediff_buffer.new_lines.push((no, "x".to_string(), ctx.clone())),
                        None => (),
                    };
                    match d_line.old_lineno() {
                        Some(no) => ediff_buffer.old_lines.push((no, "x".to_string(), ctx.clone())),
                        None => (),
                    };
                },
                DiffLineType::DeleteEOFNL => {
                    // println!("DeleteEOFNL - no idea what to do with this")
                },
                DiffLineType::FileHeader => {
                    // println!("> FileHeader");
                    let header = from_utf8(d_line.content()).unwrap().to_string();
                    if ediff_buffer.header.len() > 0 {
                        ediff_vec.push(ediff_buffer.clone());
                        ediff_buffer = EasyFileDiff::default();
                    }
                    ediff_buffer.header = header;
                    // We are going to use this later on the stage are view
                    ediff_buffer.new_file_path = match d_delta.new_file().path() {
                        Some(path) => Some(path.display().to_string()),
                        None => None,
                    };
                    ediff_buffer.old_file_path = match d_delta.old_file().path() {
                        Some(path) => Some(path.display().to_string()),
                        None => None,
                    };
                    ediff_buffer.status = Some(d_delta.status());
                },
                DiffLineType::HunkHeader => {
                    // TODO create a sub-strutct called EasyFileDiffHunk
                    // TODO make EasyFileDiffHunk actually hold old_lines and new_lines
                },
                DiffLineType::Binary => todo!(),
            };
            true
        });
        //if ediff_vec.len() == 0 {
            ediff_vec.push(ediff_buffer);
        //}
        return ediff_vec;
    }
}
