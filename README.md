<!--
SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

# co_sl

This repository is the home of my personal [Slint](https://slint-ui.com/) projects. It contains a custom library with a set of default widgets, complex custom widgets, additional platforms supports, an experimental game engine and some apps.

What the `co` stands for: cooperation.

<a href="https://codeberg.org/flovansl/co_sl">
    <img alt="Get it on Codeberg" src="https://get-it-on.codeberg.org/get-it-on-blue-on-white.png" height="40">
</a>

[![CI](https://ci.codeberg.org/api/badges/flovansl/co_sl/status.svg?branch=main)](https://ci.codeberg.org/flovansl/co_sl)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSES/MIT.txt)
[![GNU GPLv3](https://img.shields.io/badge/license-GPLv3-green.svg)](./LICENSES/GPL-3.0-only.txt)

## crates

* apps/
    * [co_calc](apps/co_calc/): Calculator build with Slint **(not yet started)**
    * [co_chat](apps/co_chat/): A visual Slint prototype of a chat app **(not yet started)**
    * [co_files](apps/co_files/): File navigation / explorer build with Slint **(not yet started)**
* platform_support/
    * [slint_psp](platform_support/slint_psp/): Slint support for the PSP gaming console **(in active development / pre-release)**
    * [slint_orbclient](platform_support/slint_orbclient/): Slint platform implementation based on [OrbClient](https://gitlab.redox-os.org/redox-os/orbclient), can be run on [Redox](https://redox-os.org/) **(in active development / pre-release)**
* gaming/
    * [pico_engine](gaming/pico_engine/): experimental engine based on slint with no_std support **(not yet started)**
    * [pico_ecs](gaming/pico_ecs/): experimental Entity Component System library with support for no_std **(not yet started)**
    * [pico_designer](gaming/pico_designer/): experimental map editor for the `pico_engine` **(not yet started)**
* widgets/
    * [book_flip](widgets/book_flip/): e-book widget for Slint **(not yet started)**
    * [co_widgets](widgets/co_widgets/): custom default widget library for Slint **(in active development / pre-release)**

## examples

* [gaming](examples/gaming/): example of `pico_engine` and `pico_ecs`
* [widgets](examples/widgets/): gallery example showing the widget of `book_flip` and `co_widget`

> Check out the [README](examples/README.md) for more details.

### live wasm preview of examples

| Widgets |
|---------|
|[![Screenshot of the Widgets Demo](https://codeberg.org/flovansl/pages/attachments/2501a785-2b21-40d8-91c7-85fee14f0045 "Widgets Demo")](https://flovansl.codeberg.page/snapshots/widgets/) |


## new to slint?

Best to start with these sources:

* getting start: https://slint-ui.com/#tryout
* Slint docs (*Slint lang docs included*): https://docs.rs/slint/latest/slint/
* Slint examples: https://github.com/slint-ui/slint/tree/master/examples
* Slint chat: https://chat.slint-ui.com/

## license

* `libs` and `examples` are available under [MIT license](LICENSE-MIT) and `apps` under [GNU GPLv3](LICENSE-GPL3).
 * `Slint` is available under either a [commercial license](https://github.com/slint-ui/slint/blob/master/LICENSES/LicenseRef-Slint-commercial.md)
or [GNU GPLv3](https://github.com/slint-ui/slint/blob/master/LICENSES/GPL-3.0-only.txt).

## contributions

For contributions check [CONTRIBUTING.md](./CONTRIBUTING.md).