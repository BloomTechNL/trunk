# trunk -- git facade encouraging trunk based development
This is `trunk`, a CLI facade for git in the style of `jj`. It restricts the interface of `git` to a small
set of subcommands which should suffice in a trunk-based development style workflow.

## Installation

### MacOS
To install `trunk`, go to [this page](https://github.com/BloomTechNL/trunk/releases/tag/latest) and download the latest
binary, called `g`. Then, run
```bash
chmod +x /path/to/the/binary
xattr -d com.apple.quarantine /path/to/the/binary
```
and put `g` in your `$PATH`. It is recommended to place `g` somewhere you own, rather than a root-owned directory. Then,
check your installation by running
```bash
g --version
```
If you have the OhMyZsh git plugin installed, you may need to run
```bash
unalias g
```

### Linux
Ensure you have the Rust toolchain installed, in particular `cargo`. Then,
```bash
git clone git@github.com:BloomTechNL/trunk.git
cd trunk
cargo build --release
cargo install --path .
g --version
```
If you have the OhMyZsh git plugin installed, you may need to run
```bash
unalias g
```

### Windows
Follow the instructions on [this page](https://wiki.archlinux.org/title/Dual_boot_with_Windows), and then follow the
instructions in the section above.
