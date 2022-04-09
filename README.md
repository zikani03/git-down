Git Down
========

`git-down` lets you download one or multiple directories from a Git repository without the
hassle of cloning or downloading the whole repository, with one simple command.

## Usage

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

### Download multiple directories

You can use git-down to download multiple directories from the same repository.

For example if you wanted to download both the dist and src directories from the Bootstrap repo you would 
use the following command.

```sh
$ git-down https://github.com/twbs/bootstrap.git:master dist src 
```

### Shortcuts

Shortcuts are intended to reduce keystrokes by allowing you to download from popular Git hosting
services without typing out the full URL of the repository.

When using shortcuts the parts of the _source_ are separated using colons (:).

The following are working examples you can use for downloading a directory from 
supported services:

**GitHub**

The following example command will download the dist directory from bootstrap repo
into boostrap-latest from the GitHub repository.

```sh
$ git-down -d bootstrap-latest gh:tbws/bootstrap:master dist
```

Get only the css directory 

```sh
$ git-down -d bootstrap-css gh:tbws/bootstrap:master dist/css
```

Get the css and img directories

```sh
$ git-down -d boostrap-css-img gh:tbws/bootstrap:master dist/css dist/img
```

**BitBucket**

```sh
$ git-down -d sqlalchemy-examples bb:zzzeek/sqlalchemy:master examples 
```

**GitLab**

```sh
$ git-down -d gitlab-scripts gl:gitlab-org/gitlab-ce:master scripts 
```

**SourceForge**

```sh
$ git-down -d nagioscore-sample-config sf:nagios/nagios-core:master sample-config 
```

## Why do I need this in my life?

It will save you time. Currently all of the ways to get just one directory from a repository 
require either _too much_ clicking or typing!

If you want to download just one (or multiple) directory from a Git repository -
like a `dist` directory of a project like bootstrap - you have several ways of 
doing so, including:
 
**Download the archive and extract**

The easiest and most straight-forward approach is to download the archive
of the repo, extract the files and get the files/directory you want.
This works best if you can get the archive via a web interface via a *Download*
button/link - which can be annoying to find in some products (BitBucket!).

**Shallow clone and mv**

Another way is to do a shallow clone and get the directories you want using a move command.
It's just too many commands and typing!

For example, getting the dist directory from bootstrap you would do something like:

```sh
$ git clone --depth 1 https://github.com/twbs/boostrap.git
$ cd bootstrap
$ mv ./dist ~/stuff/boostrap-latest 
```

C'mon, you don't have the time to be doing all that. 

## Installation

No pre-packaged binaries - you have to build it yourself, sorry. :/

You will have to install Rust. I highly recommend using [rustup](https://www.rustup.rs)

```sh
$ git clone https://github.com/zikani03/git-down.git
$ cd git-down
$ cargo build --release
$ mv target/release/git-down ~/bin/git-down
```

Assuming your `~/bin` is on your PATH you can now use it from the shell as `git-down`

## Planned Features

### From a different commit, tag, revision

This would be nice, me thinks :)

```sh
$ git-down -f v3.3.2 gh:twbs/bootstrap:dist bootstrap-3.3.2
```

Now, git down to business!

# LICENCE

MIT

---
Copyright (c) 2017, Zikani Nyirenda Mwase
