# Slint Extensions examples

## Gaming

Example how to use `pico_engine` and `pico_ecs`.

## Platform support

Example that can be run on different platforms e.g. `PSP` and `Redox OS`.

# Widgets

Example gallery that shows all widgets of `co_widgets` and `book_flip`. 

## Run on pico

```cargo +nightly build -p widgets --no-default-features --features=mcu-board-support/pico-st7789 --target=thumbv6m-none-eabi --release && elf2uf2-rs -d target/thumbv6m-none-eabi/release/widgets```