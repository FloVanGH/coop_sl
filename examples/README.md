# Slint Extensions examples

Examples for the crates `pico_engine`, `pico_ecs`, `co_widgets` and `book_flip`.

<a href="https://codeberg.org/morphUI/morph_ui">
    <img alt="Get it on Codeberg" src="https://pages.codeberg.org/pstorch/get-it-on-blue-on-white.png" height="40">
</a>

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](../LICENSE)

## gaming

Example game for `pico_engine` and `pico_ecs`. It can be run on multiple platforms.

### Run gaming

* On desktop: from repo root run `cargo run -p gaming`

## widgets

Example gallery that shows all widgets of `co_widgets` and `book_flip`. It can be run on multiple platforms.

### Run widgets

* On desktop: from repo root run `cargo run -p widgets`
* On Raspberry Pi Pico with st7789 display: from repo root run ```cargo +nightly build -p widgets --no-default-features --features=mcu-board-support/pico-st7789 --target=thumbv6m-none-eabi --release && elf2uf2-rs -d target/thumbv6m-none-eabi/release/widgets```