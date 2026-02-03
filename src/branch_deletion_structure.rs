#[derive(PartialEq)]
pub struct BranchDeletionStructure
{
    pub(crate) index: i32,
    pub(crate) branch_name: String,
    pub(crate) should_be_deleted: bool,
}