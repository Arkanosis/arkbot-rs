# arkbot-rs [![](https://img.shields.io/crates/v/arkbot.svg)](https://crates.io/crates/arkbot) [![License](https://img.shields.io/badge/license-ISC-blue.svg)](/LICENSE) [![Build status](https://travis-ci.org/Arkanosis/arkbot-rs.svg?branch=master)](https://travis-ci.org/Arkanosis/arkbot-rs)

**arkbot-rs** is Rust rewrite (and complete redesign) of Arkanosis' Wikipedia bot.

The original Arkbot is made of a mix of Python, sed, awk and wget, glued together with some zsh, which has done the job for more than ten years now, but has never been really efficient.

arkbot-rs is designed from the start to address Arkbot's shortcomings, including:
 - performance,
 - need for manual intervention.

The goal is to have Arkbot running on a small server or VM (eg. on Toolforge) and performing its tasks on its own without any human intervention.

## Usage

```
Usage: arkbot
       arkbot -h | --help
       arkbot --version

Options:
    -h, --help               Show this screen.
    --version                Show version.
```

## Compiling

Run `cargo build --release` in your working copy.

## Contributing and reporting bugs

Contributions are welcome through [GitHub pull requests](https://github.com/Arkanosis/arkbot-rs/pulls).

Please report bugs and feature requests on [GitHub issues](https://github.com/Arkanosis/arkbot-rs/issues).

## License

arkbot-rs is copyright (C) 2020-2021 Jérémie Roquet <jroquet@arkanosis.net> and
licensed under the ISC license.
