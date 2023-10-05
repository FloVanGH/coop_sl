# SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
# SPDX-License-Identifier: MIT

#!/bin/bash

cargo clean
cargo build -p widgets --no-default-features --features=slint_coop
cargo build -p desktop
