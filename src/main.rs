// git-down
// Author: zikani03
//
extern crate fs_extra;

use std::fs;
use std::path::{Path, PathBuf};

mod git;

const COLON: &'static str = ":";
const DOT_GIT: &'static str = ".git";


// git-down main
fn main() {
    let arg = std::env::args().nth(1);
    let arg_dest = std::env::args().nth(2);

    let git_url_dir = arg.unwrap();

    let (url, branch, targets) = parse_source(&git_url_dir);
    let git_dir = git::sparse_checkout(url, branch, targets).expect("Sparse checkout failed");

    let dest_dir = arg_dest.unwrap();
    let dest_path = PathBuf::from(dest_dir.clone());

    if !dest_path.exists() {
        fs::create_dir(dest_dir).expect("Cannot create destination directory");
    }

    for dir in git_dir.contents() {
        move_directory(dir.as_path(), dest_path.as_path());
    }
}


fn parse_source(source_uri: &str) -> (String, String, Vec<String>) {
    // check if we are dealing with a shortcut source first, e.g. gh:
    let mut colon_pos = 0;
    match source_uri.rfind(COLON) {
        Some(n) => {
            colon_pos = n;
        },
        None => (),
    };

    if colon_pos > 0 {
        return from_shortcut_url(source_uri);
    }

    from_url(source_uri)
}

/// Create a GitDir from a shortcut url string
/// a shortcut string looks like gh:user/repo:directory
fn from_shortcut_url<'a>(shortcut_composite: &str) -> (String, String, Vec<String>) {
    let parts: Vec<&str> = shortcut_composite.split(COLON).collect();

    let num_parts: usize = parts.len(); 
    if num_parts != 4 {
        panic!("Invalid shortcut string (e.g. gh:zikani03/git-down:src:master)");
    }
    let service = parts[0];
    let repo = parts[1];

    let full_url = service_url(service, repo);
    let url_opt = full_url.clone();
    let url = url_opt.unwrap();
    
    (url, String::from(parts[3]), parse_dirs(parts[2]))
}

/// Create a GitDir from a full url string
fn from_url<'a>(url_composite: &str) -> (String, String, Vec<String>) {
    let len_git = DOT_GIT.len();

    let pos = url_composite.rfind(DOT_GIT)
                           .expect("Url must contain a .git extension after the repo name");

    let pos_git = pos + len_git;

    let (url, branch_part) = url_composite.split_at(pos_git);

    let url_len = url.len() + 1;

    // remove .git part of the name - I tried drain but it drained all the energy out of me
    // trying to get that shit to work, so this is not as elegant as it could be
    let (_, dir_part) = url_composite.split_at(url_len);

    if !branch_part.starts_with(":") {
        panic!("Url must contain branch (e.g. https://github.com/zikani03/git-down.git:master)")
    }

    let (_, branch) = branch_part.split_at(1);

    (String::from(url), String::from(branch), parse_dirs(dir_part))
}

fn parse_dirs(dir_spec: &str) -> Vec<String> {
    let dirs: Vec<_> = dir_spec.split("+")
        .map(|s| s.to_string())
        .collect();
    dirs
}

fn move_directory(source_path: &Path , dest_path: &Path) {
    let options = fs_extra::dir::CopyOptions::new();
    let source = source_path.to_str().unwrap().to_string();
    let dest = dest_path.to_str().unwrap().to_string();

    fs_extra::dir::move_dir(source, dest, &options)
        .expect(&format!("Failed to copy files to directory. Find the files here: {}.", source_path.display()));
}

fn service_url<'a>(service: &'a str, repo: &'a str) -> Option<String> {
    match service {
        "gh" => Some(github_url(repo)),
        "bb" => Some(bitbucket_url(repo)),
        "gl" => Some(gitlab_url(repo)),
        "sf" => Some(sourceforge_url(repo)),
        _ => None,
    }
}


fn github_url(repo: &str) -> String {
    let mut url = String::from("https://github.com/");
    url.push_str(repo.clone());
    url
}

fn bitbucket_url(repo: &str) -> String {
    let mut url = String::from("https://bitbucket.org/");
    url.push_str(repo);
    url
}

fn gitlab_url(repo: &str) -> String {
    let mut url = String::from("https://gitlab.com/");
    url.push_str(repo);
    url
}
fn sourceforge_url(repo: &str) -> String {
    let mut url = String::from("https://git.code.sf.net/p/");
    url.push_str(repo);
    url
}
