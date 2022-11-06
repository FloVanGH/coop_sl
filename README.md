# slint_extensions

This repository is the home of my personal [Slint](https://slint-ui.com/) projects. It contains a custom library with a set of default widgets, complex custom widgets, additional platforms supports, an experimental game engine and some apps. This is not an official channel of Slint.

<a href="https://codeberg.org/morphUI/morph_ui">
    <img alt="Get it on Codeberg" src="https://pages.codeberg.org/pstorch/get-it-on-blue-on-white.png" height="60">
</a>

[![CI](https://ci.codeberg.org/api/badges/flovansl/slint_extensions/status.svg?branch=main)](https://ci.codeberg.org/flovansl/slint_extensions)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

## crates

* apps/
    * **pico_os**: A fake os / desktop environment as fancy showcase build with Slint **(not yet started)**
    * **van_calc**: Calculator build with Slint **(not yet started)**
    * **van_files**: File navigation / explorer build with Slint **(not yet started)**
* platform_support/
    * **slint_psp**: Slint support for the PSP gaming console **(in active development / pre-release)**
    * **slint_redox**: Slint support for Redox OS **(in active development / pre-release)**
* gaming/
    * **pico_engine**: experimental engine based on slint with no_std support **(not yet started)**
    * **pico_ecs**: experimental Entity Component System library with support for no_std **(not yet started)**
    * **pico_designer**: experimental map editor for the `pico_engine` **(not yet started)**
* widgets/
    * **book_flip**: e-book widget for Slint **(not yet started)**
    * **co_widgets**: custom default widget library for Slint **(in active development / pre-release)**

## examples

* **gaming**: example of `pico_engine` and `pico_ecs`
* **widgets**: gallery example showing the widget of `book_flip` and `co_widget`

> Check out the [README](examples/README.md) for more details.

# new to slint?

* getting start: ...
* Slint language docs: ...
* Slint examples: ...
* Slint mattermost: ...

## getting start with Slint

* Getting start: https://slint-ui.com/#tryout
* Language documentation: https://github.com/slint-ui/slint/tree/master/docs


## license

 * `Slint` is available under either a [commercial license](https://github.com/slint-ui/slint/blob/master/LICENSES/LicenseRef-Slint-commercial.md)
or [GNU GPLv3](https://github.com/slint-ui/slint/blob/master/LICENSES/GPL-3.0-only.txt).
* All source code of this repository is available under [MIT license](LICENSE)