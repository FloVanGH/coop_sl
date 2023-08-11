<!--
SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: GPL-3.0-only
-->

# coop_local

A cross platform file browser build with [Slint](https://slint.dev/).

[![GNU GPLv3](https://img.shields.io/badge/license-GPLv3-green.svg)](../../LICENSES/GPL-3.0-only.txt)

## Run

* Run from root `cargo run -p coop_local`

## Development status

I have started `coop_local` as PoC and to find a way to implement a good application architecture with `Slint` and `Rust`. From my perspective
the project is progressing well and I will continue my work to create a usable file browser. Nevertheless the development of `coop_local` is in
early stage and some things need to improved e.g. to do file operations with file descriptors instead of paths. Open topics are tracked on this issue https://codeberg.org/flovansl/co_sl/issues/33.