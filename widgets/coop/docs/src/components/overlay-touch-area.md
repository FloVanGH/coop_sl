<!--
SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

## `OverlayTouchArea`

A TouchArea with a transparent black background. Properties and callback same as [TouchArea](https://slint.dev/releases/1.2.2/docs/slint/src/language/builtins/elements#toucharea).

### Example

```slint
import { OverlayTouchArea } from "@coop/lib.slint";

export component Example inherits Window {
    width: 200px;
    height: 25px;

    OverlayTouchArea {
        clicked => {
            debug("Clicked on overlay touch area");
        }
    }
}
```