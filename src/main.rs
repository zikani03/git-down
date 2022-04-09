// git-down
// Author: zikani03
//
extern crate fs_extra;

use std::fs;
use std::path::{Path, PathBuf};

mod git;
mod git_url_parser;
mod errors;

// git-down main
fn main() -> Result<(), errors::GitDownError> {
    let url = std::env::args().nth(1).unwrap();
    let target = std::env::args().nth(2).unwrap();
    let targets = Vec::from([String::from(target)]);

    let git_url = git_url_parser::parse_url(&url)?;
    let git_dir = git::sparse_checkout(git_url.clone(), targets)?;

    let dest_path = PathBuf::from(&git_url.name);

    if !dest_path.exists() {
        fs::create_dir(git_url.name).expect("Cannot create destination directory");
    }

    for dir in git_dir.contents() {
        move_directory(dir.as_path(), dest_path.as_path());
    }

    Ok(())
}


fn move_directory(source_path: &Path , dest_path: &Path) {
    let options = fs_extra::dir::CopyOptions::new();
    let source = source_path.to_str().unwrap().to_string();
    let dest = dest_path.to_str().unwrap().to_string();

    fs_extra::dir::move_dir(source, dest, &options)
        .expect(&format!("Failed to copy files to directory. Find the files here: {}.", source_path.display()));
}

