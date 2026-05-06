# trunk -- git facade encouraging trunk based development
This is `trunk`, a CLI facade for git in the style of `jj`. It restricts the interface of `git` to a small
set of subcommands which should suffice in a trunk-based development style workflow.

## Installation

### MacOS
Run
```bash
curl -fsSL https://raw.githubusercontent.com/BloomTechNL/trunk/main/scripts/install_macos.sh | bash
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
