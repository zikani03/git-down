use regex::{Captures, Regex};

use crate::errors::GitDownError;

#[derive(Debug, Clone)]
pub struct GitUrl {
    pub url: String, // Full URL to repository (whether expanded from shortcut or as received from user
    pub name: String, // Name of the repository extracted from url above
    pub branch: String, // Target branch specified in the url
}

pub fn parse_url(url: &str) -> Result<GitUrl, GitDownError> {
    if is_shortcut_url(url) {
        return parse_shortcut_url(url);
    }

    parse_http_url(url)
}

pub fn is_shortcut_url(url: &str) -> bool {
    // FIXME: Do not rely on hardcoded forms of the shortcuts. 
    let regex = Regex::new(r"^(bb|gh|gl|sf):.*$").unwrap();

    regex.is_match(url)
}

fn parse_shortcut_url(url: &str) -> Result<GitUrl, GitDownError> {
    let regex = Regex::new(
        r"^(?P<alias>\w+):(?P<user>[^.]+)/(?P<repo_name>[^.]+)(\.git)?(:(?P<branch>.*))",
    )
    .unwrap();
    let captures = match regex.captures(url) {
        Some(value) => value,
        None => {
            return Err(GitDownError {
                message: format!("Invalid url: {}", url),
            })
        }
    };

    let alias = read_url_part(&captures, "alias")?;
    let base_url = resolve_url_host_alias(&alias)?;
    let user = read_url_part(&captures, "user")?;
    let name = read_url_part(&captures, "repo_name")?;
    let branch = read_url_part(&captures, "branch")?;

    Ok(GitUrl {
        url: format!("{}/{}/{}", base_url, user, name),
        name,
        branch,
    })
}

fn resolve_url_host_alias(alias: &str) -> Result<String, GitDownError> {
    // TODO: Maybe have the aliasing done in some configuration file
    let host = match alias {
        "gh" => "https://github.com",
        "bb" => "https://bitbucket.org",
        "gl" => "https://gitlab.org",
        "sf" => "https://git.code.sf.net/p/",
        _ => {
            return Err(GitDownError {
                message: format!("Invalid host alias: {}", alias),
            })
        }
    };

    Ok(String::from(host))
}

fn read_url_part(captures: &Captures, name: &str) -> Result<String, GitDownError> {
    match captures.name(name) {
        Some(value) => Ok(value.as_str().to_string()),
        None => {
            return Err(GitDownError {
                message: format!("Invalid url: Missing '{}'", String::from(name)),
            });
        }
    }
}

fn parse_http_url(url: &str) -> Result<GitUrl, GitDownError> {
    let regex = Regex::new(r"^(?P<url>.*/(?P<name>[^.:/]+)(\.git)?)/?:(?P<branch>[^.]+)$").unwrap();
    let captures = match regex.captures(url) {
        Some(value) => value,
        None => {
            return Err(GitDownError {
                message: format!("Invalid url: {}", url),
            })
        }
    };

    let branchless_url = read_url_part(&captures, "url")?;
    let name = read_url_part(&captures, "name")?;
    let branch = read_url_part(&captures, "branch")?;

    Ok(GitUrl {
        url: String::from(branchless_url),
        name,
        branch,
    })
}
