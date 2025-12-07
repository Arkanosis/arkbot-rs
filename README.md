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

## Installing

Run `sudo cp target/release/arkbot /usr/bin/arkbot` in your working copy.

## Configuring

Create `~/.config/arkbot/config.toml` with the following content:

```toml
login = '$BOT_NAME'
password = '$BOT_PASSWORD'
server_url = 'https://fr.wikipedia.org'
script_path = '/w'
output_directory = '/tmp/.arkbot-data'
```

And replace `$BOT_NAME` with your bot account name and `$BOT_PASSWORD` with your bot account password.
Please use a [bot password](https://www.mediawiki.org/wiki/Manual:Bot_passwords) created for arkbot-rs, and not the actual account password.

The `output_directory` will be created, but it's not used anymore and may disappear in a future version.

## Enabling as a systemd service, run hourly

```console
systemctl --user link systemd/arkbot.service systemd/arkbot.timer
systemctl --user daemon-reload
systemctl --user enable --now arkbot.timer
```

Arkbot will then check every hour if there is a new dump available.
If there is one, it will download it and update the pages on the target wiki.

Warning: as of now, the target pages are hardcoded, which makes arkbot-rs only suitable for the French Wikpedia.

## Monitoring

You can check when arkbot-rs history using `systemctl --user list-timers`.

You can read arkbot-rs logs using `journalctl --user -u arkbot -f`.

## Contributing and reporting bugs

Contributions are welcome through [GitHub pull requests](https://github.com/Arkanosis/arkbot-rs/pulls).

Please report bugs and feature requests on [GitHub issues](https://github.com/Arkanosis/arkbot-rs/issues).

## License

arkbot-rs is copyright (C) 2020-2025 Jérémie Roquet <jroquet@arkanosis.net> and
licensed under the ISC license.
