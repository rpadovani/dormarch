## Dormarch

*Retrieving SSH and GPS keys from GitHub and GitLab*

[![asciicast](https://asciinema.org/a/yfVzCpwEeVEuXwZJRaHSOYsBF.svg)](https://asciinema.org/a/yfVzCpwEeVEuXwZJRaHSOYsBF)

### Usage

After having installed Dormarch, you can see all the options with `dormarch -h`.

To retrieve SSH keys of a user from both GitLab.com and GitHub.com, just type the username:

```shell
dormarch rpadovani
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIPBcP26/1Wx695rwnfJPqChM98BZN1e2/CYjpQ1dR8l6
```

You can append the `--gpg-keys` flag if you are interested in the GPG keys of the user.

### Installation

You can download the latest version of Dormarch from the [GitHub Releases page](https://github.com/rpadovani/dormarch/releases) and place it in your executable directory (e.g., `~/.local/bin/` on Linux).

### About

This project was born from a real necessity: being able to track and retrieve colleagues' SSH keys. Not everybody knows, you can download them from GitLab and GitHub, if they are available.

So, I chose to create a small utility to do the job for me, and especially using the occasion to tries technologies I haven't used before, such Rust and its ecosystem, GitHub Actions and its ecosystem.

It is basically a project to learn new things, including some useful Rust libraries, how to properly test them, how `crates.io` works, and how to build and publish Rust software with GitHub Actions.

Any feedback, in the form of issues and pull requests, is more than welcome!

#### Why the name "Dormarch"?

> Dormarch [...] is a hound, normally used to assist hunters by tracking or chasing the animal that is being hunted.

[From Wikipedia](https://en.wikipedia.org/w/index.php?title=Dormarch&oldid=894865866)

I spend time hunting colleagues' keys, so why not having a helper?
