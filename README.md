Git Down
========

`git-down` lets you download one or multiple directories from a Git repository without the
hassle of cloning or downloading the whole repository, with one simple command.

> NOTE: This is still a work in progress. Basic download/cloning works right now if you use the 
> `git-down url.git/directory dest-directory` approach (and have `/tmp/git-down` directory) plus I've only tried it with GitHub so far.
> So, you can send pull-requests if you want to.
> I just have a bad (good?) habit of writing the docs before finishing the thing ;P

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

## Usage

```sh
$ git-down <REPO_URL.git/DIRECTORIES> <DESTINATION_DIRECTORY>
```

It's really easy to use.

> We're using the bootstrap repo as an example for how to use the command but it works with 
any repository.

For example, running the following command will create a `bootstrap-dist` directory in the current 
working directory containing bootstrap's `dist` directory.

```sh
$ git-down https://github.com/twbs/bootstrap.git/dist boostrap-dist
```

### Download multiple directories

You can use git-down to download multiple directories from the same repository.

For example if you wanted to download both the dist and src directories from the Bootstrap repo you would 
use the following command.

```sh
$ git-down https://github.com/twbs/bootstrap.git/dist+src bootstrap-stuff 
```

## Installation

No pre-packaged binaries - you have to build it yourself, sorry. :/

Install Rust. I highly recommend using `rustup`

```sh
$ git clone https://github.com/zikani03/git-down.git
$ cd git-down
$ cargo build --release
$ mv target/release/git-down ~/bin/git-down
```

Assuming your `~/bin` is on your PATH you can now use it from the shell as `git-down`

## Planned Features

### From a different commit, tag, revision

```sh
$ git-down -f v3.3.2 https://github.com/twbs/bootstrap.git/dist bootstrap-3.3.2
```

### Shortcuts

**GitHub**

```sh
$ git-down --github twbs/bootstrap --dir dist -o bootstrap-latest
# or
$ git-down gh:tbws/bootstrap dist bootstrap-latest
```

Get only the css directory 

```sh
$ git-down gh:tbws/bootstrap dist/css bootstrap-css
```

Get the css and img directories

```sh
$ git-down gh:tbws/bootstrap dist/css+img bootstrap-css-img
```

**BitBucket**

```sh
$ git-down --bitbucket twbs/bootstrap --dir dist -o bootstrap-latest
# or
$ git-down bb:tbws/bootstrap.git/dist bootstrap-latest
```

**GitLab**

```sh
$ git-down --gitlab twbs/bootstrap --dir dist -o bootstrap-latest
# or
$ git-down gl:tbws/bootstrap.git/dist bootstrap-latest
```

**SourceForge**

```sh
$ git-down --sourceforge twbs/bootstrap --dir dist -o bootstrap-latest
# or
$ git-down sf:tbws/bootstrap.git/dist bootstrap-latest
```

**SSH**

This would ofcourse assumes you have an ssh-agent running, with your git repo setup with 
an ssh key and all that stuff.

```sh
$ git-down --ssh twbs/bootstrap --dir dist -o bootstrap-latest
# or
$ git-down ssh:tbws/bootstrap.git/dist bootstrap-latest
```

## _Should_ work with

* GitHub
* BitBucket
* GitLab
* SourceForge
* Any where you can clone a Git repo from :D

Now, git down to business!

# LICENCE

MIT

---
Copyright (c) 2017, Zikani Nyirenda Mwase