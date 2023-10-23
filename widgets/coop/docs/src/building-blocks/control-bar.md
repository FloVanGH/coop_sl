<!--
SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

## `ControlBar`

A ControlBar represents a horizontal layout with a background. The intention is to use it to create a application control bar with buttons.

### Properties

-   **`alignment`**: (_in_ _[LayoutAlignment](https://slint.dev/releases/1.2.2/docs/slint/src/language/builtins/enums#layoutalignment)_): Defines the alignment of the layout (default center).

### Example

```slint
import { ControlBar, RoundButton, Icons } from "@coop/lib.slint";

export component Example inherits Window {
    width: 200px;
    height: 25px;

   ControlBar {
        RoundButton {
            icon: Icons.add;
        }

        RoundButton {
            icon: Icons.add;
        }
   }
}
```