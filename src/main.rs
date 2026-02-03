mod git_sweeper;
mod gix_adapter;
use crate::git_sweeper::print_branches;
use crate::gix_adapter::GixAdapter;
use gix::open;
use std::env;

fn main() {
    let path = env::current_dir().expect("Could not open current path");
    let repository = open(path).expect("Could not initialize git repository");
    let gix_adapter = GixAdapter { repo: repository };

    print!("{}", print_branches(&gix_adapter));
}
