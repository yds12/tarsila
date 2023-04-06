# Installing Tarsila

## Debian-based Linux

The easiest way to get Tarsila on Linux distros based on Debian 10 (buster)
or later is to use our pre-built `.deb` package on the
[releases page](https://github.com/yds12/tarsila/releases/tag/0.1.0).

If you have rust installed and want to compile from source, first install the
dependencies:

```
apt install pkg-config librust-glib-sys-dev librust-atk-sys-dev librust-gdk-sys-dev libasound2-dev
```

Then follow the general compilation instructions below.

## Arch Linux/AUR

[Tarsila is on AUR](https://aur.archlinux.org/packages/tarsila) and can
be installed using [your helper of choice](https://wiki.archlinux.org/title/AUR_helpers).

E.g.: `paru -S tarsila`

This will compile and install the latest Tarsila release.


## Other Systems

For other systems for now you need to compile Tarsila by yourself.

For this you need to have the
[rust toolchain](https://www.rust-lang.org/tools/install) installed.

Then run `cargo install tarsila`. This will install the
[crates.io](https://crates.io/crates/tarsila) version of the program.

If you want the latest changes, clone (with `git`) or download this repository
and run `cargo install --path .` from the repository's root. If you want to
compile it without installing, make sure to use `cargo build --release` for
performance's sake.

You might first have to install some system dependencies such as these
libraries:

* `pkg-config`
* `glib`
* `atk`
* `gdk`

On a fresh MacOS Ventura you probably just need to install xcode developer tools.

