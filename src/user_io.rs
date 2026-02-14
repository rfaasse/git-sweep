use crate::branch_deletion_structure::BranchDeletionStructure;
use crate::git_sweeper::toggle_branch_deletion_status;
use std::io;

pub fn get_user_defined_branch_deletion_options(branch_structure: &mut [BranchDeletionStructure]) {
    clear_console();

    println!("These are the available branches:");
    print!("{}", print_branch_structure(branch_structure));
    loop {
        println!("Type the index of the branch you want to toggle for deletion:");
        println!("(Press Enter without typing a number to finish selection)");
        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        if index.trim().is_empty() {
            break;
        }
        let index: usize = index.trim().parse().expect("Please type a number!");
        toggle_branch_deletion_status(branch_structure, index);
        clear_console();
        println!("These are the branches scheduled for deletion:");
        print!("{}", print_branch_structure(branch_structure));
    }
}

fn print_branch_structure(branch_structure: &[BranchDeletionStructure]) -> String {
    branch_structure
        .iter()
        .map(|branch| {
            format!(
                "[{}] {}. {}{}{}\n",
                if branch.should_be_deleted { "x" } else { " " },
                branch.index,
                branch.name,
                if branch.is_checked_out {
                    " (checked out)"
                } else {
                    ""
                },
                if branch.name == "main"
                    || branch.name == "master"
                    || branch.is_checked_out
                {
                    " [can't be deleted]"
                } else {
                    ""
                }
            )
        })
        .collect()
}

fn clear_console() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_print_branch_structure() {
        let input = vec![
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
                is_checked_out: true,
            },
        ];

        let expected_output_string = "[ ] 1. branch_1\n[x] 2. branch_2 (checked out) [can't be deleted]\n";

        assert_eq!(expected_output_string, print_branch_structure(&input));
    }
}
