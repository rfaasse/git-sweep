use crate::gix_adapter::Adapter;

pub struct GitSweeper {
    pub(crate) adapter: Box<dyn Adapter>,
}

impl GitSweeper {
    pub(crate) fn print_branches(&self) -> String {
        let branch_names = self.adapter.branch_names();
        let mut result = String::new();
        result.push_str("Branches: \n");
        let mut index = 1;
        for branch_name in branch_names {
            result.push_str(&index.to_string());
            result.push_str(". ");
            result.push_str(&branch_name);
            result.push_str("\n");
            index += 1;
        }

        result
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
    }

    #[test]
    fn test_print_branches() {
        let mock_adapter = MockGixAdapter {};
        let git_sweeper = GitSweeper {
            adapter: Box::new(mock_adapter),
        };

        let expected_string = String::from("Branches: \n1. branch_1\n2. branch_2\n");
        assert_eq!(git_sweeper.print_branches(), expected_string);
    }
}
