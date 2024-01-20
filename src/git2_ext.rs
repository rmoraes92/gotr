use std::str::from_utf8;

use git2::{
    Repository as Repo,
    Branch,
    Commit,
    Object,
    Revwalk,
    Sort,
    BranchType,
    Diff, DiffLineType,
};
use anyhow::{bail, Result};

pub trait ExtRepo {
    fn get_head_branch(&self) -> Result<Branch>;
    fn get_commits(&self, branch: &Branch) -> Result<Vec<Commit>>;
    fn diff_commit_to_commit(&self, old_commit: &Commit, new_commit: &Commit) -> Option<Diff>;
    fn diff_commit_to_immediate_parent(&self, new_commit: &Commit) -> Option<Diff>;
}

impl ExtRepo for Repo {
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
    pub old_lines: Vec<(u32, String, String)>,
    pub new_lines: Vec<(u32, String, String)>,
}

impl EasyFileDiff {
    pub fn default() -> Self {
        Self {
            header: String::from(""),
            old_lines: vec![],
            new_lines: vec![],
        }
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
        let mut ediff_vec: Vec<EasyFileDiff> = vec![];  // TODO this needs to be a vector
        let mut ediff_buffer: EasyFileDiff = EasyFileDiff::default();
    
        let _ = self.print(git2::DiffFormat::Patch, |_d, _h, l| {    
            match l.origin_value() {
                DiffLineType::Context => {
                    let ctx = from_utf8(l.content()).unwrap().to_string();
                    let no = l.new_lineno().unwrap();
                    ediff_buffer.old_lines.push((no, "".to_string(), ctx.clone()));
                    ediff_buffer.new_lines.push((no, "".to_string(), ctx.clone()));
                },
                DiffLineType::Addition => {
                    let ctx = from_utf8(l.content()).unwrap().to_string();
                    match l.new_lineno() {
                        Some(no) => ediff_buffer.new_lines.push((no, "+".to_string(), ctx.clone())),
                        None => (),
                    };
                    match l.old_lineno() {
                        Some(no) => ediff_buffer.old_lines.push((no, "+".to_string(), ctx.clone())),
                        None => (),
                    };
                },
                DiffLineType::Deletion => {
                    let ctx = from_utf8(l.content()).unwrap().to_string();
                    match l.new_lineno() {
                        Some(no) => ediff_buffer.new_lines.push((no, "-".to_string(), ctx.clone())),
                        None => (),
                    };
                    match l.old_lineno() {
                        Some(no) => ediff_buffer.old_lines.push((no, "-".to_string(), ctx.clone())),
                        None => (),
                    };
                },
                DiffLineType::ContextEOFNL => todo!(),
                DiffLineType::AddEOFNL => {
                    // TODO this does NOT generate any entries. We might as well skip it all time?
                    let ctx = from_utf8(l.content()).unwrap().to_string();
                    match l.new_lineno() {
                        Some(no) => ediff_buffer.new_lines.push((no, "x".to_string(), ctx.clone())),
                        None => (),
                    };
                    match l.old_lineno() {
                        Some(no) => ediff_buffer.old_lines.push((no, "x".to_string(), ctx.clone())),
                        None => (),
                    };
                },
                DiffLineType::DeleteEOFNL => todo!(),
                DiffLineType::FileHeader => {
                    if ediff_buffer.header.len() > 0 {
                        ediff_vec.push(ediff_buffer.clone());
                        ediff_buffer = EasyFileDiff::default();
                    }
                    ediff_buffer.header = from_utf8(l.content()).unwrap().to_string()
                },
                DiffLineType::HunkHeader => {
                    // TODO create a sub-strutct called EasyFileDiffHunk
                    // TODO make EasyFileDiffHunk actually hold old_lines and new_lines
                },
                DiffLineType::Binary => todo!(),
            };
            true
        });
        return ediff_vec;
    }
}