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
