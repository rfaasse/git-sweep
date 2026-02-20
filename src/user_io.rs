use crate::branch_data::BranchData;
use crate::git_sweeper::toggle_branch_deletion_status;
use std::io;

pub fn get_user_defined_branch_deletion_options(branch_structure: &mut [BranchData]) {
    clear_console();

    print_branch_structure_to_console(branch_structure);
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
        print_branch_structure_to_console(branch_structure);
    }
}

fn print_branch_structure_to_console(branch_structure: &mut [BranchData]) {
    println!("These are the branches available for deletion:");
    print!("{}", print_branch_structure(branch_structure));
}

fn print_branch_structure(branch_structure: &[BranchData]) -> String {
    branch_structure
        .iter()
        .map(|branch| {
            format!(
                "[{}] {}. {}\n",
                if branch.should_be_deleted { "x" } else { " " },
                branch.index,
                branch.name,
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

        let expected_output_string =
            "[ ] 1. branch_1\n[x] 2. branch_2\n";

        assert_eq!(expected_output_string, print_branch_structure(&input));
    }
}
