use gix::{Reference, Repository};

pub trait Adapter {
    fn branch_names(&self) -> Vec<String>;
    fn is_checked_out(&self, branch_name: &str) -> bool;
}

pub struct GixAdapter {
    pub(crate) repo: Repository,
}

impl Adapter for GixAdapter {
    fn branch_names(&self) -> Vec<String> {
        self.repo
            .references()
            .expect("")
            .prefixed("refs/heads")
            .expect("") // only local branches
            .filter_map(Result::ok) // ignore errors
            .map(|reference: Reference| reference.name().shorten().to_string()) // extract just the names
            .collect()
    }

    fn is_checked_out(&self, branch_name: &str) -> bool {
        let head = self.repo.head().expect("Could not get HEAD reference");
        if head.is_detached() {
            return false;
        }
        let branch_refname = format!("refs/heads/{branch_name}");
        if head.referent_name().unwrap().to_string() == branch_refname {
            return true;
        }
        false
    }
}

impl GixAdapter {
    pub fn delete_branch(&self, branch_name: &str) {
        // Local branches live under refs/heads/<name>
        let refname = format!("refs/heads/{branch_name}");
        let reference = self
            .repo
            .find_reference(&refname)
            .expect("Could not find reference for branch");
        reference.delete().expect("Could not remove branch"); // deletes the branch
    }
}
