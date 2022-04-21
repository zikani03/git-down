Usage
=====

It's really easy to use.

```sh
$ git-down -d <DESTINATION_DIRECTORY> <REPO_URL.git:branch> FILES
```

The `-d <DESTINATION_DIRECTORY>` option above is optional. If not specified the files
will be downloaded into a directory under the name of the target repository.

> We're using the bootstrap repo as an example for how to use the command but it works with 
any repository.

For example, running the following command will create a `bootstrap-dist` directory in the current 
working directory containing bootstrap's `dist` directory.

```sh
$ git-down -d bootstrap-dist https://github.com/twbs/bootstrap.git:master dist
```

If bootstrap-dist above is not specified, a directory named bootstrap will be created
instead.

## CLI

```
git-down 0.3.0
Download files from a git repo like a boss

USAGE:
    git-down [OPTIONS] <url> <files>...

ARGS:
    <url>
    <files>...

OPTIONS:
    -d, --directory <directory>    Download into this directory instead of the default one
    -h, --help                     Print help information
    -V, --version                  Print version information
```




## Download multiple directories

You can use git-down to download multiple directories from the same repository.

For example if you wanted to download both the dist and src directories from the Bootstrap repo you would 
use the following command.

```sh
$ git-down https://github.com/twbs/bootstrap.git:master dist src 
```

Get only the css directory 

```sh
$ git-down -d bootstrap-css gh:tbws/bootstrap:master dist/css
```

Get the css and img directories

```sh
$ git-down -d boostrap-css-img gh:tbws/bootstrap:master dist/css dist/img
```

## Shortcuts for Supported Services

Shortcuts are intended to reduce keystrokes by allowing you to download from popular Git hosting
services without typing out the full URL of the repository. When using shortcuts the parts of the _source_ are separated using colons (:).

The following are working examples you can use for downloading a directory from 
supported services:

### `gh:` GitHub

The following example command will download the dist directory from bootstrap repo
into boostrap-latest from the GitHub repository.

```sh
$ git-down -d bootstrap-latest gh:tbws/bootstrap:master dist
```

### `bb:` BitBucket

```sh
$ git-down -d sqlalchemy-examples bb:zzzeek/sqlalchemy:master examples 
```

### `gl:` GitLab

```sh
$ git-down -d gitlab-scripts gl:gitlab-org/gitlab-ce:master scripts 
```

### `sf:` SourceForge

```sh
$ git-down -d nagioscore-sample-config sf:nagios/nagios-core:master sample-config 
```
