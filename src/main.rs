// git-down
// Author: zikani03
use std::fs;
use std::process::Command;
use std::path::{Path, PathBuf};

const DOT_GIT: &'static str = ".git";

#[derive(Debug)]
struct GitDir<'a> {
    repo_url: &'a str,
    repo_name: &'a str,
    dirs: Vec<&'a str>,
}

impl<'a> GitDir<'a> {
    /// Url of the Git repository
    fn url(&self) -> &str {
        self.repo_url
    }

    /// Name of the Git repository
    fn name(&self) -> &str {
        self.repo_name
    }

    /// Directories to download from the Git repository
    fn dirs(&self) -> Vec<&str> {
        self.dirs.clone()
    }
}

// git-down main
fn main() {
    let arg = std::env::args().nth(1);
    let arg_dest = std::env::args().nth(2);

    let git_url_dir = arg.unwrap();

    let git_dir = parse_url(&git_url_dir);

    let dirs = git_dir.dirs();

    let dest_dir = arg_dest.unwrap();
    let tmp_dir = format!("/tmp/git-down/{}", git_dir.name());

    let mut git_command = Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg(git_dir.url())
        .arg(tmp_dir.clone())
        .spawn()
        .expect("Failed to download directory/files from repository");

    let exit_code = git_command.wait().expect("Failed to download directory/files from repository");

    if exit_code.success() {

        let dest_path = PathBuf::from(dest_dir.clone());

        if !dest_path.exists() {
            match fs::create_dir(dest_dir) {
                Ok(_) => (),
                Err(e) => {
                    panic!("Cannot create destination directory {}", e);
                }
            }
        }

        let mut source_path: PathBuf = PathBuf::from(tmp_dir.clone());

        for d in dirs.iter() {
            source_path.push(d.clone());
            move_directory(source_path.as_path(), dest_path.as_path());
            source_path.pop();
        }

    } else {
        panic!("Failed to download directory from repository");
    }
}


fn parse_url<'a>(url_composite: &str) -> GitDir {
    let len = url_composite.len();
    let len_git = DOT_GIT.len();

    let mut pos = len;

    match url_composite.rfind(DOT_GIT) {
        Some(n) => {
            pos = n;
        }
        None => {
            panic!("Url must contain a .git extension after the repo name");
        }
    }

    let pos_git = pos + len_git;

    let (url, _) = url_composite.split_at(pos_git);

    let url_len = url.len() + 1;

    let pos_slash = url.rfind("/");

    // assume name is between last / and .git e.g. twbs/bootstrap.git => bootstrap
    let (_, name_dot_git) = url.split_at(pos_slash.unwrap() + 1);

    // remove .git part of the name - I tried drain but it drained all the energy out of me
    // trying to get that shit to work, so this is not as elegant as it could be
    let (name, _) = name_dot_git.split_at(name_dot_git.len() - len_git);
    let (_, dir_part) = url_composite.split_at(url_len);

    let dirs: Vec<_> = dir_part.split("+").collect();

    GitDir {
        repo_url: url,
        repo_name: &name,
        dirs: dirs,
    }
}

#[cfg(windows)]
fn move_directory(source: &Path , dest: &Path) {
    Command::new("move")
        .arg(source.to_str().unwrap())
        .arg(dest.to_str().unwrap())
        .output()
        .expect(&format!("Failed to copy files to directory. Find the files here: {}.",
                         source.display()));
}

#[cfg(not(windows))]
fn move_directory(source: &Path , dest: &Path) {
    Command::new("mv")
        .arg(source.to_str().unwrap())
        .arg(dest.to_str().unwrap())
        .output()
        .expect(&format!("Failed to copy files to directory. Find the files here: {}.",
                         source.display()));
}