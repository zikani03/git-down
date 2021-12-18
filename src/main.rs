// git-down
// Author: zikani03
//
extern crate fs_extra;

use std::fs;
use std::process::Command;
use std::path::{Path, PathBuf};

const COLON: &'static str = ":";
const DOT_GIT: &'static str = ".git";

#[derive(Debug, Clone)]
struct GitDir {
    repo_url: String,
    repo_name: String,
    dirs: Vec<String>,
}

impl GitDir {
    /// Url of the Git repository
    fn url(&self) -> &str {
        self.repo_url.as_str()
    }

    /// Name of the Git repository
    fn name(&self) -> &str {
        self.repo_name.as_str()
    }

    /// Directories to download from the Git repository
    fn dirs(&self) -> Vec<String> {
        self.dirs.clone()
    }
}

// git-down main
fn main() {
    let arg = std::env::args().nth(1);
    let arg_dest = std::env::args().nth(2);

    let git_url_dir = arg.unwrap();

    let git_dir = parse_source(&git_url_dir);

    let dest_dir = arg_dest.unwrap();
    let tmp_dir = create_tmp_name(git_dir.name());

    if !download_repo(&git_dir, tmp_dir.as_str()) {
        panic!("Failed to download repository");
    }

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
    
    let dirs = git_dir.dirs();

    for d in dirs.iter() {
        source_path.push(d.clone());
        move_directory(source_path.as_path(), dest_path.as_path());
        source_path.pop();
    }

    // remove the temporary directory
    fs::remove_dir_all(tmp_dir.clone())
        .expect(format!("Failed to remove tmp directory, you can remove it from here: {}", tmp_dir).as_str());
}

fn download_repo(git_dir: &GitDir, tmp_dir: &str) -> bool {
   let mut git_command = Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg(git_dir.url())
        .arg(tmp_dir)
        .spawn()
        .expect("Failed to download directory/files from repository");

    return git_command.wait()
                      .expect("Failed to download directory/files from repository")
                      .success();
}

fn parse_source(source_uri: &str) -> GitDir {
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
fn from_shortcut_url<'a>(shortcut_composite: &str) -> GitDir {
    let parts: Vec<&str> = shortcut_composite.split(COLON).collect();

    let num_parts: usize = parts.len(); 
    if num_parts != 3 {
        panic!("Invalid shortcut string");
    }
    let service = parts[0];
    let repo = parts[1];

    let full_url = service_url(service, repo);
    let url_opt = full_url.clone();
    let url = url_opt.unwrap();
    
    let git_dir = GitDir {
        repo_name: parts[1].to_string(),
        repo_url: url,
        dirs: parse_dirs(parts[2])
    };
    git_dir
}

/// Create a GitDir from a full url string
fn from_url<'a>(url_composite: &str) -> GitDir {
    let len_git = DOT_GIT.len();

    let pos = url_composite.rfind(DOT_GIT)
                           .expect("Url must contain a .git extension after the repo name");

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

    GitDir {
        repo_url: String::from(url),
        repo_name: String::from(name),
        dirs: parse_dirs(dir_part),
    }
}

fn parse_dirs(dir_spec: &str) -> Vec<String> {
    let dirs: Vec<_> = dir_spec.split("+")
        .map(|s| s.to_string())
        .collect();
    dirs
}

fn create_tmp_name(dir_name: &str) -> String {
    let path = std::env::temp_dir().join(dir_name);
    return String::from(path.as_path().to_str().unwrap())
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
