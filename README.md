# slint_extensions

This repository contains all my personal extension crates for [Slint](https://slint-ui.com/). It contains additional platforms supports, an experimental game engine, custom widget sets and a custom default widget library.

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

## crates

* platform_support/
    * **slint_psp**: Slint support for the PSP gaming console
    * **slint_redox**: Slint support for Redox OS
* gaming/
    * **pico_engine**: experimental engine based on slint with no_std support
    * **pico_ecs**: experimental Entity Component System library with support for no_std
* widgets/
    * **book_flip**: e-book widget for Slint
    * **co_widgets**`**: custom default widget library for Slint

## examples

* **gaming**: example or `pico_engine` and `pico_ecs`
* **platform_support**: example show how to use Slint on additional platforms
* **widgets**: gallery example showing the widget of `book_flip` and `co_widget`

## notes


`Slint` is available under either a [commercial license](https://github.com/slint-ui/slint/blob/master/LICENSES/LicenseRef-Slint-commercial.md)
or [GNU GPLv3](https://github.com/slint-ui/slint/blob/master/LICENSES/GPL-3.0-only.txt).