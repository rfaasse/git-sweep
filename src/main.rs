mod git_sweeper;
mod gix_adapter;
use crate::git_sweeper::GitSweeper;
use crate::gix_adapter::GixAdapter;
use gix::open;
use std::env;
use std::path::PathBuf;

fn main() {
    let path = env::current_dir();

    let git_dir: PathBuf = path.unwrap().to_path_buf();
    println!("Current dir = {}", git_dir.display());
    let repository = open(git_dir).expect("Blabla");
    let gix_adapter = GixAdapter { repo: repository };
    let git_sweeper = GitSweeper {
        adapter: gix_adapter,
    };

    print!("{}", git_sweeper.print_branches());
}
