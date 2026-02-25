use crate::branch_data::BranchData;
use crate::git_sweeper::toggle_branch_deletion_status_by_name;
use inquire::{Confirm, MultiSelect};
pub fn get_user_defined_branch_deletion_options(branch_structure: &mut [BranchData]) {
    let options = branch_structure
        .iter()
        .map(|data| data.name.clone())
        .collect();
    let ans = MultiSelect::new("Select the branches you would like to delete:", options)
        .prompt()
        .unwrap();

    let confirm = Confirm::new("Do you want to continue?").prompt();

    match confirm {
        Ok(true) => {
            for name in ans {
                toggle_branch_deletion_status_by_name(branch_structure, name.as_str());
            }
        }
        Ok(false) => println!("Deletion was aborted"),
        Err(_) => println!("Error, try again later"),
    }
}
