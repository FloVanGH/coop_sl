# Slint Extensions examples

## Gaming

Example how to use `pico_engine` and `pico_ecs`.

## Platform support

Example that can be run on different platforms e.g. `PSP` and `Redox OS`.

# Widgets

Example gallery that shows all widgets of `co_widgets` and `book_flip`. 

## Run on pico

```cargo +nightly build -p widgets --target=thumbv6m-none-eabi --no-default-features --features=pico --release && elf2uf2-rs -d target/thumbv6m-none-eabi/release/widgets```