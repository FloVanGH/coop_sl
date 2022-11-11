# Slint Extensions examples

Examples for the crates `pico_engine`, `pico_ecs`, `co_widgets` and `book_flip`.

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](../LICENSE)

## gaming

Example how to use `pico_engine` and `pico_ecs`. It can be run on multiple platforms.

### Run gaming

* On desktop: from repo root run `cargo run -p gaming`

## widgets

Example gallery that shows all widgets of `co_widgets` and `book_flip`. It can be run on multiple platforms.

### Run widgets

* On desktop: from repo root run `cargo run -p widgets`
* On Raspberry Pi Pico with st7789 display: from repo root run ```cargo +nightly build -p widgets --no-default-features --features=mcu-board-support/pico-st7789 --target=thumbv6m-none-eabi --release && elf2uf2-rs -d target/thumbv6m-none-eabi/release/widgets```