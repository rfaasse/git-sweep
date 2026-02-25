mod branch_data;
mod git_sweeper;
mod gix_adapter;
mod user_io;

use crate::git_sweeper::create_branch_structure;
use crate::gix_adapter::GixAdapter;
use crate::user_io::get_user_defined_branch_deletion_options;
use gix::open;
use std::env;

fn main() {
    let path = env::current_dir().expect("Could not open current path");
    let repository = open(path).expect("Could not initialize git repository");
    let gix_adapter = GixAdapter { repo: repository };

    let mut branch_structure = create_branch_structure(&gix_adapter);
    get_user_defined_branch_deletion_options(&mut branch_structure);

    for branch in branch_structure.iter().filter(|b| b.should_be_deleted) {
        gix_adapter.delete_branch(&branch.name);
        println!("Deleted branch: {}", branch.name);
    }
}
