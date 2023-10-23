<!--
SPDX-FileCopyrightText: 2023 Florian Blasius <co_sl@tutanota.com>
SPDX-License-Identifier: MIT
-->

## `RoundOutlineButton`

A simple round button that can nether display an icon or a single character with an outline border.

### Properties

-   **`text`** (_in_ _string_): The text written in the button.
-   **`icon`** (_in_ _image_): The image to show in the button. Note that not all styles support drawing icons.
-   **`primary`** (_in_ _bool_): If set to true the button is displayed with the primary accent color (default: false).
-   **`primary`** (_in_ _bool_): If set to true the button is displayed with the primary accent color (default: false).
-   **`has-focus`**: (_out_ _bool_): Set to true when the button has keyboard focus.
-   **`pressed`**: (_out_ _bool_): Set to true when the button is pressed.
-   **`enabled`**: (_in_ _bool_): Defaults to true. When false, the button cannot be pressed

### Callbacks

-   **`clicked()`**

### Example

```slint
import { RoundOutlineButton } from "@coop/lib.slint";

export component Example inherits Window {
    VerticalBox {
        RoundOutlineButton {
            text: "C";
            clicked => { self.text = "Clicked"; }
        }
    }
}
```