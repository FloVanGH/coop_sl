<!--
SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

# coop_sl

This repository is the home of my personal [Slint](https://slint-ui.com/) projects. It contains a custom library with a set of default widgets, complex custom widgets, additional platforms supports, an experimental game engine and some apps.

What the `coop` stands for: cooperation.

<a href="https://codeberg.org/flovansl/coop_sl">
    <img alt="Get it on Codeberg" src="https://get-it-on.codeberg.org/get-it-on-blue-on-white.png" height="40">
</a>

[![CI](https://ci.codeberg.org/api/badges/flovansl/coop_sl/status.svg?branch=main)](https://ci.codeberg.org/flovansl/coop_sl)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSES/MIT.txt)
[![GNU GPLv3](https://img.shields.io/badge/license-GPLv3-green.svg)](./LICENSES/GPL-3.0-only.txt)

## crates

* apps/
    * [coop_calc](apps/coop_calc/): Calculator build with Slint **(not yet started)**
    * [coop_chat](apps/coop_chat/): A visual Slint prototype of a chat app **(in active development / prototyping)**
    * [coop_files](apps/coop_files/): File navigation / explorer build with Slint **(not yet started)** 
* display/
    * **About:** [coop_display](display/README.md) is an experimental frame buffer based display server that can be used e.g. to render a standalone app inside of a Slint app.
    *  [coop_client](display/coop_client/): Client library that can be used to make an app work with `coop_server` **(in active development / prototyping)**
    *  [coop_server](display/coop_server/): Server part of the display server, can handle multiple `coop_client` apps **(in active development / prototyping)**
    *  [coop_protocol](display/coop_protocol/): Protocol enums and structs used for communication between `coop_client` and `coop_server` apps **(in active development / prototyping)**
   
* platform_support/
    * [slint_coop](platform_support/slint_coop/): Slint platform implementation based on `coop_client` **(in active development / prototyping)**
    * [slint_orbclient](platform_support/slint_orbclient/): Slint platform implementation based on [OrbClient](https://gitlab.redox-os.org/redox-os/orbclient), can be run on [Redox](https://redox-os.org/) **(in active development / pre-release)**
* gaming/
    * [pico_engine](gaming/pico_engine/): experimental engine based on slint with no_std support **(not yet started)**
    * [pico_designer](gaming/pico_designer/): experimental map editor for the `pico_engine` **(not yet started)**
* widgets/
    * [book_flip](widgets/book_flip/): e-book widget for Slint **(not yet started)**
    * [coop_widgets](widgets/coop_widgets/): custom default widget library for Slint **(in active development / pre-release)**

## examples

* [desktop](examples/desktop/): experimental example desktop environment based on `coop_display` and Slint
* [gaming](examples/gaming/): example of `pico_engine`
* [widgets](examples/widgets/): gallery example showing the widget of `book_flip` and `co_widget`

> Check out the [README](examples/README.md) for more details.

### live wasm preview of examples

| Widgets |
|---------|
|[![Screenshot of the Widgets Demo](https://codeberg.org/flovansl/pages/attachments/2501a785-2b21-40d8-91c7-85fee14f0045 "Widgets Demo")](https://flovansl.codeberg.page/coop_sl/snapshots/examples/widgets/) |


## new to slint?

Best to start with these sources:

* getting start: https://slint-ui.com/#tryout
* Slint docs (*Slint lang docs included*): https://docs.rs/slint/latest/slint/
* Slint examples: https://github.com/slint-ui/slint/tree/master/examples
* Slint chat: https://chat.slint-ui.com/

## license

* `libs` and `examples` are available under [MIT license](../../LICENSES/MIT.txt) and `apps` under [GNU GPLv3](../../LICENSES/GPL-3.0-only.txt).
 * `Slint` is available under either a [commercial license](https://github.com/slint-ui/slint/blob/master/LICENSES/LicenseRef-Slint-commercial.md)
or [GNU GPLv3](https://github.com/slint-ui/slint/blob/master/LICENSES/GPL-3.0-only.txt).

## contributions

For contributions check [CONTRIBUTING.md](./CONTRIBUTING.md).