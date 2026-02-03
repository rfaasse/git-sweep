use crate::gix_adapter::{Adapter, GixAdapter};

pub struct GitSweeper {
    pub(crate) adapter: GixAdapter,
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
