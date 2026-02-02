use std::env;
use std::path::PathBuf;
use gix::{discover, Repository};

fn main() {
    println!("Hello, world!");
    let path = env::current_dir();
    let git_dir : PathBuf = path.unwrap().to_path_buf();
    println!("Current dir = {}", git_dir.display());
    let repo : Repository = discover(".").expect("Nope");
    let branch_names = repo.branch_names();
    let number_of_branches = branch_names.len();
    for branch in branch_names
    {
        println!("branch {branch}");
    }
    println!("Number of branches = {number_of_branches}");
}
