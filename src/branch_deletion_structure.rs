#[derive(PartialEq, Clone)]
pub struct BranchDeletionStructure {
    pub(crate) index: usize,
    pub(crate) branch_name: String,
    pub(crate) should_be_deleted: bool,
    pub(crate) is_checked_out: bool,
}
