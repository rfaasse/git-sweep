use gix::Repository;

pub trait Adapter {
    fn branch_names(&self) -> Vec<String>;
}

pub struct GixAdapter {
    pub(crate) repo: Repository,
}

impl Adapter for GixAdapter {
    fn branch_names(&self) -> Vec<String> {
        let mut result = Vec::new();
        for branch_name in self.repo.branch_names() {
            result.push(String::from(branch_name));
        }
        result
    }
}
