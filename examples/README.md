<!--
SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

# coop_sl examples

Examples for `coop_widgets` and `book_flip`.

<a href="https://codeberg.org/flovansl/coop_sl">
    <img alt="Get it on Codeberg" src="https://get-it-on.codeberg.org/get-it-on-blue-on-white.png" height="40">
</a>

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](../LICENSES/MIT.txt)

## desktop

Experimental example desktop environment based on `coop_display` and Slint.

### Run desktop

* Desktop example needs `gallery` to be build first with `slint_coop` features
* Build widgets: `cargo build -p gallery --no-default-features --features=slint_coop`
* Run desktop:  `cargo run -p desktop`

## gallery

Example gallery that shows all widgets of `coop_widgets` and `book_flip`. It can be run on multiple platforms.

## live wasm preview

| Gallery |
|---------|
|[![Screenshot of the Widgets Demo](https://codeberg.org/flovansl/pages/attachments/2501a785-2b21-40d8-91c7-85fee14f0045 "Gallery")](https://flovansl.codeberg.page/coop_sl/snapshots/main/demos/gallery/) |

### Run widgets

* On desktop: from repo root run `cargo run -p gallery`
* On Raspberry Pi Pico with st7789 display: from repo root run ```cargo +nightly build -p gallery --no-default-features --features=mcu-board-support/pico-st7789 --target=thumbv6m-none-eabi --release && elf2uf2-rs -d target/thumbv6m-none-eabi/release/gallery```