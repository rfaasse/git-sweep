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
            name,
            should_be_deleted: false,
        })
        .collect()
}

pub(crate) fn toggle_branch_deletion_status(branch_structure: &mut [BranchData], index: usize) {
    if let Some(branch) = branch_structure.iter_mut().find(|b| b.index == index) {
        branch.should_be_deleted = !branch.should_be_deleted;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockGixAdapter {}

    impl Adapter for MockGixAdapter {
        fn branch_names(&self) -> Vec<String> {
            Vec::from([
                "branch_1".to_string(),
                "branch_2".to_string(),
                "main".to_string(),
                "master".to_string(),
                "checked_out".to_string(),
            ])
        }
        fn is_checked_out(&self, branch_name: &str) -> bool {
            branch_name == "checked_out"
        }
    }

    #[test]
    fn test_create_branch_structure() {
        let mock_adapter = MockGixAdapter {};

        // The branches main and master are filtered out
        let mut expected_branch_structure: Vec<BranchData> = Vec::new();
        expected_branch_structure.push(BranchData {
            index: 1,
            name: "branch_1".to_string(),
            should_be_deleted: false,
        });
        expected_branch_structure.push(BranchData {
            index: 2,
            name: "branch_2".to_string(),
            should_be_deleted: false,
        });

        let actual_branch_structure = create_branch_structure(&mock_adapter);
        assert_eq!(
            expected_branch_structure.len(),
            actual_branch_structure.len()
        );
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
            },
            BranchData {
                index: 2,
                name: "branch_2".to_string(),
                should_be_deleted: true,
            },
        ];

        toggle_branch_deletion_status(&mut input, 2);

        assert_eq!(input[0].should_be_deleted, false);
        assert_eq!(input[1].should_be_deleted, false);
    }
}
