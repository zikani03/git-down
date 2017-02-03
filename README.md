Git Down
========

Ever wanted to get the contents of just one directory from a git repo?
Well, there are several ways you could use. 

**Download the zip and extract**

The easiest and most straight-forward approach is to download the archive
of the repo extract the files and get the files/directory you want - but
this only works best if you can get the archive (.zip) via a web interface with a *download*
button/link - which can be annoying to find in some products (BitBucket!)

**Shallow clone and mv**

Another way is to do `git clone --depth 1 REPO_URL && mv /path/to/dir /new/path/to/dir`
this works like a charm but it's two commands and seems like too much work!

**Get files via `wget` or `curl`**

If you can get the files via http, like from GitHub's raw files CDN, 
you *could* find the url for each file in the directory you want and use
`wget` or `curl` to download each file - why anybody would *actually* do that given
 the options above is beyond me.

And then there is the `git-down` way!

## git-down: _The convenient way_

`git-down` works exactly like the shallow-clone and move but does it all in one command. **One command!**

> NOTE: This is still a work in progress. Basic download/cloning works right now if you use the 
> `git-down url.git/directory dest-directory` approach (and have `/tmp/git-down` directory) plus I've only tried it with GitHub so far.
> So, you can send pull-requests if you want to.
> I just have a bad (good?) habit of writing the docs before finishing the thing ;P

## Usage

This command will create a `bootstrap-dist` directory in the current working directory
which will contain the files in the `dist` directory of the bootstrap repo at `HEAD`

```sh
$ git-down https://github.com/twbs/bootstrap.git/dist boostrap-dist
```

This command does the same thing as the one above. Note the .git extension is gone and the source directory 
is the second argument.

```sh
$ git-down https://github.com/twbs/bootstrap dist boostrap-dist
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

### Multiple directories.

```sh
$ git-down https://github.com/twbs/bootstrap.git/dist+src bootstrap-stuff 
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