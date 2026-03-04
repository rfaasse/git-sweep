use crate::branch_data::BranchData;
use crate::git_sweeper::toggle_branch_deletion_status_by_name;
use inquire::{Confirm, MultiSelect};
pub fn get_user_defined_branch_deletion_options(branch_data: &mut [BranchData]) {
    let options: Vec<String> = branch_data.iter().map(|data| data.name.clone()).collect();
    if options.is_empty() {
        println!("No branches available for deletion.");
        return;
    }
    let branch_names_scheduled_for_deletion =
        MultiSelect::new("Select the branches you would like to delete:", options)
            .prompt()
            .unwrap_or(Vec::new());

    if branch_names_scheduled_for_deletion.is_empty() {
        println!("No branches selected for deletion.");
        return;
    }

    let confirm = Confirm::new("Do you want to continue?").prompt();

    match confirm {
        Ok(true) => {
            for name in branch_names_scheduled_for_deletion {
                toggle_branch_deletion_status_by_name(branch_data, name.as_str());
            }
        }
        Ok(false) => println!("Deletion was aborted."),
        Err(_) => {}
    }
}
