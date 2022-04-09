// git-down
// Author: zikani03
//
extern crate clap;
extern crate fs_extra;
extern crate regex;
extern crate tempfile;

use std::fs;
use std::path::{Path, PathBuf};

use clap::{Arg, Command};

mod errors;
mod git;
mod git_url_parser;

// git-down main
fn main() -> Result<(), errors::GitDownError> {
    let matches = Command::new("git-down")
        .version("0.3.0")
        .about("Download files from a git repo like a boss")
        .arg(
            Arg::new("directory")
                .short('d')
                .long("directory")
                .takes_value(true)
                .help("Download into this directory instead of the default one"),
        )
        .arg(Arg::new("url").required(true))
        .arg(
            Arg::new("files")
                .multiple_occurrences(true)
                .max_occurrences(10)
                .required(true),
        )
        .get_matches();

    let url = matches.value_of("url").unwrap();
    let targets: Vec<String> = matches
        .values_of("files")
        .unwrap()
        .map(|file| String::from(file))
        .collect();

    // Clone repository
    let git_url = git_url_parser::parse_url(&url)?;
    let git_dir = git::sparse_checkout(git_url.clone(), targets)?;

    let dest_path = if matches.is_present("directory") {
        PathBuf::from(matches.value_of("directory").unwrap())
    } else {
        PathBuf::from(&git_url.name)
    };

    move_files(&git_dir.contents(), &dest_path);

    Ok(())
}

fn move_files(source_paths: &Vec<PathBuf>, dest_path: &Path) {
    let options = fs_extra::dir::CopyOptions::new();

    if !dest_path.exists() {
        fs::create_dir(dest_path).expect("Cannot create destination directory");
    }

    let dest = dest_path.to_str().unwrap().to_string();
    let mut sources: Vec<String> = Vec::new();

    for path in source_paths {
        sources.push(path.to_str().unwrap().to_string());
    }

    fs_extra::move_items(&sources, &dest, &options)
        .expect(&format!("Failed to move files to {}", dest));
}
