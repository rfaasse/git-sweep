use crate::branch_deletion_structure::BranchDeletionStructure;
use crate::gix_adapter::Adapter;
pub(crate) fn create_branch_structure(adapter: &dyn Adapter) -> Vec<BranchDeletionStructure> {
    let branch_names = adapter.branch_names();
    branch_names
        .into_iter()
        .enumerate()
        .map(|(i, branch_name)| BranchDeletionStructure {
            index: i + 1,
            branch_name,
            should_be_deleted: false,
        })
        .collect()
}

pub(crate) fn print_branch_structure(branch_structure: &Vec<BranchDeletionStructure>) -> String {
    branch_structure
        .iter()
        .map(|branch| {
            format!(
                "[{}] {}. {}\n",
                if branch.should_be_deleted { "x" } else { " " },
                branch.index,
                branch.branch_name
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockGixAdapter {}

    impl Adapter for MockGixAdapter {
        fn branch_names(&self) -> Vec<String> {
            Vec::from(["branch_1".to_string(), "branch_2".to_string()])
        }
    }

    #[test]
    fn test_create_branch_structure() {
        let mock_adapter = MockGixAdapter {};

        let mut expected_branch_structure: Vec<BranchDeletionStructure> = Vec::new();
        expected_branch_structure.push(BranchDeletionStructure {
            index: 1,
            branch_name: "branch_1".to_string(),
            should_be_deleted: false,
        });
        expected_branch_structure.push(BranchDeletionStructure {
            index: 2,
            branch_name: "branch_2".to_string(),
            should_be_deleted: false,
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
    fn test_print_branch_structure() {
        let input = vec![
            BranchDeletionStructure {
                index: 1,
                branch_name: "branch_1".to_string(),
                should_be_deleted: false,
            },
            BranchDeletionStructure {
                index: 2,
                branch_name: "branch_2".to_string(),
                should_be_deleted: true,
            },
        ];

        let expected_output_string = "[ ] 1. branch_1\n[x] 2. branch_2\n";

        assert_eq!(expected_output_string, print_branch_structure(&input));
    }
}
