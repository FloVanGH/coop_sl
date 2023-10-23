<!--
SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

## `Slider`

### Properties

-   **`enabled`**: (_in_ _bool_): Defaults to true. You can't interact with the slider if enabled is false.
-   **`has-focus`**: (_out_ _bool_): Set to true when the slider currently has the focus
-   **`value`** (_in-out_ _float_): The value.
-   **`minimum`** (_in_ _float_): The minimum value (default: 0)
-   **`maximum`** (_in_ _float_): The maximum value (default: 100)
-   **`orientation`** (_in_ _enum [`Orientation`](https://slint.dev/releases/1.2.2/docs/slint/src/language/builtins/enums#orientation): If set to true the Slider is displayed vertical (default: horizontal).

### Callbacks

-   **`changed(/* value */ float)`**: The value was changed.

### Example

```slint
import { Slider } from "@coop/lib.slint";

export component Example inherits Window {
    width: 200px;
    height: 25px;
    Slider {
        width: parent.width;
        height: parent.height;
        value: 42;
    }
}
```
