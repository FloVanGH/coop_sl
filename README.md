<!--
SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

# coop_sl

This repository is the home of my personal [Slint](https://slint.dev/) projects. It contains a custom library with a set of default widgets, complex custom widgets, additional platforms supports, an experimental game engine and some apps.

What the `Coop` stands for: cooperation.

<a href="https://codeberg.org/flovansl/coop_sl">
    <img alt="Get it on Codeberg" src="https://get-it-on.codeberg.org/get-it-on-blue-on-white.png" height="40">
</a>

[![CI](https://ci.codeberg.org/api/badges/flovansl/co_sl/status.svg?branch=main)](https://ci.codeberg.org/flovansl/co_sl)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSES/MIT.txt)
[![GNU GPLv3](https://img.shields.io/badge/license-GPLv3-green.svg)](./LICENSES/GPL-3.0-only.txt)

## Crates

* apps/
    * [coop_local](apps/coop_local/): Cross platform file browser **(in active development / pre-release)**
* display/
    * [coop_display](display/README.md) is an experimental frame buffer based display server that can be used e.g. to render a standalone app inside of a Slint app.
    *  [coop_client](display/coop_client/): Client library that can be used to make an app work with `coop_server` **(on hold)**
    *  [coop_server](display/coop_server/): Server part of the display server, can handle multiple `coop_client` apps **(on hold)**
    *  [coop_protocol](display/coop_protocol/): Protocol enums and structs used for communication between `coop_client` and `coop_server` apps **(on hold)**
* platform_support/
    * [slint_coop](platform_support/slint_coop/): Slint platform implementation based on `coop_client` **(on hold)**
* widgets/
    * [book_flip](widgets/book_flip/): e-book widget for Slint **(on hold)**
    * [coop_widgets](widgets/coop_widgets/): `coop_widgets` is a custom widget and component library for [Slint](https://slint.dev/) with a custom simple, consistence and clean design. **(in active development / pre-release)**

## examples

* [desktop](examples/desktop/): experimental example desktop environment based on `coop_display` and Slint
* [gallery](examples/gallery/): gallery example showing the widget of `book_flip` and `co_widget`

> Check out the [README](examples/README.md) for more details.

### Live wasm preview of examples

| Widgets |
|---------|
|[![Screenshot of the Widgets Demo](https://codeberg.org/flovansl/pages/attachments/2501a785-2b21-40d8-91c7-85fee14f0045 "Widgets Demo")](https://flovansl.codeberg.page/coop_sl/snapshots/main/demos/gallery/) |


## New to Slint?

Best to start with these sources:

* Slint docs: https://slint.dev/docs
* Slint examples: https://github.com/slint-ui/slint/tree/master/examples
* Slint chat: https://chat.slint-ui.com/

## License

* Source code of library crates and examples are available under [MIT license](LICENSES/MIT.txt) and source code of application projects `apps` under [GNU GPLv3](LICENSES/GPL-3.0-only.txt).

## contributions

For contributions check [CONTRIBUTING.md](./CONTRIBUTING.md).