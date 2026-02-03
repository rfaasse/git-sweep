mod git_sweeper;
mod gix_adapter;
mod branch_deletion_structure;

use crate::git_sweeper::{create_branch_structure, print_branch_structure};
use crate::gix_adapter::GixAdapter;
use gix::open;
use std::env;

fn main() {
    let path = env::current_dir().expect("Could not open current path");
    let repository = open(path).expect("Could not initialize git repository");
    let gix_adapter = GixAdapter { repo: repository };

    let branch_structure = create_branch_structure(&gix_adapter);

    println!("These are the available branches:");
    print!("{}", print_branch_structure(&branch_structure));

    println!("Please supply a space separated list of the branches you'd like to delete");
}
