use git2::{Branch, BranchType, Commit, Object, Repository, Revwalk, Sort};

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

pub fn get_commits<'a>(repo: &'a Repository, branch: &'a Branch<'a>) -> Option<Vec<Commit<'a>>> {
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