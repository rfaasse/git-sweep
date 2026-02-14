use crate::branch_deletion_structure::BranchDeletionStructure;
use crate::gix_adapter::Adapter;
pub(crate) fn create_branch_structure(adapter: &dyn Adapter) -> Vec<BranchDeletionStructure> {
    let branch_names = adapter.branch_names();
    branch_names
        .into_iter()
        .enumerate()
        .map(|(i, branch_name)| BranchDeletionStructure {
            index: i + 1,
            is_checked_out: adapter.is_checked_out(&branch_name),
            name: branch_name,
            should_be_deleted: false,
        })
        .collect()
}

pub(crate) fn toggle_branch_deletion_status(
    branch_structure: &mut [BranchDeletionStructure],
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

        let mut expected_branch_structure: Vec<BranchDeletionStructure> = Vec::new();
        expected_branch_structure.push(BranchDeletionStructure {
            index: 1,
            name: "branch_1".to_string(),
            should_be_deleted: false,
            is_checked_out: false,
        });
        expected_branch_structure.push(BranchDeletionStructure {
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
            BranchDeletionStructure {
                index: 1,
                name: "branch_1".to_string(),
                should_be_deleted: false,
                is_checked_out: false,
            },
            BranchDeletionStructure {
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

    #[test]
    fn test_toggle_branch_deletion_status_when_branch_is_checked_out_should_not_change_state() {
        let mut input = vec![BranchDeletionStructure {
            index: 1,
            name: "branch_1".to_string(),
            should_be_deleted: false,
            is_checked_out: true,
        }];

        toggle_branch_deletion_status(&mut input, 1);

        assert_eq!(input[0].should_be_deleted, false);
    }

    #[test]
    fn test_toggle_branch_deletion_status_when_branch_is_main_should_not_change_state() {
        let mut input = vec![BranchDeletionStructure {
            index: 1,
            name: "main".to_string(),
            should_be_deleted: false,
            is_checked_out: false,
        }];

        toggle_branch_deletion_status(&mut input, 1);

        assert_eq!(input[0].should_be_deleted, false);
    }

    #[test]
    fn test_toggle_branch_deletion_status_when_branch_is_master_should_not_change_state() {
        let mut input = vec![BranchDeletionStructure {
            index: 1,
            name: "master".to_string(),
            should_be_deleted: false,
            is_checked_out: false,
        }];

        toggle_branch_deletion_status(&mut input, 1);

        assert_eq!(input[0].should_be_deleted, false);
    }
}
