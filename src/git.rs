use std::fs::remove_dir_all;
use std::ops::Drop;
use std::path::PathBuf;
use std::process::Command;

use crate::errors::GitDownError;
use crate::git_url_parser::GitUrl;

#[derive(Debug)]
pub struct GitDir {
    remote_url: GitUrl,           // URL to source repository
    target_files: Vec<String>,    // Files/folders to pull from remote
    local_dir: tempfile::TempDir, // Local partial copy of remote repository
}

impl GitDir {
    pub fn path(&self) -> Result<&str, GitDownError> {
        let path = self.local_dir.path().to_str();
        match path {
            Some(p) => Ok(p),
            None => Err(GitDownError {
                message: String::from("Failed to read temp dir"),
            }),
        }
    }

    pub fn contents(&self) -> Vec<PathBuf> {
        let mut paths: Vec<PathBuf> = Vec::new();
        let path = self.local_dir.path();

        for filename in &self.target_files {
            let path_buf = path.join(filename);

            paths.push(path_buf);
        }

        paths
    }
}

impl Drop for GitDir {
    fn drop(&mut self) {
        remove_dir_all(self.local_dir.path()).unwrap();
    }
}

pub fn sparse_checkout(
    remote_url: GitUrl,
    target_files: Vec<String>,
) -> Result<Box<GitDir>, GitDownError> {
    // TempDir deletes directory once it goes out of scope hence
    // boxing GitDir to prevent having copies outside of this
    // with a dangling local_dir
    let dir = Box::new(GitDir {
        remote_url,
        target_files,
        local_dir: tempfile::tempdir()?,
    });

    println!("Cloning {}:{} into {}", dir.remote_url.url, dir.remote_url.branch, dir.path()?);
    exec_git(&dir, &["clone", "--filter=tree:0", "--no-checkout", &dir.remote_url.url, "."])?;
    exec_git(&dir, &["sparse-checkout", "init", "--cone"])?;

    let mut checkout_args = Vec::from(["sparse-checkout", "set"]);
    for path in &dir.target_files {
        checkout_args.push(path);
    }

    exec_git(&dir, &checkout_args)?;
    exec_git(&dir, &["checkout"])?;

    return Ok(dir);
}

fn exec_git(git_dir: &GitDir, git_command: &[&str]) -> Result<(), GitDownError> {
    println!("Running git {}...", git_command[0]);
    let mut command = Command::new("git");
    let mut child = command
        .args(["-C", git_dir.path()?])
        .args(git_command)
        .spawn()?;
    child.wait()?;

    Ok(())
}
