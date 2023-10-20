<!--
SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: GPL-3.0-only
-->

# coop_local

A cross platform file browser build with [Slint](https://slint.dev/).

<a href="https://codeberg.org/flovansl/co_sl">
    <img alt="Get it on Codeberg" src="https://get-it-on.codeberg.org/get-it-on-blue-on-white.png" height="60">
</a>
<a href="https://slint.dev">
    <img alt="#MadeWithSlint" src="https://raw.githubusercontent.com/slint-ui/slint/master/logo//MadeWithSlint-logo-light.svg" height="60">
</a>

[![GNU GPLv3](https://img.shields.io/badge/license-GPLv3-green.svg)](../../LICENSES/GPL-3.0-only.txt)


## Disclaimer

**This is a hobby project and still early in development. It is not recommended to use it in production. Lost of files during usage could be possible.**

## About

The project is mainly a hobby project of myself. I have started it to evaluate a good way for an application architecture with `Slint` and `Rust`.
Now I'm on to create a file browser that fits my personal needs and taste. So it will be including extra features like. a game launcher, music player and image viewer.
Also if there are some features implemented now the project is still in early stage. As mentioned it's mainly a personal project but if someone else is interested in `coop_local` you can check further topics and progress on https://codeberg.org/flovansl/co_sl/issues/33.

 <img alt="coop_local" src="https://codeberg.org/flovansl/pages//attachments/9ba12373-c14d-4016-a875-401b9c3df3d0" width="120" />

## Features

* View, copy, remove files and folders
* View images
* View and edit texts
* Integrated game launcher

## Input

coop_local can be controlled by mouse, touchpad, touchscreen and a game controller.

## Dependencies

### Linux

* To run coop_local on Linux with `games` feature `libudev-dev` mus be installed.

## Run

* Run from root `cargo run -p coop_local`

## License

* Source code an binaries of this projects are available under [GNU GPLv3](../../LICENSES/GPL-3.0-only.txt) license.
