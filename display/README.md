<!--
SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

# coop_display

`coop_display` is a frame buffer based display server that uses inter process communication (IPC) for communication between server and clients.

<a href="https://codeberg.org/flovansl/co_sl">
    <img alt="Get it on Codeberg" src="https://get-it-on.codeberg.org/get-it-on-blue-on-white.png" height="40">
</a>


[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](../LICENSES/MIT.txt)
## crates

*  [coop_client](coop_client/): Client library that can be used to make an app work with `coop_server` **(in active development / experimental)**
*  [coop_server](coop_server/): Server part of the display server, can handle multiple `coop_client` apps **(in active development / experimental)**
*  [coop_protocol](coop_protocol/): Protocol enums and structs used for communication between `coop_client` and `coop_server` apps **(in active development / experimental)**
*  [slint_coop](slint_coop/): Slint platform implementation based on `coop_client` **(in active development / experimental)**