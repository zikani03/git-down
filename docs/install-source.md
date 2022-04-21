Installation from Source
========================

No pre-packaged binaries - you have to build it yourself, sorry. :/

You will have to install Rust. I highly recommend using [rustup](https://www.rustup.rs)

```sh
$ git clone https://github.com/zikani03/git-down.git
$ cd git-down
$ cargo build --release
$ mv target/release/git-down ~/bin/git-down
```

Assuming your `~/bin` is on your PATH you can now use it from the shell as `git-down`