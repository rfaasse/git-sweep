use crate::branch_data::BranchData;
use crate::gix_adapter::Adapter;
pub(crate) fn create_branch_structure(adapter: &dyn Adapter) -> Vec<BranchData> {
    let branch_names = adapter.branch_names();
    branch_names
        .into_iter()
        .filter(|name| !adapter.is_checked_out(&name))
        .filter(|name| name != "main" && name != "master")
        .enumerate()
        .map(|(i, name)| BranchData {
            index: i + 1,
            is_checked_out: adapter.is_checked_out(&name),
            name,
            should_be_deleted: false,
        })
        .collect()
}

pub(crate) fn toggle_branch_deletion_status(
    branch_structure: &mut [BranchData],
    index: usize,
) {
    if let Some(branch) = branch_structure
        .iter_mut()
        .find(|b| b.index == index)
        .filter(|b| b.name != "main")
        .filter(|b| b.name != "master")
        .filter(|b| !b.is_checked_out)
    {
        branch.should_be_deleted = !branch.should_be_deleted;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockGixAdapter {}

    impl Adapter for MockGixAdapter {
        fn branch_names(&self) -> Vec<String> {
            Vec::from(["branch_1".to_string(), "branch_2".to_string()])
        }
        fn is_checked_out(&self, _branch_name: &str) -> bool {
            false
        }
    }

    #[test]
    fn test_create_branch_structure() {
        let mock_adapter = MockGixAdapter {};

        let mut expected_branch_structure: Vec<BranchData> = Vec::new();
        expected_branch_structure.push(BranchData {
            index: 1,
            name: "branch_1".to_string(),
            should_be_deleted: false,
            is_checked_out: false,
        });
        expected_branch_structure.push(BranchData {
            index: 2,
            name: "branch_2".to_string(),
            should_be_deleted: false,
            is_checked_out: false,
        });

        let actual_branch_structure = create_branch_structure(&mock_adapter);
        let matching = expected_branch_structure
            .iter()
            .zip(&actual_branch_structure)
            .filter(|&(a, b)| a == b)
            .count();
        assert_eq!(matching, expected_branch_structure.len());
    }

    #[test]
    fn test_toggle_branch_deletion_status() {
        let mut input = vec![
            BranchData {
                index: 1,
                name: "branch_1".to_string(),
                should_be_deleted: false,
                is_checked_out: false,
            },
            BranchData {
                index: 2,
                name: "branch_2".to_string(),
                should_be_deleted: true,
                is_checked_out: false,
            },
        ];

        toggle_branch_deletion_status(&mut input, 2);

        assert_eq!(input[0].should_be_deleted, false);
        assert_eq!(input[1].should_be_deleted, false);
    }

    fn create_branch_deletion_structure(
        name: &str,
        is_checked_out: bool,
    ) -> Vec<BranchData> {
        vec![BranchData {
            index: 1,
            name: name.to_string(),
            should_be_deleted: false,
            is_checked_out,
        }]
    }

    #[test]
    fn test_toggle_branch_deletion_status_when_branch_is_checked_out_should_not_change_state() {
        let name = "branch_1";
        let is_checked_out = true;
        let mut input = create_branch_deletion_structure(name, is_checked_out);

        toggle_branch_deletion_status(&mut input, 1);

        assert_eq!(input[0].should_be_deleted, false);
    }

    #[test]
    fn test_toggle_branch_deletion_status_when_branch_is_main_should_not_change_state() {
        let name = "main";
        let is_checked_out = false;
        let mut input = create_branch_deletion_structure(name, is_checked_out);

        toggle_branch_deletion_status(&mut input, 1);

        assert_eq!(input[0].should_be_deleted, false);
    }

    #[test]
    fn test_toggle_branch_deletion_status_when_branch_is_master_should_not_change_state() {
        let name = "master";
        let is_checked_out = false;
        let mut input = create_branch_deletion_structure(name, is_checked_out);

        toggle_branch_deletion_status(&mut input, 1);

        assert_eq!(input[0].should_be_deleted, false);
    }
}
