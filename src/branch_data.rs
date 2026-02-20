#[derive(PartialEq, Clone)]
pub struct BranchData {
    pub(crate) index: usize,
    pub(crate) name: String,
    pub(crate) should_be_deleted: bool,
    pub(crate) is_checked_out: bool,
}
