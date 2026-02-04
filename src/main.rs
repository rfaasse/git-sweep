mod branch_deletion_structure;
mod git_sweeper;
mod gix_adapter;

use crate::git_sweeper::{
    create_branch_structure, print_branch_structure, toggle_branch_deletion_status,
};
use crate::gix_adapter::GixAdapter;
use gix::open;
use std::{env, io};

fn main() {
    let path = env::current_dir().expect("Could not open current path");
    let repository = open(path).expect("Could not initialize git repository");
    let gix_adapter = GixAdapter { repo: repository };

    let branch_structure = create_branch_structure(&gix_adapter);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    println!("These are the available branches:");
    print!("{}", print_branch_structure(&branch_structure));

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Please type a number!");
    let updated_branch_structure = toggle_branch_deletion_status(&branch_structure, index);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("These are the branches scheduled for deletion:");
    print!("{}", print_branch_structure(&updated_branch_structure));
}
