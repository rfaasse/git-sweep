use gix::{Reference, Repository};

pub trait Adapter {
    fn branch_names(&self) -> Vec<String>;
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
