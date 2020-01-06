<h3 align="center"><img src="https://raw.githubusercontent.com/rsfetch/rsfetch/master/Screenshots/new-logo.jpg" alt="logo" height="100px"></h3>
<p align="center">Fast (<1ms execution time) and somewhat(?) minimal fetch program written in Rust.</p>

<p align="center">
<img alt="crates.io version" src="https://img.shields.io/badge/crates.io-v1.9.0-blue.svg">
<img alt="GitHub code size in bytes" src="https://img.shields.io/github/languages/code-size/rsfetch/rsfetch.svg">
<img alt="GitHub" src="https://img.shields.io/github/license/rsfetch/rsfetch.svg">
<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/rsfetch/rsfetch.svg">
<img alt="GitHub issues" src="https://img.shields.io/github/issues/rsfetch/rsfetch.svg?color=gren">
</p>

### Table of Contents
* [Preface](#preface)
* [Things to Know](#things-to-know)
* [Installation](#installation)
* [Screenshots](#screenshots)
* [Benchmarks](#benchmarks)
* [License](#license)
* [Changelog](#changelog)

### Preface

**Why I originally made it:** I used to use neofetch, ufetch, and 
aurafetch a lot, but then that got me thinking. I have the skill to make
 my own script, so why not? At first, it was just a simple BASH script. 
Then I decided I wanted to try my hand at Rust, as it has interested me 
for a while. So... here's the end result. I am open to any criticisms 
you have. After all, I wish to make this better (as well as improve my 
Rust skills).

**Why I want to continue improving this:** When this first started out, it 
was a simple pet project for learning how to code in Rust. Now that I have
people helping me with this project, I would love to see this turn into a
worthy alternative to neofetch/ufetch/screenfetch. Please, if there's anything 
that you feel is missing from this, open an issue. I would love to hear people's
thoughts on how this is.

### Things to Know

1. If you plan to help, note that indentation is done with 4 spaces. It used to be tabs, but I have been told that spaces are the preferred indentation in the Rust community. ~~Any new changes should be recorded in [CHANGELOG.md](CHANGELOG.md).~~

2. These are the current package managers supported. If I missed any, please report it in [this](https://github.com/rsfetch/rsfetch/issues/28) issue.
    - pacman
    - apt
    - xbps
    - dnf
    - eopkg
    - pkg
    - rpm
    - apk
    - portage (note: requires portage-utils to be installed as it uses qlist)
    - cargo
    - pip

3. The music info obtains the info from a locally running `mpd` server using `mpc`. If you want more music sources supported, let me know by opening up an issue.

### Installation
I have prebuilt binaries in the releases tab for people who don't want to build from source, otherwise you can do this.

#### Prerequisites
- the Rust compiler toolchain
- **GNU** Make (makefile not compatible with `bmake`!)

1. Clone the repository.
2. `cd rsfetch; make; sudo make install`

Uninstall with `sudo make uninstall`.

#### AUR package
Optionally, if you use a Arch-based distro, you can use either of the following AUR packages:
- [rsfetch-git](https://aur.archlinux.org/packages/rsfetch-git/) (This requires Rust as it builds from source.)
- [rsfetch-bin](https://aur.archlinux.org/packages/rsfetch-bin/) (This grabs the latest binary from releases.)

#### ebuild
~~I just switched over to Gentoo a few days ago, so whenever I figure out how to make (and share) ebuilds, there will be one released. :)~~ So it turns out that not only am I terrible at pkgbuilds, but also ebuilds. If anyone could make one for me, that would be amazing.

#### Cargo
If you already have `rust` and `cargo` setup, you can install it with `cargo install rsfetch`.

### Screenshots

**Help**
![Help](Screenshots/help.png?raw=true "Help")

### Benchmarks

Here's a detailed benchmark; with rsfetch versus ufetch, pfetch, Neofetch, and ScreenFetch:

![Benchmarks](Screenshots/benchmarks.png?raw=true "Benchmarks")

### License

- License has been set to "MIT". Anybody can do anything with this repository, provided that they include the license if they choose to redistribute. [View license](https://raw.githubusercontent.com/rsfetch/rsfetch/master/LICENSE)

### Changelog

- Changelog was generated (because I'm too lazy) by [github_changelog_generator](https://github.com/skywinder/Github-Changelog-Generator). [View changelog](https://raw.githubusercontent.com/rsfetch/rsfetch/master/CHANGELOG.md)
