use std::fs::remove_dir_all;
use std::io::{Error, ErrorKind};
use std::ops::Drop;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug)]
pub struct GitDir {
    remote_url: String, // URL to source repository
    branch: String,
    target_files: Vec<String>, // Files/folders to pull from remote
    local_dir: tempfile::TempDir, // Local partial copy of remote repository
}

impl GitDir {
    pub fn path(&self) -> Result<&str, Error> {
        let path = self.local_dir.path().to_str();
        match path {
            Some(p) => Ok(p),
            None => Err(Error::new(ErrorKind::Other, "Failed to read temp dir"))
        }
    }

    pub fn contents(&self) -> Vec<PathBuf> {
        let mut paths: Vec<PathBuf> = Vec::new();

        for filename in &self.target_files {
            let path = self.local_dir.path();
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

pub fn sparse_checkout(remote_url: String, branch: String, target_files: Vec<String>) -> Result<Box<GitDir>, Error> {
    let dir = Box::new(GitDir {
        remote_url,
        branch,
        target_files,
        local_dir: tempfile::tempdir()?
    });

    git_init(&dir)?;

    return Ok(dir);
}

fn exec_git(git_dir: &GitDir, git_command: &[&str]) -> Result<(), Error> {
    println!("Running git {}", git_command[0]);
    let mut command = Command::new("git");
    let mut child = command.args(["-C", git_dir.path()?]).args(git_command).spawn()?;
    child.wait()?;

    Ok(())
}

fn git_init(git_dir: &GitDir) -> Result<(), Error> {
    exec_git(git_dir, &["init"])?;
    exec_git(git_dir, &["config", "core.sparsecheckout", "true"])?;

    let mut checkout_args = Vec::from(["sparse-checkout", "set"]);
    for path in &git_dir.target_files {
        checkout_args.push(path);
    }

    exec_git(git_dir, &checkout_args)?;
    exec_git(git_dir, &["remote", "add", "origin", git_dir.remote_url.as_str()])?;
    exec_git(git_dir, &["pull", "origin", &git_dir.branch])?;

    Ok(())
}


