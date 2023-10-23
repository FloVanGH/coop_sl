<!--
SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

## `LoadingIndicator`

LoadingIndicator` can be used as overlay to block interaction and to display there is an ongoing loading task.

### Example

```slint
import { LoadingIndicator } from "@coop/lib.slint";

export component Example inherits Window {
    in property <bool> loading;

    width: 150px;
    height: 150px;

    if (root.loading) : LoadingIndicator {}
}
```