use std::{str::from_utf8, vec};

use git2::{Branch, BranchType, Commit, Delta, Diff, DiffDelta, DiffFile, DiffHunk, DiffLine, Object, Oid, Patch, Repository, Revwalk, Sort};

pub fn open<S: Into<String>>(path: S) -> Repository {
    Repository::open(path.into()).unwrap()
}

pub fn get_head_branch(repo: &Repository) -> Option<Branch> {
    let branches = repo.branches(
        Some(BranchType::Local)).unwrap();
    for item in branches {
        let (branch, _btype) = match item {
            Ok(t) => t,
            _ => continue,
        };
        if branch.is_head() {
            return Some(branch);
        }
    }
    None
}

pub fn get_commits<'a>(
    repo: &'a Repository,
    branch: &'a Branch<'a>,
) -> Option<Vec<Commit<'a>>> {
    let name = branch.name().unwrap().unwrap();
    let rev_path = format!("heads/{}", name);
    let obj: Object<'_> = repo.revparse_single(&rev_path).unwrap();
    let mut walker: Revwalk<'_> = repo.revwalk().unwrap();
    walker.set_sorting(Sort::TOPOLOGICAL).unwrap();
    walker.push(obj.id()).unwrap();
    let mut ret: Vec<Commit> = vec![];
    for (_, item) in walker.into_iter().enumerate() {
        let oid = item.unwrap();
        let commit = repo.find_commit(oid).unwrap();
        ret.push(commit);
    }
    return Some(ret);
}

pub fn get_commit<'a>(repo: &'a Repository, oid: &str) -> Commit<'a> {
    repo.find_commit(Oid::from_str(oid).unwrap()).unwrap()
}

pub fn get_parent_commit<'a>(commit: &'a Commit) -> Option<Commit<'a>> {
    commit.parent(0).ok()
}

pub fn get_diff<'a>(repo: &'a Repository, commit_prev: &'a Commit, commit_curr: &'a Commit) -> Option<Diff<'a>> {
    let old_tree = commit_prev.tree().unwrap();
    let new_tree = commit_curr.tree().unwrap();
    repo.diff_tree_to_tree(
        Some(&old_tree),
        Some(&new_tree),
        None,
    ).ok()
}

#[derive(Debug, Clone)]
pub struct MyDiffFileHunk {
    pub old_start_line: u32,
    pub new_start_line: u32,
    pub old_lines_count: u32,
    pub new_lines_count: u32,
    pub old_lines: Vec<String>,
    pub new_lines: Vec<String>,
}

impl MyDiffFileHunk {
    pub fn from_diffhunk(hunk: DiffHunk, lines: Vec<DiffLine>) -> Self {
        let mut ret = MyDiffFileHunk {
            old_start_line: hunk.old_start(),
            new_start_line: hunk.new_start(),
            old_lines_count: hunk.old_lines(),
            new_lines_count: hunk.new_lines(),
            old_lines: vec![],
            new_lines: vec![],
        };
        for diff_line in lines {
            let line_str = from_utf8(diff_line.content()).unwrap().to_string();
            if diff_line.new_lineno().is_some() {
                ret.new_lines.push(line_str)
            }
            else if diff_line.old_lineno().is_some() {
                ret.old_lines.push(line_str)
            }
        }
        return ret;
    }
    pub fn truncate_old_lines(&self) -> String {
        self.old_lines.join("\n")
    }
    pub fn truncate_new_lines(&self) -> String {
        self.new_lines.join("\n")
    }
}

#[derive(Debug, Clone)]
pub struct  MyDiffFile {
    pub from: Option<String>,
    pub to: Option<String>,
    pub status: String,
    pub hunks: Vec<MyDiffFileHunk>,
}

impl MyDiffFile {
    pub fn from_diff_delta(delta: DiffDelta) -> Self {
        let old_file: git2::DiffFile<'_> = delta.old_file();
        let new_file: git2::DiffFile<'_> = delta.new_file();
        let old_file_path_str: Option<String> = match old_file.path() {
            Some(path) => Some(path.display().to_string()),
            None => None,
        };
        let new_file_path_str: Option<String> = match new_file.path() {
            Some(path) => Some(path.display().to_string()),
            None => None,
        };
        Self {
            from: old_file_path_str,
            to: new_file_path_str,
            status: delta_status_to_string(delta.status()),
            hunks: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct MyDiff {
    pub entries: Vec<MyDiffFile>,
}

impl MyDiff {
    pub fn from_diff(diff: &Diff) -> Self {
        let mut ret = Self{entries: vec![]};
        for (i, delta) in diff.deltas().enumerate() {
            println!("delta[{}]", i);
            let patch = Patch::from_diff(&diff, i).unwrap().unwrap();
            // assert_eq!(delta.status(), patch.delta().status());
            let mut my_file = MyDiffFile::from_diff_delta(delta);
            for hunk_idx in 0..patch.num_hunks() {
                let (hunk, _) = patch.hunk(hunk_idx).unwrap();
                let lines = get_diff_lines_from_hunk(&patch, hunk_idx);
                my_file.hunks.push(
                    MyDiffFileHunk::from_diffhunk(hunk, lines)
                );
            }
            ret.entries.push(my_file);
        }
        return ret;
    }
}

pub fn get_diff_lines_from_hunk<'a>(patch: &'a Patch, hunk_idx: usize) -> Vec<DiffLine<'a>> {
    (0..patch.num_lines_in_hunk(hunk_idx).unwrap()).into_iter().map(|line_of_hunk| {
        patch.line_in_hunk(hunk_idx, line_of_hunk).unwrap()
    }).collect()
}

pub fn delta_status_to_string(delta: Delta) -> String {
    match delta {
        Delta::Added => "+",
        Delta::Deleted => "-",
        Delta::Conflicted => "x",
        Delta::Copied => "c",
        Delta::Ignored => "i",
        Delta::Modified => "~",
        Delta::Renamed => "r",
        Delta::Typechange => "t",
        Delta::Unmodified => "u",
        Delta::Unreadable => "?",
        Delta::Untracked => "t"
    }.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2;

    #[test]
    fn differ() {
        let repo_path: &str = ".";
        let repo: git2::Repository = open(repo_path);
        let commit: git2::Commit = get_commit(&repo, "57667d2ccc657021dd330d70400321cc8d7f0053");
        let parent_commit = get_parent_commit(&commit).unwrap();
        let diff = get_diff(&repo, &parent_commit, &commit).unwrap();
        let patch1: Patch = Patch::from_diff(&diff, 0).unwrap().unwrap();
        println!("hunks {}", patch1.num_hunks());
        let my_diff: MyDiff = MyDiff::from_diff(&diff);
        assert_eq!("bar", "bar");
    }
}
